use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("{} qidirilmoqda", config.sorov);
    println!("{} faylida", config.fayl_yoli);

    let tarkib = fs::read_to_string(config.fayl_yoli)
        .expect("Faylni o'qiy olishi kerak edi");

    // --snip--
    // ANCHOR_END: here

    println!("Fayl tarkibi:\n{tarkib}");
    // ANCHOR: here
}

struct Config {
    sorov: String,
    fayl_yoli: String,
}

fn parse_config(args: &[String]) -> Config {
    let sorov = args[1].clone();
    let fayl_yoli = args[2].clone();

    Config { sorov, fayl_yoli }
}
// ANCHOR_END: here
