use std::error::Error;
use std::fs;

pub struct Config {
    pub sorov : String,
    pub fayl_yoli: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argumentlar yetarli emas");
        }

        let sorov  = args[1].clone();
        let fayl_yoli = args[2].clone();

        Ok(Config { sorov , fayl_yoli })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tarkib = fs::read_to_string(config.fayl_yoli)?;

    for line in qidiruv(&config.sorov , &tarkib) {
        println!("{line}");
    }

    Ok(())
}

pub fn qidiruv<'a>(sorov : &str, tarkib: &'a str) -> Vec<&'a str> {
    let mut natijalar = Vec::new();

    for line in tarkib.lines() {
        if line.contains(sorov ) {
            natijalar.push(line);
        }
    }

    natijalar
}

// ANCHOR: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harflarga_etiborli() {
        let sorov  = "marali";
        let tarkib = "\
Rust:
xavfsiz, tez, samarali.
Uchtasini tanlang.
Duct tape.";

        assert_eq!(vec!["xavfsiz, tez, samarali."], qidiruv(sorov , tarkib));
    }

    #[test]
    fn harflarga_etiborsiz() {
        let sorov  = "rUsT";
        let tarkib = "\
Rust:
xavfsiz, tez, samarali.
Uchtasini tanlang.
Menga ishoning.";

        assert_eq!(
            vec!["Rust:", "Menga ishoning."],
            harflarga_etiborsiz_qidirish(sorov , tarkib)
        );
    }
}
// ANCHOR_END: here
