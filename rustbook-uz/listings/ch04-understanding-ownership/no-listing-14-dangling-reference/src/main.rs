fn main() {
    let dangle_reference = dangle();
}

fn dangle() -> &String {
    let s = String::from("salom");

    &s
}
