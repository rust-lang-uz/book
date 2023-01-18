// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Raqamni topish o'yini!");

    println!("Iltimos, taxminingizni kiriting.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut taxmin = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut taxmin)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Satrni o‘qib bo‘lmadi");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Sizning taxminingiz: {taxmin}");
    // ANCHOR_END: print_guess
}
// ANCHOR: all
