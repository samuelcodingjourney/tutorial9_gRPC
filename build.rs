fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .build_Server(true)
    .compile(
        &["proto/services.proto"], // Path to your proto file
        &["proto"], // Directory where the proto file is located
    )?;
    Ok(())
}