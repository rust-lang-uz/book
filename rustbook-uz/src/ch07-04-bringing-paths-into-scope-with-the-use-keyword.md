## `use` kalit so'zi bilan yo'llarni doiraga kiritish

Funksiyalarni chaqirish yo'llarini yozishga to'g'ri kelishi noqulay va takroriy tuyulishi mumkin. 7-7-Ro'yxatda `navbat_listiga_qoshish` funksiyasiga mutlaq yoki nisbiy yoʻlni tanladikmi, har safar `navbat_listiga_qoshish` funksiyasiga murojat qilmoqchi boʻlganimizda, `uyning_oldi` va `xizmat`ni ham belgilashimiz kerak edi. Yaxshiyamki, bu jarayonni soddalashtirishning bir usuli bor: biz bir marta `use` kalit so‘zi bilan yo‘lga nom yaratishimiz mumkin, so‘ngra boshqa hamma joyda qisqaroq nomdan foydalanishimiz mumkin.

7-11 ro'yxatda biz `crate::uyning_oldi::xizmat` modulini `restoranda_ovqatlanish` funksiyasi doirasiga kiritamiz, shuning uchun `restoranda_ovqatlanish`dagi `navbat_listiga_qoshish` funksiyasini chaqirish uchun faqat `xizmat::navbat_listiga_qoshish` ni belgilashimiz kerak.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-11: Modulni `use` bilan qamrab olish</span>

`use` va sohaga yo'lni qo'shish fayl tizimida ramziy havola yaratishga o'xshaydi. Crate ildiziga `use crate::uyning_oldi::xizmat` ni qo‘shish orqali `xizmat` endi bu doirada haqiqiy nom bo‘lib qoladi, xuddi `xizmat` moduli crate ildizida aniqlangandek. `use` doirasiga kiritilgan yo'llar boshqa yo'llar kabi maxfiylikni ham tekshiradi.

E'tibor bering, `use` faqat `use` ishlaydigan aniq doira uchun yorliqni yaratadi. 7-12 roʻyxat `restoranda_ovqatlanish` funksiyasini `mijoz` nomli yangi bolalar moduliga oʻtkazadi, bu keyinchalik `use` statementidan farq qiladi, shuning uchun funksiyaning tanasi kompilyatsiya qilinmaydi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-12: `use` statementi faqat u joylashgan doirada qo'llaniladi</span>

Kompilyator xatosi yorliq endi `mijoz` modulida qo'llanilmasligini ko'rsatadi:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

E'tibor bering, `use` endi uning doirasida qo'llanilmasligi haqida ogohlantirish ham bor! Bu muammoni hal qilish uchun `use` ni `mijoz` moduliga ham o‘tkazing yoki `mijoz` modulidagi `super::xizmat` bilan ota-moduldagi yorliqlarga murojaat qiling.

### `use` bilan idiomatik yo'llarni yaratish

7-11 ro'yxatda siz shunday deb hayron bo'lishingiz mumkin,Nima uchun biz bir xil natijaga erishish uchun `navbat_listiga_qoshish` funksiyasigacha toʻliq yoʻlni ishlatish oʻrniga, `crate::uyning_oldi::xizmat` ni ishlatishni belgilab qoʻydik va keyin `restoranda_ovqatlanish` ichidagi `xizmat::navbat_listiga_qoshish` ga murojat qildik, 7-13 ro'yxatdagi kabi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-13: `navbat_listiga_qoshish` funksiyasini `use` bilan qamrab olish, bu unidiomatikdir</span>

Garchi 7-11 va 7-13 ro'yxatlari bir xil vazifani bajarsa-da, 7-11 ro'yxat funksiyani `use` bilan qamrab olishning idiomatik usulidir. Funksiyaning ota-modulini `use` bilan qamrab olish funksiyani chaqirishda ota-modulni belgilashimiz kerakligini anglatadi. Funksiyani chaqirishda ota-modulni ko'rsatish, to'liq yo'lning takrorlanishini minimallashtirish bilan birga, funksiya mahalliy sifatida aniqlanmaganligini aniq ko'rsatadi. 7-13 ro'yxatda `navbat_listiga_qoshish` qayerda aniqlangani aniq emas.

Boshqa tomondan, `use` bilan structlar, enumlar va boshqa elementlarni keltirishda to'liq yo'lni ko'rsatish idiomatikdir. 7-14 ro'yxat standart kutubxonaning `HashMap` structini binary crate doirasiga olib kirishning idiomatik usulini ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Ro'yxat 7-14: `HashMap` ni idiomatik tarzda qamrab olish</span>

Bu idioma ortida hech qanday yaxshi sabab yo'q: Bu shunchaki konventsiya paydo bo'ldi va odamlar Rust kodini shu tarzda o'qish va yozishga o'rganib qolgan.

Bu idiomadan istisno shundaki, biz bir xil nomdagi ikkita elementni `use` statementi yordamida doiraga kiritganimizda - Rust bunga yo'l qo'ymaydi. 7-15 ro'yxatda bir xil nomga ega, ammo har xil ota-modullarga ega bo'lgan ikkita `Result` turini qanday ko'rinishga kiritish va ularga qanday murojaat qilish kerakligi ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 7-15: Bir xil nomdagi ikkita turni bir xil doiraga kiritish uchun ularning ota-modullaridan foydalanish talab etiladi.</span>

Ko'rib turganingizdek, ota-modullardan foydalanish ikkita `Result` turini ajratib turadi.
Buning o'rniga `use std::fmt::Result` va `us std::io::Result` ni belgilagan bo'lsak, bizda bir xil miqyosda ikkita `Result` turi bo'lar edi va Rust `Result` dan foydalanganda qaysi birini nazarda tutganimizni bilmas edi.

### `as` kalit so'zi bilan yangi nomlarni taqdim etish

Bir xil nomdagi ikkita turni `use` bilan bir xil doiraga olib kirish muammosining yana bir yechimi bor: yoʻldan soʻng biz `as` va yangi mahalliy nom yoki tur uchun *taxallus* belgilashimiz mumkin. 7-16 ro'yxatda ikkita `Result` turidan birini `as` yordamida qayta nomlash orqali 7-15 ro'yxatdagi kodni yozishning yana bir usuli ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 7-16: `as` kalit so'zi bilan qamrovga kiritilgan tur nomini o'zgartirish</span>

Ikkinchi `use` statementida biz `std::io::Result` turi uchun yangi `IoResult` nomini tanladik, bu endi `std::fmt` dan `Result` turiga zid kelmaydi, u ham doiraga kiradi. 7-15 va 7-16 ro'yxatlar idiomatik hisoblanadi, shuning uchun tanlov sizga bog'liq!

### `pub use` bilan nomlarni qayta eksport(re-eksport) qilish

`use` kalit so'zidan foydalanib, nomni qamrovga kiritganimizda, yangi doirada mavjud bo'lgan nom shaxsiy bo'ladi. Bizning kodimizni chaqiradigan kodni xuddi shu kod doirasida aniqlangandek ushbu nomga murojaat qilishini yoqish uchun biz `pub` va `use` ni birlashtira olamiz. Bu usul *re-eksport* deb nomlanadi, chunki biz obyektni qamrovga kiritmoqdamiz, lekin elementni boshqa qamrovlarga kiritish uchun ham mavjud qilamiz.

7-17 ro'yxatda 7-11 ro'yxatdagi kod ko'rsatilgan, ildiz modulidagi `use` `pub use` ga o'zgartirilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: Making a name available for any code to use
from a new scope with `pub use`</span>

Before this change, external code would have to call the `add_to_waitlist`
function by using the path
`restaurant::front_of_house::hosting::add_to_waitlist()`. Now that this `pub
use` has re-exported the `hosting` module from the root module, external code
can now use the path `restaurant::hosting::add_to_waitlist()` instead.

Re-exporting is useful when the internal structure of your code is different
from how programmers calling your code would think about the domain. For
example, in this restaurant metaphor, the people running the restaurant think
about “front of house” and “back of house.” But customers visiting a restaurant
probably won’t think about the parts of the restaurant in those terms. With
`pub use`, we can write our code with one structure but expose a different
structure. Doing so makes our library well organized for programmers working on
the library and programmers calling the library. We’ll look at another example
of `pub use` and how it affects your crate’s documentation in the [“Exporting a
Convenient Public API with `pub use`”][ch14-pub-use]<!-- ignore --> section of
Chapter 14.

### Using External Packages

In Chapter 2, we programmed a guessing game project that used an external
package called `rand` to get random numbers. To use `rand` in our project, we
added this line to *Cargo.toml*:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and any dependencies from [crates.io](https://crates.io/) and
make `rand` available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the crate, `rand`, and listed the items
we wanted to bring into scope. Recall that in the [“Generating a Random
Number”][rand]<!-- ignore --> section in Chapter 2, we brought the `Rng` trait
into scope and called the `rand::thread_rng` function:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Members of the Rust community have made many packages available at
[crates.io](https://crates.io/), and pulling any of them into your package
involves these same steps: listing them in your package’s *Cargo.toml* file and
using `use` to bring items from their crates into scope.

Note that the standard `std` library is also a crate that’s external to our
package. Because the standard library is shipped with the Rust language, we
don’t need to change *Cargo.toml* to include `std`. But we do need to refer to
it with `use` to bring items from there into our package’s scope. For example,
with `HashMap` we would use this line:

```rust
use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library
crate.

### Using Nested Paths to Clean Up Large `use` Lists

If we’re using multiple items defined in the same crate or same module,
listing each item on its own line can take up a lot of vertical space in our
files. For example, these two `use` statements we had in the Guessing Game in
Listing 2-4 bring items from `std` into scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Instead, we can use nested paths to bring the same items into scope in one
line. We do this by specifying the common part of the path, followed by two
colons, and then curly brackets around a list of the parts of the paths that
differ, as shown in Listing 7-18.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listing 7-18: Specifying a nested path to bring multiple
items with the same prefix into scope</span>

In bigger programs, bringing many items into scope from the same crate or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!

We can use a nested path at any level in a path, which is useful when combining
two `use` statements that share a subpath. For example, Listing 7-19 shows two
`use` statements: one that brings `std::io` into scope and one that brings
`std::io::Write` into scope.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listing 7-19: Two `use` statements where one is a subpath
of the other</span>

The common part of these two paths is `std::io`, and that’s the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path, as shown in Listing 7-20.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Combining the paths in Listing 7-19 into
one `use` statement</span>

This line brings `std::io` and `std::io::Write` into scope.

### The Glob Operator

If we want to bring *all* public items defined in a path into scope, we can
specify that path followed by the `*` glob operator:

```rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into
the current scope. Be careful when using the glob operator! Glob can make it
harder to tell what names are in scope and where a name used in your program
was defined.

The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the [“How to Write
Tests”][writing-tests]<!-- ignore --> section in Chapter 11. The glob operator
is also sometimes used as part of the prelude pattern: see [the standard
library documentation](../std/prelude/index.html#other-preludes)<!-- ignore -->
for more information on that pattern.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
