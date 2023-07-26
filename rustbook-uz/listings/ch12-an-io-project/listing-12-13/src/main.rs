use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Argumentlarni tahlil qilish muammosi: {err}");
        process::exit(1);
    });

    println!("{} qidirilmoqda", config.sorov);
    println!("{} faylida", config.fayl_yoli);

    if let Err(e) = run(config) {
        println!("Dastur xatosi: {e}");
        process::exit(1);
    }
}
