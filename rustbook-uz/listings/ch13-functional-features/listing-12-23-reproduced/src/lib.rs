use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub sorov: String,
    pub fayl_yoli: String,
    pub ignore_case: bool,
}

// ANCHOR: ch13
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argumentlar yetarli emas");
        }

        let sorov = args[1].clone();
        let fayl_yoli = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            sorov,
            fayl_yoli,
            ignore_case,
        })
    }
}
// ANCHOR_END: ch13

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tarkib = fs::read_to_string(config.fayl_yoli)?;

    let natijalar = if config.ignore_case {
        search_case_insensitive(&config.sorov, &tarkib)
    } else {
        search(&config.sorov, &tarkib)
    };

    for line in natijalar {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(sorov: &str, tarkib: &'a str) -> Vec<&'a str> {
    let mut natijalar = Vec::new();

    for line in tarkib.lines() {
        if line.contains(sorov) {
            natijalar.push(line);
        }
    }

    natijalar
}

pub fn search_case_insensitive<'a>(
    sorov: &str,
    tarkib: &'a str,
) -> Vec<&'a str> {
    let sorov = sorov.to_lowercase();
    let mut natijalar = Vec::new();

    for line in tarkib.lines() {
        if line.to_lowercase().contains(&sorov) {
            natijalar.push(line);
        }
    }

    natijalar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harflarga_etiborli() {
        let sorov = "duct";
        let tarkib = "\
Rust:
xavfsiz, tez, samarali.
Uchtasini tanlang.
Duct tape.";

        assert_eq!(vec!["xavfsiz, tez, samarali."], search(sorov, tarkib));
    }

    #[test]
    fn harflarga_etiborsiz() {
        let sorov = "rUsT";
        let tarkib = "\
Rust:
xavfsiz, tez, samarali.
Uchtasini tanlang.
Menga ishoning.";

        assert_eq!(
            vec!["Rust:", "Menga ishoning."],
            harflarga_etiborsiz_qidirish(sorov, tarkib)
        );
    }
}
