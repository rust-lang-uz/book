##  `if let` bilan  Control Flow

`if let` sintaksisi sizga `if` va `let` ni birlashtirib, qolganlarini e'tiborsiz qoldirib, bitta patternga mos keladigan qiymatlarni boshqarishning kamroq batafsil metodiga imkon beradi. 6-6 ro'yxatdagi dasturni ko'rib chiqaylik, u `max_sozlama` o'zgaruvchisidagi `Variant<u8>` qiymatiga mos keladigan, lekin `Some` varianti boʻlsagina kodni bajarishni xohlaydigan dasturni koʻrib chiqamiz.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">Ro'yxat 6-6. Qiymat `Some` bo'lsagina kodni bajaradigan `match` ifoda</span>

Agar qiymat `Some` bo'lsa, biz qiymatni patterndagi `max` o'zgaruvchisiga bog'lash orqali `Some` variantidagi qiymatni chop qilamiz. Biz `None` qiymati bilan hech narsa qilishni xohlamaymiz. `match` ifodasini qondirish uchun faqat bitta variantni qayta ishlagandan so‘ng `_ => ()` qo‘shishimiz kerak, bu esa qo‘shish uchun zerikarli boilerplate koddir.

Buning o'rniga, biz buni qisqaroq qilib `if let` yordamida yozishimiz mumkin. Quyidagi kod 6-6 ro'yxatdagi `match` bilan bir xil ishlaydi:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` sintaksisi teng belgisi bilan ajratilgan pattern va ifodani oladi. U xuddi `match` bilan ishlaydi, bunda ifoda `match`ga beriladi va pattern uning birinchi armi hisoblanadi. Bunday holda, pattern `Some(max)` bo'lib, `max`  `Some` ichidagi qiymatga bog'lanadi. Shundan so'ng biz `if let` blokining tanasida `max` dan xuddi mos keladigan `match` armida `max` dan foydalanganimiz kabi foydalanishimiz mumkin. Qiymat patternga mos kelmasa, `if let` blokidagi kod ishga tushmaydi.

`if let` dan foydalanish kamroq yozish, kamroq chekinish va kamroq kodli kodni bildiradi.
Biroq, siz `match` amal qiladigan to'liq tekshirishni yo'qotasiz. `match` va `if let` o‘rtasida tanlov qilish sizning muayyan vaziyatingizda nima qilayotganingizga va ixchamlikka ega bo‘lish to‘liq tekshirishni yo‘qotish uchun to‘g‘ri kelishilganligiga bog‘liq.

Boshqacha qilib aytganda, siz `if let` konstruktsiyasini `match` uchun sintaktik shakar sifatida o'ylab ko'rishingiz mumkin, agar kiritilgan qiymat bitta patterga mos kelsa va boshqa barcha qiymatlarga e'tibor bermasa, kodni bajaradi.

Biz `else`ni `if let` bilan kiritishimiz mumkin. `else` bilan birlashtirilgan kod bloki `if let` va `else`ga ekvivalent bo‘lgan `match` ifodasidagi `_` registriga mos keladigan kod bloki bilan bir xil. 6-4 roʻyxatdagi `Tanga` definitionni eslang, bunda `Quarter` varianti ham `UsState` qiymatiga ega edi. Agar biz quarterlarning holatini e'lon qilishda ko'rgan barcha quarter bo'lmagan tangalarni sanashni istasak, buni quyidagi kabi `match` ifodasi bilan qilishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Yoki `if let` va `else` ifodalaridan foydalanishimiz mumkin, masalan:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

Agar dasturingizda `match` yordamida ifodalash uchun juda batafsil mantiq mavjud bo'lsa, Rust toolboxda `if let` ham mavjudligini unutmang.

## Xulosa

Endi biz sanab o'tilgan qiymatlar to'plamidan biri bo'lishi mumkin bo'lgan maxsus turlarni yaratish uchun enumlardan qanday foydalanishni ko'rib chiqdik. Biz standart kutubxonaning `Option<T>` turi xatolarni oldini olish uchun type tizimidan qanday foydalanishni ko'rsatdik. Enum qiymatlari ichida ma'lumotlar mavjud bo'lsa, siz qancha holatlarni ko'rib chiqishingiz kerakligiga qarab, ushbu qiymatlarni ajratib olish va ishlatish uchun `match` yoki `if let` dan foydalanishingiz mumkin.

Rust dasturlaringiz endi structlar va enumlar yordamida domeningizdagi tushunchalarni ifodalashi mumkin. API-da foydalanish uchun maxsus turlarni yaratish turdagi xavfsizligini ta'minlaydi: kompilyator sizning funksiyalaringiz faqat har bir funktsiya kutgan turdagi qiymatlarni olishiga ishonch hosil qiladi.

Foydalanuvchilaringizga foydalanish uchun qulay va faqat sizning foydalanuvchilarga nima kerakligini aniq ko'rsatadigan yaxshi tashkil etilgan API taqdim etish uchun endi Rust modullariga murojaat qilaylik.

