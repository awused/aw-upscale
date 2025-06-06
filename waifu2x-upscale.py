#! /usr/bin/env python

# waifu2x-upscale.py upscales an image file.
# Installation: copy this script somewhere and edit these lines.

# This implementation uses waifu2x-ncnn-vulkan and imagemagick.
# Python and imagemagick must be installed and present on your PATH.
# https://github.com/nihui/waifu2x-ncnn-vulkan
# https://imagemagick.org/script/download.php

# The waifu2x executable.
# If this is not on your PATH, edit it to the absolute path to the executable, like
# r'C:\path\to\waifu2c-ncnn-vulkan.exe'
waifu2x = r'waifu2x-ncnn-vulkan'

# The path to the chosen model for waifu2x-ncnn-vulkan.
# By default it is a path relative to the directory containing the executable.
# The CUNET model is recommended for quality.
# If necessary make this into an absolute path to the model.
# waifu2x_model = r'C:\path\to\waifu2x-ncnn-vulkan\models\models-cunet'
waifu2x_model = r'models-cunet'

# -------------------------------------------------------------------------------
# If you are not looking to implement your own upscaler you can stop reading now.
# -------------------------------------------------------------------------------

# The script MUST write the destination file or exit with an error.
# The script MUST write the resolution of the resulting file, and nothing else, to stdout on success.
# The format of the resolution MUST be WIDTHxHEIGHT. Example: 3840x2160
# The script CAN symlink the destination file to the source file if they share extensions and no
# processing is required.
# The script SHOULD NOT write the file without returning a success code.
# The script not writing the destination file is treated as an error.
# It is valid to call this script with no scaling factor, target width/height,
# or denoise operations specified.

# This script is called with several environment variables.

# The path to the source file.
# The source file MUST NOT be modified.
# This may be any valid image format understood by pixbuf or ImageMagick.
# UPSCALE_SOURCE

# The destination file.
# If it already exists it is expected to be overwritten.
# The format will be decided by the extension.
# This will be a png.
# UPSCALE_DESTINATION

# If set and not empty/0, multiply both dimensions of the image by this factor.
# This is mutually exclusive with target width/height.
# UPSCALE_SCALING_FACTOR

# If set and not empty/zero, use these as the target resolution.
# The output image should at least "touch" the inside of this rectangle: the height or width should
# be at least as large as the target height or width.
# Output images may be larger than these targets.
# If one is not set or zero it should be ignored.
# These are mutually exclusive with scale factor.
# UPSCALE_TARGET_WIDTH
# UPSCALE_TARGET_HEIGHT

# If set and not empty/zero, use these as the minimum output resolutions.
# The output image should completely "fill" this rectangle. Both the height and width should be at
# least as large as these minimums.
# These could potentially be larger than the TARGET values.
# UPSCALE_MIN_WIDTH
# UPSCALE_MIN_HEIGHT

# If set and not empty it will be a signed integer.
# Higher values imply higher denoising, if available.
# UPSCALE_DENOISE

# If set and not empty it will be the timeout, given in seconds as a float.
# The upscaler cannot rely on having the entire available time slice since the timeout starts
# when the process is spawned.
# UPSCALE_TIMEOUT

# -------------------------------------------------------------------------------

import math
import os
import shutil
import subprocess
import sys

# Use gdk-pixbuf2 through PyGobject if available, if not fall back to the imagemagick executable.
try:
    import gi
    gi.require_version('GdkPixbuf', '2.0')
    from gi.repository import GdkPixbuf
    use_gdk = True
except:
    print('gdk-pixbuf2 unavailable', file=sys.stderr)
    use_gdk = False

src = os.getenv('UPSCALE_SOURCE', '')
dst = os.getenv('UPSCALE_DESTINATION', '')
scale = int(os.getenv('UPSCALE_SCALING_FACTOR') or 0)
tx = int(os.getenv('UPSCALE_TARGET_WIDTH') or 0)
ty = int(os.getenv('UPSCALE_TARGET_HEIGHT') or 0)
mx = int(os.getenv('UPSCALE_MIN_WIDTH') or 0)
my = int(os.getenv('UPSCALE_MIN_HEIGHT') or 0)
denoise = int(os.getenv('UPSCALE_DENOISE')
              or 0) if os.getenv('UPSCALE_DENOISE') is not None else None
timeout = float(os.getenv('UPSCALE_TIMEOUT') or 0) or None

if not bool(src) or not bool(dst):
    raise Exception('Source and destination must be present')

if not os.path.isfile(src):
    raise Exception('Source must be a valid file.')

if scale < 0 or tx < 0 or ty < 0 or mx < 0 or my < 0:
    raise Exception('Scale, heights, and widths cannot be negative')

if scale > 0 and (tx > 0 or ty > 0 or mx > 0 or my > 0):
    raise Exception(
        'Cannot specify scaling factor alongside widths or heights.')

# This script gives the full timeout to waifu2x, even though it really won't have that long to run.
if timeout is not None and (timeout < 0 or not math.isfinite(timeout)):
    raise Exception('Cannot specify negative or infinite timeout')

startupinfo = None
# Never spawn console windows on Windows
if os.name == 'nt':
    startupinfo = subprocess.STARTUPINFO()
    startupinfo.dwFlags |= subprocess.STARTF_USESHOWWINDOW

kwargs = {
    'stdout': subprocess.PIPE,
    'stderr': sys.stderr,
    'startupinfo': startupinfo,
    'encoding': 'utf-8',
}

magick7 = bool(shutil.which('magick'))
magick = ['convert']
if magick7:
    magick = ['magick', 'convert']

format = None
width, height = 0, 0
if use_gdk:
    format, width, height = GdkPixbuf.Pixbuf.get_file_info(src)
    if format:
        format = format.get_name()
    else:
        use_gdk = False

if not use_gdk:
    args = magick.copy()
    args.extend([src, '-format', '%w %h %m', 'info:'])
    cp = subprocess.run(args, **kwargs)
    cp.check_returncode()
    width, height, format = cp.stdout.split(' ', 2)
    format = format.lower()
    width, height = int(width), int(height)

if not format:
    raise Exception('Unrecognized format.')

if scale == 0 and (tx != 0 or ty != 0):
    if tx != 0 and ty != 0:
        scale = math.ceil(min(tx / width, ty / height))
    elif tx == 0:
        scale = math.ceil(ty / height)
    else:
        scale = math.ceil(tx / width)

if mx != 0:
    scale = max(math.ceil(mx / width), scale)

if my != 0:
    scale = max(math.ceil(my / height), scale)

# Round up to power of 2
scale = 2**(scale - 1).bit_length() if scale > 0 else 1
if scale > 32:
    print('Scale cannot be above 32 for waifu2x', file=sys.stderr)
    scale = 32

# waifu2x-ncnn-vulkan only understands jpeg, png, and webp
# We can write to the destination file, but we should remove it if upscaling fails later.
wrote_dst = False
if (scale == 1 and denoise is None) or format not in ['png', 'jpeg', 'webp']:
    if format == "png":
        # Creating a symlink is legal.
        os.symlink(src, dst)
        print(f"{scale*width}x{scale*height}")
        sys.exit(0)

    if use_gdk:
        pb = GdkPixbuf.Pixbuf.new_from_file(src)
        if not pb:
            raise Exception('Unrecognized format.')
        pb.savev(dst, 'png', [], [])
    else:
        args = magick.copy()
        args.extend([src, dst])
        cp = subprocess.run(args, **kwargs)
        cp.check_returncode()
    src = dst
    wrote_dst = True

if scale == 1 and denoise is None:
    print(f"{scale*width}x{scale*height}")
    sys.exit(0)

# Clamp denoise to valid values
if denoise is None:
    denoise = -1

denoise = min(max(denoise, -1), 3)

# yapf: disable
cp = subprocess.run([
    waifu2x,
    '-m', waifu2x_model,
    '-i', src,
    '-o', dst,
    '-s', str(scale),
    '-n', str(denoise),
    # Force tile size of 400, which is the highest waifu2x-ncnn-vulkan will autoselect
    '-t', '400',
], timeout=timeout, **kwargs)
# yapf: enable
try:
    cp.check_returncode()
except Exception as e:
    if wrote_dst:
        os.remove(dst)
    raise Exception(e)

print(f"{scale*width}x{scale*height}")
