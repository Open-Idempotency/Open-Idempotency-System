use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_proto_file = env::var("MAIN_PROTO_FILE").unwrap_or("protos/idempotency.proto".into_string());
    let descriptor_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("idempotency_descriptor.bin");
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(&descriptor_path)
        //.out_dir("./protos/complied")
        .compile(&[main_proto_file], &[env::current_dir()?])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
    println!("cargo:rerun-if-changed={}", main_proto_file);
    Ok(())
}
