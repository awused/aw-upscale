// +build windows

package upscale

import "syscall"

var sysProcAttr = &syscall.SysProcAttr{HideWindow: true}
