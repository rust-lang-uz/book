## Paketlar va Cratelar

Biz qamrab oladigan modul tizimining birinchi qismlari paketlar va cratelardir.

*Crate* - bu Rust kompilyatori bir vaqtning o'zida ko'rib chiqadigan eng kichik kod miqdori. Agar siz `cargo` o'rniga `rustc` ni ishga tushirsangiz va bitta manba faylga o'tsangiz ham (biz 1-bob, "Rust dasturini yozish va ishga tushirish"da qilganimiz kabi), kompilyator bu faylni crate sifatida ko'radi. Cratelar modullarni o'z ichiga olishi mumkin va modullar crate bilan tuzilgan boshqa fayllarda aniqlanishi mumkin, buni keyingi bo'limlarda ko'rib chiqamiz.

Crate ikkita shakldan birida bo'lishi mumkin: binary crate yoki kutubxona cratesi.
*Binary cratelar* - bu buyruq qatori dasturi yoki server kabi ishga tushirishingiz mumkin bo'lgan bajariladigan faylga kompilyatsiya qilishingiz mumkin bo'lgan dasturlar. Har birida bajariladigan fayl ishga tushganda nima sodir bo'lishini belgilaydigan `main` funksiyasi bo'lishi kerak. Biz hozirgacha yaratgan barcha cratelar binary cratelar edi.

*Kutubxona cratelari* `main` funksiyaga ega emas va ular bajariladigan faylga kompilyatsiya qilinmaydi. Buning o'rniga, ular bir nechta loyihalar bilan bo'lishish uchun mo'ljallangan funksionallikni belgilaydi. Misol uchun, biz [2-bobda][rand]<!-- ignore -->  ishlatgan `rand` cratesi tasodifiy sonlarni yaratuvchi funksionallikni taʼminlaydi.
Ko'pincha Rustaceanlar  “crate” deganda, ular kutubxona crateni anglatadi va ular "kutubxona" ning umumiy dasturlash tushunchasi bilan "crate" dan foydalanadilar.

*Crate root* bu Rust kompilyatori cratengizning ildiz modulini yaratishni boshlaydigan manba fayldir (biz modullarni [“Qo'llanish doirasi va maxfiylikni nazorat qilish uchun modullarni aniqlash”][modules]<!-- ignore --> bo‘limida chuqur tushuntiramiz).

*Paket* - bu bir yoki bir nechta cratelar to'plami bo'lib, u funksiyalar to'plamini ta'minlaydi. Paketda ushbu cratelarni qanday build qilishni tavsiflovchi *Cargo.toml* fayli mavjud. Cargo - bu sizning kodingizni yaratishda foydalanayotgan buyruq qatori vositasi uchun binary crateni o'z ichiga olgan paket. Cargo paketida binary crate bog'liq bo'lgan kutubxona cratesi ham mavjud. Boshqa loyihalar Cargo buyruq qatori vositasi ishlatadigan mantiqdan foydalanish uchun cargo kutubxonasi cratesiga bog'liq bo'lishi mumkin.

Paket siz xohlagancha ko'p ikkilik binary cratelarni o'z ichiga olishi mumkin, lekin bitta kutubxona cratesidan ko'p bo'lmasligi kerak. Paketda kamida bitta crate bo'lishi kerak, u kutubxona yoki binary crate bo'lishi kerak.

Keling, paketni yaratganimizda nima sodir bo'lishini ko'rib chiqaylik. Birinchidan, biz `cargo new` buyrug'ini kiritamiz:

```console
$ cargo new mening-paketim
     Created binary (application) `mening-paketim` package
$ ls mening-paketim
Cargo.toml
src
$ ls mening-paketim/src
main.rs
```

Biz `cargo new` ni ishga tushirgandan so'ng, biz cargo nima yaratganini ko'rish uchun `ls` dan foydalanamiz. `mening-paketim` jildida bizga paketni taqdim qiluvchi *Cargo.toml* fayli mavjud. Shuningdek, *main.rs* ni o'z ichiga olgan *src* jildi ham mavjud. Matn muharririda *Cargo.toml* ni oching va *src/main.rs* haqida hech qanday eslatma yo‘qligiga e'tibor bering. Cargo, *src/main.rs* paket bilan bir xil nomga ega binary cratening crate ildizi ekanligi haqidagi konventsiyaga amal qiladi. Xuddi shunday, Cargo ham biladiki, agar paket jildida *src/lib.rs* bo'lsa, paketda paket bilan bir xil nomdagi kutubxona cratesi mavjud va *src/lib.rs* uning crate ildizi hisoblanadi. Cargo kutubxona yoki binaryni yaratish uchun cratening ildiz fayllarini `rustc` ga o'tkazadi.

Bu yerda bizda faqat *src/main.rs* ni o'z ichiga olgan paket bor, ya'ni u faqat `mening-paketim` nomli binary crateni o'z ichiga oladi. Agar paketda `src/main.rs` va `src/lib.rs` bo'lsa, unda ikkita crate, binary crate va kutubxona cratesi mavjud bo'lib, ikkalasi ham paket bilan bir xil nomga ega. Paketda fayllarni *src/bin* jildiga joylashtirish orqali bir nechta binary cratelar bo'lishi mumkin: har bir fayl alohida binary crate bo'ladi.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
