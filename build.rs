use anyhow::Result;
use rs4j::build::BindgenConfig;

fn main() -> Result<()> {
    BindgenConfig::new()
        .package("com.dimforge.rapier3d")
        .bindings(format!("{}/src/bindings.rs", env!("CARGO_MANIFEST_DIR")))
        .glob(format!("{}/bindings/**/*.rs4j", env!("CARGO_MANIFEST_DIR")))?
        .output(format!("{}/generated", env!("CARGO_MANIFEST_DIR")))
        .annotations(true)
        .generate()?;

    Ok(())
}
