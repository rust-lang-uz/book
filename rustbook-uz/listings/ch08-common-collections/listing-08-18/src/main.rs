fn main() {
    // ANCHOR: here
    let s1 = String::from("Salom, ");
    let s2 = String::from("Rust!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // ANCHOR_END: here
}
