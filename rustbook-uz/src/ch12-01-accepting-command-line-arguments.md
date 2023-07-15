## Buyruqlar qatori argumentlarini qabul qilish

Keling, har doimgidek, `cargo new` bilan yangi loyiha yarataylik. Loyihamizni tizimingizda mavjud boʻlgan `grep` konsol dasturidan farqlash uchun uni `minigrep` deb ataymiz.

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

Birinchi vazifa `minigrep` ni ikkita buyruq qatori argumentlarini qabul qilishdir: fayl yo'li va izlash uchun satr. Ya'ni, biz o'z dasturimizni `cargo run` bilan ishga tushirishni xohlaymiz, quyidagi argumentlar `cargo` uchun emas, balki dasturimizga tegishli ekanligini ko'rsatadigan ikkita tire(qo'shaloq chiziq), qidirish uchun satr va qidiruv uchun faylga yo'l. ichida, shunga o'xshash:

```console
$ cargo run -- qidiruv-matni namuna-fayl.txt
```

Hozirda `cargo new` tomonidan yaratilgan dastur biz bergan argumentlarni qayta ishlay olmaydi. [crates.io](https://crates.io/)-dagi ba'zi mavjud kutubxonalar buyruq qatori argumentlarini qabul qiladigan dastur yozishda yordam berishi mumkin, ammo siz ushbu kontseptsiyani endigina o'rganayotganingiz uchun keling, bu imkoniyatni o'zimiz amalga oshiraylik.

### Argument qiymatlarini o'qish

`minigrep` ga biz o'tadigan buyruq qatori argumentlarining qiymatlarini o'qishni yoqish uchun bizga Rust standart kutubxonasida taqdim etilgan `std::env::args` funksiyasi kerak bo'ladi. Bu funksiya `minigrep` ga uzatilgan buyruq qatori argumentlarining iteratorini qaytaradi. Biz iteratorlarni [13-bobda][ch13]<!-- ignore
--> to'liq ko'rib chiqamiz. Hozircha siz iteratorlar haqida faqat ikkita ma'lumotni bilishingiz kerak: iteratorlar bir qator qiymatlarni ishlab chiqaradi va biz uni vector kabi to'plamga(collection) aylantirish uchun iteratorda `collect` metodini chaqirishimiz mumkin,iterator ishlab chiqaradigan barcha elementlarni o'z ichiga oladi.

12-1 ro'yxatidagi kod `minigrep` dasturingizga unga berilgan har qanday buyruq qatori argumentlarini o'qish va keyin qiymatlarni vectorga yig'ish imkonini beradi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

<span class="caption">Ro'yxat 12-1: buyruq qatori argumentlarini vectorga yig'ish va ularni chop etish</span>

Birinchidan, biz `std::env` modulini `use` statementi bilan qamrab olamiz, shunda uning `args` funksiyasidan foydalanamiz. `std::env::args` funksiyasi modullarning ikki darajasida joylashganligiga e'tibor bering. Biz [7-bobda][ch7-idiomatic-use]<!-- ignore --> muhokama qilganimizdek, istalgan funksiya bir nechta modulda joylashgan bo‘lsa, biz funksiyani emas, balki ota-modulni qamrab olishni tanladik. Shunday qilib, biz `std::env` dan boshqa funksiyalardan bemalol foydalanishimiz mumkin. Bu, shuningdek, `use std::env::args` ni qo‘shib, so‘ngra funksiyani faqat `args` bilan chaqirishdan ko‘ra kamroq noaniqdir, chunki `args` joriy modulda aniqlangan funksiya bilan osongina xato qilishi mumkin.

> ### `args` funksiyasi va notog'ri Unicode
>
> E'tibor bering, agar biron bir argumentda noto'g'ri Unicode bo'lsa, `std::env::args`
> panic qo'zg'atadi. Agar dasturingiz noto'g'ri Unicode o'z ichiga olgan argumentlarni qabul qilishi kerak bo'lsa,
> o'rniga `std::env::args_os` dan foydalaning. Bu funksiya `String` qiymatlari o‘rniga `OsString`
> qiymatlarini ishlab chiqaruvchi iteratorni qaytaradi. Biz bu yerda soddalik uchun
> `std::env::args` dan foydalanishni tanladik, chunki `OsString` qiymatlari platformalar
> uchun farq qiladi va ular bilan ishlash `String` qiymatlariga qaraganda murakkabroq.

`main` ning birinchi qatorida biz `env::args` deb nomlaymiz va iteratorni iterator tomonidan ishlab chiqarilgan barcha qiymatlarni o'z ichiga olgan vectorga aylantirish uchun darhol `collect` dan foydalanamiz. Biz ko'p turdagi to'plamlarni(collection) yaratish uchun `collect` funksiyasidan foydalanishimiz mumkin, shuning uchun biz stringlar vectorini xohlashimizni ko'rsatish uchun `args` turiga aniq izoh beramiz. Rust-da turlarga juda kamdan-kam izoh qo'yishimiz kerak bo'lsa-da, `collect` funksiyasi siz tez-tez izohlashingiz kerak bo'lgan funksiyadir, chunki Rust siz xohlagan to'plam turini aniqlay olmaydi.

Nihoyat, debug makrosi yordamida vectorni chop etamiz. Keling, kodni avval argumentsiz, keyin esa ikkita argument bilan ishga tushirishga harakat qilaylik:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

E'tibor bering, vectordagi birinchi qiymat `"target/debug/minigrep"` bo'lib, bu bizning ikkilik(binary) faylimiz nomidir. Bu C dagi argumentlar ro'yxatining xatti-harakatiga mos keladi, bu dasturlarga ularni bajarishda chaqirilgan nomdan foydalanishga imkon beradi. Agar siz uni xabarlarda chop qilmoqchi bo'lsangiz yoki dasturni chaqirish uchun qanday buyruq qatori taxalluslari(alias) ishlatilganiga qarab dasturning harakatini o'zgartirmoqchi bo'lsangiz, dastur nomiga kirish ko'pincha qulaydir. Ammo ushbu bobning maqsadlari uchun biz buni e'tiborsiz qoldiramiz va faqat bizga kerak bo'lgan ikkita argumentni saqlaymiz.

### Argument qiymatlarini o'zgaruvchilarda saqlash

Dastur hozirda buyruq qatori argumentlari sifatida ko'rsatilgan qiymatlarga kirish imkoniyatiga ega. Endi biz ikkita argumentning qiymatlarini o'zgaruvchilarda saqlashimiz kerak, shuning uchun biz dasturning qolgan qismida qiymatlardan foydalanishimiz mumkin. Biz buni 12-2 ro'yxatda qilamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

<span class="caption">Ro'yxat 12-2: So'rov argumenti va fayl yo'li argumentini saqlash uchun o'zgaruvchilar yaratish</span>

Biz vectorni chop etganimizda ko'rganimizdek, dastur nomi vectordagi birinchi qiymatni `args[0]` oladi, shuning uchun biz `1` indeksidan argumentlarni boshlaymiz. `minigrep`ning birinchi argumenti biz qidirayotgan satrdir, shuning uchun biz birinchi argumentga referenceni `sorov` o‘zgaruvchisiga qo‘yamiz. Ikkinchi argument fayl yo'li bo'ladi, shuning uchun biz `fayl_yoli` o'zgaruvchisiga ikkinchi argumentga reference qilamiz.

Kod biz xohlagandek ishlayotganini isbotlash uchun biz ushbu o'zgaruvchilarning qiymatlarini vaqtincha chop qilamiz. `test` va `namuna.txt` argumentlari bilan ushbu dasturni qayta ishga tushiramiz:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Ajoyib, dastur ishlayapti! Bizga kerak bo'lgan argumentlarning qiymatlari to'g'ri o'zgaruvchilarga saqlanmoqda. Keyinchalik ba'zi potentsial noto'g'ri vaziyatlarni hal qilish uchun xatolarni qayta ishlash usullarini qo'shamiz, masalan, foydalanuvchi hech qanday argument keltirmasa; Hozircha biz bu holatni e'tiborsiz qoldiramiz va uning o'rniga fayllarni o'qish imkoniyatlarini qo'shish ustida ishlaymiz.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
