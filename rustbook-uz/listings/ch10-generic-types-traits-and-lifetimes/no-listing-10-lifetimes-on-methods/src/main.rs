struct ImportantExcerpt<'a> {
    qism: &'a str,
}

// ANCHOR: 1st
impl<'a> ImportantExcerpt<'a> {
    fn daraja(&self) -> i32 {
        3
    }
}
// ANCHOR_END: 1st

// ANCHOR: 3rd
impl<'a> ImportantExcerpt<'a> {
    fn qismni_elon_qilish_qaytarih(&self, elon_qilish: &str) -> &str {
        println!("Diqqat iltimos: {}", elon_qilish);
        self.qism
    }
}
// ANCHOR_END: 3rd

fn main() {
    let roman = String::from("Meni yaxshi dasturchi edim. Bir necha yil oldin...");
    let birinchi_jumla = roman.split('.').next().expect("'.' belgisini topib bo'lmadi.");
    let i = ImportantExcerpt {
        qism: birinchi_jumla,
    };
}
