## Hello, Cargo!

Cargo - bu Rustning qurish tizimi va paketlar menejeri. Aksariyat Rustaceanlar o'zlarining Rust loyihalarini boshqarish uchun ushbu vositadan foydalanadilar, chunki Cargo siz uchun kodni yaratish, kodingizga bog'liq kutubxonalarni yuklab olish va ushbu kutubxonalarni yaratish kabi ko'plab vazifalarni bajaradi.(Biz sizning kodingizga kerak bo'lgan kutubxonalarni chaqiramiz
*dependencies*.)

Eng oddiy Rust dasturlari, biz hozirgacha yozganimiz kabi, hech qanday dependencieslarga ega emas. Agar biz  “Hello, world!” Cargo bilan loyiha bo'lsa, u faqat sizning kodingizni yaratish bilan shug'ullanadigan Cargo qismidan foydalanadi. Murakkab Rust dasturlarini yozganingizda, siz dependencylarni qo'shasiz va agar siz Cargo yordamida loyihani boshlasangiz, dependencylarni qo'shish osonroq bo'ladi.

Rust loyihalarining aksariyati Cargolardan foydalanganligi sababli, ushbu kitobning qolgan qismida siz ham Cargodan foydalanasiz deb taxmin qilinadi. [O'rnatish][installation]<!-- ignore -->  bo'limida muhokama qilingan rasmiy o'rnatuvchilardan foydalansangiz, Cargo Rust bilan birga keladi. Agar siz Rust-ni boshqa vositalar orqali o'rnatgan bo'lsangiz, terminalingizga quyidagilarni kiritish orqali Cargo o'rnatilganligini tekshiring:

```console
$ cargo --version
```

Agar siz versiya raqamini ko'rsangiz, sizda bor! Agar siz `command not found` kabi xatolikni ko'rsangiz, Cargoni qanday qilib alohida o'rnatish bo'yicha texnik hujjatlarni ko'rib chiqing.

### Cargo bilan loyiha yaratish

Keling, Cargo-dan foydalanib yangi loyiha yarataylik va u bizning asl “Hello, world!” loyihadan qanday farq qilishini ko'rib chiqaylik. O'zingizning *projects* jildigiga (yoki kodingizni saqlashga qaror qilgan joyingizga) qayting. Keyin istalgan operatsion tizimda quyidagilarni bajaring:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

Birinchi buyruq *hello_cargo* nomli yangi jild va loyihani yaratadi.
Biz loyihamizga *hello_cargo* deb nom berdik va Cargo o'z fayllarini xuddi shu nomdagi jildda yaratadi.

*hello_cargo* jildiga o'ting va fayllar ro'yxatini ko'ring.Cargo biz uchun ikkita fayl va bitta jild yaratganini ko'rasiz: *Cargo.toml* fayli va ichida *main.rs* fayli bo'lgan *src* jildi.

Shuningdek, u *.gitignore* fayli bilan birga yangi Git omborini ishga tushirdi. Mavjud Git omborida `cargo new` ni ishga tushirsangiz, Git fayllari yaratilmaydi; `cargo new - vcs=git` yordamida bu xatti-harakatni bekor qilishingiz mumkin.

> Eslatma: Git keng tarqalgan versiya boshqaruv tizimidir. Siz `--vcs` buyrugʻi yordamida
> `cargo new` ni boshqa versiyani boshqarish tizimidan foydalanishga yoki versiyani boshqarish
> tizimisiz foydalanishga oʻzgartirishingiz mumkin. Mavjud variantlarni
> ko'rish uchun `cargo new --help` ni ishga tushiring.

Siz tanlagan matn muharririda *Cargo.toml*ni oching. U 1-2 ro'yxatdagi kodga o'xshash bo'lishi kerak.

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

<span class="caption">Ro'yxat 1-2: `cargo new` tomonidan yaratilgan *Cargo.toml* tarkibi</span>

Bu fayl [*TOML*][toml]<!-- ignore --> da ((*Tom’s Obvious, Minimal
Language*) formati, bu Cargo konfiguratsiya formati.

Birinchi qator, `[package]`, bo'lim sarlavhasi bo'lib, quyidagi iboralar paketni sozlayotganligini bildiradi.Ushbu faylga qo'shimcha ma'lumot qo'shsak, biz boshqa bo'limlarni qo'shamiz.

Keyingi uchta qatorda Cargo dasturingizni kompilyatsiya qilish uchun kerak bo'lgan konfiguratsiya ma'lumotlarini o'rnatadi: Rustning nomi, versiyasi va foydalanish uchun nashri.
[E ilovasi][appendix-e]<!-- ignore -->da `edition` kaliti haqida gaplashamiz.

Oxirgi qator, `[dependencies]`, loyihangizning har qanday dependencylarini ro'yxatlash uchun bo'limning boshlanishi. Rustda kod paketlari *crates* deb ataladi. Ushbu loyiha uchun bizga boshqa crateslar kerak bo'lmaydi, lekin biz 2-bobdagi birinchi loyihada bo'lamiz, shuning uchun biz ushbu dependencies bo'limidan foydalanamiz.

Endi *src/main.rs* oching va qarang:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo “Hello, world!” siz uchun dastur, xuddi biz Ro'yxat 1-1 da yozganimiz kabi! Hozircha, bizning loyihamiz va yaratilgan Cargo loyihasi o'rtasidagi farq shundaki, Cargo kodni *src* jildiga joylashtirgan va bizda yuqori jildda *Cargo.toml* konfiguratsiya fayli mavjud.

Cargo expects your source files to live inside the *src* directory. Yuqori darajadagi loyiha jildi faqat README fayllari, litsenziya maʼlumotlari, konfiguratsiya fayllari va kodingizga aloqador boʻlmagan boshqa narsalar uchun moʻljallangan. Cargo-dan foydalanish loyihalaringizni tartibga solishga yordam beradi. Hamma narsaning o'rni bor va hamma narsa o'z o'rnida.

Agar siz “Hello, world!” bilan qilganimizdek, Cargo-dan foydalanmaydigan loyihani boshlagan bo'lsangiz, uni Cargo-dan foydalanadigan loyihaga aylantirishingiz mumkin. Loyiha kodini *src* jildiga o'tkazing va tegishli *Cargo.toml* faylini yarating.

### Cargo loyihasini qurish va ishga tushirish

Keling, “Hello, world!” ni qurish va ishga tushirishda nima farq qilishini ko'rib chiqaylik. Cargo bilan dasturni *hello_cargo* jildidan quyidagi buyruqni kiritish orqali loyihangizni quring:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Ushbu buyruq bajariladigan faylni joriy jildingizda emas, balki *target/debug/hello_cargo* da (yoki Windowsda *target\debug\hello_cargo.exe*)da  yaratadi. Odatiy tuzilish debug tuzilishi bo'lgani uchun Cargo ikkilik faylni *debug* nomli jildga joylashtiradi. Ushbu buyruq bilan bajariladigan faylni ishga tushirishingiz mumkin:

```console
$ ./target/debug/hello_cargo # yoki .\target\debug\hello_cargo.exe Windowsda
Hello, world!
```

Agar hammasi yaxshi bo'lsa, `Hello, world!` terminalga chop etilishi kerak.`cargo build` ni birinchi marta ishga tushirish ham Cargoning yuqori darajadagi yangi faylni yaratishiga olib keladi: *Cargo.lock*. Ushbu fayl loyihangizdagi dependencylarning aniq versiyalarini kuzatib boradi. Ushbu loyihada dependencylar yo'q, shuning uchun faylda kod biroz kam. Siz hech qachon ushbu faylni qo'lda o'zgartirishingiz shart emas; Cargo uning tarkibini siz uchun boshqaradi.

Biz hozirgina `cargo build` orqali loyihasini qurdik va uni `./target/debug/hello_cargo` bilan ishga tushirdik, lekin kodni kompilyatsiya qilish uchun `cargo run` dan ham foydalanishimiz va natijada bajariladigan faylni bitta buyruqda ishga tushirishimiz mumkin:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

`cargo run` dan foydalanish `cargo build` ni ishga tushirishdan ko'ra qulayroqdir va keyin ikkilik yo'lni to'liq ishlatadi, shuning uchun ko'pchilik ishlab chiquvchilar `cargo run` dan foydalanadilar.

E'tibor bering, bu safar biz `Hello_cargo` ni kompilyatsiya qilayotganini ko'rsatadigan natijani ko'rmadik. Cargo fayllar o'zgarmaganligini aniqladi, shuning uchun u qayta tiklanmadi, balki ikkilik faylni ishga tushirdi. Agar siz manba kodingizni o'zgartirgan bo'lsangiz, Cargo loyihani ishga tushirishdan oldin uni qayta build qilgan bo'lar edi va siz ushbu natijani ko'rgan bo'lar edingiz:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo shuningdek, `cargo check` deb nomlangan buyruqni taqdim etadi. Bu buyruq kompilyatsiya qilish uchun kodingizni tezda tekshiradi, lekin bajariladigan fayl yaratmaydi:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Nima uchun bajariladigan faylni xohlamaysiz? Ko'pincha `cargo check` `cargo build`dan ko'ra tezroq bo'ladi,, chunki u bajariladigan faylni yaratish bosqichini o'tkazib yuboradi. Agar siz kod yozish paytida ishingizni doimiy ravishda tekshirayotgan bo'lsangiz, `cargo check` dan foydalanish loyihangiz hali ham kompilyatsiya qilinayotganligini bildirish jarayonini tezlashtiradi! Shunday qilib, ko'plab Rustaceanlar vaqti-vaqti bilan `cargo check` ni amalga oshiradilar, chunki ular o'z dasturlarini kompilyatsiya qilishiga ishonch hosil qilish uchun yozadilar. Keyin ular bajariladigan fayldan foydalanishga tayyor bo'lgach, `cargo build` ni ishga tushiradilar.

Cargo haqida shu paytgacha o'rganganlarimizni takrorlaymiz:

* Biz `cargo new` yordamida loyiha yaratamiz.
* `cargo build` yordamida loyihani build qilishimi mumkin.
* Biz `cargo run` yordamida bir bosqichda loyiha build qilishimiz va ishga tushirishimiz mumkin.
* `cargo check` yordamida xatolarni tekshirish uchun binary  ishlab chiqarmasdan loyihani build qilishimiz mumkin.
* Build natijasini bizning kodimiz bilan bir xil jildda saqlash o'rniga, Cargo uni *target/debug* jildida saqlaydi.

Cargo-dan foydalanishning qo'shimcha afzalligi shundaki, qaysi operatsion tizimda ishlayotganingizdan qat'i nazar, buyruqlar bir xil bo'ladi. Shunday qilib, biz endi Linux va MacOS uchun Windows-ga nisbatan maxsus ko'rsatmalar bermaymiz.

### Loyihani Reliz qilish

When your project is finally ready for release, you can use `cargo build
--release` to compile it with optimizations. This command will create an
executable in *target/release* instead of *target/debug*. The optimizations
make your Rust code run faster, but turning them on lengthens the time it takes
for your program to compile. This is why there are two different profiles: one
for development, when you want to rebuild quickly and often, and another for
building the final program you’ll give to a user that won’t be rebuilt
repeatedly and that will run as fast as possible. If you’re benchmarking your
code’s running time, be sure to run `cargo build --release` and benchmark with
the executable in *target/release*.

### Cargo as Convention

With simple projects, Cargo doesn’t provide a lot of value over just using
`rustc`, but it will prove its worth as your programs become more intricate.
Once programs grow to multiple files or need a dependency, it’s much easier to
let Cargo coordinate the build.

Even though the `hello_cargo` project is simple, it now uses much of the real
tooling you’ll use in the rest of your Rust career. In fact, to work on any
existing projects, you can use the following commands to check out the code
using Git, change to that project’s directory, and build:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

For more information about Cargo, check out [its documentation][cargo].

## Summary

You’re already off to a great start on your Rust journey! In this chapter,
you’ve learned how to:

* Install the latest stable version of Rust using `rustup`
* Update to a newer Rust version
* Open locally installed documentation
* Write and run a “Hello, world!” program using `rustc` directly
* Create and run a new project using the conventions of Cargo

This is a great time to build a more substantial program to get used to reading
and writing Rust code. So, in Chapter 2, we’ll build a guessing game program.
If you would rather start by learning how common programming concepts work in
Rust, see Chapter 3 and then return to Chapter 2.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
