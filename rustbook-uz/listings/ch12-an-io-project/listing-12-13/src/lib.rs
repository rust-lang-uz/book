// ANCHOR: here
use std::error::Error;
use std::fs;

pub struct Config {
    pub sorov: String,
    pub fayl_yoli: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
        // ANCHOR_END: here
        if args.len() < 3 {
            return Err("argumentlar yetarli emas");
        }

        let sorov = args[1].clone();
        let fayl_yoli = args[2].clone();

        Ok(Config { sorov, fayl_yoli })
        // ANCHOR: here
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    // ANCHOR_END: here
    let tarkib = fs::read_to_string(config.fayl_yoli)?;

    println!("Fayl tarkibi:\n{tarkib}");

    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
