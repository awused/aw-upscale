# AW-UPSCALE

A small library to wrap arbitrary executable upscalers for images, such as waifu2x.

# Requirements

There are no requirements in general, but using the default implementation has several:

* Python 3
* Either:
    * ImageMagick (6 or 7) executables on your PATH.
    * gdk-pixbuf2 and [PyGObject](https://pygobject.readthedocs.io/)
        * Additionally, any pixbuf loaders for formats you plan to handle, like webp-pixbuf-loader.
* [waifu2x-ncnn-vulkan](https://github.com/nihui/waifu2x-ncnn-vulkan)


Python 3 and waifu2x-ncnn-vulkan must be present on the PATH of the system running the library.

gdk-pixbuf2 will be preferred over ImageMagick when available.

When installing waifu2x-ncnn-vulkan, make sure that the directory containing the waifu2x-ncnn-vulkan binary also contains the [models-cunet](https://github.com/nihui/waifu2x-ncnn-vulkan/tree/master/models/models-cunet) directory. Extracting the Windows binary release zip files already places the binary and models into the same directory, they just need to be added to your PATH.

Try running one of these lines to see if everything is working. If everything is set up correctly no errors will be printed and the last line output will similar to `rose.png PNG 280x184 280x184+0+0 8-bit sRGB 88401B 0.000u 0:00.000`.
```
ImageMagick 6:

convert rose: rose.bmp; identify rose.bmp; UPSCALE_SOURCE=rose.bmp UPSCALE_DESTINATION=rose.png UPSCALE_SCALING_FACTOR=4 python waifu2x-upscale.py; identify rose.png; rm rose.bmp; rm rose.png

ImageMagick 7:

magick convert rose: rose.bmp; magick identify rose.bmp; UPSCALE_SOURCE=rose.bmp UPSCALE_DESTINATION=rose.png UPSCALE_SCALING_FACTOR=4 python waifu2x-upscale.py; magick identify rose.png; rm rose.bmp; rm rose.png

```

# Alternative Upscalers

An upscaler is just an executable which is provided with a set of environment variables. See [waifu2x-upscale.py](waifu2x-upscale.py) to modify it or create your own.

