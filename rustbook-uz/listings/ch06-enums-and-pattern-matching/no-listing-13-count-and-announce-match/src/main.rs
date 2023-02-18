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
    match tanga {
        Tanga::Quarter(shtat) => println!("{:?} dan shtat kvartal!", shtat),
        _ => hisobchi += 1,
    }
    // ANCHOR_END: here
}
