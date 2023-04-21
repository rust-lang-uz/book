## Test tashkil etish

Bobning boshida aytib o'tilganidek, test murakkab intizom bo'lib, turli odamlar turli terminologiya va tashkilotdan foydalanadilar. Rust hamjamiyati testlarni ikkita asosiy toifaga ko'ra o'ylaydi: birlik testlari(unit test) va integratsiya testlari(integration test). *Unit testlari* kichikroq va ko'proq yo'naltirilgan bo'lib, bir vaqtning o'zida bitta modulni alohida sinovdan o'tkazadi va private interfeyslarni sinab ko'rishi mumkin. Integratsiya testlari kutubxonangizdan(library) butunlay tashqarida bo'lib, kodingizdan faqat public interfeysdan foydalangan holda va har bir test uchun bir nechta modullardan foydalangan holda boshqa har qanday tashqi kod kabi foydalaning.

Kutubxonangizning qismlari siz kutgan narsani alohida va birgalikda bajarishini ta'minlash uchun ikkala turdagi testlarni yozish muhimdir.

### Unit Testlar

Unit testlarining maqsadi kodning qayerda ekanligi va kutilganidek ishlamayotganligini tezda aniqlash uchun kodning har bir birligini(unit) qolgan kodlardan alohida tekshirishdan iborat. Unit testlarini har bir fayldagi *src* jildiga ular tekshirayotgan kod bilan joylashtirasiz. Konventsiya har bir faylda test funktsiyalarini o'z ichiga olgan `tests` nomli modul yaratish va modulga `cfg(test)` bilan izoh berishdan iborat.

#### Testlar moduli va `#[cfg(test)]`

Testlar modulidagi `#[cfg(test)]` izohi Rustga test kodini faqat `cargo test`ni bajarganingizda kompilyatsiya qilishni va ishga tushirishni aytadi, `cargo build`ni ishga tushirganingizda emas.
Bu siz faqat kutubxona qurmoqchi bo'lganingizda kompilyatsiya vaqtini tejaydi va natijada tuzilgan artefaktda joyni tejaydi, chunki testlar kiritilmagan. Integratsiya testlari boshqa jildga o‘tgani uchun ularga `#[cfg(test)]` izohi kerak emasligini ko‘rasiz. Biroq, unit testlari kod bilan bir xil fayllarda joylashganligi sababli, ular kompilyatsiya qilingan natijaga kiritilmasligini belgilash uchun `#[cfg(test)]` dan foydalanasiz.

Eslatib o'tamiz, biz ushbu bobning birinchi qismida yangi `qoshuvchi` loyihasini yaratganimizda, Cargo biz uchun ushbu kodni yaratdi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Bu kod avtomatik ravishda yaratilgan test modulidir. `cfg` atributi *konfiguratsiya(configuration)* degan ma'noni anglatadi va Rustga quyidagi element faqat ma'lum bir konfiguratsiya opsiyasi berilganda kiritilishi kerakligini aytadi. Bunday holda, konfiguratsiya opsiyasi `test` bo'lib, u Rust tomonidan testlarni kompilyatsiya qilish va ishga tushirish uchun taqdim etiladi. `cfg` atributidan foydalanib, Cargo bizning test kodimizni faqat `cargo test` bilan faol ravishda o'tkazganimizdagina kompilyatsiya qiladi. Bunga `#[test]` bilan izohlangan funksiyalarga qoʻshimcha ravishda ushbu modulda boʻlishi mumkin boʻlgan har qanday yordamchi funksiyalar kiradi.

#### Private funksiyalarni testdan o'tkazish

Sinov hamjamiyatida private(xususiy) funksiyalarni to'g'ridan-to'g'ri testdan o'tkazish kerakmi yoki yo'qmi degan bahs-munozaralar mavjud va boshqa tillar private funktsiyalarni test qilib ko'rishni qiyinlashtiradi yoki imkonsiz qiladi. Qaysi sinov mafkurasiga rioya qilishingizdan qat'i nazar, Rust maxfiylik qoidalari sizga private funksiyalarni test qilish imkonini beradi.
11-12 roʻyxatdagi kodni `ichki_qoshuvchi` private funksiyasi bilan koʻrib chiqing.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<span class="caption">Ro'yxat 11-12: Private funksiyani test qilib ko'rish</span>

Esda tutingki, `ichki_qoshuvchi` funksiyasi `pub` sifatida belgilanmagan. Testlar shunchaki Rust kodi va `tests` moduli shunchaki boshqa moduldir. ["Modul daraxtidagi elementga murojaat qilish yo'llari"][paths]<!-- ignore --> bo'limida muhokama qilganimizdek, bolalar modullaridagi elementlar o'zlarining asosiy modullaridagi elementlardan foydalanishlari mumkin. Ushbu testda biz `test` modulining ota-onasining barcha elementlarini  `use super::*` yordamida qamrab olamiz va keyin test `ichki_qoshuvchi` ni chaqirishi mumkin. Agar private(shaxsiy) funksiyalarni sinab ko'rish kerak deb o'ylamasangiz, Rustda sizni bunga majbur qiladigan hech narsa yo'q.

### Integratsiya testlari

Rust-da integratsiya testlari kutubxonangizdan butunlay tashqarida. Ular kutubxonangizdan boshqa kodlar kabi foydalanadilar, ya'ni ular faqat kutubxonangizning umumiy API qismi bo'lgan funksiyalarni chaqira oladi. Ularning maqsadi kutubxonangizning ko'p qismlari to'g'ri ishlashini tekshirishdir. O'z-o'zidan to'g'ri ishlaydigan kod birliklari integratsiyalashganda muammolarga duch kelishi mumkin, shuning uchun integratsiyalangan kodni sinovdan o'tkazish ham muhimdir. Integratsiya testlarini yaratish uchun sizga birinchi navbatda *tests* jildi kerak bo'ladi.

#### *tests* jildi

Biz loyiha jildimizning yuqori darajasida, *src* yonida *tests* jildini yaratamiz. Cargo ushbu jildda integratsiya test fayllarini qidirishni biladi. Keyin biz xohlagancha test fayllarini yaratishimiz mumkin va Cargo har bir faylni alohida crate sifatida tuzadi.

Keling, integratsiya testini yarataylik. 11-12 ro'yxatdagi kod hali ham *src/lib.rs* faylida bo'lsa, *tests* jildini yarating va *tests/integratsiya_test.rs* nomli yangi fayl yarating. Sizning fayl tuzilishingiz tuzilishi quyidagicha ko'rinishi kerak:

```text
qoshuvchi
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integratsiya_test.rs
```

11-13 ro'yxatdagi kodni *tests/integratsiya_test.rs* fayliga kiriting:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<span class="caption">Ro'yxat 11-13: `qoshuvchi` cratesidagi funksiyaning integratsiya testi</span>

`tests` jildidagi har bir fayl alohida cratedir, shuning uchun biz kutubxonamizni har bir test cratesi doirasiga kiritishimiz kerak. Shuning uchun biz kodning yuqori qismiga unit testlarida kerak bo'lmagan  `use qoshuvchi` ni qo'shamiz.

Bizga *tests/integration_test.rs* da `#[cfg(test)]` bilan hech qanday kodga izoh berish shart emas. Cargo `tests` jildini maxsus ko'rib chiqadi va bu jilddagi fayllarni faqat biz `cargo test` buyrug'ini ishga tushirganimizda kompilyatsiya qiladi. Keling `cargo test` qilib ishlatamiz:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Chiqishning(output) uchta bo'limiga unit testlari, integratsiya testlari va doc testlari kiradi. E'tibor bering, agar bo'limdagi biron bir test muvaffaqiyatsiz bo'lsa, keyingi bo'limlar bajarilmaydi. Misol uchun, agar unit testi muvaffaqiyatsiz bo'lsa, integratsiya va doc testlari uchun hech qanday natija bo'lmaydi, chunki bu testlar faqat barcha unit testlari o'tgan taqdirdagina amalga oshiriladi.

Unit testlari uchun birinchi bo'lim biz ko'rganimiz bilan bir xil: har bir unit testi uchun bitta satr (biz 11 12 roʻyxatga qoʻshgan `ichki` deb nomlangan) va keyin unit testlari uchun xulosa qator.

Integratsiya testlari bo'limi `Running tests/integration_test.rs` qatoridan boshlanadi. Keyin, ushbu integratsiya testidagi har bir test funksiyasi uchun qator va `Doc-tests adder` boʻlimi boshlanishidan oldin integratsiya testi natijalari uchun xulosa qatori mavjud.

Har bir integratsiya test faylining o'z bo'limi bor, shuning uchun *tests* jildiga ko'proq fayllar qo'shsak, ko'proq integratsiya test bo'limlari bo'ladi.

We can still run a particular integration test function by specifying the test
function’s name as an argument to `cargo test`. To run all the tests in a
particular integration test file, use the `--test` argument of `cargo test`
followed by the name of the file:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

This command runs only the tests in the *tests/integration_test.rs* file.

#### Submodules in Integration Tests

As you add more integration tests, you might want to make more files in the
*tests* directory to help organize them; for example, you can group the test
functions by the functionality they’re testing. As mentioned earlier, each file
in the *tests* directory is compiled as its own separate crate, which is useful
for creating separate scopes to more closely imitate the way end users will be
using your crate. However, this means files in the *tests* directory don’t
share the same behavior as files in *src* do, as you learned in Chapter 7
regarding how to separate code into modules and files.

The different behavior of *tests* directory files is most noticeable when you
have a set of helper functions to use in multiple integration test files and
you try to follow the steps in the [“Separating Modules into Different
Files”][separating-modules-into-files]<!-- ignore --> section of Chapter 7 to
extract them into a common module. For example, if we create *tests/common.rs*
and place a function named `setup` in it, we can add some code to `setup` that
we want to call from multiple test functions in multiple test files:

<span class="filename">Filename: tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

When we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions nor
did we call the `setup` function from anywhere:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Having `common` appear in the test results with `running 0 tests` displayed for
it is not what we wanted. We just wanted to share some code with the other
integration test files.

To avoid having `common` appear in the test output, instead of creating
*tests/common.rs*, we’ll create *tests/common/mod.rs*. The project directory
now looks like this:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

This is the older naming convention that Rust also understands that we
mentioned in the [“Alternate File Paths”][alt-paths]<!-- ignore --> section of
Chapter 7. Naming the file this way tells Rust not to treat the `common` module
as an integration test file. When we move the `setup` function code into
*tests/common/mod.rs* and delete the *tests/common.rs* file, the section in the
test output will no longer appear. Files in subdirectories of the *tests*
directory don’t get compiled as separate crates or have sections in the test
output.

After we’ve created *tests/common/mod.rs*, we can use it from any of the
integration test files as a module. Here’s an example of calling the `setup`
function from the `it_adds_two` test in *tests/integration_test.rs*:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

Note that the `mod common;` declaration is the same as the module declaration
we demonstrated in Listing 7-21. Then in the test function, we can call the
`common::setup()` function.

#### Integration Tests for Binary Crates

If our project is a binary crate that only contains a *src/main.rs* file and
doesn’t have a *src/lib.rs* file, we can’t create integration tests in the
*tests* directory and bring functions defined in the *src/main.rs* file into
scope with a `use` statement. Only library crates expose functions that other
crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* file that calls logic that lives in the
*src/lib.rs* file. Using that structure, integration tests *can* test the
library crate with `use` to make the important functionality available.
If the important functionality works, the small amount of code in the
*src/main.rs* file will work as well, and that small amount of code doesn’t
need to be tested.

## Summary

Rust’s testing features provide a way to specify how code should function to
ensure it continues to work as you expect, even as you make changes. Unit tests
exercise different parts of a library separately and can test private
implementation details. Integration tests check that many parts of the library
work together correctly, and they use the library’s public API to test the code
in the same way external code will use it. Even though Rust’s type system and
ownership rules help prevent some kinds of bugs, tests are still important to
reduce logic bugs having to do with how your code is expected to behave.

Let’s combine the knowledge you learned in this chapter and in previous
chapters to work on a project!

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#alternate-file-paths
