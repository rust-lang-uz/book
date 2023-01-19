// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Raqamni topish o'yini!");

    // ANCHOR: ch07-04
    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Yashirin raqam: {yashirin_raqam}");

    println!("Iltimos, taxminingizni kiriting.");

    let mut taxmin = String::new();

    io::stdin()
        .read_line(&mut taxmin)
        .expect("Satrni o‘qib bo‘lmadi");

    println!("Sizning taxminingiz: {taxmin}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
