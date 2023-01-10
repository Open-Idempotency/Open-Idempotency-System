use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_proto_file = env::var("MAIN_PROTO_FILE").unwrap_or(String::from("protos/idempotency.proto"));

    // The environment variable 'OUT_DIR' is specified by the build process.
    // If you run this code directly with force-build, the out dir
    // will not be specified and will cause this build of the proto files to fail.
    // This file should run when you run 'cargo build'.
    let out_dir = env::var("OUT_DIR").expect("ENV OUT_DIR NOT FOUND");

    let descriptor_path =
        PathBuf::from(out_dir).join("idempotency_descriptor.bin");

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(&descriptor_path)
        .compile(&[&main_proto_file], &[env::current_dir()?])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", &main_proto_file);

    Ok(())
}
