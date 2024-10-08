use anyhow::Result;
use rs4j::build::BindgenConfig;

fn main() -> Result<()> {
    let out_path = format!("{}/generated", env!("CARGO_MANIFEST_DIR"));
    let src_path = format!("{}/project/src/generated", env!("CARGO_MANIFEST_DIR"));

    BindgenConfig::new()
        .package("com.dimforge.rapier3d")
        .bindings(format!("{}/src/bindings.rs", env!("CARGO_MANIFEST_DIR")))
        .glob(format!("{}/bindings/**/*.rs4j", env!("CARGO_MANIFEST_DIR")))?
        .output(out_path)
        .annotations(true)
        .post_build()?
        .copy_to(src_path)?;

    Ok(())
}
