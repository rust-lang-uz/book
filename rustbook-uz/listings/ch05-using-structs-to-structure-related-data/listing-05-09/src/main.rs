fn main() {
    let kvadrat1 = (30, 50);

    println!(
        "To'rtburchakning maydoni {} kvadrat pikselga teng.",
        area(kvadrat1)
    );
}

fn area(olchamlari: (u32, u32)) -> u32 {
    olchamlari.0 * olchamlari.1
}
