package upscale

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

// Upscaler wraps an external executable that can upscale images.
// It can be reused multiple times with the same parameters.
type Upscaler struct {
	useDefault                bool
	executable                string
	scale                     int
	targetWidth, targetHeight int
	minWidth, minHeight       int
	denoise                   bool
}

// New returns a new upscaler that will use the provided executable.
// If the provided executable is empty, it will use the default
// embedded python script.
func New(executable string) *Upscaler {
	return &Upscaler{
		useDefault: executable == "",
		executable: executable,
	}
}

// SetScale sets the multiplication factor that will be applied to all the
// dimensions of the image. This unsets the target resolution.
func (u *Upscaler) SetScale(scale int) *Upscaler {
	u.scale = scale
	u.targetWidth, u.targetHeight = 0, 0
	u.minWidth, u.minHeight = 0, 0
	return u
}

// SetTargetRes specifies the resolution to target using the "fit to container"
// strategy. After scaling the image's width OR height will be at least the width or
// height specified.
// If either width or height is zero, they will be ignored.
// This unsets the scale setting.
func (u *Upscaler) SetTargetRes(width, height int) *Upscaler {
	u.scale = 0
	u.targetWidth, u.targetHeight = width, height
	return u
}

// SetMinRes specifies the resolution to target using the "fill container"
// strategy. After scaling the image's width AND height will BOTH be at least the width or
// height specified.
// If either width or height is zero, they will be ignored.
// This unsets the scale setting.
func (u *Upscaler) SetMinRes(width, height int) *Upscaler {
	u.scale = 0
	u.minWidth, u.minHeight = width, height
	return u
}

// SetDenoise sets whether denoising is to be done on the image.
// It's possible that a particular upscaler may not support this.
func (u *Upscaler) SetDenoise(denoise bool) *Upscaler {
	u.denoise = denoise
	return u
}

// Process will apply the configured operations to the source image and create
// the destination image.
func (u *Upscaler) Process(source, destination string) error {
	ext := strings.ToLower(filepath.Ext(destination))
	if ext != ".png" {
		return errors.New("Destination file must be png")
	}

	if u.scale < 0 || u.targetWidth < 0 || u.targetHeight < 0 || u.minWidth < 0 || u.minHeight < 0 {
		return errors.New(
			"Scale, height, and width cannot be negative")
	}

	if u.scale > 0 && (u.targetWidth > 0 || u.targetHeight > 0 || u.minWidth > 0 || u.minHeight > 0) {
		// This should never happen
		return errors.New(
			"Cannot specify scaling factor alongside target width or height")
	}

	var cmd *exec.Cmd
	if u.executable != "" {
		cmd = exec.Command(u.executable)
	} else {
		cmd = exec.Command("python", "-c", defaultUpscaler)
	}
	cmd.SysProcAttr = sysProcAttr
	cmd.Env = os.Environ()
	addEnv(cmd, "UPSCALE_SOURCE", source)
	addEnv(cmd, "UPSCALE_DESTINATION", destination)

	if u.scale != 0 {
		addEnv(cmd, "UPSCALE_SCALING_FACTOR", strconv.Itoa(u.scale))
	}
	if u.targetWidth != 0 || u.targetHeight != 0 {
		addEnv(cmd, "UPSCALE_TARGET_WIDTH", strconv.Itoa(u.targetWidth))
		addEnv(cmd, "UPSCALE_TARGET_HEIGHT", strconv.Itoa(u.targetHeight))
	}

	if u.minWidth != 0 || u.minHeight != 0 {
		addEnv(cmd, "UPSCALE_MIN_WIDTH", strconv.Itoa(u.minWidth))
		addEnv(cmd, "UPSCALE_MIN_HEIGHT", strconv.Itoa(u.minHeight))
	}

	if u.denoise {
		addEnv(cmd, "UPSCALE_DENOISE", "true")
	}

	out, err := cmd.Output()
	if err != nil {
		return err
	}

	outstr := string(out)
	outstr = strings.TrimSpace(outstr)

	_, err = os.Stat(destination)
	return err
}

func addEnv(cmd *exec.Cmd, k, v string) {
	cmd.Env = append(cmd.Env, k+"="+v)
}
