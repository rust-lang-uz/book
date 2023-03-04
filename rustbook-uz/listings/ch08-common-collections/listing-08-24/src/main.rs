fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut ballar = HashMap::new();
    ballar.insert(String::from("Yashil"), 10);

    ballar.entry(String::from("Sariq")).or_insert(50);
    ballar.entry(String::from("Yashil")).or_insert(50);

    println!("{:?}", ballar);
    // ANCHOR_END: here
}
