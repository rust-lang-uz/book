fn main() {
    let s1 = egalik_beradi();         // egalik_beradi o'zining return qiymatini
                                        // s1 ga o'tkazadi

    let s2 = String::from("salom");     // s2 scopega kiradi

    let s3 = oladi_va_qaytaradi(s2);  // s2 oladi_va_qaytaradi ichiga 
                                        // ko'chiriladi, u ham o'zining return
                                        // qiymatini s3 ga o'tkazadi
} // Bu erda s3 scopedan chiqib ketadi va o'chiriladi. s2 ko'chirildi, shuning uchun
  // hech narsa sodir bo'lmaydi. s1 scopedan chiqib ketadi va o'chiriladi.

fn egalik_beradi() -> String {             // egalik_beradi o'zining return
                                             // qiymatini uni chaqiradigan
                                             // funksiyaga o'tkazadi

    let some_string = String::from("rust"); // some_string scopea kiradi

    some_string                              // some_string return qilinadi va
                                             // chaqiruv funksiyasiga 
                                             // o'tadi
}

// Bu funksiya Stringni oladi va bittasini qaytaradi
fn oladi_va_qaytaradi(a_string: String) -> String { // a_string scopega 
                                                      // kiradi

    a_string  // a_string qaytariladi va chaqiruv funksiyasiga o'tadi
}
