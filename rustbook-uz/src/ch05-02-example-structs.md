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

Bu erda biz structi aniqladik va uni `Kvadrat` deb nomladik. Jingalak qavslar ichida biz maydonlarni `kenglik` va `balandlik` sifatida belgiladik, ularning ikkalasi ham `u32` turiga ega. Keyin, `main` da biz `Kvadrat` ning ma'lum bir misolini yaratdik, uning kengligi `30` va balandligi `50`.

Bizning `area` funksiyamiz endi biz `kvadrat` deb nomlagan bitta parametr bilan aniqlanadi, uning turi `Kvadrat` structi misolining o‘zgarmas borrowidir. 4-bobda aytib o'tilganidek, biz unga ownershiplik qilishdan ko'ra, structi borrow qilishni xohlaymiz. Shunday qilib, `main` o'z ownershipini saqlab qoladi va `kvadrat1` dan foydalanishni davom ettirishi mumkin, shuning uchun biz funktsiya signaturesida `&` dan foydalanamiz va biz funktiyani chaqiramiz.

`area` funksiyasi `Kvadrat` misolining `kenglik` va `balandlik`  maydonlariga kiradi (esda tutingki, borrow qilingan struct misolining maydonlariga kirish maydon qiymatlarini ko'chirmaydi, shuning uchun siz ko'pincha structlarning borrowlarini ko'rasiz). Endi `area` funksiyasi signaturesi biz nimani nazarda tutayotganimizni aniq aytadi: `Kvadrat` maydonini uning `kenglik` va `balandlik` maydonlaridan foydalanib hisoblang. Bu kenglik va balandlik bir-biri bilan bog'liqligini bildiradi va `0` va `1` qator indeks qiymatlarini ishlatishdan ko'ra, qiymatlarga tavsiflovchi nomlar beradi. Bu aniqlik uchun g'alaba.

### Adding Useful Functionality with Derived Traits

It’d be useful to be able to print an instance of `Rectangle` while we’re
debugging our program and see the values for all its fields. Listing 5-11 tries
using the [`println!` macro][println]<!-- ignore --> as we have used in
previous chapters. This won’t work, however.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">Listing 5-11: Attempting to print a `Rectangle`
instance</span>

When we compile this code, we get an error with this core message:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

The `println!` macro can do many kinds of formatting, and by default, the curly
brackets tell `println!` to use formatting known as `Display`: output intended
for direct end user consumption. The primitive types we’ve seen so far
implement `Display` by default because there’s only one way you’d want to show
a `1` or any other primitive type to a user. But with structs, the way
`println!` should format the output is less clear because there are more
display possibilities: Do you want commas or not? Do you want to print the
curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn’t try to guess what we want, and structs don’t have a provided
implementation of `Display` to use with `println!` and the `{}` placeholder.

If we continue reading the errors, we’ll find this helpful note:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Let’s try it! The `println!` macro call will now look like `println!("rect1 is
{:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells
`println!` we want to use an output format called `Debug`. The `Debug` trait
enables us to print our struct in a way that is useful for developers so we can
see its value while we’re debugging our code.

Compile the code with this change. Drat! We still get an error:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

But again, the compiler gives us a helpful note:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust *does* include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the outer attribute `#[derive(Debug)]` just before the
struct definition, as shown in Listing 5-12.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">Listing 5-12: Adding the attribute to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting</span>

Now when we run the program, we won’t get any errors, and we’ll see the
following output:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string. In
this example, using the `{:#?}` style will output the following:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Another way to print out a value using the `Debug` format is to use the [`dbg!`
macro][dbg]<!-- ignore -->, which takes ownership of an expression (as opposed
to `println!`, which takes a reference), prints the file and line number of
where that `dbg!` macro call occurs in your code along with the resultant value
of that expression, and returns ownership of the value.

> Note: Calling the `dbg!` macro prints to the standard error console stream
> (`stderr`), as opposed to `println!`, which prints to the standard output
> console stream (`stdout`). We’ll talk more about `stderr` and `stdout` in the
> [“Writing Error Messages to Standard Error Instead of Standard Output”
> section in Chapter 12][err]<!-- ignore -->.

Here’s an example where we’re interested in the value that gets assigned to the
`width` field, as well as the value of the whole struct in `rect1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

We can put `dbg!` around the expression `30 * scale` and, because `dbg!`
returns ownership of the expression’s value, the `width` field will get the
same value as if we didn’t have the `dbg!` call there. We don’t want `dbg!` to
take ownership of `rect1`, so we use a reference to `rect1` in the next call.
Here’s what the output of this example looks like:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

We can see the first bit of output came from *src/main.rs* line 10 where we’re
debugging the expression `30 * scale`, and its resultant value is `60` (the
`Debug` formatting implemented for integers is to print only their value). The
`dbg!` call on line 14 of *src/main.rs* outputs the value of `&rect1`, which is
the `Rectangle` struct. This output uses the pretty `Debug` formatting of the
`Rectangle` type. The `dbg!` macro can be really helpful when you’re trying to
figure out what your code is doing!

In addition to the `Debug` trait, Rust has provided a number of traits for us
to use with the `derive` attribute that can add useful behavior to our custom
types. Those traits and their behaviors are listed in [Appendix C][app-c]<!--
ignore -->. We’ll cover how to implement these traits with custom behavior as
well as how to create your own traits in Chapter 10. There are also many
attributes other than `derive`; for more information, see [the “Attributes”
section of the Rust Reference][attributes].

Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle` struct
because it won’t work with any other type. Let’s look at how we can continue to
refactor this code by turning the `area` function into an `area` *method*
defined on our `Rectangle` type.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
