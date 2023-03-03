fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut ballar = HashMap::new();

    ballar.insert(String::from("Yashil"), 10);
    ballar.insert(String::from("Sariq"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // ANCHOR_END: here
}
