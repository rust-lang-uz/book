// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let sorov = &args[1];
    let fayl_yoli = &args[2];

    println!("{} qidirilmoqda", sorov);
    // ANCHOR: here
    println!("{} faylida", fayl_yoli);

    let tarkib = fs::read_to_string(fayl_yoli)
        .expect("Faylni o'qiy olishi kerak edi");

    println!("Fayl tarkibi:\n{tarkib}");
}
// ANCHOR_END: here
