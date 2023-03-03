fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let maydon_nomi = String::from("Sevimli rang");
    let maydon_qiymati = String::from("Yashil");

    let mut map = HashMap::new();
    map.insert(maydon_nomi, maydon_qiymati);
    // maydon_nomi va maydon_qiymati hozirda yaroqsiz, ulardan foydalanib ko'ring va qanday
    // kompilyator xatosiga yo'l qo'yganingizni ko'ring!
    // ANCHOR_END: here
}
