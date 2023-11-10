#[derive(Debug)]
struct Kvadrat {
    kengligi: u32,
    balandligi: u32,
}

fn main() {
    let mut list = [
        Kvadrat { kengligi: 10, balandligi: 1 },
        Kvadrat { kengligi: 3, balandligi: 5 },
        Kvadrat { kengligi: 7, balandligi: 12 },
    ];

    let mut raqam_saralash_operatsiyalari = 0;
    list.sort_by_key(|r| {
        raqam_saralash_operatsiyalari += 1;
        r.kengligi
    });
    println!("{:#?}, {raqam_saralash_operatsiyalari} operatsiyalarida tartiblangan", list);
}