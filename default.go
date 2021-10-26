package upscale

import (
	// Embed files
	_ "embed"
)

//go:embed waifu2x-upscale.py
var defaultUpscaler string
