// ANCHOR: here
fn main() {
    let string1 = String::from("uzundan uzun string");
    let natija;
    {
        let string2 = String::from("xyz");
        natija = eng_uzun(string1.as_str(), string2.as_str());
    }
    println!("Eng uzun satr {}", natija);
}
// ANCHOR_END: here

fn eng_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
