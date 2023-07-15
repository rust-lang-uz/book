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

Notice that the first value in the vector is `"target/debug/minigrep"`, which
is the name of our binary. This matches the behavior of the arguments list in
C, letting programs use the name by which they were invoked in their execution.
It’s often convenient to have access to the program name in case you want to
print it in messages or change behavior of the program based on what command
line alias was used to invoke the program. But for the purposes of this
chapter, we’ll ignore it and save only the two arguments we need.

### Saving the Argument Values in Variables

The program is currently able to access the values specified as command line
arguments. Now we need to save the values of the two arguments in variables so
we can use the values throughout the rest of the program. We do that in Listing
12-2.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

<span class="caption">Listing 12-2: Creating variables to hold the query
argument and file path argument</span>

As we saw when we printed the vector, the program’s name takes up the first
value in the vector at `args[0]`, so we’re starting arguments at index `1`. The
first argument `minigrep` takes is the string we’re searching for, so we put a
reference to the first argument in the variable `query`. The second argument
will be the file path, so we put a reference to the second argument in the
variable `file_path`.

We temporarily print the values of these variables to prove that the code is
working as we intend. Let’s run this program again with the arguments `test`
and `sample.txt`:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Great, the program is working! The values of the arguments we need are being
saved into the right variables. Later we’ll add some error handling to deal
with certain potential erroneous situations, such as when the user provides no
arguments; for now, we’ll ignore that situation and work on adding file-reading
capabilities instead.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
