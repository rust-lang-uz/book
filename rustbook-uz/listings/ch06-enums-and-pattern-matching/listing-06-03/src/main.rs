// ANCHOR: here
enum Tanga {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn sentdagi_qiymat(tanga: Tanga) -> u8 {
    match tanga {
        Tanga::Penny => 1,
        Tanga::Nickel => 5,
        Tanga::Dime => 10,
        Tanga::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
