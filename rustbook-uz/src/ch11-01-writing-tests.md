## Testlarni qanday yozish kerak

Testlar - bu sinovdan tashqari kod kutilgan tarzda ishlayotganligini tasdiqlovchi Rust funksiyalari. Test funksiyalari organlari odatda ushbu uchta harakatni bajaradi:

1. Har qanday kerakli ma'lumotlarni yoki holatni o'rnating.
2. Test qilmoqchi bo'lgan kodni ishga tushiring.
3. Natijalar siz kutgan narsa ekanligini tasdiqlang.

Keling, Rust ushbu amallarni bajaradigan testlarni yozish uchun taqdim etgan xususiyatlarni ko'rib chiqaylik, ular orasida `test` atributi, bir nechta makroslar va `should_panic` atributi mavjud.

### Test funksiyasining anatomiyasi

Eng sodda qilib aytganda, Rust-dagi test `test` atributi bilan izohlangan funksiyadir. Atributlar Rust kodining bo'laklari haqidagi metama'lumotlardir; bir misol, biz 5-bobda structlar bilan ishlatgan `derive` atributidir. Funksiyani test funksiyasiga oʻzgartirish uchun `fn` oldidan qatorga `#[test]` qoʻshing. `cargo test` buyrug'i bilan testlarni o'tkazganingizda, Rust izohli funksiyalarni ishga tushiradigan test dasturining binaryrini yaratadi va har bir test funksiyasidan o'tgan yoki muvaffaqiyatsizligi haqida hisobot beradi.

Har safar biz Cargo bilan yangi kutubxona loyihasini yaratganimizda, biz uchun test funksiyasi bo'lgan test moduli avtomatik ravishda yaratiladi. Ushbu modul sizga testlarni yozish uchun shablonni taqdim etadi, shuning uchun har safar yangi loyihani boshlaganingizda aniq struktura va sintaksisni izlashga hojat qolmaydi. Siz xohlagancha qo'shimcha test funksiyalari va test modullarini qo'shishingiz mumkin!

Har qanday kodni sinab ko'rishdan oldin shablon testi bilan tajriba o'tkazish orqali testlar qanday ishlashining ba'zi jihatlarini o'rganamiz. Keyin biz yozgan ba'zi kodlarni chaqiradigan va uning xatti-harakati to'g'riligini tasdiqlaydigan haqiqiy dunyo testlarini yozamiz.

Keling, ikkita raqamni qo'shadigan `qoshuvchi` nomli yangi kutubxona loyihasini yarataylik:

```console
$ cargo new qoshuvchi --lib
     Created library `qoshuvchi` project
$ cd qoshuvchi
```

`qoshuvchi` kutubxonangizdagi *src/lib.rs* faylining mazmuni 11-1 roʻyxatdagi kabi koʻrinishi kerak.

<span class="filename">Fayl nomi: src/lib.rs</span>

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
cargo test
git co output.txt
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">Ro'yxat 11-1: Test moduli va funksiyasi avtomatik ravishda `cargo new` tomonidan yaratilgan</span>

Hozircha, keling, yuqoridagi ikkita qatorga e'tibor bermaylik va funksiyaga e'tibor qarataylik. `#[test]` izohiga e'tibor bering: bu atribut bu test funksiyasi ekanligini bildiradi, shuning uchun test ishtirokchisi bu funksiyani test sifatida ko'rishni biladi. Umumiy stsenariylarni oʻrnatish yoki umumiy operatsiyalarni bajarishda yordam beradigan `tests` modulida testdan tashqari funksiyalar ham boʻlishi mumkin, shuning uchun biz har doim qaysi funksiyalar test ekanligini koʻrsatishimiz kerak.

Misol funksiya tanasi 2 va 2 qo‘shilishi natijasini o‘z ichiga olgan `natija` 4 ga teng ekanligini tasdiqlash uchun `assert_eq!` makrosidan foydalanadi. Ushbu tasdiq odatiy test formatiga misol bo'lib xizmat qiladi. Ushbu sinovdan o'tishini ko'rish uchun uni ishga tushiramiz.

`cargo test` buyrug'i 11-2 ro'yxatda ko'rsatilganidek, loyihamizdagi barcha testlarni amalga oshiradi.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">Ro'yxat 11-2: Avtomatik ishlab chiqarilgan testni bajarishdan olingan natija</span>

Cargo kompilyatsiya qilindi va sinovdan o'tdi. Biz `running 1 test` qatorini ko'ramiz. Keyingi qatorda `ishlaydi` deb nomlangan yaratilgan test funksiyasining nomi va bu testni bajarish natijasi `ok` ekanligini ko'rsatadi. Umumiy xulosa test natijasi `test result: ok.` barcha testlardan muvaffaqiyatli oʻtganligini va `1 passed;` deb yozilgan qismi muvaffaqiyatli oʻtganligini bildiradi; `0 failed` muvaffaqiyatsiz boʻlgan testlar sonini ifodalaydi.

Muayyan misolda ishlamasligi uchun testni e'tiborsiz(ignor) deb belgilash mumkin; Biz buni ushbu bobning keyingi qismida ["Agar aniq talab qilinmasa, ba'zi testlarni e'tiborsiz qoldirish"][ignoring]<!-- ignore --> bo'limida ko'rib chiqamiz. Bu yerda biz buni qilmaganimiz sababli, xulosada  `0 ignored` 0-ta eʼtibor berilmagan koʻrsatiladi. Shuningdek, biz argumentni faqat nomi satrga mos keladigan testlarni o'tkazish uchun `cargo test` buyrug'iga o'tkazishimiz mumkin; bu *filtrlash* deb ataladi va biz buni ["Testlar to'plamini nomi bo'yicha ishga tushirish"][subset]<!-- ignore --> bo'limida ko'rib chiqamiz. Shuningdek, biz bajarilayotgan testlarni filtrlamadik, shuning uchun xulosa oxirida `0 filtered out` 0-ta filtrlangan deb ko‘rsatiladi.

`0 measured` statistikasi samaradorlikni o'lchaydigan benchmark testlari uchundir.
Benchmark testlari, ushbu yozuvdan boshlab, faqat nightly Rust-da mavjud. Batafsil ma'lumot olish uchun [benchmark testlari haqidagi hujjatlarga][bench] qarang.

`Doc-tests adder`(Hujjat testlari qoʻshuvchisi) dan boshlanadigan test natijasining keyingi qismi har qanday hujjat sinovlari natijalariga moʻljallangan. Bizda hali hech qanday hujjat sinovlari yo'q, lekin Rust API hujjatlarida ko'rinadigan har qanday kod misollarini to'plashi mumkin.
Bu xususiyat hujjatlaringiz va kodingizni sinxronlashtirishga yordam beradi! Hujjat testlarini qanday yozishni 14-bobning [“Hujjatlarga sharhlar test sifatida”][doc-comments]<!-- ignore --> bo‘limida muhokama qilamiz. Hozircha biz `Doc-tests` chiqishini e'tiborsiz qoldiramiz.

Keling, testni o'z ehtiyojlarimizga moslashtirishni boshlaylik. Avval `ishlaydi` funksiyasining nomini `tadqiqot` kabi boshqa nomga o'zgartiring, masalan:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Keyin yana `cargo test` bajaring. Chiqish(output) endi `ishlaydi` o‘rniga `tadqiqot`ni ko‘rsatadi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Endi biz yana bir test qo'shamiz, lekin bu safar muvaffaqiyatsiz bo'lgan testni qilamiz! Test funktsiyasidagi biror narsa panic qo'zg'atganda, testlar muvaffaqiyatsiz tugaydi. Har bir test yangi threadda o'tkaziladi va asosiy(main) thread sinov chizig'i o'lganini ko'rsa, test muvaffaqiyatsiz deb belgilanadi. 9-bobda biz panic qo'zg'ashning eng oddiy yo'li `panic!` makrosini chaqirish haqida gapirdik. Yangi testni `boshqa` funksiya sifatida kiriting, shunda *src/lib.rs* faylingiz 11-3 roʻyxatiga oʻxshaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-3: Muvaffaqiyatsiz bo'ladigan ikkinchi testni qo'shish, chunki biz `panic!` makrosini chaqiramiz.</span>

`cargo test` yordamida testlarni qaytadan test qiling. Chiqish 11-4 ro'yxatga o'xshash bo'lishi kerak, bu bizning `tadqiqot` sinovimizdan o'tganligini va `boshqa` muvaffaqiyatsiz ekanligini ko'rsatadi.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">Ro'yxat 11-4: Bitta test sinovdan o'tgan va bitta test muvaffaqiyatsizlikka uchragan sinov natijalari</span>

`OK` o'rniga `test tests::boshqa` qatori `FAILED`ni koʻrsatadi. Shaxsiy natijalar va xulosa o'rtasida ikkita yangi bo'lim paydo bo'ladi: birinchisida har bir sinov muvaffaqiyatsizligining batafsil sababi ko'rsatiladi. Bunday holda, biz *src/lib.rs* faylidagi 10-qatordagi `panicked at 'Make this test fail'` da panic qo'ygani uchun `boshqa` muvaffaqiyatsizlikka uchraganligi haqidagi tafsilotlarni olamiz. Keyingi bo'limda barcha muvaffaqiyatsiz testlarning nomlari keltirilgan, bu juda ko'p sinovlar va ko'plab batafsil muvaffaqiyatsiz sinov natijalari mavjud bo'lganda foydalidir. Muvaffaqiyatsiz test nomidan uni osonroq debug qilish uchun ishlatishimiz mumkin; testlarni o'tkazish usullari haqida ko'proq ["Testlar qanday o'tkazilishini nazorat qilish"][controlling-how-tests-are-run]<!-- ignore
--> section bo'limida gaplashamiz.

Xulosa qatori oxirida ko'rsatiladi: umuman olganda, bizning test natijasimiz `FAILED` muvaffaqiyatsiz. Bizda bitta test sinovi bor edi va bitta sinov muvaffaqiyatsiz tugadi.

Sinov natijalari turli stsenariylarda qanday ko‘rinishini ko‘rganingizdan so‘ng, keling, testlarda foydali bo‘lgan  `panic!`dan tashqari ba’zi makrolarni ko‘rib chiqaylik.

### Natijalarni `assert!` makrosi bilan tekshirish!

Standart kutubxona tomonidan taqdim etilgan `assert!` makrosi testdagi baʼzi shartlar `true`(toʻgʻri) boʻlishini taʼminlashni istasangiz foydali boʻladi. Biz `assert!` makrosiga mantiqiy(boolean) qiymatga baholovchi argument beramiz. Qiymat `true` bo'lsa, hech narsa sodir bo'lmaydi va sinovdan o'tadi. Agar qiymat `false` bo‘lsa, `assert!` makros testning muvaffaqiyatsiz bo‘lishiga olib kelishi uchun `panic!` chaqiradi. `assert!` makrosidan foydalanish bizning kodimiz biz rejalashtirgan tarzda ishlayotganligini tekshirishga yordam beradi.

5-bob, 5-15-ro'yxarda biz 11-5-ro'yxardada takrorlangan `Kvadrat` strukturasi va `ushlab_tur` metodidan foydalandik. Keling, ushbu kodni *src/lib.rs* fayliga joylashtiramiz, so'ngra `assert!` makrosidan foydalanib, u uchun testlarni yozamiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-5: 5-bobdagi `Kvadrat` strukturasi va uning `ushlab_tur` metodidan foydalanish</span>

`ushlab_tur` metodi mantiqiy(boolean) qiymatini qaytaradi, ya'ni bu `assert!` makrosi uchun mukammal foydalanish holati. 11-6 ro'yxatda biz kengligi 8 va balandligi 7 bo'lgan `Kvadrat` misolini yaratish va uning kengligi 5 va balandligi 1 bo'lgan boshqa `Kvadrat` misolini ushlab turishi mumkinligini tekshirish orqali `ushlab_tur` metodini bajaradigan testni yozamiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-6: Kattaroq kvadrat haqiqatan ham kichikroq kvadratni sig'dira oladimi yoki yo'qligini tekshiradigan `ushlab_tur` testi</span>

E'tibor bering, biz `tests` moduliga yangi qator qo'shdik: `use super::*;`. `tests` moduli odatiy modul bo'lib, biz 7-bobda ["Modul daraxtidagi elementga murojaat qilish yo'llari"][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore --> bo'limida ko'rib chiqqan odatiy ko'rinish qoidalariga amal qiladi. `tests` moduli ichki modul bo'lgani uchun biz tashqi moduldagi sinovdan o'tayotgan kodni ichki modul doirasiga kiritishimiz kerak. Biz bu yerda globdan foydalanamiz, shuning uchun tashqi modulda biz aniqlagan har qanday narsa ushbu `tests` modulida mavjud bo'ladi.

Biz sinovimizga `katta_kichikni_ushlab_turadi` deb nom berdik va o‘zimizga kerak bo‘lgan ikkita `Kvadrat` misolini yaratdik.
Keyin biz `assert!` makrosini chaqirdik va uni `kattaroq.ushlab_tur(&kichikroq)` deb chaqirish natijasini berdik. Bu ifoda `true` ni qaytarishi kerak, shuning uchun testimiz muvaffaqiyatli o'tishi kerak. Keling, bilib olaylik!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

Test muvaffaqiyatli o'tadi! Keling, yana bir sinovni qo'shamiz, bu safar kichikroq kvadrat kattaroq kvadratni ushlab turolmaydi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Chunki bu holda `ushlab_tur` funksiyasining to'g'ri natijasi `false` bo'lsa, biz uni `assert!` makrosiga o'tkazishdan oldin bu natijani inkor etishimiz kerak. Natijada, agar `ushlab_tur` `false` qiymatini qaytarsa, testimiz o'tadi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Ikkita sinovdan o'tadi! Keling, kodimizga xatolik kiritganimizda test natijalarimiz bilan nima sodir bo'lishini ko'rib chiqaylik. Kengliklarni solishtirganda katta belgisini kichikroq belgisi bilan almashtirish orqali `ushlab_tur` metodini amalga oshirishni o‘zgartiramiz:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Running the tests now produces the following:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Our tests caught the bug! Because `larger.width` is 8 and `smaller.width` is
5, the comparison of the widths in `can_hold` now returns `false`: 8 is not
less than 5.

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

A common way to verify functionality is to test for equality between the result
of the code under test and the value you expect the code to return. You could
do this using the `assert!` macro and passing it an expression using the `==`
operator. However, this is such a common test that the standard library
provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test
more conveniently. These macros compare two arguments for equality or
inequality, respectively. They’ll also print the two values if the assertion
fails, which makes it easier to see *why* the test failed; conversely, the
`assert!` macro only indicates that it got a `false` value for the `==`
expression, without printing the values that led to the `false` value.

In Listing 11-7, we write a function named `add_two` that adds `2` to its
parameter, then we test this function using the `assert_eq!` macro.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">Listing 11-7: Testing the function `add_two` using the
`assert_eq!` macro</span>

Let’s check that it passes!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

We pass `4` as the argument to `assert_eq!`, which is equal to the result of
calling `add_two(2)`. The line for this test is `test tests::it_adds_two ...
ok`, and the `ok` text indicates that our test passed!

Let’s introduce a bug into our code to see what `assert_eq!` looks like when it
fails. Change the implementation of the `add_two` function to instead add `3`:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Run the tests again:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Our test caught the bug! The `it_adds_two` test failed, and the message tells
us that the assertion that fails was `` assertion failed: `(left == right)` ``
and what the `left` and `right` values are. This message helps us start
debugging: the `left` argument was `4` but the `right` argument, where we had
`add_two(2)`, was `5`. You can imagine that this would be especially helpful
when we have a lot of tests going on.

Note that in some languages and test frameworks, the parameters to equality
assertion functions are called `expected` and `actual`, and the order in which
we specify the arguments matters. However, in Rust, they’re called `left` and
`right`, and the order in which we specify the value we expect and the value
the code produces doesn’t matter. We could write the assertion in this test as
`assert_eq!(add_two(2), 4)`, which would result in the same failure message
that displays `` assertion failed: `(left == right)` ``.

The `assert_ne!` macro will pass if the two values we give it are not equal and
fail if they’re equal. This macro is most useful for cases when we’re not sure
what a value *will* be, but we know what the value definitely *shouldn’t* be.
For example, if we’re testing a function that is guaranteed to change its input
in some way, but the way in which the input is changed depends on the day of
the week that we run our tests, the best thing to assert might be that the
output of the function is not equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All primitive types and most of
the standard library types implement these traits. For structs and enums that
you define yourself, you’ll need to implement `PartialEq` to assert equality of
those types. You’ll also need to implement `Debug` to print the values when the
assertion fails. Because both traits are derivable traits, as mentioned in
Listing 5-12 in Chapter 5, this is usually as straightforward as adding the
`#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See
Appendix C, [“Derivable Traits,”][derivable-traits]<!-- ignore --> for more
details about these and other derivable traits.

### Adding Custom Failure Messages

You can also add a custom message to be printed with the failure message as
optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any
arguments specified after the required arguments are passed along to the
`format!` macro (discussed in Chapter 8 in the [“Concatenation with the `+`
Operator or the `format!`
Macro”][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -->
section), so you can pass a format string that contains `{}` placeholders and
values to go in those placeholders. Custom messages are useful for documenting
what an assertion means; when a test fails, you’ll have a better idea of what
the problem is with the code.

For example, let’s say we have a function that greets people by name and we
want to test that the name we pass into the function appears in the output:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

The requirements for this program haven’t been agreed upon yet, and we’re
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don’t want to have to update the test when the requirements change,
so instead of checking for exact equality to the value returned from the
`greeting` function, we’ll just assert that the output contains the text of the
input parameter.

Now let’s introduce a bug into this code by changing `greeting` to exclude
`name` to see what the default test failure looks like:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Running this test produces the following:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

This result just indicates that the assertion failed and which line the
assertion is on. A more useful failure message would print the value from the
`greeting` function. Let’s add a custom failure message composed of a format
string with a placeholder filled in with the actual value we got from the
`greeting` function:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Now when we run the test, we’ll get a more informative error message:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.

### Checking for Panics with `should_panic`

In addition to checking return values, it’s important to check that our code
handles error conditions as we expect. For example, consider the `Guess` type
that we created in Chapter 9, Listing 9-13. Other code that uses `Guess`
depends on the guarantee that `Guess` instances will contain only values
between 1 and 100. We can write a test that ensures that attempting to create a
`Guess` instance with a value outside that range panics.

We do this by adding the attribute `should_panic` to our test function. The
test passes if the code inside the function panics; the test fails if the code
inside the function doesn’t panic.

Listing 11-8 shows a test that checks that the error conditions of `Guess::new`
happen when we expect them to.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">Listing 11-8: Testing that a condition will cause a
`panic!`</span>

We place the `#[should_panic]` attribute after the `#[test]` attribute and
before the test function it applies to. Let’s look at the result when this test
passes:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Looks good! Now let’s introduce a bug in our code by removing the condition
that the `new` function will panic if the value is greater than 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

When we run the test in Listing 11-8, it will fail:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

We don’t get a very helpful message in this case, but when we look at the test
function, we see that it’s annotated with `#[should_panic]`. The failure we got
means that the code in the test function did not cause a panic.

Tests that use `should_panic` can be imprecise. A `should_panic` test would
pass even if the test panics for a different reason from the one we were
expecting. To make `should_panic` tests more precise, we can add an optional
`expected` parameter to the `should_panic` attribute. The test harness will
make sure that the failure message contains the provided text. For example,
consider the modified code for `Guess` in Listing 11-9 where the `new` function
panics with different messages depending on whether the value is too small or
too large.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">Listing 11-9: Testing for a `panic!` with a panic message
containing a specified substring</span>

This test will pass because the value we put in the `should_panic` attribute’s
`expected` parameter is a substring of the message that the `Guess::new`
function panics with. We could have specified the entire panic message that we
expect, which in this case would be `Guess value must be less than or equal to
100, got 200.` What you choose to specify depends on how much of the panic
message is unique or dynamic and how precise you want your test to be. In this
case, a substring of the panic message is enough to ensure that the code in the
test function executes the `else if value > 100` case.

To see what happens when a `should_panic` test with an `expected` message
fails, let’s again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

This time when we run the `should_panic` test, it will fail:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

The failure message indicates that this test did indeed panic as we expected,
but the panic message did not include the expected string `'Guess value must be
less than or equal to 100'`. The panic message that we did get in this case was
`Guess value must be greater than or equal to 1, got 200.` Now we can start
figuring out where our bug is!

### Using `Result<T, E>` in Tests

Our tests so far all panic when they fail. We can also write tests that use
`Result<T, E>`! Here’s the test from Listing 11-1, rewritten to use `Result<T,
E>` and return an `Err` instead of panicking:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

The `it_works` function now has the `Result<(), String>` return type. In the
body of the function, rather than calling the `assert_eq!` macro, we return
`Ok(())` when the test passes and an `Err` with a `String` inside when the test
fails.

Writing tests so they return a `Result<T, E>` enables you to use the question
mark operator in the body of tests, which can be a convenient way to write
tests that should fail if any operation within them returns an `Err` variant.

You can’t use the `#[should_panic]` annotation on tests that use `Result<T,
E>`. To assert that an operation returns an `Err` variant, *don’t* use the
question mark operator on the `Result<T, E>` value. Instead, use
`assert!(value.is_err())`.

Now that you know several ways to write tests, let’s look at what is happening
when we run our tests and explore the different options we can use with `cargo
test`.

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
