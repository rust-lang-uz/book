fn main() {
    let s = String::from("salom");

    almashtirish(&s);
}

fn almashtirish(some_string: &String) {
    some_string.push_str(", rust");
}
