// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn fayldan_foydalanuvchi_nomini_olish() -> Result<String, io::Error> {
    let foydalanuvchi_fayli_natijasi = File::open("olma.txt");

    let mut foydalanuvchi_fayli = match foydalanuvchi_fayli_natijasi {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut foydalanuvchi = String::new();

    match foydalanuvchi_fayli.read_to_string(&mut foydalanuvchi) {
        Ok(_) => Ok(foydalanuvchi),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let foydalanuvchi = fayldan_foydalanuvchi_nomini_olish().expect("Foydalanuvchi nomini olish imkonsiz");
}