use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Raqamni topish o'yini!");

    let yashirin_raqam = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    loop {
        // --snip--

        // ANCHOR_END: here
        println!("Iltimos, taxminingizni kiriting.");

        let mut taxmin = String::new();

        io::stdin()
            .read_line(&mut taxmin)
            .expect("Satrni o‘qib bo‘lmadi");

        // ANCHOR: here
        let taxmin: i32 = match taxmin.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if taxmin < 1 || taxmin > 100 {
            println!("Yashirin raqam 1 dan 100 gacha bo'ladi.");
            continue;
        }

        match taxmin.cmp(&yashirin_raqam) {
            // --snip--
            // ANCHOR_END: here
            Ordering::Less => println!("Raqam Kichik!"),
            Ordering::Greater => println!("Raqam katta!"),
            Ordering::Equal => {
                println!("Siz yutdingiz!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
