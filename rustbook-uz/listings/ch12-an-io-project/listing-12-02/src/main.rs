use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let sorov = &args[1];
    let fayl_yoli = &args[2];

    println!("{} qidirilmoqda", sorov);
    println!("{} faylida", fayl_yoli);
}
