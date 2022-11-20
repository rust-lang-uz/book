## Ilova A: Kalit so'zlar

Quyidagi ro'yxatda Rust tili tomonidan joriy yoki kelajakda foydalanish uchun ajratilgan kalit so'zlar mavjud. Shunday qilib, ularni identifikator sifatida ishlatib bo'lmaydi ( “[Raw
identifikatorlar][raw-identifiers]<!-- ignore -->” bo‘limida muhokama qilinadigan raw identifikatorlar bundan mustasno.). Identifikatorlar - bu funksiyalar, o'zgaruvchilar, parametrlar, struktura maydonlari, modullar, cratelar, doimiylar, makroslar, statik qiymatlar, atributlar, turlar, belgilar yoki davr nomlari.

[raw-identifiers]: #raw-identifiers

### Hozirda foydalanilayotgan kalit so'zlar

Quyida hozirda ishlatilayotgan kalit so‘zlar ro‘yxati, ularning funksiyalari tasvirlangan.

* `as` - ibtidoiy kastingni amalga oshiring, o'z ichiga olgan o'ziga xos xususiyatni aniqlang
   elementni yoki `use` iboralaridagi elementlarning nomini o'zgartiring
* `async` -  joriy mavzuni bloklash o'rniga `Future` ni qaytarish
* `await` - `Future` natijasi tayyor bo'lgunga qadar ijroni to'xtatib turish
* `break` - zudlik bilan tsikldan chiqish
* `const` - doimiy elementlarni yoki doimiy raw ko'rsatkichlarni aniqlash
* `continue` - keyingi sikl iteratsiyasiga davom etish
* `crate` - modul yo'lida, crate ildiziga ishora qiladi
* `dyn` - xususiyat obyektiga dinamik jo'natish
* `else` - `if` va `if let` oqim konstruksiyalarini boshqarish uchun zaxira
* `enum` - ro'yxatni belgilash
* `extern` - tashqi funksiya yoki o‘zgaruvchini bog‘lash
* `false` - Mantiqiy noto'g'ri ma'noda
* `fn` - funktsiya yoki funktsiya ko'rsatkichi turini aniqlash
* `for` - iteratordagi elementlarni aylantiring, xususiyatni amalga oshiring yoki yuqori darajali ishlash muddatini belgilang
* `if` - shartli ifoda natijasiga asoslangan branch
* `impl` - o'ziga xos yoki xususiyat funksionalligini amalga oshirish
* `in` - `for` sikl sintaksisining bir qismi
* `let` - o'zgaruvchini bog'lash
* `loop` - shartsiz aylanish
* `match` - qiymatni patternga moslashtirish
* `mod` - modulni aniqlash
* `move` - yopishni uning barcha qo'lga olishlariga egalik qilish
* `mut` - havolalar, raw ko'rsatkichlar yoki pattern bog'lashlardagi o'zgaruvchanlikni bildiradi
* `pub` - struct maydonlari, `impl` bloklari yoki modullarda ommaviy ko'rinishni bildiradi
* `ref` - havola orqali bog'lash
* `return` - funktsiyani qaytarish
* `Self` - biz belgilayotgan yoki amalga oshirayotgan tur uchun turdagi taxallus
* `self` - usul mavzusi yoki joriy modul
* `static` - global o'zgaruvchi yoki butun dasturning bajarilishi uchun davr
* `struct` - strukturani aniqlash
* `super` - joriy modulning asosiy moduli
* `trait` - xususiyatni aniqlash
* `true` - Mantiqiy to'g'ri ma'noda
* `type` - turdagi taxallus yoki bog'langan turni aniqlash
* `union` - define a [union][union]<!-- ignore -->; is only a keyword when used
  in a union declaration
* `unsafe` - xavfli kod, funktsiyalar, xususiyatlar yoki ilovalarni bildiradi
* `use` - belgilarni qamrab olish
* `where` - turni cheklovchi bandlarni bildiradi
* `while` - ifoda natijasi asosida shartli sikl

[union]: ../reference/items/unions.html

### Kelajakda foydalanish uchun ajratilgan kalit so'zlar

Quyidagi kalit so'zlar hali hech qanday funksiyaga ega emas, lekin kelajakda foydalanish uchun Rust tomonidan zahiralangan.

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### Raw identifikatorlar

*Raw identifikatorlar* odatda ruxsat berilmagan kalit so'zlardan foydalanish imkonini beruvchi sintaksisdir. Kalit so'zni `r#` bilan oldindan belgilash orqali siz raw identifikatordan foydalanasiz.

Masalan, `match` kalit so'zdir. Agar siz quyidagi funktsiyani kompilyatsiya qilishga harakat qilsangiz
nomi sifatida `match` dan foydalanadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

siz ushbu xatoni olasiz:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Xato, siz kalit so'z `match` funktsiya identifikatori sifatida ishlata olmasligingizni ko'rsatadi. `match` dan funksiya nomi sifatida foydalanish uchun siz raw identifikator sintaksisidan foydalanishingiz kerak, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Ushbu kod hech qanday xatosiz kompilyatsiya qilinadi. Funktsiya nomidagi `r#` prefiksini uning ta'rifida, shuningdek, funktsiyaning `main` da chaqirilgan joyiga e'tibor bering.

Raw identifikatorlari siz tanlagan har qanday so'zni identifikator sifatida ishlatishga imkon beradi, hatto bu so'z zaxiralangan kalit so'z bo'lsa xam. Bu bizga identifikator nomlarini tanlashda ko'proq erkinlik beradi, shuningdek, bu so'zlar kalit so'zlar bo'lmagan tilda yozilgan dasturlar bilan integratsiyalashish imkonini beradi. Bundan tashqari, raw identifikatorlar
sizga cratengizdan ko'ra boshqa Rust nashrida yozilgan kutubxonalardan foydalanish imkonini beradi. Misol uchun, `try` 2015 yilgi nashrda kalit so'z emas, balki 2018 yildagi nashrda mavjud. Agar siz 2015 yilgi nashrdan foydalangan holda yozilgan kutubxonaga bog'liq bo'lsangiz va
`try` funksiyasiga ega boʻlsa, siz `r#try` da raw identifikator sintaksisidan foydalanishingiz kerak boʻladi bu holda, ushbu funktsiyani 2018 yilgi nashr kodingizdan chaqirish uchun.Nashrlar haqida qo'shimcha ma'lumot olish uchun [E Iloavsi][appendix-e]<!-- ignore --> ga qarang.


[appendix-e]: appendix-05-editions.html
