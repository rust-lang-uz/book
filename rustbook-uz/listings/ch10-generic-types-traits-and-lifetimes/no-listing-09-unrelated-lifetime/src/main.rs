fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let natija = eng_uzun(string1.as_str(), string2);
    println!("Eng uzun satr {}", natija);
}

// ANCHOR: here
fn eng_uzun<'a>(x: &str, y: &str) -> &'a str {
    let natija = String::from("haqiqatan ham uzun satr");
    natija.as_str()
}
// ANCHOR_END: here
