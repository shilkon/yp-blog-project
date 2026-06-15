use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = env::var("BLOG_PROTO_FILE")?;
    let proto_dir = env::var("BLOG_PROTO_DIR")?;

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[&proto_file],
            &[&proto_dir],
        )?;

    println!("cargo:rerun-if-changed={proto_file}");

    Ok(())
} 