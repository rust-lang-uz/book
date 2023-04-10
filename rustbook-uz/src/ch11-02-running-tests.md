## Testlar qanday o'tkazilishini nazorat qilish

Xuddi `cargo run` kodingizni kompilyatsiya qilib, natijada olingan binaryni ishga tushirganidek, `cargo test` kodingizni test rejimida kompilyatsiya qiladi va natijada olingan binary testni ishga tushiradi. `cargo test` tomonidan ishlab chiqarilgan binary faylning standart xatti-harakati barcha testlarni parallel ravishda bajarish va sinov testlari paytida hosil bo'lgan chiqishni(output) olish, natijaning ko'rsatilishiga yo'l qo'ymaslik va sinov natijalari bilan bog'liq chiqishni o'qishni osonlashtirishdir. Biroq, siz ushbu standart xatti-harakatni o'zgartirish uchun buyruq qatori parametrlarini belgilashingiz mumkin.

Ba'zi buyruq qatori opsiyalari `cargo test` ga, ba'zilari esa natijada olingan binary testga o'tadi. Ushbu ikki turdagi argumentlarni ajratish uchun siz `cargo test` ga, so'ngra ajratuvchi `--` ga o'tadigan argumentlarni, so'ngra test binarysiga o'tadigan argumentlarni sanab o'tasiz. `cargo test --help`ni ishga tushirish `cargo test` bilan foydalanishingiz mumkin bo'lgan variantlarni ko'rsatadi va `cargo test -- --help`ni ishga tushirish ajratuvchidan(separator) keyin foydalanishingiz mumkin bo'lgan variantlarni ko'rsatadi.

### Testlarni parallel yoki ketma-ket bajarish

Bir nechta testlarni bajarganingizda, standart bo'yicha ular threadlar yordamida parallel ravishda ishlaydi, ya'ni ular tezroq ishlashni tugatadi va siz tezroq fikr-mulohaza olasiz. Testlar bir vaqtning o'zida ishlayotganligi sababli, sizning testlaringiz bir-biriga yoki umumiy holatga, jumladan, joriy ishchi jildi yoki muhit o'zgaruvchilari kabi umumiy muhitga bog'liq emasligiga ishonch hosil qilishingiz kerak.

Misol uchun, sizning har bir testingiz diskda *test-output.txt* nomli fayl yaratadigan va ushbu faylga ba'zi ma'lumotlarni yozadigan ba'zi kodlarni ishga tushiradi. Keyin har bir test ushbu fayldagi ma'lumotlarni o'qiydi va faylda har bir testda har xil bo'lgan ma'lum bir qiymat borligini tasdiqlaydi. Testlar bir vaqtning o'zida bajarilganligi sababli, bitta test faylni boshqa test yozish va o'qish oralig'ida faylni qayta yozishi mumkin. Ikkinchi test kod noto'g'ri bo'lgani uchun emas, balki parallel ravishda ishlayotganda testlar bir-biriga xalaqit bergani uchun muvaffaqiyatsiz bo'ladi. Bitta yechim har bir test boshqa faylga yozishiga ishonch hosil qilishdir; yana bir yechim testlarni birma-bir bajarishdir.

Agar siz testlarni parallel ravishda o'tkazishni xohlamasangiz yoki ishlatilgan threadlar sonini yanada aniqroq nazorat qilishni istasangiz, siz `--test threads` buyru'gini va foydalanmoqchi bo'lgan threadlar sonini test binaryga yuborishingiz mumkin. Quyidagi misolni ko'rib chiqing:

```console
$ cargo test -- --test-threads=1
```

Biz dasturga parallelizmdan foydalanmaslikni aytib, test threadlari sonini `1` ga o'rnatdik. Testlarni bitta thread yordamida o'tkazish ularni parallel ravishda bajarishdan ko'ra ko'proq vaqt talab etadi, ammo agar ular umumiy holatga ega bo'lsa, testlar bir-biriga xalaqit bermaydi.

### Funktsiya natijalarini ko'rsatish

Odatiy bo'lib, agar testdan o'tgan bo'lsa, Rustning test kutubxonasi standart chiqishda chop etilgan barcha narsalarni yozib oladi. Misol uchun, agar testda `println!` ni chaqirsak va testdan o'tgan bo'lsa, terminalda `println!` chiqishini ko'rmaymiz; biz faqat testdan o'tganligini ko'rsatadigan qatorni ko'ramiz. Agar test muvaffaqiyatsiz tugasa, biz xato xabarining qolgan qismi bilan standart chiqishda chop etilganini ko'ramiz.

Misol tariqasida, 11-10 ro'yxatida o'z parametrining qiymatini chop etadigan va 10 ni qaytaradigan ahmoqona funksiya, shuningdek, o'tgan test va muvaffaqiyatsizlikka uchragan test mavjud.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

<span class="caption">Ro'yxat 11-10: `println!`ni chaqiruvchi funksiya uchun testlar</span>

Ushbu testlarni `cargo test` bilan bajarganimizda, biz quyidagi natijani ko'ramiz:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

E'tibor bering, bu chiqishning hech bir joyida `Men 4 qiymatini oldim` ni ko'rmaymiz, ya'ni testdan o'tganda chop etiladi. Bu chiqish qo'lga olindi. Muvaffaqiyatsiz bo'lgan test natijasi, `Men 8-qiymatni oldim`, test xulosasi chiqishi bo'limida paydo bo'ladi, bu test muvaffaqiyatsizligi sababini ham ko'rsatadi.

Agar biz testlardan o'tish uchun yozilgan qiymatlarni ham ko'rishni istasak, Rust-ga `--show-output` bilan muvaffaqiyatli testlar natijasini ham ko'rsatishni aytishimiz mumkin.

```console
$ cargo test -- --show-output
```

11-10 ro'yxatdagi testlarni yana `--show-output` buyrug'i bilan o'tkazganimizda, biz quyidagi natijani ko'ramiz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Testlar to'plamini nomi bo'yicha bajarish(ishga tushirish)

Ba'zan to'liq test to'plamini ishga tushirish uzoq vaqt talab qilishi mumkin. Agar siz ma'lum bir sohada kod ustida ishlayotgan bo'lsangiz, faqat ushbu kodga tegishli testlarni o'tkazishni xohlashingiz mumkin. Argument sifatida oʻtkazmoqchi boʻlgan test(lar)ning nomi yoki nomlarini `cargo test` oʻtish orqali qaysi testlarni oʻtkazishni tanlashingiz mumkin.

Testlar kichik to‘plamini qanday bajarishni ko‘rsatish uchun avval 11-11 ro‘yxatda ko‘rsatilganidek, `ikkita_qoshish` funksiyamiz uchun uchta test yaratamiz va qaysi birini bajarishni tanlaymiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<span class="caption">Ro'yxat 11-11: Uch xil nomga ega uchta test</span>

If we run the tests without passing any arguments, as we saw earlier, all the
tests will run in parallel:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Running Single Tests

We can pass the name of any test function to `cargo test` to run only that test:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Only the test with the name `one_hundred` ran; the other two tests didn’t match
that name. The test output lets us know we had more tests that didn’t run by
displaying `2 filtered out` at the end.

We can’t specify the names of multiple tests in this way; only the first value
given to `cargo test` will be used. But there is a way to run multiple tests.

#### Filtering to Run Multiple Tests

We can specify part of a test name, and any test whose name matches that value
will be run. For example, because two of our tests’ names contain `add`, we can
run those two by running `cargo test add`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

This command ran all tests with `add` in the name and filtered out the test
named `one_hundred`. Also note that the module in which a test appears becomes
part of the test’s name, so we can run all the tests in a module by filtering
on the module’s name.

### Ignoring Some Tests Unless Specifically Requested

Sometimes a few specific tests can be very time-consuming to execute, so you
might want to exclude them during most runs of `cargo test`. Rather than
listing as arguments all tests you do want to run, you can instead annotate the
time-consuming tests using the `ignore` attribute to exclude them, as shown
here:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```

After `#[test]` we add the `#[ignore]` line to the test we want to exclude. Now
when we run our tests, `it_works` runs, but `expensive_test` doesn’t:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

The `expensive_test` function is listed as `ignored`. If we want to run only
the ignored tests, we can use `cargo test -- --ignored`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

By controlling which tests run, you can make sure your `cargo test` results
will be fast. When you’re at a point where it makes sense to check the results
of the `ignored` tests and you have time to wait for the results, you can run
`cargo test -- --ignored` instead. If you want to run all tests whether they’re
ignored or not, you can run `cargo test -- --include-ignored`.
