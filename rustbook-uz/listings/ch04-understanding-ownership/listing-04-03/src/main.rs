fn main() {
    let s = String::from("salom");  // s scopega kiradi

    ownershiplik_qiladi(s);             // s qiymati funksiyaga o'tadi ...
                                    // ... va shuning uchun bu yerda endi amal qilmaydi

    let x = 5;                      // x scopega kiradi

    nusxasini_yaratadi(x);                  // x funksiyaga o'tadi,
                                    // lekin i32 nusxa ko'chirish, shuning uchun tinch qo'yish yaxshidir
                                    // keyin x dan foydalaning

} // Bu erda x scopedan chiqib ketadi, keyin s. Lekin s qiymati ko'chirilganligi sababli, hech
  // qanday maxsus narsa sodir bo'lmaydi.

fn ownershiplik_qiladi(some_string: String) { // some_string scopega kiradi
    println!("{}", some_string);
} // Bu yerda some_string scopedan chiqib ketadi va `drop` deb ataladi. Qo'llab-quvvatlovchi
  // xotira bo'shatiladi.

fn nusxasini_yaratadi(some_integer: i32) { // some_integer scopega kiradi
    println!("{}", some_integer);
} // Bu erda some_integer scopedan tashqariga chiqadi. Hech qanday maxsus narsa bo'lmaydi.
