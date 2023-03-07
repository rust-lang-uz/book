use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let fayl_ochish = File::open("olma.txt");

    let fayl = match fayl_ochish {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("olma.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Fayl yaratishda muammo: {:?}", e),
            },
            boshqa_xato => {
                panic!("Faylni ochishda muammo: {:?}", boshqa_xato);
            }
        },
    };
}
