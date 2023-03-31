struct ImportantExcerpt<'a> {
    qism: &'a str,
}

fn main() {
    let roman = String::from("Meni yaxshi dasturchi edim. Bir necha yil oldin...");
    let birinchi_jumla = roman.split('.').next().expect("'.' belgisini topib bo'lmadi.");
    let i = ImportantExcerpt {
        qism: birinchi_jumla,
    };
}
