// ANCHOR: here
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Argumentlarni tahlil qilish muammosi: {err}");
        process::exit(1);
    });

    println!("{} qidirilmoqda", config.sorov);
    println!("{} faylida", config.fayl_yoli);

    // ANCHOR: here
    if let Err(e) = minigrep::run(config) {
        // --snip--
        // ANCHOR_END: here
        println!("Dastur xatosi: {e}");
        process::exit(1);
        // ANCHOR: here
    }
}
// ANCHOR_END: here
