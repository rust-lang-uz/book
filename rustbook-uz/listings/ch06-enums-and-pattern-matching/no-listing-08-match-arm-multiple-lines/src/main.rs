enum Tanga {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ANCHOR: here
fn sentdagi_qiymat(tanga: Tanga) -> u8 {
    match tanga {
        Tanga::Penny => {
            println!("Omadli tanga!");
            1
        }
        Tanga::Nickel => 5,
        Tanga::Dime => 10,
        Tanga::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
