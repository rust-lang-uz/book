## Paketlar va Cratelar

Biz qamrab oladigan modul tizimining birinchi qismlari paketlar va cratelardir.

*Crate* - bu Rust kompilyatori bir vaqtning o'zida ko'rib chiqadigan eng kichik kod miqdori. Agar siz `cargo` o'rniga `rustc` ni ishga tushirsangiz va bitta manba faylga o'tsangiz ham (biz 1-bob, "Rust dasturini yozish va ishga tushirish"da qilganimiz kabi), kompilyator bu faylni crate sifatida ko'radi. Cratelar modullarni o'z ichiga olishi mumkin va modullar crate bilan tuzilgan boshqa fayllarda aniqlanishi mumkin, buni keyingi bo'limlarda ko'rib chiqamiz.

Crate ikkita shakldan birida bo'lishi mumkin: binary crate yoki kutubxona cratesi.
*Binary cratelar* - bu buyruq qatori dasturi yoki server kabi ishga tushirishingiz mumkin bo'lgan bajariladigan faylga kompilyatsiya qilishingiz mumkin bo'lgan dasturlar. Har birida bajariladigan fayl ishga tushganda nima sodir bo'lishini belgilaydigan `main` funksiyasi bo'lishi kerak. Biz hozirgacha yaratgan barcha cratelar binary cratelar edi.

*Kutubxona cratelari* `main` funksiyaga ega emas va ular bajariladigan faylga kompilyatsiya qilinmaydi. Buning o'rniga, ular bir nechta loyihalar bilan bo'lishish uchun mo'ljallangan funksionallikni belgilaydi. Misol uchun, biz [2-bobda][rand]<!-- ignore -->  ishlatgan `rand` cratesi tasodifiy sonlarni yaratuvchi funksionallikni taʼminlaydi.
Ko'pincha Rustaceanlar  “crate” deganda, ular kutubxona crateni anglatadi va ular "kutubxona" ning umumiy dasturlash tushunchasi bilan "crate" dan foydalanadilar.

*Crate root* bu Rust kompilyatori cratengizning ildiz modulini yaratishni boshlaydigan manba fayldir (biz modullarni [“Qo'llanish doirasi va maxfiylikni nazorat qilish uchun modullarni aniqlash”][modules]<!-- ignore --> bo‘limida chuqur tushuntiramiz).

A *package* is a bundle of one or more crates that provides a set of
functionality. A package contains a *Cargo.toml* file that describes how to
build those crates. Cargo is actually a package that contains the binary crate
for the command-line tool you’ve been using to build your code. The Cargo
package also contains a library crate that the binary crate depends on. Other
projects can depend on the Cargo library crate to use the same logic the Cargo
command-line tool uses.

A package can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that’s a
library or binary crate.

Let’s walk through what happens when we create a package. First, we enter the
command `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

After we run `cargo new`, we use `ls` to see what Cargo creates. In the project
directory, there’s a *Cargo.toml* file, giving us a package. There’s also a
*src* directory that contains *main.rs*. Open *Cargo.toml* in your text editor,
and note there’s no mention of *src/main.rs*. Cargo follows a convention that
*src/main.rs* is the crate root of a binary crate with the same name as the
package. Likewise, Cargo knows that if the package directory contains
*src/lib.rs*, the package contains a library crate with the same name as the
package, and *src/lib.rs* is its crate root. Cargo passes the crate root files
to `rustc` to build the library or binary.

Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
