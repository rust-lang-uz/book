## `Result` bilan tiklanadigan xatolar

Ko'pgina xatolar dasturni butunlay to'xtatishni talab qiladigan darajada jiddiy emas.
Ba'zan, funksiya bajarilmasa, bu siz osongina talqin qilishingiz va javob berishingiz mumkin bo'lgan sababdir. Misol uchun, agar siz faylni ochishga urinib ko'rsangiz va fayl mavjud bo'lmagani uchun bu operatsiya bajarilmasa, jarayonni tugatish o'rniga faylni yaratishni xohlashingiz mumkin.

2-bobdagi [“Potentsial muvaffaqiyatsizlikni `Result` bilan hal qilish”][handle_failure]<!-- ignore --> bo'limini eslang: biz u yerda muvaffaqiyatsizliklarni hal qilish uchun ikkita variantga ega bo'lgan `Ok` va `Err` varianti bo'lgan `Result` enumidan foydalanganmiz. Enumning o'zi quyidagicha aniqlanadi:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` va `E` umumiy turdagi parametrlardir: biz generiklarni 10-bobda batafsilroq muhokama qilamiz. Siz hozir bilishingiz kerak bo'lgan narsa shundaki, `T` `Ok` variantida muvaffaqiyatli holatda qaytariladigan qiymat turini bildiradi, va `E` `Err`(Xato) variantida xatolik holatida qaytariladigan xato turini ifodalaydi. `Result`da ushbu umumiy turdagi parametrlar mavjud bo'lganligi sababli, biz qaytarmoqchi bo'lgan muvaffaqiyatli qiymat va xato qiymati har xil bo'lishi mumkin bo'lgan turli vaziyatlarda `Result` turidan va unda belgilangan funksiyalardan foydalanishimiz mumkin.

Keling, `Result` qiymatini qaytaruvchi funksiyani chaqiraylik, chunki funksiya bajarilmasligi mumkin. 9-3 ro'yxatda biz faylni ochishga harakat qilamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Ro'yxat 9-3: Faylni ochish</span>

`File::open` return(qaytish) turi `Result<T, E>`dir. `File::open`ni amalga oshirishdagi umumiy `T` turi muvaffaqiyatli qabul qilingan qiymat turiga, `std::fs::File`ga, ya'ni fayl deskriptoriga mos keladi. Xato qiymatida ishlatiladigan `E` turi `std::io::Error`. Qaytish(return) turi `File::open` ga chaqiruv muvaffaqiyatli bo'lishi va biz o'qishimiz yoki yozishimiz mumkin bo'lgan fayl handleni qaytarishi mumkinligini anglatadi. Funksiya chaqiruvi ham muvaffaqiyatsiz bo'lishi mumkin: masalan, fayl mavjud bo'lmasligi yoki faylga kirish uchun ruxsatimiz bo'lmasligi mumkin. `File::open` funksiyasi muvaffaqiyatli yoki muvaffaqiyatsiz bo'lganligini va bir vaqtning o'zida bizga fayl identifikatori yoki xato haqida ma'lumot beradigan metodga ega bo'lishi kerak. Ushbu ma'lumot aynan `Result` enumini bildiradi.

Agar `File::open` muvaffaqiyatli bo'lsa, `fayl_ochish` qiymati fayl identifikatorini o'z ichiga olgan `Ok` misoli bo'ladi.
Muvaffaqiyatsiz bo'lgan taqdirda, `fayl_ochish` dagi qiymat `Err` misoli bo'lib, sodir bo'lgan xato turi haqida qo'shimcha ma'lumotni o'z ichiga oladi.

`File::open` qiymatiga qarab turli amallarni bajarish uchun 9-3-Ro'yxatdagi kodga o'zgartirishimiz kerak. 9-4 ro'yxatda biz 6-bobda muhokama qilgan asosiy tool - `match` ifodasi yordamida `Result` ni boshqarishning bir usuli ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Roʻyxat 9-4: Qaytarilishi mumkin boʻlgan `Result` variantlarini boshqarish uchun `match` ifodasidan foydalanish</span>

E'tibor bering, `Option` enumi kabi, `Result` enumi va uning variantlari avtomatik import (prelude) orqali kiritilgan, shuning uchun biz `match`  qatoridagi `Ok` va `Err`  variantlaridan oldin `Result::` ni belgilashimiz shart emas.

Natija `Ok` bo'lsa, bu kod `Ok` variantidan ichki `file` qiymatini qaytaradi va biz ushbu faylni ishlov berish qiymatini `fayl_ochish` o'zgaruvchisiga tayinlaymiz. `match`dan so'ng biz o'qish yoki yozish uchun fayl boshqaruvidan foydalanishimiz mumkin.

`match`ning boshqa qismi `File::open` dan `Err` qiymatini oladigan holatni boshqaradi. Ushbu misolda biz `panic!`  makrosini tanladik. Agar joriy jildimizda *olma.txt* nomli fayl bo‘lmasa va biz ushbu kodni ishga tushirsak, biz `panic!` makrosidan quyidagi natijani ko‘ramiz:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Odatdagidek, bu chiqish bizga nima noto'g'ri ketganligini aniq aytadi.

### Turli xil xatolarga moslashish

9-4 roʻyxatdagi kod nima uchun `File::open` muvaffaqiyatsiz boʻlishidan qatʼiy nazar `panic!` qoʻyadi.
Biroq, biz turli sabablarga ko'ra turli xil harakatlarni amalga oshirishni xohlaymiz: agar fayl mavjud bo'lmagani uchun `File::open` muvaffaqiyatsiz bo'lsa, biz faylni yaratmoqchimiz va fayl boshqaruvini yangi faylga qaytaramiz. Agar `File::open` boshqa sabablarga ko'ra, masalan, faylni ochishga ruxsatimiz yo'qligi sababli muvaffaqiyatsiz bo'lsa, biz hali ham kodga 9-4 ro'yxatdagi kabi `panic!` qo'yishini xohlaymiz. Buning uchun biz 9-5 ro'yxatda ko'rsatilgan ichki `match` ifodasini qo'shamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Ro'yxat 9-5: Har xil turdagi xatolarni turli yo'llar bilan hal qilish</span>

`File::open` `Err` variantida qaytaradigan qiymat turi `io::Error` bo'lib, bu standart kutubxona tomonidan taqdim etilgan strukturadir. Ushbu strukturada `io::ErrorKind` qiymatini olish uchun chaqirishimiz mumkin bo'lgan `kind` metodi mavjud. `io::ErrorKind` enumi standart kutubxona tomonidan taqdim etilgan va `io` operatsiyasi natijasida yuzaga kelishi mumkin bo'lgan turli xil xatolarni ifodalovchi variantlarga ega. Biz foydalanmoqchi boʻlgan variant `ErrorKind::NotFound` boʻlib, biz ochmoqchi boʻlgan fayl hali mavjud emasligini bildiradi. Shunday qilib, biz `fayl_ochish` bo'yicha mos kelamiz, lekin bizda `error.kind()` da ichki match ham bor.

Biz ichki matchni tekshirmoqchi bo'lgan shart - `error.kind()` tomonidan qaytarilgan qiymat `ErrorKind` enumining `NotFound` variantidir. Agar shunday bo'lsa, biz faylni `File::create` yordamida yaratishga harakat qilamiz. Biroq, `File::create` ham muvaffaqiyatsiz bo'lishi mumkinligi sababli, bizga ichki `match` ifodasida ikkinchi arm kerak. Faylni yaratib bo'lmaganda, boshqa xato xabari chop etiladi. Tashqi `match` ning ikkinchi armi bir xil bo'lib qoladi, shuning uchun dastur yetishmayotgan fayl xatosidan tashqari har qanday xato haqida panic qo'yadi.

> ### `Result<T, E>` bilan `match` dan foydalanishning muqobillari
>
> Bu juda ko'p `match`! `match` ifodasi juda foydali, lekin ayni paytda
> juda primitivdir. 13-bobda siz `Result<T, E>` da belgilangan koʻplab
> metodlarda qoʻllaniladigan yopilishlar(closures) haqida bilib olasiz. Ushbu
> metodlar kodingizdagi `Result<T, E>` qiymatlari bilan ishlashda `match` 
> dan foydalanishdan ko'ra qisqaroq bo'lishi mumkin.
>
> Misol uchun, 9-5 ro'yxatda ko'rsatilgan mantiqni yozishning yana bir
> usuli, bu safar closures va `unwrap_or_else` metodi yordamida:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let fayl_ochish = File::open("olma.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("olma.txt").unwrap_or_else(|error| {
>                 panic!("Fayl yaratishda muammo: {:?}", error);
>             })
>         } else {
>             panic!("Faylni ochishda muammo: {:?}", error);
>         }
>     });
> }
> ```
>
> Garchi bu kod 9-5 roʻyxatdagi kabi harakatga ega boʻlsa-da, unda
> `match` iboralari mavjud emas va oʻqish uchun qulayroq. 13-bobni o‘qib bo‘lgach,
> ushbu misolga qayting va standart kutubxona hujjatlarida `unwrap_or_else`
> metodini qidiring. Ushbu metodlarning ko'pchiligi xatolar bilan
> shug'ullanayotganda katta o'rinli `match`  iboralarni tozalashi mumkin.

### Xatoda panic uchun yorliqlar: `unwrap` va `expect`

`match` dan foydalanish yetarlicha yaxshi ishlaydi, lekin u biroz batafsil bo'lishi mumkin va har doim ham maqsadni yaxshi bildirmaydi. `Result<T, E>` turida turli, aniqroq vazifalarni bajarish uchun belgilangan koʻplab yordamchi metodlar mavjud. `unwrap` metodi biz 9-4 ro'yxatda yozgan `match` iborasi kabi implemen qilinadigan yorliq metodidir. Agar `Result` qiymati `Ok` varianti bo'lsa, `unwrap` qiymati `Ok` ichidagi qiymatni qaytaradi. Agar `Result` `Err` varianti bo‘lsa, `unwrap` biz uchun `panic!` makrosini chaqiradi. Mana amaldagi `unwrap` misoli:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Agar biz ushbu kodni *olma.txt* faylisiz ishga tushiradigan bo‘lsak, biz `panic!` chaqiruvidan xato xabarini ko‘ramiz.

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

Xuddi shunday, `expect` metodi bizga `panic!` xato xabarini tanlash imkonini beradi.
`unwrap` o'rniga `expect` dan foydalanish va yaxshi xato xabarlarini taqdim etish niyatingizni bildirishi va panic manbasini kuzatishni osonlashtirishi mumkin. `expect` sintaksisi quyidagicha ko'rinadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

Biz `expect` dan xuddi `unwrap` kabi foydalanamiz: fayl boshqaruvini qaytarish yoki `panic!` makrosini chaqirish uchun.`panic!` chaqiruvida `expect` tomonidan foydalanilgan xato xabari `unwrap` ishlatadigan standart `panic!` xabari emas, balki `expect` parametriga o‘tadigan parametr bo‘ladi. Bu qanday ko'rinishga ega:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'olma.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

Ishlab chiqarish sifati kodida ko'pchilik Rustaceanlar `unwrap` o'rniga  `expect` ni tanlaydilar va nima uchun operatsiya har doim muvaffaqiyatli bo'lishi kutilayotgani haqida ko'proq kontekst beradi. Shunday qilib, agar sizning taxminlaringiz noto'g'ri ekanligi isbotlangan bo'lsa, debuggingda foydalanish uchun ko'proq ma'lumotga ega bo'lasiz.

### Xatoni yo'naltirish - Propagating

Funksiyani amalga oshirish muvaffaqiyatsiz bo'lishi mumkin bo'lgan narsani chaqirganda, xatoni funksiyaning o'zida hal qilish o'rniga, nima qilish kerakligini hal qilish uchun xatoni chaqiruvchi kodga qaytarishingiz mumkin. Bu xatoni *propagating* deb nomlanadi va chaqiruv kodini ko'proq nazorat qiladi, bu yerda kodingiz kontekstida mavjud bo'lgan narsadan ko'ra xatoni qanday hal qilish kerakligini ko'rsatadigan ko'proq ma'lumot yoki mantiq bo'lishi mumkin.

Misol uchun, 9-6 ro'yxati fayldan foydalanuvchi nomini o'qiydigan funksiyani ko'rsatadi. Agar fayl mavjud bo'lmasa yoki o'qib bo'lmasa, bu funksiya ushbu xatolarni funksiya chaqirgan kodga qaytaradi.

<span class="filename">Fayl nomi: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Ro'yxat 9-6: `match` yordamida chaqiruv kodiga xatoliklarni qaytaruvchi funksiya</span>

Bu funksiyani ancha qisqaroq tarzda yozish mumkin, lekin biz xatolarni qayta ishlashni o'rganish uchun uning ko'p qismini qo'lda qilishdan boshlaymiz; oxirida biz qisqaroq yo'lni ko'rsatamiz. Avval funksiyaning qaytish turini ko'rib chiqamiz: `Result<String, io::Error>`. Bu funksiya `Result<T, E>` turidagi qiymatni qaytarayotganini bildiradi, bunda parametr `T` aniq turdagi `String` bilan to'ldirilgan, va `E` umumiy turi aniq turdagi `io::Error` bilan to`ldirilgan.

Agar bu funksiya hech qanday muammosiz muvaffaqiyatli bajarilsa, ushbu funksiyani chaqiruvchi kod `String` ga ega boʻlgan `Ok` qiymatini oladi – bu funksiya fayldan o'qigan foydalanuvchi nomi. Agar bu funksiya biron bir muammoga duch kelsa, murojaat qiluvchi kod `io::Error` misolini o'z ichiga olgan `Err` qiymatini oladi, unda muammolar nima bo'lganligi haqida qo'shimcha ma'lumot mavjud. Biz ushbu funktsiyaning qaytish turi sifatida `io::Error` ni tanladik, chunki bu funksiyaning tanasida bajarilmay qolishi mumkin bo‘lgan ikkala operatsiyadan qaytarilgan xato qiymatining turi: `File::open` funksiyasi va `read_to_string` metodi.

Funksiyaning asosiy qismi `File::open` funksiyasini chaqirish orqali boshlanadi. Keyin biz `Result` qiymatini 9-4 ro'yxatdagi `match`ga o'xshash `match` bilan ishlaymiz.
Agar `File::open` muvaffaqiyatli bajarilsa, `file` pattern o'zgaruvchisidagi fayl ishlovi `foydalanuvchi_fayli` o'zgaruvchan o'zgaruvchisidagi qiymatga aylanadi va funksiya davom etadi. `Err` holatida, `panic!` deb chaqirish o‘rniga, biz `return`  kalit so‘zidan funksiyadan to‘liq chiqib ketish uchun foydalanamiz va xato qiymatini `File::open` dan, endi `e` pattern o‘zgaruvchisiga o‘tkazamiz, bu funksiya xato qiymati sifatida chaqiruvchi kodga qaytaradi.

Shunday qilib, agar bizda `foydalanuvchi_fayli` da fayl boshqaruvi mavjud bo'lsa, keyin funksiya `foydalanuvchi` o'zgaruvchisida yangi `String` yaratadi va fayl mazmunini `foydalanuvchi` ni o'qish uchun `foydalanuvchi_fayli` da fayl boshqaruvidagi `read_to_string` metodini chaqiradi. `read_to_string` metodi ham `Result`ni qaytaradi, chunki u `File::open` muvaffaqiyatli bo'lishi ham mumkin, muvaffaqiyatsiz bo'lishi ham mumkin. Demak, ushbu `Result` bilan ishlash uchun bizga yana bir `match` kerak bo'ladi: agar `read_to_string` muvaffaqiyatli bo'lsa, demak, bizning funksiyamiz muvaffaqiyatli bo'ldi va biz foydalanuvchi nomini hozirda `Ok` bilan o'ralgan `foydalanuvchi` faylidan qaytaramiz. Agar `read_to_string` bajarilmasa, biz xato qiymatini xuddi `File::open` qiymatini qayta ishlagan `match` da xato qiymatini qaytarganimizdek qaytaramiz. Biroq, biz `return` ni aniq aytishimiz shart emas, chunki bu funksiyadagi oxirgi ifoda.

Ushbu kodni chaqiruvchi kod foydalanuvchi nomini o'z ichiga olgan `Ok`  qiymatini yoki `io::Error` ni o'z ichiga olgan `Err` qiymatini olishni boshqaradi. Ushbu qiymatlar bilan nima qilishni hal qilish chaqiruv kodiga bog'liq. Agar chaqiruv kodi `Err` qiymatini olsa, u `panic!` deb chaqirishi va dasturni buzishi mumkin, standart foydalanuvchi nomidan foydalaning yoki foydalanuvchi nomini fayldan boshqa joydan qidiring, masalan. Bizda chaqiruv kodi aslida nima qilmoqchi ekanligi haqida yetarli ma'lumot yo'q, shuning uchun biz barcha muvaffaqiyat yoki xato ma'lumotlarini to'g'ri ishlashi uchun xatolarni propagate qilamiz.

Xatolarni propagating qilish namunasi Rustda shu qadar keng tarqalganki, Rust buni osonlashtirish uchun savol belgisi operatori `?` beradi.

#### Propagating xatolar uchun qisqa kod: `?` operatori

9-7 ro'yxatda 9-6 ro'yxatdagi kabi funksiyaga ega bo'lgan `foydalanuvchi_fayli` ilovasi ko'rsatilgan, ammo bu dastur `?` operatoridan foydalanadi.

<span class="filename">Fayl nomi: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 9-7: `?` operatori yordamida chaqiruvchi kodga xatoliklarni qaytaruvchi funksiya</span>

`Result` qiymatidan keyin qoʻyilgan `?` 9-6 roʻyxatdagi `Result` qiymatlarini boshqarish uchun biz belgilagan `match` iboralari bilan deyarli bir xil ishlaydi. Agar `Result` qiymati `Ok` bo'lsa, `Ok` ichidagi qiymat ushbu ifodadan qaytariladi va dastur davom etadi. Agar qiymat `Err` bo'lsa, `Err` butun funktsiyadan qaytariladi, xuddi biz `return` kalit so'zidan foydalanganimizdek, xato qiymati chaqiruvchi kodga propagate qiladi.

9-6 roʻyxatdagi `match` ifodasi va `?` operatori nima qilishi oʻrtasida farq bor: `?` operatori chaqirilgan xato qiymatlari `from` funksiyasidan oʻtadi, qiymatlarni bir turdan ikkinchi turga aylantirish uchun foydalaniladigan standart kutubxonadagi `From` traitida aniqlanadi.
`?` operatori `from` funksiyasini chaqirganda, qabul qilingan xato turi joriy funksiyaning qaytish turida aniqlangan xato turiga aylanadi. Bu funksiya muvaffaqiyatsiz bo'lishi mumkin bo'lgan barcha usullarni ifodalash uchun bitta xato turini qaytarganda foydalidir, agar qismlar turli sabablarga ko'ra ishlamay qolsa ham.

Misol uchun, biz 9-7 ro'yxatdagi `fayldan_foydalanuvchi_nomini_olish` funksiyasini o'zgartirishimiz mumkin, bu biz aniqlagan `OurError`  nomli maxsus xato turini qaytarishimiz mumkin. Agar `io::Error` dan `OurError` misolini yaratish uchun `OurError` uchun `impl From<io::Error> for OurError` ni ham aniqlasak, keyin `fayldan_foydalanuvchi_nomini_olish` asosiy qismidagi `?` operatori chaqiruvlari `from`ga murojaat qiladi va funksiyaga boshqa kod qo'shmasdan xato turlarini o'zgartiradi.
`foydalanuvchi_fayli` o'zgaruvchisiga qaytaradi.Agar xatolik yuzaga kelsa, `?` operatori butun funksiyadan erta qaytadi va chaqiruvchi kodga istalgan `Err` qiymatini beradi. Xuddi shu narsa `read_to_string` chaqiruvi oxiridagi `?` uchun ham amal qiladi.

`?` operatori ko'plab nosozliklarni bartaraf qiladi va bu funksiyani amalga oshirishni soddalashtiradi. 9-8 ro'yxatda ko'rsatilganidek, biz ushbu kodni `?` dan keyin metod chaqiruvlar zanjiridan foydalansak, bu kodni yanada qisqartirishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Ro'yxat 9-8: `?` operatoridan keyin zanjirlash(chaining) metodi chaqiruvlari</span>

Biz `foydalanuvchi` da yangi `String` yaratishni funksiya boshiga o‘tkazdik; bu qism o'zgarmagan. `foydalanuvchi_fayli` oʻzgaruvchisini yaratish oʻrniga, `File::open("olma.txt")?` natijasiga toʻgʻridan-toʻgʻri `read_to_string` chaqiruvlarini bogʻladik. Bizda `read_to_string`  chaqiruvi oxirida hali ham `?` bor va biz xatoliklarni qaytarish oʻrniga `File::open` va `read_to_string` muvaffaqiyatli boʻlganda ham `foydalanuvchi`ni oʻz ichiga olgan `OK` qiymatini qaytaramiz. Funksionallik yana 9-6 va 9-7 ro'yxatdagi kabi; Bu uni yozishning boshqacha, ergonomik usuli.

9-9 ro'yxati `fs::read_to_string` yordamida buni yanada qisqartirish yo'lini ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Roʻyxat 9-9: faylni ochish va keyin oʻqish oʻrniga `fs::read_to_string` dan foydalanish</span>

Faylni stringda o'qish juda keng tarqalgan operatsiya, shuning uchun standart kutubxona faylni ochadigan, yangi `String` yaratadigan qulay `fs::read_to_string` funksiyasini ta'minlaydi fayl mazmunini o'qiydi, mazmunini o'sha `String` ga qo'yadi va uni qaytaradi. Albatta, `fs::read_to_string` dan foydalanish bizga xatolarni qanday hal qilishni tushuntirishga imkon bermaydi, shuning uchun biz birinchi navbatda uzoq yo'lni o'rgandik.

#### `?` Operatoridan qayerda foydalanish mumkin

`?` operatori faqat qaytarish turi `?` ishlatiladigan qiymatga mos keladigan funksiyalarda ishlatilishi mumkin. Buning sababi, `?` operatori biz 9-6 ro'yxatda belgilagan `match` ifodasi kabi funksiyadan tashqari qiymatni erta qaytarish uchun belgilangan. 9-6 roʻyxatda `match` `Result` qiymatidan foydalanilgan va erta qaytish armi `Err(e)` qiymatini qaytargan. Funksiyaning qaytish turi `Result` bo'lishi kerak, shunda u ushbu `return` bilan mos keladi.

9-10 ro'yxatda, agar biz `?` dan foydalanadigan qiymat turiga mos kelmaydigan qaytish turi bilan `main` funksiyada `?` operatoridan foydalansak, qanday xatoga duch kelamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Roʻyxat 9-10: `()` qaytaradigan `main`  funksiyadagi `?` dan foydalanishga urinish kompilyatsiya qilinmaydi.</span>

Ushbu kod faylni ochadi, bu muvaffaqiyatsiz bo'lishi mumkin. `?` operatori `File::open` tomonidan qaytarilgan `Result` qiymatiga amal qiladi, lekin bu `main` funksiya `Result` emas, `()` qaytish turiga ega. Ushbu kodni kompilyatsiya qilganimizda, biz quyidagi xato xabarini olamiz:


```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Bu xato bizga `?` operatoridan faqat `Result`, `Option` yoki `FromResidual`ni qo'llaydigan boshqa turdagi qaytaruvchi funksiyada foydalanishga ruxsat berilganligini ko`rsatadi.

Xatoni tuzatish uchun sizda ikkita variant bor. Tanlovlardan biri, funksiyangizning qaytish turini `?` operatoridan foydalanayotgan qiymatga mos keladigan qilib o'zgartirish, agar bunga hech qanday cheklovlar bo'lmasa. Boshqa usul esa, `Result<T, E>` ni mos keladigan usulda boshqarish uchun `match` yoki `Result<T, E>` metodlaridan birini qo`llashdir.

Xato xabarida, shuningdek, `?` ni `Option<T>` qiymatlari bilan ham foydalanish mumkinligi aytilgan. `Result`da `?` dan foydalanish kabi, siz `?` dan faqat `Option` ni qaytaradigan funksiyada foydalanishingiz mumkin. `?` operatorining `Option<T>` bo'yicha chaqirilgandagi xatti-harakati `Result<T, E>` da chaqirilgandagi xatti-harakatiga o'xshaydi: agar qiymat `None` bo'lsa `None` bo'ladi o'sha paytda  funksiyadan erta qaytariladi. Agar qiymat `Some` bo'lsa, `Some` ichidagi qiymat ifodaning natijaviy qiymati bo`lib, funksiya davom etadi. 9-11 ro'yxatda berilgan matndagi birinchi qatorning oxirgi belgisini topadigan funksiya misoli mavjud:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Roʻyxat 9-11: `Option`da `?` operatoridan foydalanish<T>`
value</span>

Bu funksiya `Option<char>`ni qaytaradi, chunki u yerda belgi(character) boʻlishi mumkin, lekin yoʻq boʻlishi ham mumkin. Bu kod `matn` string argumentini oladi va undagi `lines` metodini chaqiradi, bu esa satrdagi satrlar ustidan iteratorni qaytaradi. Ushbu funksiya birinchi qatorni tekshirmoqchi bo'lganligi sababli, iteratordan birinchi qiymatni olish uchun iteratorda `next` ni chaqiradi. Agar `matn` boʻsh qator boʻlsa, `next` ga murojat qilish `None`ni qaytaradi, bu holda biz `birinchi_satrning_oxirgi_belgisi`dan `None`ni toʻxtatish va qaytarish uchun `?` operatoridan foydalanamiz. Agar `matn` bo'sh qator bo'lmasa, `next` `matn`dagi birinchi qatorning string sliceni o'z ichiga olgan `Some` qiymatini qaytaradi.

`?` operatori satr bo'lagini chiqaradi va biz uning belgilarining iteratorini olish uchun ushbu qator bo'limidagi `chars`larni chaqirishimiz mumkin. Bizni ushbu birinchi qatordagi oxirgi belgi qiziqtiradi, shuning uchun biz iteratordagi oxirgi elementni qaytarish uchun `last` deb chaqiramiz.
Bu `Option`dir, chunki birinchi qator boʻsh satr boʻlishi mumkin, masalan, `matn` boʻsh satr bilan boshlansa, lekin `"\nhi"`dagi kabi boshqa qatorlarda belgilar boʻlsa. Biroq, agar birinchi qatorda oxirgi belgi bo'lsa, u `Some` variantida qaytariladi. O'rtadagi `?` operatori bu mantiqni ifodalashning ixcham usulini beradi, bu funksiyani bir qatorda amalga oshirish imkonini beradi. Agar biz `Option` da`?` operatoridan foydalana olmasak, biz bu mantiqni ko'proq metod chaqiruvlari yoki `match` ifodasi yordamida amalga oshirishimiz kerak edi.

Esda tutingki, `?` operatoridan `Result` qaytaruvchi funksiyada `Result`da foydalanishingiz mumkin, va `?` operatoridan `Option` qaytaradigan funksiyada `Option`da foydalanishingiz mumkin, lekin siz aralashtirib, moslashtira olmaysiz. `?` operatori `Result`ni avtomatik ravishda `Option`ga yoki aksincha o'zgartirmaydi; Bunday hollarda konvertatsiyani aniq amalga oshirish uchun `Result`dagi `ok` metodi yoki `Option`dagi `ok_or` kabi metodlardan foydalanishingiz mumkin.

Hozirgacha biz ishlatgan barcha `main` funksiyalar `()` ni qaytaradi. `main` funksiya maxsus, chunki u bajariladigan dasturlarning kirish va chiqish nuqtasi bo'lib, dasturlar kutilgandek harakat qilishi uchun uning qaytish(return) turi qanday bo'lishi mumkinligiga cheklovlar mavjud.

Yaxshiyamki, `main` `Result<(), E>`ni ham qaytarishi mumkin. 9-12 ro'yxatda 9-10 ro'yxatdagi kod mavjud, biroq biz `main` ning qaytish turini `Result<(),  Box<dyn Error>>` qilib o'zgartirdik va oxiriga `Ok(())`  qaytish qiymatini qo'shdik. Ushbu kod endi kompilyatsiya qilinadi:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Roʻyxat 9-12: `main`ni `Result<(), E>` qaytarishga oʻzgartirish `Result` qiymatlarida `?` operatoridan foydalanish imkonini beradi.</span>

`Box<dyn Error>` turi bu *trait ob'ekti* bo'lib, biz 17-bobning ["Turli turdagi qiymatlarga ruxsat beruvchi trait ob'ektlaridan foydalanish"][trait-objects]<!-- ignore -->  bo'limida gaplashamiz. Hozircha siz `Box<dyn Error>`ni “har qanday xato” degan ma'noni anglatadi deb o'ylashingiz mumkin. `Box<dyn Error>` xato turi bilan `main` funksiyadagi `Result` qiymatida `?` dan foydalanishga ruxsat beriladi, chunki bu har qanday `Err` qiymatini erta qaytarish imkonini beradi. Garchi bu `main` funksiyaning tanasi faqat `std::io::Error` turidagi xatolarni qaytarsa ham, `Box<dyn Error>` ni belgilab, `main` funksiyaga boshqa xatolarni qaytaruvchi ko'proq kod qo'shilsa ham, bu kod to'g'ri bo'lib qoladi.

`main`  funksiya `Result<(), E>`ni qaytarsa, bajariladigan fayl(executable file) `0` qiymati bilan chiqadi, agar `main` `Ok(())` qaytarsa va `main` `Err` qiymatini qaytarsa nolga teng bo'lmagan qiymat bilan chiqadi. C tilida yozilgan bajariladigan fayllar(executable file) chiqqanda butun sonlarni qaytaradi: muvaffaqiyatli chiqqan dasturlar `0` butun sonini qaytaradi, xatoga yo'l qo'ygan dasturlar esa `0` dan boshqa butun sonni qaytaradi. Rust shuningdek, ushbu konventsiyaga mos kelishi uchun bajariladigan fayllardan butun(integer) sonlarni qaytaradi.


`main` funksiya [`std::process::Termination` traitini][termination]<!-- ignore --> amalga oshiradigan har qanday turlarni qaytarishi mumkin, bunda `ExitCode` qaytaruvchi `report` funksiyasi mavjud. O'zingizning turlaringiz uchun `Termination` traitini qo'llash bo'yicha qo'shimcha ma'lumot olish uchun standart kutubxona texnik hujjatlariga murojaat qiling.

Endi biz `panic!` chaqirish yoki `Result`ni qaytarish tafsilotlarini muhokama qilganimizdan so‘ng, keling, qaysi hollarda qaysi biri to‘g‘ri kelishini hal qilish mavzusiga qaytaylik.

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: ../std/process/trait.Termination.html
