fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let natija = elon_bilan_eng_uzun(
        string1.as_str(),
        string2,
        "Bugun kimningdir tug'ilgan kuni!",
    );
    println!("Eng uzun satr {}", natija);
}

// ANCHOR: here
use std::fmt::Display;

fn elon_bilan_eng_uzun<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("E'lon! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// ANCHOR_END: here
