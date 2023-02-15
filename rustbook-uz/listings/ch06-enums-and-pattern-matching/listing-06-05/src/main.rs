fn main() {
    // ANCHOR: here
    fn bir_qoshish(x: Option<i32>) -> Option<i32> {
        match x {
            // ANCHOR: first_arm
            None => None,
            // ANCHOR_END: first_arm
            // ANCHOR: second_arm
            Some(i) => Some(i + 1),
            // ANCHOR_END: second_arm
        }
    }

    let besh = Some(5);
    let olti = bir_qoshish(besh);
    let yoq = bir_qoshish(None);
    // ANCHOR_END: here
}
