#[derive(Debug)]
struct Rectangle {
    kengligi: u32,
    balandligi: u32,
}

fn main() {
    let mut list = [
        Rectangle { kengligi: 10, balandligi: 1 },
        Rectangle { kengligi: 3, balandligi: 5 },
        Rectangle { kengligi: 7, balandligi: 12 },
    ];

    let mut saralash_operatsiyalari = vec![];
    let qiymat = String::from("chaqirilgan kalit orqali");

    list.sort_by_key(|r| {
        saralash_operatsiyalari.push(qiymat);
        r.kengligi
    });
    println!("{:#?}", list);
}