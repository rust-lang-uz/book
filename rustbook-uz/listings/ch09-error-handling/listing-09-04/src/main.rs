use std::fs::File;

fn main() {
    let fayl_ochish = File::open("olma.txt");

    let fayl = match fayl_ochish {
        Ok(file) => file,
        Err(error) => panic!("Faylni ochishda muammo: {:?}", error),
    };
}
