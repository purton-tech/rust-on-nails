use ructe::{Result, Ructe};

fn main() -> Result<()> {

    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("dist").unwrap();
    statics.add_files("asset-pipeline/images").unwrap();
    ructe.compile_templates("templates").unwrap();

    Ok(())
}