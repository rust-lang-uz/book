// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Raqamni topish o'yini!");

    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);

    println!("Yashirin raqam: {yashirin_raqam}");

    println!("Iltimos, taxminingizni kiriting.");

    let mut taxmin = String::new();

    io::stdin()
        .read_line(&mut taxmin)
        .expect("Satrni o‘qib bo‘lmadi");
    // ANCHOR: here

    println!("Sizning taxminingiz: {taxmin}");

    match taxmin.cmp(&yashirin_raqam) {
        Ordering::Less => println!("Raqam Kichik!"),
        Ordering::Greater => println!("Raqam katta!"),
        Ordering::Equal => println!("Siz yutdingiz!"),
    }
}
// ANCHOR_END: here
