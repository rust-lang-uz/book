## Metod Sintaksisi

*Metodlar* funksiyalarga oʻxshaydi: biz ularni `fn` kalit soʻzi va nomi bilan eʼlon qilamiz, ular parametrlari va qaytish qiymatiga ega boʻlishi mumkin va ular boshqa joydan metod chaqirilganda ishga tushadigan kodni oʻz ichiga oladi. Funktsiyalardan farqli o'laroq, metodlar struct (yoki biz mos ravishda [6-bob][enums]<!-- ignore --> va [17-bobda][trait-objects]<!-- ignore --> ko'rib chiqiladigan enum yoki trait obyekti) kontekstida aniqlanadi va ularning birinchi parametri har doim `self` dir metod chaqirilayotgan structning namunasini ifodalaydi.

### Metodlarni aniqlash

Parametr sifatida `Kvadrat` misoliga ega bo‘lgan `area` funksiyasini o‘zgartiramiz va uning o‘rniga 5-13 ro‘yxatda ko'rsatilganidek, `Kvadrat` structida belgilangan `area` metodini yaratamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Ro'yxat 5-13: `Kvadrat` structida `area` metodini aniqlash</span>

`Kvadrat` kontekstida funksiyani aniqlash uchun `Kvadrat` uchun `impl` (implementation) blokini ishga tushiramiz. Ushbu `impl` blokidagi hamma narsa `Kvadrat` turi bilan bog'lanadi. Keyin biz  `area` funksiyasini `impl` jingalak qavslar ichida harakatlantiramiz va birinchi (va bu holda, faqat) parametrni signatureda va tananing hamma joyida `self` o‘zgartiramiz. `main` da, biz `area` funksiyasini chaqirib, argument sifatida `kvadrat1` ni topshirgan bo‘lsak, o‘rniga `Kvadrat` misolida `area` metodini chaqirish uchun *metod sintaksisi* dan foydalanishimiz mumkin. Metod sintaksisi misoldan keyin keladi: biz nuqta qo'shamiz, undan keyin metod nomi, qavslar va har qanday argumentlar qo'shiladi.

`area` uchun signatureda `kvadrat: &Kvadrat` o‘rniga `&self` dan foydalanamiz. `&self` aslida  `self: &Self` ning qisqartmasi. `impl` blokida `Self` turi `impl` bloki uchun bo'lgan turdagi taxallusdir. Metodlar birinchi parametri uchun `Self` turidagi `self` deb nomlangan parametrga ega bo'lishi kerak, shuning uchun Rust birinchi parametr joyida faqat `self` nomi bilan qisqartirish imkonini beradi.
Esda tutingki, biz hali ham `kvadrat: &Kvadrat` da qilganimizdek, bu metod `Self` misolini olishini koʻrsatish uchun `Self` stenografiyasi oldida `&` dan foydalanishimiz kerak. Boshqa har qanday parametr singari, metodlar `self` egallashi, o'zgarmas `self` borrow qilishi mumkin, xuddi biz bu yerda qilganimizdek yoki o'zgaruvchan `self`ni borrow qilishi mumkin.

Biz bu yerda funksiya versiyasida `&Kvadrat` dan foydalanganimiz uchun xuddi shu sababga ko‘ra `&self` tanladik: biz ownershiplik qilishni istamaymiz va faqat structdagi ma’lumotlarni o‘qishni istaymiz, unga yozishni emas. Agar biz ushbu metodning bir qismi sifatida chaqirgan misolni o'zgartirmoqchi bo'lsak, birinchi parametr sifatida `&mut self` dan foydalanamiz. Birinchi parametr sifatida faqat `self`ni ishlatib, misolga ownershiplik qiladigan metod kamdan-kam uchraydi; bu metod odatda `self`ni boshqa narsaga aylantirganda va siz murojat qiluvchiga transformatsiyadan keyin asl nusxadan foydalanishiga yo'l qo'ymaslikni istasangiz ishlatiladi.

Funktsiyalar o'rniga metodlardan foydalanishning asosiy sababi, har bir metod signaturesida `self`turini takrorlashning hojati bo'lmagan metod sintaksisidan tashqari, kodni tashkil qilishdir. Biz kelajakdagi kod foydalanuvchilarini biz taqdim etayotgan kutubxonaning turli joylarida `Kvadrat` imkoniyatlarini izlashga majburlashdan ko‘ra, biz tur namunasi bilan qila oladigan barcha narsalarni bitta `impl` blokiga joylashtirdik.

E'tibor bering, biz metodga structning maydonlaridan biri bilan bir xil nom berishni tanlashimiz mumkin. Misol uchun, biz `Kvadrat` da `kenglik` deb nomlangan metodni belgilashimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Bu yerda, agar misolning `kenglik` maydonidagi qiymat `0` dan katta bo‘lsa, `kenglik` metodi `true` qiymatini qaytaradi, agar qiymat `0` bo'lsa, `false` bo‘lishini tanlaymiz: biz bir xil nomdagi metod ichidagi maydonni istalgan maqsadda ishlatishimiz mumkin. `main` da, biz qavslar bilan `kvadrat1.kenglik` ga amal qilsak, Rust `kenglik` metodini nazarda tutayotganimizni biladi. Qavslardan foydalanmasak, Rust `kenglik` maydonini nazarda tutayotganimizni biladi.

Ko'pincha, lekin har doim emas, biz metodga maydon bilan bir xil nom berganimizda, biz u faqat maydondagi qiymatni qaytarishini va boshqa hech narsa qilmasligini xohlaymiz. Shunga o'xshash metodlar *getters* deb ataladi va Rust ularni boshqa tillarda bo'lgani kabi tizim maydonlari uchun avtomatik ravishda amalga oshirmaydi. Getterslar foydalidir, chunki siz maydonni shaxsiy, lekin metodni hammaga ochiq qilib qo'yishingiz mumkin va shu tariqa ushbu maydonga umumiy API ning bir qismi sifatida faqat o'qish uchun ruxsatni yoqishingiz mumkin. Biz [7-bobda][public]<!-- ignore --> public va private nima ekanligini va qanday qilib maydon yoki metodni public yoki private deb belgilashni muhokama qilamiz.

> ### `->` operatori qayerda ishlatiladi?
>
> C va C++ tillarida metodlarni chaqirish uchun ikki xil operator qo'llaniladi:
> obyektdagi metodni to'g'ridan-to'g'ri chaqirayotgan bo'lsangiz `.` va agar
> siz ko'rsatgichdagi metodni obyektga chaqirayotgan bo'lsangiz va avval
> ko'rsatgichni yo'qotishingiz kerak bo'lsa `->` dan foydalanasiz. Boshqacha qilib aytganda,
> agar `object` havola bo'lsa, u holda `object->something()` va `(*object).something()` metodi
> chaqiruvlari bir xil bo'ladi.
>
> Rust `->` operatoriga ekvivalentga ega emas;  Buning o'rniga Rustda
> *avtomatik reference va dereferencing* deb nomlangan xususiyat mavjud. Metodni chaqirish
> Rustda bunday xatti-harakatlarga ega bo'lgan kam sonli joylardan biridir.
>
> Bu shunday ishlaydi: `object.something()` bilan metodni chaqirganingizda,
> Rust avtomatik ravishda `&`, `&mut` yoki `*` ni qo'shadi, shuning uchun `object`
> metod signaturega mos keladi. Boshqacha qilib aytganda, quyidagilar bir xil:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn masofa(&self, other: &Point) -> f64 {
> #        let x_kvadrat = f64::powi(other.x - self.x, 2);
> #        let y_kvadrat = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_kvadrat + y_kvadrat)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.masofa(&p2);
> (&p1).masofa(&p2);
> ```
>
> Birinchisi ancha toza ko'rinadi. Ushbu avtomatik reference qilish harakati,
> metodlar aniq qabul qiluvchiga ega bo'lganligi sababli ishlaydi - `self` turi. Qabul qiluvchi
> va metod nomini hisobga olgan holda, Rust ma'lum bir holatda kod nima qilayotganini aniq aniqlashi mumkin:
> o'qish `(&self)`, o'zgartirish (`&mut self`) yoki iste'mol qilish  (`self`). Rust metodi
> qabul qiluvchilar uchun borrow qilishni yashirin qilib qo'yganligi amalda ownershipni
> ergonomik qilishning katta qismidir.

### Ko'proq parametrlarga ega metodlar

`Kvadrat` structida ikkinchi metodni implement qilish orqali metodlardan foydalanishni mashq qilaylik. Bu safar biz `Kvadrat` misoli `Kvadrat` ning boshqa nusxasini olishini va agar ikkinchi `Kvadrat` to'liq o'ziga (birinchi `Kvadrat`) sig'ishi mumkin bo'lsa, `true` qiymatini qaytarishini istaymiz; aks holda u `false`ni qaytarishi kerak.
Ya'ni, `can_hold` metodini aniqlaganimizdan so'ng, biz 5-14 ro'yxatda ko'rsatilgan dasturni yozish imkoniyatiga ega bo'lishni xohlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Using the as-yet-unwritten `can_hold`
method</span>

The expected output would look like the following because both dimensions of
`rect2` are smaller than the dimensions of `rect1`, but `rect3` is wider than
`rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we’d need a mutable borrow),
and we want `main` to retain ownership of `rect2` so we can use it again after
calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter</span>

When we run this code with the `main` function in Listing 5-14, we’ll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.

### Associated Functions

All functions defined within an `impl` block are called *associated functions*
because they’re associated with the type named after the `impl`. We can define
associated functions that don’t have `self` as their first parameter (and thus
are not methods) because they don’t need an instance of the type to work with.
We’ve already used one function like this: the `String::from` function that’s
defined on the `String` type.

Associated functions that aren’t methods are often used for constructors that
will return a new instance of the struct. These are often called `new`, but
`new` isn’t a special name and isn’t built into the language. For example, we
could choose to provide an associated function named `square` that would have
one dimension parameter and use that as both width and height, thus making it
easier to create a square `Rectangle` rather than having to specify the same
value twice:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

The `Self` keywords in the return type and in the body of the function are
aliases for the type that appears after the `impl` keyword, which in this case
is `Rectangle`.

To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in [Chapter
7][modules]<!-- ignore -->.

### Multiple `impl` Blocks

Each struct is allowed to have multiple `impl` blocks. For example, Listing
5-15 is equivalent to the code shown in Listing 5-16, which has each method in
its own `impl` block.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Rewriting Listing 5-15 using multiple `impl`
blocks</span>

There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.

## Summary

Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.

But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.

[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
