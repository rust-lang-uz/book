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

fn main() {
    let tanga = Tanga::Penny;
    // ANCHOR: here
    let mut hisobchi = 0;
    if let Tanga::Quarter(shtat) = tanga {
        println!("{:?} dan shtat kvartal!", shtat);
    } else {
        hisobchi += 1;
    }
    // ANCHOR_END: here
}
