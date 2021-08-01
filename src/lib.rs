use std::error::Error;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::from_utf8;

#[derive(Default, Debug)]
pub struct Upscaler {
    executable: Option<String>,
    scale: Option<u8>,
    width: Option<u32>,
    height: Option<u32>,
    denoise: bool,
    fill: bool,
}

#[derive(Debug)]
pub enum UpscaleError {
    DestinationNotPng,
    ProcessError(Box<dyn Error>),
    IncorrectOutput,
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
    pub fn new(upscaler: Option<String>) -> Self {
        Self {
            executable: upscaler,
            ..Default::default()
        }
    }

    pub fn set_scale(&mut self, scale: u8) -> &mut Self {
        self.scale = Some(scale);
        self.width = None;
        self.height = None;
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.scale = None;
        self.height = Some(height);
        self
    }

    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.scale = None;
        self.width = Some(width);
        self
    }

    pub fn set_denoise(&mut self, denoise: bool) -> &mut Self {
        self.denoise = denoise;
        self
    }

    pub fn set_fill(&mut self, fill: bool) -> &mut Self {
        self.fill = fill;
        self
    }

    /// Run the configred upscaler to convert from `source` to `destination`.
    ///
    /// `destination` must be a PNG.
    pub fn run<P: AsRef<Path>>(
        &self,
        source: P,
        destination: P,
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

        cmd.stdout(Stdio::piped());
        cmd.env("UPSCALE_SOURCE", source.as_ref());
        cmd.env("UPSCALE_DESTINATION", destination.as_ref());

        if let Some(scale) = self.scale {
            cmd.env("UPSCALE_SCALING_FACTOR", scale.to_string());
        }

        if let Some(height) = self.height {
            cmd.env("UPSCALE_TARGET_HEIGHT", height.to_string());
        }

        if let Some(width) = self.width {
            cmd.env("UPSCALE_TARGET_WIDTH", width.to_string());
        }

        if self.fill {
            cmd.env("UPSCALE_TARGET_FILL", "true");
        }

        if self.denoise {
            cmd.env("UPSCALE_DENOISE", "true");
        }

        let mut spawned = match cmd.spawn() {
            Ok(sp) => sp,
            Err(e) => return Err(UpscaleError::ProcessError(Box::from(e))),
        };


        if let Some(mut stdin) = spawned.stdin.take() {
            if let Err(e) = stdin.write_all(DEFAULT_UPSCALER) {
                return Err(UpscaleError::ProcessError(Box::from(e)));
            }
        }


        let output = match spawned.wait_with_output() {
            Ok(out) => out,
            Err(e) => return Err(UpscaleError::ProcessError(Box::from(e))),
        };

        if !output.status.success() {
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
