fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut ballar = HashMap::new();

    ballar.insert(String::from("Yashil"), 10);
    ballar.insert(String::from("Sariq"), 50);

    let jamoa_nomi = String::from("Yashil");
    let ball = ballar.get(&jamoa_nomi).copied().unwrap_or(0);
    // ANCHOR_END: here
}
