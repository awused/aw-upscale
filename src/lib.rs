use std::error::Error;
use std::fmt::Display;
use std::io::{stderr, Write};
#[cfg(target_family = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::time::Duration;

use derive_more::From;
use process_control::{ChildExt, Control, Output};

#[cfg(target_family = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// enum UpscaleTarget {
//     None,
//     Scale(u8),
//     Res {
//         target_width: Option<u32>,
//         target_height: Option<u32>,
//         min_width: Option<u32>,
//         min_height: Option<u32>,
//     },
// }

// TODO -- redo with a nicer enum
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

#[derive(From)]
pub enum UpscaleError {
    /// The destination format must be PNG
    DestinationNotPng,
    /// The program failed to start or complete.
    ProcessError(std::io::Error),
    /// The program completed unsuccessfully.
    ExitError(Output),
    /// The output from the program was not in the correct format.
    InvalidOutput(Vec<u8>),
    /// The program failed to complete within the specified time and was killed.
    Timeout,
}

#[cfg(unix)]
mod tostr {
    use std::ffi::OsStr;
    use std::os::unix::prelude::OsStrExt;

    pub fn convert(v: &[u8]) -> &OsStr {
        OsStr::from_bytes(v)
    }
}

#[cfg(windows)]
mod tostr {
    use std::str::from_utf8;

    pub fn convert(v: &[u8]) -> String {
        match from_utf8(v) {
            Ok(s) => s.to_owned(),
            // Can't be bothered to decode UTF-16, at least for now.
            Err(_e) => format!("{v:?}"),
        }
    }
}


impl std::fmt::Debug for UpscaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DestinationNotPng => write!(f, "DestinationNotPng"),
            Self::ProcessError(arg0) => f.debug_tuple("ProcessError").field(arg0).finish(),
            Self::ExitError(arg0) => f
                .debug_tuple("ExitError")
                .field(&arg0.status)
                .field(&tostr::convert(&arg0.stdout))
                .field(&tostr::convert(&arg0.stderr))
                .finish(),
            Self::InvalidOutput(arg0) => {
                f.debug_tuple("InvalidOutput").field(&tostr::convert(arg0)).finish()
            }
            Self::Timeout => write!(f, "Timeout"),
        }
    }
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
        Self { executable: upscaler, ..Self::default() }
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

        let mut spawned = cmd.spawn()?;


        if let Some(mut stdin) = spawned.stdin.take() {
            if let Err(e) = stdin.write_all(DEFAULT_UPSCALER) {
                // Attempt to kill the child process, but ignore any failures.
                // Use wait_with_output to avoid deadlocks if the process is blocked on sending to
                // stdout.
                drop(spawned.kill());
                drop(spawned.wait_with_output());
                return Err(e.into());
            }
        }

        let output = if let Some(timeout) = self.timeout {
            match spawned
                .controlled_with_output()
                .time_limit(timeout)
                .terminate_for_timeout()
                .wait()?
            {
                Some(output) => output,
                None => return Err(UpscaleError::Timeout),
            }
        } else {
            spawned.wait_with_output().map(Into::into)?
        };

        if !output.status.success() {
            drop(stderr().write_all(&output.stderr));

            return Err(output.into());
        }

        let res = (|| {
            let outstr = from_utf8(&output.stdout).ok()?;
            let (w, h) = outstr.trim().split_once('x')?;

            Some((w.parse::<u32>().ok()?, h.parse::<u32>().ok()?))
        })();

        match res {
            Some(res) => Ok(res),
            None => Err(output.stdout.into()),
        }
    }
}
