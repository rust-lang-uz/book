use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let fayl_ochish = File::open("olma.txt")?;

    Ok(())
}
