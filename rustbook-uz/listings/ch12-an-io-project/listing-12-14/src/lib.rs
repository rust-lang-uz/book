use std::error::Error;
use std::fs;

pub struct Config {
    pub sorov: String,
    pub fayl_yoli: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argumentlar yetarli emas");
        }

        let sorov = args[1].clone();
        let fayl_yoli = args[2].clone();

        Ok(Config { sorov, fayl_yoli })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tarkib = fs::read_to_string(config.fayl_yoli)?;

    println!("Fayl tarkibi:\n{tarkib}");

    Ok(())
}
