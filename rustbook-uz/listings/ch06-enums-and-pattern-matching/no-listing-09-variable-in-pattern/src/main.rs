#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Tanga {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// ANCHOR: here
fn sentdagi_qiymat(tanga: Tanga) -> u8 {
    match tanga {
        Tanga::Penny => 1,
        Tanga::Nickel => 5,
        Tanga::Dime => 10,
        Tanga::Quarter(shtat) => {
            println!("{:?} dan shtat quarter!", shtat);
            25
        }
    }
}
// ANCHOR_END: here

fn main() {
    sentdagi_qiymat(Tanga::Quarter(UsState::Alaska));
}
