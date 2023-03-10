// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn fayldan_foydalanuvchi_nomini_olish() -> Result<String, io::Error> {
    let mut foydalanuvchi_fayli = File::open("olma.txt")?;
    let mut foydalanuvchi = String::new();
    foydalanuvchi_fayli.read_to_string(&mut foydalanuvchi)?;
    Ok(foydalanuvchi)
}
// ANCHOR_END: here

fn main() {
    let foydalanuvchi = fayldan_foydalanuvchi_nomini_olish().expect("Foydalanuvchi nomini olish imkonsiz");
}
