fn main() {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    tonic_prost_build::configure()
        .compile_with_config(config, &["proto/upscale.proto"], &["proto"])
        .unwrap()
}
