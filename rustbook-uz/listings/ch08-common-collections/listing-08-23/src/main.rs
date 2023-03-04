fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut ballar = HashMap::new();

    ballar.insert(String::from("Yashil"), 10);
    ballar.insert(String::from("Yashil"), 25);

    println!("{:?}", ballar);
    // ANCHOR_END: here
}
