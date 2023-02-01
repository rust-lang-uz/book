// ANCHOR: here
fn birinchi_soz(s: &str) -> &str {
    // ANCHOR_END: here
    let baytlar = s.as_bytes();

    for (i, &item) in baytlar.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let mening_stringim = String::from("hello world");

    // `birinchi_soz` `String` ning qisman yoki to'liq slicelarida ishlaydi
    let soz = birinchi_soz(&mening_stringim[0..6]);
    let soz = birinchi_soz(&mening_stringim[..]);
    // `birinchi_soz`, shuningdek, `String` ning butun slicelariga ekvivalent bo`lgan
    // `String`-ga referencelar ustida ham ishlaydi.
    let soz = birinchi_soz(&mening_stringim);

    let mening_literal_stringim = "hello world";

    // `birinchi_soz` qisman yoki to'liq bo'lgan string literallari slicelarida ishlaydi
    let soz = birinchi_soz(&mening_literal_stringim [0..6]);
    let soz = birinchi_soz(&mening_literal_stringim [..]);

    // String literallari  allaqachon string slicelari bo'lganligi sababli,
    // bu slice sintaksisisiz ham ishlaydi!
    let soz = birinchi_soz(mening_literal_stringim );
}
// ANCHOR_END: usage
