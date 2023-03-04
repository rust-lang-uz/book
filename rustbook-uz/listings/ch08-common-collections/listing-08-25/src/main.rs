fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let matn = "salom rust qiziqarli rust";

    let mut map = HashMap::new();

    for soz in matn.split_whitespace() {
        let hisoblash = map.entry(soz).or_insert(0);
        *hisoblash += 1;
    }

    println!("{:?}", map);
    // ANCHOR_END: here
}
