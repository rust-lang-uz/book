fn main() {
    // ANCHOR: here
    let max_sozlama = Some(3u8);
    match max_sozlama {
        Some(max) => println!("Maksimal {} qilib sozlangan", max),
        _ => (),
    }
    // ANCHOR_END: here
}
