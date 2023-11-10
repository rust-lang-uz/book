#[derive(Debug)]
struct Kvatrat {
    kengligi: u32,
    balandligi: u32,
}

fn main() {
    let mut list = [
        Kvatrat { kengligi: 10, balandligi: 1 },
        Kvatrat { kengligi: 3, balandligi: 5 },
        Kvatrat { kengligi: 7, balandligi: 12 },
    ];

    list.sort_by_key(|r| r.kengligi);
    println!("{:#?}", list);
}
