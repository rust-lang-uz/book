## Structlar yordamida namunaviy dastur

Structlarni qachon ishlatishimiz mumkinligini tushunish uchun to'rtburchakning maydonini hisoblaydigan dastur yozaylik. Biz bitta o'zgaruvchilardan foydalanishni boshlaymiz, so'ngra uning o'rniga structlardan foydalanmagunimizcha dasturni qayta yaxshilab boramiz.

Keling, cargo bilan *kvadratlar* deb nomlangan yangi binary loyihani yarataylik, u piksellarda ko'rsatilgan to'rtburchakning kengligi va balandligini oladi va to'rtburchakning maydonini hisoblaydi. 5-8 ro'yxatda loyihaning *src/main.rs* faylida nima qilishimiz kerakligini aniq bajarishga imkon beradigan bitta qisqa kod ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">Ro'yxat 5-8: Alohida kenglik va balandlik o'zgaruvchilari bilan belgilangan to'rtburchaklar maydonini hisoblash</span>

Endi ushbu dasturni `cargo run` yordamida ishga tushiring:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Ushbu kod har bir o'lcham bilan `area` funksiyasini chaqirish orqali to'rtburchakning maydonini aniqlashga muvaffaq bo'ladi, ammo biz ushbu kodni aniq va o'qilishi uchun ko'proq narsani qilishimiz mumkin.

Ushbu kod bilan bog'liq muammo `area` signaturesida aniq ko'rinadi:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

`area` funksiyasi bitta to'rtburchakning maydonini hisoblashi kerak, lekin biz yozgan funksiya ikkita parametrga ega va bizning dasturimizning hech bir joyida parametrlar o'zaro bog'liqligi aniq emas. Kenglik va balandlikni birgalikda guruhlash yanada o'qilishi va boshqarilishi oson bo'lishi mumkin edi.3-bobning [”Tuple Turi”][the-tuple-type]<!-- ignore --> bo'limida biz buni amalga oshirishning bir usulini, ya'ni tuplelardan foydalanishni muhokama qildik.

### Tuplelar yordamida Refaktoring

5-9 ro'yxatda tuplelardan foydalanadigan dasturimizning boshqa versiyasi ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">Ro'yxat 5-9: To'rtburchakning kengligi va balandligini tuple yordamida aniqlash</span>

Bir tomondan, bu dastur yaxshiroq. Tuplar bizga biroz struct qo'shishga imkon beradi va biz hozir faqat bitta argumentni keltiramiz. Ammo boshqa yo'l bilan, bu versiya unchalik aniq emas: tuplelar o'z elementlarini nomlamaydi, shuning uchun biz hisob-kitobimizni kamroq aniq qilib, tuple qismlariga indeks qilishimiz kerak.

Kenglik va balandlikni aralashtirish maydonni hisoblash uchun muhim emas, lekin agar biz ekranda to'rtburchak chizmoqchi bo'lsak, bu muhim bo'ladi! Shuni yodda tutishimiz kerakki, `kenglik` indeks `0` da, `balandlik` esa `1` indeksda. Agar kimdir bizning kodimizdan foydalansa, buni tushunish va yodda tutish qiyinroq bo'ladi. Kodimizda ma'lumotlarimizning ma'nosini etkazmaganimiz sababli, endi xatolarni kiritish osonroq.

### Struktuctlar bilan Refaktoring: ko'proq ma'no qo'shish

Biz ma'lumotlarni etiketlash orqali ma'no qo'shish uchun structlardan foydalanamiz. Biz foydalanayotgan tupleni 5-10 ro'yxatda ko'rsatilganidek, butun nomi bilan bir qatorda qismlar nomlari bilan tuzilishga aylantirishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">Ro'yxat 5-10: `Kvadrat` strukturasini aniqlash</span>

Bu yerda biz structni aniqladik va uni `Kvadrat` deb nomladik. Jingalak qavslar ichida biz maydonlarni `kenglik` va `balandlik` sifatida belgiladik, ularning ikkalasi ham `u32` turiga ega. Keyin, `main` da biz `Kvadrat` ning ma'lum bir misolini yaratdik, uning kengligi `30` va balandligi `50`.

Bizning `area` funksiyamiz endi biz `kvadrat` deb nomlagan bitta parametr bilan aniqlanadi, uning turi `Kvadrat` structi misolining o‘zgarmas borrowidir. 4-bobda aytib o'tilganidek, biz unga ownershiplik qilishdan ko'ra, structi borrow qilishni xohlaymiz. Shunday qilib, `main` o'z ownershipini saqlab qoladi va `kvadrat1` dan foydalanishni davom ettirishi mumkin, shuning uchun biz funktsiya signaturesida `&` dan foydalanamiz va biz funktiyani chaqiramiz.

`area` funksiyasi `Kvadrat` misolining `kenglik` va `balandlik`  maydonlariga kiradi (esda tutingki, borrow qilingan struct misolining maydonlariga kirish maydon qiymatlarini ko'chirmaydi, shuning uchun siz ko'pincha structlarning borrowlarini ko'rasiz). Endi `area` funksiyasi signaturesi biz nimani nazarda tutayotganimizni aniq aytadi: `Kvadrat` maydonini uning `kenglik` va `balandlik` maydonlaridan foydalanib hisoblang. Bu kenglik va balandlik bir-biri bilan bog'liqligini bildiradi va `0` va `1` qator indeks qiymatlarini ishlatishdan ko'ra, qiymatlarga tavsiflovchi nomlar beradi. Bu aniqlik uchun g'alaba.

### Olingan Traitlar bilan foydali funksionallikni qo'shish

Dasturimizni debug qilish va uning barcha maydonlari uchun qiymatlarni ko'rish paytida `Kvadrat` misolini chop etish foydali bo'lar edi. 5-11 ro'yxatda biz avvalgi boblarda foydalanganimizdek [`println!` makrosidan][println]<!-- ignore --> foydalanishga harakat qiladi. Biroq, bu ishlamaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">Ro'yxat 5-11: `Kvadrat`ni chop etishga urinish
misoli</span>

Ushbu kodni kompilyatsiya qilishda, biz ushbu asosiy xabar bilan xatoga duch kelamiz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` makrosi ko'plab formatlash turlarini amalga oshirishi mumkin va standart bo'yicha jingalak qavslar `println!`ga `Display` deb nomlanuvchi formatlashdan foydalanishni bildiradi: to'g'ridan-to'g'ri oxirgi foydalanuvchi iste'moli uchun mo'ljallangan chiqish. Biz hozirgacha ko'rgan primitiv turlar standart bo'yicha `Display` ni qo'llaydi, chunki foydalanuvchiga `1` yoki boshqa primitiv turni ko'rsatishning faqat bitta usuli bor. Lekin structlar bilan `println!` ning chiqishni formatlash metodi unchalik aniq emas, chunki koʻproq koʻrsatish imkoniyatlari mavjud: vergul qoʻyishni xohlaysizmi yoki yoʻqmi? Jingalak qavslarni chop qilmoqchimisiz? Barcha maydonlar ko'rsatilishi kerakmi? Ushbu noaniqlik tufayli Rust biz xohlagan narsani taxmin qilishga urinmaydi va structlarda `println!` va `{}` to'ldiruvchisi bilan foydalanish uchun `Display` ning taqdim etilgan ilovasi yo'q.

Agar xatolarni o'qishda davom etsak, biz ushbu foydali eslatmani topamiz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Keling, sinab ko'raylik! `println!` macro chaqiruvi endi `println!("kvadrat1 bu {}", kvadrat1);` kabi ko'rinadi. `:?` spetsifikatsiyasini jingalak qavslar ichiga qo'yish `println!` biz `Debug` deb nomlangan chiqish formatidan foydalanmoqchi ekanligimizni bildiradi. `Debug` traiti bizga sturctni ishlab chiquvchilar uchun foydali bo'lgan tarzda chop etish imkonini beradi, shuning uchun biz kodimizni tuzatish paytida uning qiymatini ko'rishimiz mumkin.

Keling, ushbu o'zgarishlar bilan kodni kompilyatsiya qilaylik. Ehh! Biz hali ham xatoni olamiz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Ammo yana, kompilyator bizga foydali eslatma beradi:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust debug ma'lumotlarini chop etish funksiyasini o'z ichiga oladi, lekin biz ushbu funksiyani structimiz uchun mavjud qilish uchun ochiqdan-ochiq rozi bo'lishimiz kerak.
Buni amalga oshirish uchun 5-12 ro'yxatda ko'rsatilganidek, struct ta'rifidan oldin `#[derive(Debug)]` tashqi atributini qo'shamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">Ro'yxat 5-12: `Debug` traitini olish uchun atribut qo‘shish va debug formatlash yordamida `Kvadrat` misolini chop etish</span>

Endi dasturni ishga tushirganimizda, biz hech qanday xatolikka yo'l qo'ymaymiz va biz quyidagi natijani ko'ramiz:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Yaxshi! Bu eng yaxshi natija emas, lekin u ushbu misol uchun barcha maydonlarning qiymatlarini ko'rsatadi, bu disk raskadrovka paytida albatta yordam beradi. Kattaroq structlarga ega bo'lsak, o'qishni biroz osonlashtiradigan chiqishga ega bo'lish foydalidir; bunday hollarda `println!` qatoridagi `{:?}` o'rniga `{:#?}` dan foydalanishimiz mumkin. Ushbu misolda `{:#?}` uslubidan foydalanish quyidagi natijalarni beradi:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

`Debug` formati yordamida qiymatni chop etishning yana bir usuli [`dbg!` macro][dbg]<!-- ignore --> Ifodaga egalik qiluvchi macro (mos referencelar oladigan println! dan farqli o'laroq) o'sha `dbg!` qayerda fayl va satr raqamini chop etadi! macro murojati sizning kodingizda ushbu ifodaning natijaviy qiymati bilan birga sodir bo'ladi va qiymatga egalik huquqini qaytaradi.

> Eslatma: `dbg!` makrosini chaqirish standart chiqish konsoli stremiga
> (`stdout`) chop qiluvchi `println!`dan farqli ravishda standart xato
> konsoli stremiga (`stderr`) chop etadi. Biz 12-bobdagi [”Xato xabarlarini standart
> chiqish o‘rniga standart xatoga yozish”][err]<!-- ignore --> bo‘limida `stderr` va `stdout` haqida ko‘proq
> gaplashamiz.

Mana bizni `kenglik` maydoniga tayinlanadigan qiymat, shuningdek `kvadrat1`dagi butun structning qiymati qiziqtiradigan misol:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

Biz `30 * masshtab` iborasi atrofida `dbg!` qo'yishimiz mumkin va `dbg!` ifoda qiymatiga egalik huquqini qaytargani uchun, `kenglik` maydoni bizda `dbg!` chaqiruvi bo'lmagani kabi bir xil qiymatga ega bo'ladi. Biz `dbg!` `kvadrat1`ga egalik qilishini istamaymiz, shuning uchun keyingi chaqiruvda `kvadrat1` ga referencedan foydalanamiz.
Ushbu misolning natijasi quyidagicha ko'rinadi:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

Biz birinchi debuggingda *src/main.rs* ning 10-qatoridan kelganini ko'rishimiz mumkin, bu erda biz *30 * masshtab* ifodani debugging qilamiz va uning natijaviy qiymati `60` (butun sonlar uchun `Debug` formati faqat ularning qiymatini chop etish uchun ishlatiladi). *src/main.rs* ning 14-qatoridagi `dbg!` chaqiruvi `&kvadrat1` qiymatini chiqaradi, bu `Kvadrat` structidir. Ushbu chiqishda `Kvadrat` turidagi chiroyli `Debug` formatlash qo'llaniladi. `dbg!` makrosi sizning kodingiz nima qilayotganini aniqlashga harakat qilayotganingizda juda foydali bo'lishi mumkin!

Rust `Debug` traitiga qo‘shimcha ravishda `derive` atributi bilan foydalanishimiz uchun bir qancha taritlarni taqdim etdi, ular bizning odatiy turlarimizga foydali xatti-harakatlar qo‘shishi mumkin.Ushbu traitlar va ularning xatti-harakatlari [C ilovasida][app-c]<!-- ignore --> keltirilgan. Biz 10-bobda ushbu traittlarni odatiy xatti-harakatlar bilan qanday implement qilishni, shuningdek, o'z traitlaringizni qanday yaratishni ko'rib chiqamiz.Bundan tashqari, 'derive' dan boshqa ko'plab atributlar mavjud; qo'shimcha ma'lumot olish uchun [Rust Referencening "Atributlar" bo'limiga][attributes] qarang.

Bizning `area` funksiyamiz juda aniq: u faqat to'rtburchaklar maydonini hisoblaydi. Ushbu xatti-harakatni `Kvadrat` structimiz bilan yaqinroq bog'lash foydali bo'ladi, chunki u boshqa turlar bilan ishlamaydi. Keling, ushbu kodni qanday qilib qayta ishlashni davom ettirishimiz mumkinligini ko'rib chiqaylik, bu `area` funksiyasini `Kvadrat` turida aniqlangan `area` *method* ga aylantiradi.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
