use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("{} qidirilmoqda", config.sorov);
    println!("{} faylida", config.fayl_yoli);

    let tarkib = fs::read_to_string(config.fayl_yoli)
        .expect("Faylni o'qiy olishi kerak edi");

    println!("Fayl tarkibi:\n{tarkib}");
}

struct Config {
    sorov: String,
    fayl_yoli: String,
}

// ANCHOR: here
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argumentlar yetarli emas");
        }

        let sorov = args[1].clone();
        let fayl_yoli = args[2].clone();

        Ok(Config { sorov, fayl_yoli })
    }
}
// ANCHOR_END: here
