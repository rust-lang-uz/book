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

    Ok(())
}

// ANCHOR: here
pub fn qidiruv<'a>(sorov: &str, tarkib: &'a str) -> Vec<&'a str> {
    vec![]
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn birinchi_natija() {
        let sorov = "marali";
        let tarkib = "\
Rust:
xavfsiz, tez, samarali.
Uchtasini tanlang.";

        assert_eq!(vec!["xavfsiz, tez, samarali."], qidiruv(sorov, tarkib));
    }
}