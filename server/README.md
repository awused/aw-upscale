# AW-UPSCALE-SERVER

A simple GRPC server to run aw-upscale on another machine and return the results. Useful if you only have one GPU on your and are using waifu2x, but would prefer to offload upscaling to another device.

# Requirements

* grpcio - `pip3 install grpcio`

The same requirements as aw-upscale apply, but the upscaler has to be specified at runtime for the server.

# Usage

`cargo install --git https://github.com/awused/aw-upscale/server --locked`

After setting up the upscaler, run aw-upscale-server with the address, port, and upscaler as arguments. Configure the client application to use the included aw-upscale-client.py script which must be edited for your specific setup.


# Development

Rebuild the python grpc file with `python -m grpc_tools.protoc -I proto --python_out=. --grpc_python_out=. proto/upscale.proto`

