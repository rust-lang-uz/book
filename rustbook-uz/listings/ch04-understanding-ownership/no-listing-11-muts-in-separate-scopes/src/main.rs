fn main() {
    // ANCHOR: here
    let mut s = String::from("salom");

    {
        let r1 = &mut s;
    } // r1 bu yerda scopedan chiqib ketadi, shuning uchun biz hech 
    //qanday muammosiz yangi reference qilishimiz mumkin.

    let r2 = &mut s;
    // ANCHOR_END: here
}
