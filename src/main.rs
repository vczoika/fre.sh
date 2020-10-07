use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("fre.sh")?;
    file.write_all(
    b"chmod +x fre.sh
    echo \"Hello friend. Hello friend? That\'s lame. Maybe I should give you a name.\"
    sleep 3
    yes | sudo apt-get update
    sudo apt-get upgrade
    echo \"Goodbye, friend. That\'s lame. We\'re talking to an imaginary person. Just like we always have.\"")?;
    Ok(())
}