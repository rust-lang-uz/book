## Structlarni aniqlash va yaratish

Structlar [“Tuple turi“][tuples]<!-- ignore --> bo'limida muhokama qilingan tuplelarga o'xshaydi, chunki ikkalasi ham bir-biriga bog'liq bo'lgan bir nechta qiymatlarga ega.
Tuplelar singari, structning qismlari ham har xil turdagi bo'lishi mumkin. Tuplelardan farqli o'laroq, structda siz har bir ma'lumot qismini nomlaysiz, shunda qiymatlar nimani anglatishini tushunasiz. Ushbu nomlarni qo'shish structlar tuplelardan ko'ra moslashuvchanroq ekanligini anglatadi: misol qiymatlarini belgilash yoki ularga kirish uchun ma'lumotlar tartibiga ishonishingiz shart emas.

Structni aniqlash uchun biz `struct` kalit so`zini kiritamiz va butun tuzilishga nom beramiz. Struct nomi birgalikda guruhlangan ma'lumotlar bo'laklarining ahamiyatini tavsiflashi kerak. Keyin, jingalak qavslar ichida biz *maydonlar* deb ataydigan ma'lumotlar qismlarining nomlari va turlarini aniqlaymiz. Masalan, 5-1 ro'yxati foydalanuvchi hisobi haqidagi ma'lumotlarni saqlaydigan structni ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-1: `Foydalanuvchi` structi ta'rifi</span>

Struct aniqlangandan so'ng, tegishli ma'lumotlar turiga ega bo'lgan har bir maydonga ma'lum bir qiymat berish orqali uni yaratish mumkin. Biz structning nomini ko'rsatish orqali misol yaratamiz va keyin *kalit: qiymat*(key: value) juftlarini o'z ichiga olgan jingalak qavslarni qo'shamiz, bu erda kalitlar maydonlarning nomlari va qiymatlar biz o'sha maydonlarda saqlamoqchi bo'lgan ma'lumotlardir.Biz maydonlarni structda e'lon qilgan tartibda ko'rsatishimiz shart emas. Boshqacha qilib aytganda, structning ta'rifi tur uchun umumiy shablonga o'xshaydi va misollar tur qiymatlarini yaratish uchun ushbu shablonni ma'lum ma'lumotlar bilan to'ldiradi. Misol uchun, biz 5-2 ro'yxatda ko'rsatilganidek, ma'lum bir foydalanuvchini e'lon qilishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-2: `Foydalanuvchi` nusxasini yaratish
structi</span>

Structdan ma'lum bir qiymat olish uchun biz nuqta belgilaridan foydalanamiz. Masalan, ushbu foydalanuvchining elektron pochta manziliga kirish uchun biz `foydalanuvchi1.email` dan foydalanamiz. Agar misol o'zgaruvchan bo'lsa, biz nuqta belgisi yordamida qiymatni o'zgartirishimiz va ma'lum bir maydonga belgilashimiz mumkin. 5-3 ro'yxatda o'zgaruvchan `Foydalanuvchi` misolining `email` maydonidagi qiymatni qanday o'zgartirish mumkinligi ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-3: `Foydalanuvchi` misolining `email` maydonidagi qiymatni o'zgartirish</span>

E'tibor bering, butun misol o'zgaruvchan bo'lishi kerak; Rust bizga faqat ma'lum maydonlarni o'zgaruvchan deb belgilashga ruxsat bermaydi. Har qanday ifodada bo'lgani kabi, biz ushbu yangi misolni bilvosita qaytarish uchun funksiya tanasidagi oxirgi ifoda sifatida structning yangi nusxasini qurishimiz mumkin.

5-4 roʻyxatda berilgan email va foydalanuvchi nomi bilan `Foydalanuvchi` misolini qaytaruvchi `foydalanuvchi_yaratish` funksiyasi koʻrsatilgan. `faollik` maydoni `true` qiymatini, `kirish_hisobi` esa `1` qiymatini oladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Roʻyxat 5-4: `foydalanuvchi_yaratish` funksiyasi email va foydalanuvchi nomini oladi va `Foydalanuvchi` misolini qaytaradi</span>

Funksiya parametrlarini struct maydonlari bilan bir xil nom bilan nomlash mantiqan to‘g‘ri keladi, lekin `email` va `foydalanuvchi` maydon nomlari va o‘zgaruvchilarini takrorlash biroz zerikarli. Agar structda ko'proq maydonlar bo'lsa, har bir nomni takrorlash yanada zerikarli bo'ladi. Yaxshiyamki, qulay Shorthand bor!
<!-- Old heading. Do not remove or links may break. -->
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Field Init Shorthandan foydalanish

Parametr nomlari va struct maydonlarining nomlari 5-4 ro'yxatda aynan bir xil bo'lgani uchun, `foydalanuvchi_yaratish`ni qayta yozish uchun *init shorthand* sintaksisidan foydalanishimiz mumkin, shuning uchun u xuddi shunday ishlaydi, lekin `foydalanuvchi` va `email`ni takrorlamaydi, 5-5 ro'yxatda ko'rsatilganidek.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Roʻyxat 5-5: `foydalanuvchi` va `email` parametrlari struct maydonlari bilan bir xil nomga ega boʻlgani uchun init shorthanddan foydalanadigan `foydalanuvchi_yaratish` funksiyasi</span>

Bu yerda biz `Foydalanuvchi` structining yangi nusxasini yaratmoqdamiz, unda `email` deb nomlangan maydon mavjud. Biz `email` maydonining qiymatini `foydalanuvchi_yaratish` funksiyasining `email` parametridagi qiymatga o‘rnatmoqchimiz. `email` maydoni va `email` parametri bir xil nomga ega bo'lgani uchun biz `email: email` emas, balki `email` yozishimiz kerak.

### Structni update sintaksisidan foydalanib, boshqa tuzilma nusxasidan tuzilma namunasini yaratish

Ko'pincha boshqa namunadagi qiymatlarning ko'pini o'z ichiga olgan, lekin ularning ba'zilarini o'zgartiradigan structning yangi nusxasini yaratish foydali bo'ladi. Buni *struct update sintaksisi* yordamida amalga oshirishingiz mumkin.

Birinchidan, 5-6 ro'yxatda biz yangilanish sintaksisisiz muntazam ravishda `foydalanuvchi2` da yangi `Foydalanuvchi` misolini qanday yaratishni ko'rsatamiz. Biz `email` uchun yangi qiymat o‘rnatdik, lekin aks holda 5-2 ro‘yxatda yaratgan `foydalanuvchi1` qiymatidan foydalanamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Roʻyxat 5-6: foydalanuvchi1 qiymatlaridan biri yordamida yangi `Foydalanuvchi` namunasini yaratish</span>

Strukturani yangilash sintaksisidan foydalanib, biz 5-7 ro'yxatda ko'rsatilganidek, kamroq kod bilan bir xil effektga erishishimiz mumkin. `..` sintaksisi qolgan maydonlar aniq o'rnatilganligini va belgilangan namunadagi qiymatlarga ega bo'lishi kerakligini ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-7: `Foydalanuvchi` misoli uchun yangi `email` qiymatini o'rnatish, lekin `foydalanuvchi1` dagi qolgan qiymatlardan foydalanish uchun structni yangilash sintaksisidan foydalanish</span>

5-7 roʻyxatdagi kod `foydalanuvchi2` da `email` uchun boshqa qiymatga ega, lekin `foydalanuvchi1` dan `foydalanuvchi`, `faollik` va `kirish_hisobi` maydonlari uchun bir xil qiymatlarga ega boʻlgan misol yaratadi. `..foydalanuvchi1` qolgan maydonlar o‘z qiymatlarini `foydalanuvchi1` dagi tegishli maydonlardan olishi kerakligini ko‘rsatish uchun oxirgi o‘rinda turishi kerak, lekin biz istalgan tartibda xohlagancha ko'p maydonlar uchun qiymatlarni belgilashni tanlashimiz mumkin, strukturaning ta'rifidagi maydonlar tartibidan qat'i nazar.

E'tibor bering, strukturani yangilash sintaksisi topshiriq kabi `=` dan foydalanadi; Buning sababi, biz [”O'zgaruvchilar va ma'lumotlarni ko'chirish bilan o'zaro ta'sir qilish”][move]<!-- ignore --> bo'limida ko'rganimizdek, u ma'lumotlarni harakatlantiradi. Ushbu misolda biz `foydalanuvchi2` ni yaratganimizdan keyin `foydalanuvchi1` dan bir butun sifatida foydalana olmaymiz, chunki `foydalanuvchi1`ning `foydalanuvchi` maydonidagi `String` `foydalanuvchi2` ga koʻchirilgan. Agar biz `foydalanuvchi2` ga `email` va `foydalanuvchi` uchun yangi `String` qiymatlarini bergan bo‘lsak va shuning uchun `foydalanuvchi1`dan faqat `faollik` va `kirish_hisobi` qiymatlarini ishlatgan bo‘lsak, keyin `foydalanuvchi1` `foydalanuvchi2` yaratilgandan keyin ham amal qiladi. `faollik` va `kirish_hisobi` turlari nusxa ko'chirish xususiyatini amalga oshiradigan turlardir, shuning uchun biz [”Stek ma'lumotlari: Nusxalash”][copy]<!-- ignore --> bo'limida muhokama qilgan xatti-harakatlar qo'llaniladi.

### Har xil turlarni yaratish uchun nomli maydonlarsiz tuplelardan foydalanish

Rust shuningdek, *tuple struct*lar deb ataladigan tuplelarga o'xshash structlarni qo'llab-quvvatlaydi.
Tuple structlari struct nomi taqdim etadigan qo'shimcha ma'noga ega, ammo ularning maydonlari bilan bog'langan nomlari yo'q; aksincha, ular faqat maydonlarning turlariga ega. Tuple structlari butun tuplega nom berish va tupleni boshqa tuplelardan farqli turga aylantirish zarur bo‘lganda foydali bo‘ladi va har bir maydonni oddiy structdagi kabi nomlashda batafsil yoki ortiqcha bo‘ladi.

Tuple structini aniqlash uchun `struct` kalit so‘zi va struct nomidan keyin tupledagi turlardan boshlang. Masalan, bu yerda biz `Rang` va `Nuqta` nomli ikkita tuple structini aniqlaymiz va foydalanamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

Esda tutingki, `qora` va `kelib_chiqishi` qiymatlari har xil turdagi, chunki ular turli xil tuple structlarining namunalaridir. Structdagi maydonlar bir xil turlarga ega bo'lishi mumkin bo'lsa ham, siz belgilagan har bir struct o'z turiga ega. Masalan, `Rang` turidagi parametrni qabul qiladigan funksiya, har ikkala tur ham uchta `i32` qiymatidan iborat bo‘lsa ham, `Nuqta`ni argument sifatida qabul qila olmaydi. Aks holda, tuple structi namunalari tupelarga o'xshaydi, chunki siz ularni alohida qismlarga ajratishingiz mumkin va individual qiymatga kirish uchun `.` va keyin indeksdan foydalanishingiz mumkin.

### Hech qanday maydonsiz unit kabi structlar

Shuningdek, siz hech qanday maydonga ega bo'lmagan structlarni belgilashingiz mumkin! Ular *unita o'xshash structlar* deb ataladi, chunki ular biz [”Tuple turi”][tuples]<!-- ignore --> bo'limida aytib o'tgan unit turiga `()` o'xshash harakat qiladilar. Unitga o'xshash structlar qaysidir turdagi traitni qo'llash kerak bo'lganda foydali bo'lishi mumkin, ammo sizda turning o'zida saqlash uchun ma'lumotlaringiz yo'q. Traitlarni 10-bobda muhokama qilamiz. Mana `AlwaysEqual` nomli unit structini e’lon qilish va instantsiyalash misoli:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

`AlwaysEqual` ni aniqlash uchun biz `struct` kalit so'zidan, kerakli nomdan va keyin nuqta-verguldan foydalanamiz. Jingalak qavslar yoki qavslar kerak emas! Shunda biz `subject` o'zgaruvchisida `AlwaysEqual` misolini xuddi shunday tarzda olishimiz mumkin: biz belgilagan nomdan foydalanib, hech qanday jingalak qavs yoki qavslarsiz. Tasavvur qiling-a, keyinchalik biz ushbu turdagi xatti-harakatlarni shunday amalga oshiramizki, `AlwaysEqual` ning har bir nusxasi har doim boshqa turdagi har bir misolga teng bo'ladi, ehtimol sinov uchun ma'lum natijaga ega bo'lishi mumkin. Ushbu xatti-harakatni amalga oshirish uchun bizga hech qanday ma'lumot kerak emas! Traitlarni qanday aniqlash va ularni har qanday turdagi, shu jumladan unitga o'xshash structlarda amalga oshirishni 10-bobda ko'rasiz.

> ### Strukturaviy ma'lumotlarga ownershiplik qilish
>
> 5-1 roʻyxatdagi `Foydalanuvchi` structi taʼrifida biz `&str` string slice turidan
> koʻra tegishli `String` turidan foydalandik. Bu ataylab qilingan tanlov, chunki
> biz ushbu structning har bir nusxasi uning barcha maʼlumotlariga ega boʻlishini
> va bu maʼlumotlar butun struct amalda boʻlgunga qadar amal qilishini istaymiz.
>
> Structlar boshqa narsaga tegishli maʼlumotlarga referencelarni saqlashi ham mumkin,
> ammo buning uchun biz 10-bobda muhokama qiladigan Rust xususiyatidan  *lifetimelar*
> foydalanishni talab qiladi. Lifetime struct tomonidan reference qilingan ma'lumotlar
> struct mavjud bo'lgunga qadar amal qilishini ta'minlaydi. Aytaylik, siz ma'lumotnomani
> lifetimeni ko'rsatmasdan structda saqlashga harakat qildingiz, quyidagi kabi; bu ishlamaydi:
>
> <span class="filename">Fayl nomi: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct Foydalanuvchi {
>     faollik: bool,
>     foydalanuvchi: &str,
>     email: &str,
>     kirish_hisobi: u64,
> }
>
> fn main() {
>     let foydalanuvchi1 = Foydalanuvchi {
>         faollik: true,
>         foydalanuvchi: "ismoilovdev",
>         email: "ismoilovdev@example.com",
>         kirish_hisobi: 1,
>     };
> }
> ```
>
> Kompilyator referencelarning lifetimeni aniqlash zarurati haqida shikoyat qiladi:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     foydalanuvchi: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Foydalanuvchi<'a> {
> 2 |     faollik: bool,
> 3 ~     foydalanuvchi: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Foydalanuvchi<'a> {
> 2 |     faollik: bool,
> 3 |     foydalanuvchi: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> 10-bobda biz ushbu xatolarni qanday tuzatishni muhokama qilamiz, shunda siz
> referencelarni structlarda saqlashingiz mumkin, ammo hozircha biz bu kabi
> xatolarni `&str` kabi referencelar oʻrniga `String` kabi tegishli turlardan foydalanib tuzatamiz.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
