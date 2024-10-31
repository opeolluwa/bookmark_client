use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .protoc_arg("--proto_path=../proto")
        // .build_client(true) // don't compile the client code
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
        .out_dir("./src/grpc")
        .compile_protos(
            &[
                "../proto/vaults.proto",
                "../proto/vault_entries.proto",
                "../proto/activity.proto",
                "../proto/authentication.proto",
            ],
            &["../proto"],
        )?;

    Ok(())
}
