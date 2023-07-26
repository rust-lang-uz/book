use std::env;
use std::error::Error;
use std::fs;
use std::process;

// ANCHOR: here
fn main() {
    // --snip--

    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Argumentlarni tahlil qilish muammosi: {err}");
        process::exit(1);
    });

    // ANCHOR: here
    println!("{} qidirilmoqda", config.sorov);
    println!("{} faylida", config.fayl_yoli);

    if let Err(e) = run(config) {
        println!("Dastur xatosi: {e}");
        process::exit(1);
    }
}
// ANCHOR_END: here

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tarkib = fs::read_to_string(config.fayl_yoli)?;

    println!("Fayl tarkibi:\n{tarkib}");

    Ok(())
}

struct Config {
    sorov: String,
    fayl_yoli: String,
}

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
