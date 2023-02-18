fn main() {
    // ANCHOR: here
    let max_sozlama = Some(3u8);
    if let Some(max) = max_sozlama {
        println!("Maksimal {} qilib sozlangan", max);
    }
    // ANCHOR_END: here
}
