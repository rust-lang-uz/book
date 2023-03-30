fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let natija = eng_uzun(string1.as_str(), string2);
    println!("Eng uzun satr {}", natija);
}

// ANCHOR: here
fn eng_uzun<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// ANCHOR_END: here
