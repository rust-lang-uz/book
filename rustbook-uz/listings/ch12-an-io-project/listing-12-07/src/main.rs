use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // ANCHOR_END: here

    println!("{} qidirilmoqda", config.sorov);
    println!("{} faylida", config.fayl_yoli);

    let tarkib = fs::read_to_string(config.fayl_yoli)
        .expect("Faylni o'qiy olishi kerak edi");

    println!("Fayl tarkibi:\n{tarkib}");
    // ANCHOR: here

    // --snip--
}

// --snip--

// ANCHOR_END: here
struct Config {
    sorov: String,
    fayl_yoli: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Config {
        let sorov = args[1].clone();
        let fayl_yoli = args[2].clone();

        Config { sorov, fayl_yoli }
    }
}
// ANCHOR_END: here
