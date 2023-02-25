use rand::Rng;
// ANCHOR: here
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// ANCHOR_END: here

fn main() {
    println!("Raqamni topish o'yini!");

    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);

    println!("Yashirin raqam: {yashirin_raqam}");

    println!("Iltimos, taxminingizni kiriting.");

    let mut taxmin = String::new();

    io::stdin()
        .read_line(&mut taxmin)
        .expect("Satrni o'qib bo'lmadi");

    println!("Sizning taxminingiz: {taxmin}");

    match taxmin.cmp(&yashirin_raqam) {
        Ordering::Less => println!("Raqam Kichik!"),
        Ordering::Greater => println!("Raqam katta!"),
        Ordering::Equal => println!("Siz yutdingiz!"),
    }
}
