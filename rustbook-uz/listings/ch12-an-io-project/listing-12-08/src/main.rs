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

impl Config {
    // ANCHOR: here
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("argumentlar yetarli emas");
        }
        // --snip--
        // ANCHOR_END: here

        let sorov= args[1].clone();
        let fayl_yoli = args[2].clone();

        Config { sorov, fayl_yoli }
    }
}
