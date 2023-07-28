## `use` kalit so'zi bilan yo'llarni doiraga kiritish

Having to write out the paths to call functions can feel inconvenient and repetitive. In Listing 7-7, whether we chose the absolute or relative path to the `add_to_waitlist` function, every time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and `hosting` too. Fortunately, there’s a way to simplify this process: we can create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope.

7-11 ro'yxatda biz `crate::uyning_oldi::xizmat` modulini `restoranda_ovqatlanish` funksiyasi doirasiga kiritamiz, shuning uchun `restoranda_ovqatlanish`dagi `navbat_listiga_qoshish` funksiyasini chaqirish uchun faqat `xizmat::navbat_listiga_qoshish` ni belgilashimiz kerak.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```


<span class="caption">Ro'yxat 7-11: Modulni `use` bilan qamrab olish</span>

Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope, just as though the `hosting` module had been defined in the crate root. Paths brought into scope with `use` also check privacy, like any other paths.

Note that `use` only creates the shortcut for the particular scope in which the `use` occurs. Listing 7-12 moves the `eat_at_restaurant` function into a new child module named `customer`, which is then a different scope than the `use` statement, so the function body won’t compile:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```


<span class="caption">Ro'yxat 7-12: `use` statementi faqat u joylashgan doirada qo'llaniladi</span>

Kompilyator xatosi yorliq endi `mijoz` modulida qo'llanilmasligini ko'rsatadi:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Notice there’s also a warning that the `use` is no longer used in its scope! To fix this problem, move the `use` within the `customer` module too, or reference the shortcut in the parent module with `super::hosting` within the child `customer` module.

### `use` bilan idiomatik yo'llarni yaratish

7-11 ro'yxatda siz shunday deb hayron bo'lishingiz mumkin,Nima uchun biz bir xil natijaga erishish uchun `navbat_listiga_qoshish` funksiyasigacha toʻliq yoʻlni ishlatish oʻrniga, `crate::uyning_oldi::xizmat` ni ishlatishni belgilab qoʻydik va keyin `restoranda_ovqatlanish` ichidagi `xizmat::navbat_listiga_qoshish` ga murojat qildik, 7-13 ro'yxatdagi kabi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```


<span class="caption">Ro'yxat 7-13: `navbat_listiga_qoshish` funksiyasini `use` bilan qamrab olish, bu unidiomatikdir</span>

Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with `use`. Bringing the function’s parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. The code in Listing 7-13 is unclear as to where `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s `HashMap` struct into the scope of a binary crate.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```


<span class="caption">Ro'yxat 7-14: `HashMap` ni idiomatik tarzda qamrab olish</span>

Bu idioma ortida hech qanday yaxshi sabab yo'q: Bu shunchaki konventsiya paydo bo'ldi va odamlar Rust kodini shu tarzda o'qish va yozishga o'rganib qolgan.

The exception to this idiom is if we’re bringing two items with the same name into scope with `use` statements, because Rust doesn’t allow that. Listing 7-15 shows how to bring two `Result` types into scope that have the same name but different parent modules and how to refer to them.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```


<span class="caption">Ro'yxat 7-15: Bir xil nomdagi ikkita turni bir xil doiraga kiritish uchun ularning ota-modullaridan foydalanish talab etiladi.</span>

As you can see, using the parent modules distinguishes the two `Result` types. If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d have two `Result` types in the same scope and Rust wouldn’t know which one we meant when we used `Result`.

### `as` kalit so'zi bilan yangi nomlarni taqdim etish

There’s another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or *alias*, for the type. Listing 7-16 shows another way to write the code in Listing 7-15 by renaming one of the two `Result` types using `as`.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```


<span class="caption">Ro'yxat 7-16: `as` kalit so'zi bilan qamrovga kiritilgan tur nomini o'zgartirish</span>

In the second `use` statement, we chose the new name `IoResult` for the `std::io::Result` type, which won’t conflict with the `Result` from `std::fmt` that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are considered idiomatic, so the choice is up to you!

### `pub use` bilan nomlarni qayta eksport(re-eksport) qilish

When we bring a name into scope with the `use` keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`. This technique is called *re-exporting* because we’re bringing an item into scope but also making that item available for others to bring into their scope.

7-17 ro'yxatda 7-11 ro'yxatdagi kod ko'rsatilgan, ildiz modulidagi `use` `pub use` ga o'zgartirilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```


<span class="caption">Listing 7-17: Making a name available for any code to use from a new scope with `pub use`</span>

Before this change, external code would have to call the `add_to_waitlist` function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`. Now that this `pub
use` has re-exported the `hosting` module from the root module, external code can now use the path `restaurant::hosting::add_to_waitlist()` instead.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With `pub use`, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library. We’ll look at another example of `pub use` and how it affects your crate’s documentation in the [“Exporting a Convenient Public API with `pub use`”][ch14-pub-use]<!-- ignore --> section of Chapter 14.

### Tashqi paketlardan foydalanish

In Chapter 2, we programmed a guessing game project that used an external package called `rand` to get random numbers. To use `rand` in our project, we added this line to *Cargo.toml*:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

*Cargo.toml*-ga `rand`ni dependency sifatida qo'shish Cargo-ga [crates.io](crates.io)-dan `rand` paketini va har qanday bog'liqliklarni yuklab olishni va `rand`ni loyihamiz uchun ishlatishni aytadi.

Then, to bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the crate, `rand`, and listed the items we wanted to bring into scope. Recall that in the [“Generating a Random Number”][rand]<!-- ignore --> section in Chapter 2, we brought the `Rng` trait into scope and called the `rand::thread_rng` function:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust hamjamiyatining a'zolari [crates.io](crates.io) saytida ko'plab paketlarni taqdim etishdi va ulardan birini o'z paketingizga olish xuddi shu bosqichlarni o'z ichiga oladi: ularni paketingizning *Cargo.toml* faylida roʻyxatga kiriting va `use` dan foydalanib, ularni cratelaridagi elementlarni qamrab oling.

Note that the standard `std` library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change *Cargo.toml* to include `std`. But we do need to refer to it with `use` to bring items from there into our package’s scope. For example, with `HashMap` we would use this line:

```rust
use std::collections::HashMap;
```

Bu standart kutubxona cratesining nomi bo'lgan `std` bilan boshlanadigan mutlaq yo'ldir.

### Uzun `use` ro'yxatini qisqartirish uchun ichki yo'llardan foydalanish

If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. For example, these two `use` statements we had in the Guessing Game in Listing 2-4 bring items from `std` into scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ, as shown in Listing 7-18.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```


<span class="caption">Listing 7-18: Specifying a nested path to bring multiple items with the same prefix into scope</span>

Kattaroq dasturlarda bir xil crate yoki moduldan ko'plab elementlarni o'rnatilgan yo'llar yordamida qamrab olish juda ko'p talab qilinadigan alohida `use` statementlari sonini kamaytirishi mumkin!

We can use a nested path at any level in a path, which is useful when combining two `use` statements that share a subpath. For example, Listing 7-19 shows two `use` statements: one that brings `std::io` into scope and one that brings `std::io::Write` into scope.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```


<span class="caption">Ro'yxat 7-19: biri ikkinchisining bir qismi bo'lgan ikkita `use` statementi</span>

The common part of these two paths is `std::io`, and that’s the complete first path. To merge these two paths into one `use` statement, we can use `self` in the nested path, as shown in Listing 7-20.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```


<span class="caption">Ro'yxat 7-20: Ro'yxat 7-19dagi yo'llarni bitta `use` statementiga birlashtirish</span>

Bu satr `std::io` va `std::io::Write` ni qamrab oladi.

### Glob operatori

Agar biz yo'lda belgilangan *barcha* umumiy elementlarni qamrovga kiritmoqchi bo'lsak, biz `*` glob operatori tomonidan keyingi yo'lni belgilashimiz mumkin:

```rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test into the `tests` module; we’ll talk about that in the [“How to Write Tests”][writing-tests]<!-- ignore --> section in Chapter 11. The glob operator is also sometimes used as part of the prelude pattern: see [the standard library documentation](../std/prelude/index.html#other-preludes)<!-- ignore -->
for more information on that pattern.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
