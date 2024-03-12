use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed={}", "protos");
    tonic_build::configure()
        .out_dir("src/proto")
        .file_descriptor_set_path("src/proto/reflection-descriptor.bin")
        .compile(
            &["protos/tinykvpb.proto"], 
            &["protos/"]
        )?;
    Ok(())
}
