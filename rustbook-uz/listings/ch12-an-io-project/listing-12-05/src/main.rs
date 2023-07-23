use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let (sorov, fayl_yoli) = parse_config(&args);

    // --snip--
    // ANCHOR_END: here

    println!("{} qidirilmoqda", sorov);
    println!("{} faylida", fayl_yoli);

    let tarkib = fs::read_to_string(fayl_yoli)
        .expect("Faylni o'qiy olishi kerak edi");

    println!("Fayl tarkibi:\n{tarkib}");
    // ANCHOR: here
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let sorov = &args[1];
    let fayl_yoli = &args[2];

    (sorov, fayl_yoli)
}
// ANCHOR_END: here
