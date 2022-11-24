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

Cargo expects your source files to live inside the *src* directory. The
top-level project directory is just for README files, license information,
configuration files, and anything else not related to your code. Using Cargo
helps you organize your projects. There’s a place for everything, and
everything is in its place.

If you started a project that doesn’t use Cargo, as we did with the “Hello,
world!” project, you can convert it to a project that does use Cargo. Move the
project code into the *src* directory and create an appropriate *Cargo.toml*
file.

### Building and Running a Cargo Project

Now let’s look at what’s different when we build and run the “Hello, world!”
program with Cargo! From your *hello_cargo* directory, build your project by
entering the following command:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates an executable file in *target/debug/hello_cargo* (or
*target\debug\hello_cargo.exe* on Windows) rather than in your current
directory. Because the default build is a debug build, Cargo puts the binary in
a directory named *debug*. You can run the executable with this command:

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

If all goes well, `Hello, world!` should print to the terminal. Running `cargo
build` for the first time also causes Cargo to create a new file at the top
level: *Cargo.lock*. This file keeps track of the exact versions of
dependencies in your project. This project doesn’t have dependencies, so the
file is a bit sparse. You won’t ever need to change this file manually; Cargo
manages its contents for you.

We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can also use `cargo run` to compile the
code and then run the resultant executable all in one command:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Using `cargo run` is more convenient than having to remember to run `cargo
build` and then use the whole path to the binary, so most developers use `cargo
run`.

Notice that this time we didn’t see output indicating that Cargo was compiling
`hello_cargo`. Cargo figured out that the files hadn’t changed, so it didn’t
rebuild but just ran the binary. If you had modified your source code, Cargo
would have rebuilt the project before running it, and you would have seen this
output:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called `cargo check`. This command quickly checks
your code to make sure it compiles but doesn’t produce an executable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? Often, `cargo check` is much faster than
`cargo build` because it skips the step of producing an executable. If you’re
continually checking your work while writing the code, using `cargo check` will
speed up the process of letting you know if your project is still compiling! As
such, many Rustaceans run `cargo check` periodically as they write their
program to make sure it compiles. Then they run `cargo build` when they’re
ready to use the executable.

Let’s recap what we’ve learned so far about Cargo:

* We can create a project using `cargo new`.
* We can build a project using `cargo build`.
* We can build and run a project in one step using `cargo run`.
* We can build a project without producing a binary to check for errors using
  `cargo check`.
* Instead of saving the result of the build in the same directory as our code,
  Cargo stores it in the *target/debug* directory.

An additional advantage of using Cargo is that the commands are the same no
matter which operating system you’re working on. So, at this point, we’ll no
longer provide specific instructions for Linux and macOS versus Windows.

### Building for Release

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
