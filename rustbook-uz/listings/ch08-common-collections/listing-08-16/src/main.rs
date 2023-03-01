fn main() {
    // ANCHOR: here
    let mut s1 = String::from("dastur");
    let s2 = "chi";
    s1.push_str(s2);
    println!("s2 - {s2}");
    // ANCHOR_END: here
}
