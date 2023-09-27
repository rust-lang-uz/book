fn main() {
    // ANCHOR: here
    let namuna_closure = |x| x;

    let s = namuna_closure(String::from("salom"));
    let n = namuna_closure(5);
    // ANCHOR_END: here
}
