fn main() {
    let dangle_reference = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle Stringga referencei qaytaradi

    let s = String::from("salom"); // s - yangi String

    &s // biz Stringga referenceni return qilamiz, s
} // Bu yerda s scopedan chiqib ketadi va drop qilinadi. Uning xotirasi yo'qoladi.
  // Xavf!
// ANCHOR_END: here
