//! The builder for the backend.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("generated")
        .build_server(true)
        .build_client(true)
        .compile(&["proto/demo.proto"], &["proto"])?;
    Ok(())
}
