use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let source = fs::read_to_string("temp/test.yaml")?;

    Ok(())
}
