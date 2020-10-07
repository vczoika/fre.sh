use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("fre.sh")?;
    file.write_all(b"sudo apt-get update")?;
    Ok(())
}