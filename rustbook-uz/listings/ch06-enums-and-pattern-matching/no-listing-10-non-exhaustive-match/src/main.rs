fn main() {
    // ANCHOR: here
    fn bir_qoshish(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // ANCHOR_END: here

    let besh = Some(5);
    let olti = bir_qoshish(besh);
    let yoq = bir_qoshish(None);
}
