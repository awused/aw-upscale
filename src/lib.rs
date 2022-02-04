use std::error::Error;
use std::fmt::Display;
use std::io::{stderr, Write};
#[cfg(target_family = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::time::Duration;

use process_control::{ChildExt, Control};

#[cfg(target_family = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;


#[derive(Default, Debug, Clone)]
pub struct Upscaler {
    executable: Option<PathBuf>,
    scale: Option<u8>,
    target_width: Option<u32>,
    target_height: Option<u32>,
    min_width: Option<u32>,
    min_height: Option<u32>,
    denoise: Option<i32>,
    timeout: Option<Duration>,
}

#[derive(Debug)]
pub enum UpscaleError {
    DestinationNotPng,
    ProcessError(Box<dyn Error>),
    IncorrectOutput,
    Timeout,
}

impl Display for UpscaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for UpscaleError {}

const DEFAULT_UPSCALER: &[u8] = include_bytes!("../waifu2x-upscale.py");

impl Upscaler {
    /// Create a new upscaler using the given executable. If none, will use an embedded python
    /// script to call waifu2x-ncnn-vulkan.
    #[must_use]
    pub fn new(upscaler: Option<PathBuf>) -> Self {
        Self {
            executable: upscaler,
            ..Self::default()
        }
    }

    /// Sets the scale for the upscaler, overriding any previously set widths or heights.
    /// The image should be upscaled at least this much, but it could potentially be by a greater
    /// scale.
    pub fn set_scale(&mut self, scale: u8) -> &mut Self {
        self.scale = Some(scale);
        self.target_width = None;
        self.target_height = None;
        self.min_width = None;
        self.min_height = None;
        self
    }

    /// Sets the target height, overriding any previously set scale.
    /// The image will be upscaled to be at least this tall OR at least as wide as any target width.
    pub fn set_target_height(&mut self, height: u32) -> &mut Self {
        self.scale = None;
        self.target_height = Some(height);
        self
    }

    /// Sets the target width, overriding any previously set scale.
    /// The image will be upscaled to be at least this wide OR at least as tall as any
    /// target height.
    pub fn set_target_width(&mut self, width: u32) -> &mut Self {
        self.scale = None;
        self.target_width = Some(width);
        self
    }

    /// Sets the minimum height, overriding any previously set scale.
    /// The image will be upscaled to be at least this tall AND at least as wide as any minimum
    /// width.
    pub fn set_min_height(&mut self, height: u32) -> &mut Self {
        self.scale = None;
        self.min_height = Some(height);
        self
    }

    /// Sets the minimum width, overriding any previously set scale.
    /// The image will be upscaled to be at least this wide AND at least as tall as any minimum
    /// height.
    pub fn set_min_width(&mut self, width: u32) -> &mut Self {
        self.scale = None;
        self.min_width = Some(width);
        self
    }

    /// Sets the denoise level.
    /// Implementations are free to use a reasonable default when this is not set, and the exact
    /// meaning is implementation specific.
    pub fn set_denoise(&mut self, denoise: Option<i32>) -> &mut Self {
        self.denoise = denoise;
        self
    }

    /// Sets the timeout for the process.
    /// This is only intended to kill stuck processes rather than allowing them to hang indefinitely
    /// and should be set generously.
    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Run the configured upscaler to convert from `source` to `destination`.
    ///
    /// `destination` must be a PNG.
    pub fn run<P: AsRef<Path>, T: AsRef<Path>>(
        &self,
        source: P,
        destination: T,
    ) -> Result<(u32, u32), UpscaleError> {
        if let Some(ext) = destination.as_ref().extension() {
            if ext.to_ascii_lowercase() != "png" {
                return Err(UpscaleError::DestinationNotPng);
            }
        } else {
            return Err(UpscaleError::DestinationNotPng);
        }

        let mut cmd = if let Some(exe) = &self.executable {
            Command::new(exe)
        } else {
            let mut cmd = Command::new("python");
            cmd.arg("-").stdin(Stdio::piped());
            cmd
        };

        #[cfg(target_family = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.env("UPSCALE_SOURCE", source.as_ref());
        cmd.env("UPSCALE_DESTINATION", destination.as_ref());

        if let Some(scale) = self.scale {
            cmd.env("UPSCALE_SCALING_FACTOR", scale.to_string());
        }

        if let Some(height) = self.target_height {
            cmd.env("UPSCALE_TARGET_HEIGHT", height.to_string());
        }

        if let Some(width) = self.target_width {
            cmd.env("UPSCALE_TARGET_WIDTH", width.to_string());
        }

        if let Some(height) = self.min_height {
            cmd.env("UPSCALE_MIN_HEIGHT", height.to_string());
        }

        if let Some(width) = self.min_width {
            cmd.env("UPSCALE_MIN_WIDTH", width.to_string());
        }

        if let Some(denoise) = self.denoise {
            cmd.env("UPSCALE_DENOISE", denoise.to_string());
        }

        if let Some(timeout) = self.timeout {
            cmd.env("UPSCALE_TIMEOUT", timeout.as_secs_f64().to_string());
        }

        let mut spawned = match cmd.spawn() {
            Ok(sp) => sp,
            Err(e) => return Err(UpscaleError::ProcessError(Box::from(e))),
        };


        if let Some(mut stdin) = spawned.stdin.take() {
            if let Err(e) = stdin.write_all(DEFAULT_UPSCALER) {
                // Attempt to kill the child process, but ignore any failures.
                // Use wait_with_output to avoid deadlocks if the process is blocked on sending to
                // stdout.
                drop(spawned.kill());
                drop(spawned.wait_with_output());
                return Err(UpscaleError::ProcessError(Box::from(e)));
            }
        }

        let output = if let Some(timeout) = self.timeout {
            match spawned
                .controlled_with_output()
                .time_limit(timeout)
                .terminate_for_timeout()
                .wait()
            {
                Ok(Some(output)) => Ok(output),
                Ok(None) => return Err(UpscaleError::Timeout),
                Err(e) => Err(e),
            }
        } else {
            spawned.wait_with_output().map(Into::into)
        };

        let output = match output {
            Ok(out) => out,
            Err(e) => return Err(UpscaleError::ProcessError(Box::from(e))),
        };

        if !output.status.success() {
            drop(stderr().write_all(&output.stderr));
            return Err(UpscaleError::ProcessError(output.status.to_string().into()));
        }

        let res = (|| {
            let outstr = from_utf8(&output.stdout).ok()?;
            let (w, h) = outstr.trim().split_once('x')?;

            Some((w.parse::<u32>().ok()?, h.parse::<u32>().ok()?))
        })();

        match res {
            Some(res) => Ok(res),
            None => Err(UpscaleError::ProcessError(
                "Invalid output".to_string().into(),
            )),
        }
    }
}
