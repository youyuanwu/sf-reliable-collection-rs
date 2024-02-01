use std::{env, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let package_root = env::var("CARGO_MANIFEST_DIR").unwrap();

    // repo root.
    let abs_dir = Path::new(&package_root)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut exe = abs_dir.to_path_buf();
    exe.push("build/_deps/protoc-src/bin/protoc.exe");

    std::env::set_var("PROTOC", exe);
    // generate kvstore for grpc
    tonic_build::compile_protos("proto/kvstore.proto")?;

    Ok(())
}
