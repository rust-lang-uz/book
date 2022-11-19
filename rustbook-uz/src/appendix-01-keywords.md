## Ilova A: Kalit so'zlar

Quyidagi ro'yxatda Rust tili tomonidan joriy yoki kelajakda foydalanish uchun ajratilgan kalit so'zlar mavjud. Shunday qilib, ularni identifikator sifatida ishlatib bo‘lmaydi (biz “ [Xom identifikatorlar] ” bo‘limida muhokama qiladigan xom identifikatorlardan tashqari<!-- e'tibor bermaslik --> " Bo'lim). Identifikatorlar funksiyalar, o'zgaruvchilar, parametrlar, struktura maydonlari, modullar, qutilar, konstantalar, makroslar, statik qiymatlar, atributlar, turlar, belgilar yoki hayot muddatlarining nomlari.

### Hozirda foydalanilayotgan kalit so'zlar

Quyida hozirda ishlatilayotgan kalit so‘zlar ro‘yxati, ularning funksiyalari tasvirlangan.

- `as` - ibtidoiy kastingni amalga oshiring, elementni o'z ichiga olgan o'ziga xos xususiyatni ajratib ko'rsating yoki `use` bayonotlarida elementlarning nomini o'zgartiring
-  `async` - joriy ipni bloklash o'rniga `Future` ni qaytaring
-  `await` - `Future` natijasi tayyor bo'lgunga qadar ijroni to'xtatib turish
-  `break` - zudlik bilan tsikldan chiqish
-  `const` - doimiy elementlarni yoki doimiy xom ko'rsatkichlarni aniqlang
-  `continue` eting - keyingi sikl iteratsiyasiga davom eting
-  `crate` - modul yo'lida, sandiq ildiziga ishora qiladi
-  `dyn` - xususiyat ob'ektiga dinamik jo'natish
-  `else` - `if` va `if let` boshqaruv oqimi konstruksiyalari uchun zaxira
-  `enum` - raqamni aniqlang
-  `extern` - tashqi funktsiya yoki o'zgaruvchini bog'lash
-  `false` - mantiqiy noto'g'ri harf
-  `fn` - funktsiya yoki funktsiya ko'rsatkichi turini aniqlang
-  `for` - iteratordagi elementlarni aylantiring, xususiyatni amalga oshiring yoki yuqori darajali ishlash muddatini belgilang
-  `if` - shartli ifoda natijasiga asoslangan filial
-  `impl` - o'ziga xos yoki xususiyat funksionalligini amalga oshirish
-  `in` - `for` loop sintaksisining bir qismi
-  `let` - o'zgaruvchini bog'lash
-  `loop` - so'zsiz aylanish
-  `match` - qiymatni naqshlarga moslashtirish
-  `mod` - modulni aniqlash
-  `move` - yopishni uning barcha qo'lga olishlariga egalik qilish
-  `mut` - havolalar, xom ko'rsatkichlar yoki naqsh bog'lashlardagi o'zgaruvchanlikni bildiradi
-  `pub` - struct maydonlarida, `impl` bloklarida yoki modullarda ommaviy ko'rinishni bildiradi
-  `ref` - mos yozuvlar bo'yicha bog'lash
-  `return` - funktsiyadan qaytish
-  `Self` - biz belgilayotgan yoki amalga oshirayotgan tur uchun turdagi taxallus
-  o'z- `self` usul mavzusi yoki joriy modul
-  `use` - belgilarni qamrab olish
-  `where` - turni cheklovchi gaplarni bildiradi
-  `while` - ifoda natijasi asosida shartli sikl

### Kelajakda foydalanish uchun ajratilgan kalit so'zlar

Quyidagi kalit so'zlar hali hech qanday funksiyaga ega emas, lekin kelajakda foydalanish uchun Rust tomonidan zahiralangan.

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### Xom identifikatorlar

*Xom identifikatorlar* odatda ruxsat berilmagan kalit so'zlardan foydalanish imkonini beruvchi sintaksisdir. Kalit so'zni `r#` bilan oldindan belgilash orqali siz xom identifikatordan foydalanasiz.

Masalan, `match` kalit so'zdir. Agar siz o'z nomi sifatida `match` dan foydalanadigan quyidagi funktsiyani kompilyatsiya qilmoqchi bo'lsangiz:

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

Xato, siz kalit so'z `match` funktsiya identifikatori sifatida ishlata olmasligingizni ko'rsatadi. `match` dan funksiya nomi sifatida foydalanish uchun siz xom identifikator sintaksisidan foydalanishingiz kerak, masalan:

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

Raw identifikatorlari identifikator sifatida tanlagan har qanday so'zdan foydalanishga imkon beradi, hatto bu so'z zahiradagi kalit so'z bo'lsa ham. Bu bizga identifikator nomlarini tanlashda ko'proq erkinlik beradi, shuningdek, bu so'zlar kalit so'zlar bo'lmagan tilda yozilgan dasturlar bilan integratsiyalashish imkonini beradi. Bundan tashqari, raw identifikatorlari sizga kassadagidan boshqa Rust nashrida yozilgan kutubxonalardan foydalanish imkonini beradi. Misol uchun, `try` 2015 yilgi nashrda kalit so'z emas, balki 2018 yilgi nashrda. Agar siz 2015-yil nashri yordamida yozilgan va `try` funksiyasiga ega kutubxonaga bogʻliq boʻlsangiz, 2018-yilgi nashr kodingizdan ushbu funksiyani chaqirish uchun xom identifikator sintaksisidan foydalanishingiz kerak boʻladi, bu holda `r#try` . [E ilovasiga] qarang<!-- e'tibor bermaslik --> nashrlar haqida ko'proq ma'lumot olish uchun.


[Xom identifikatorlar]: #raw-identifiers
[E ilovasiga]: appendix-05-editions.html