# Guessing o'yinini dasturlash

Keling, birgalikda amaliy loyiha orqali Rustga o'taylik! Ushbu bob sizni bir nechta umumiy Rust tushunchalari bilan tanishtirib, ulardan haqiqiy dasturda qanday foydalanishni ko'rsatib beradi.  Siz `let`, `match`, usullari, bog'langan funksiyalar, external cratelardan foydalanish va boshqalar haqida bilib olasiz! Keyingi boblarda biz ushbu fikrlarni batafsilroq ko'rib chiqamiz. Ushbu bobda siz faqat asoslarni mashq qilasiz.

Biz klassik boshlang'ich dasturlash muammosini amalga oshiramiz: taxmin qilish o'yini. Bu qanday ishlaydi: dastur 1 dan 100 gacha tasodifiy butun son hosil qiladi. Keyin u o'yinchini taxmin qilishni taklif qiladi.Tahmin kiritilgandan so'ng, dastur taxmin kichik yoki katta ekanligini ko'rsatadi. IAgar taxmin to'g'ri bo'lsa, o'yin tabrik xabarini chop etadi va chiqadi.

## Yangi loyiha yaratish

Yangi loyihani o'rnatish uchun 1-bobda yaratgan *projects* jildiga o'ting va Cargo-dan foydalanib yangi loyiha yarating, masalan:

```console
$ cargo new guessing_game
$ cd guessing_game
```

Birinchi `cargo new` buyrug'i birinchi argument sifatida loyiha nomini (`guessing_game`)ni oladi. Ikkinchi buyruq yangi loyiha jildiga kiradi.

Yaratilgan *Cargo.toml* fayliga qarang:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

1-bobda ko'rganingizdek, `cargo new` “Hello, world!”  so'zini yaratadi. siz uchun dastur. *src/main.rs* faylini tekshiring:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Keling, ushbu "Hello, world!" dasturni yarating va cargo run buyrug'i yordamida ishga tushiring :

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Hello, world!
```

`run` buyrug‘i loyihani tezda takrorlash kerak bo‘lganda foydali bo‘ladi, biz bu o‘yinda qilganimizdek, keyingisiga o‘tishdan oldin har bir iteratsiyani tezda sinab ko‘ramiz.

*src/main.rs* faylini qayta oching. Siz ushbu fayldagi barcha kodlarni yozasiz.

## Guessga o'yiniga ishlov berish

Taxmin qilish o'yini dasturining birinchi qismi foydalanuvchi kiritishini so'raydi, ushbu kiritishni qayta ishlaydi va kirish kutilgan shaklda ekanligini tekshiradi. Boshlash uchun biz o'yinchiga taxmin kiritishga ruxsat beramiz. 2-1 ro'yxatdagi kodni *src/main.rs* ichiga kiriting.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
use std::io;

fn main() {
    println!("Raqamni topish o'yini!");

    println!("Iltimos, taxminingizni kiriting.");

    let mut taxmin = String::new();

    io::stdin()
        .read_line(&mut taxmin)
        .expect("Satrni o‘qib bo‘lmadi");

    println!("Sizni taxminingiz: {guess}");
}
```

<span class="caption">Ro'yxat 2-1: Foydalanuvchi tomonidan taxmin qilinadigan va uni chop etadigan kod</span>

Ushbu kod juda ko'p ma'lumotlarni o'z ichiga oladi, shuning uchun uni satrga o'tkazamiz. Foydalanuvchi kiritishini olish va natijani chiqish sifatida chop etish uchun biz `io` kirish/chiqish kutubxonasini qamrab olishimiz kerak. `io` kutubxonasi `std` deb nomlanuvchi standart kutubxonadan keladi:

```rust,ignore
use std::io;
```

Odatda, Rust standart kutubxonada belgilangan elementlar to'plamiga ega bo'lib, u har bir dastur doirasiga kiradi. Ushbu to'plam *prelude* deb ataladi va siz undagi hamma narsani [standart kutubxona texnik hujjatlarida][prelude] ko'rishingiz mumkin.

Agar siz foydalanmoqchi bo'lgan tur muqaddimada bo'lmasa, siz ushbu turni `use` iborasi bilan aniq kiritishingiz kerak. `std::io` kutubxonasidan foydalanish sizga bir qator foydali xususiyatlarni, jumladan, foydalanuvchi kiritishini qabul qilish imkoniyatini beradi.

1-bobda ko'rganingizdek, `main` funksiya dasturga kirish nuqtasidir:

```rust,ignore
fn main() {
```

`fn` sintaksisi yangi funktsiyani e'lon qiladi; Qavslar, `()`, hech qanday parametr yo'qligini bildiradi; va jingalak qavs, `{`, funksiyaning asosiy qismini boshlaydi.

1-bobda ham bilib olganingizdek, `println!` bu ekranga satrni chop etuvchi makros:

```rust,ignore
    println!("Raqamni topish o'yini!");

    println!("Iltimos, taxminingizni kiriting.");
```

Ushbu kod o'yin nima ekanligini ko'rsatuvchi va foydalanuvchidan ma'lumot so'rashni so'rashni chop etadi.

### O'zgaruvchilar bilan qiymatlarni saqlash

Keyinchalik, foydalanuvchi ma'lumotlarini saqlash uchun *o'zgaruvchi* yaratamiz, masalan:

```rust,ignore
    let mut taxmin = String::new();
```

Endi dastur qiziqarli bo'lib bormoqda! Bu kichik satrda juda ko'p narsa bor. O'zgaruvchini yaratish uchun `let` iborasidan foydalanamiz. Mana yana bir misol:

```rust,ignore
let olmalar = 5;
```

Bu qator `olmalar` nomli yangi o‘zgaruvchini yaratadi va uni 5 qiymatiga bog‘laydi. Rustda o'zgaruvchilar standard bo'yicha o'zgarmasdir, ya'ni o'zgaruvchiga qiymat berganimizdan keyin qiymat o'zgarmaydi.Biz ushbu kontseptsiyani 3-bobdagi [”O'zgaruvchilar va O'zgaruvchanlik”][variables-and-mutability]<!-- ignore --> bo'limida batafsil muhokama qilamiz. Oʻzgaruvchini oʻzgaruvchan qilish uchun oʻzgaruvchi nomidan oldin `mut` qoʻshamiz:

```rust,ignore
let olmalar = 5; // o'zgarmas
let mut bananlar = 5; // o'zgaruvchan
```

> Eslatma: `//` sintaksisi satr oxirigacha davom etadigan izohni
> boshlaydi. Rust izohlarda hamma narsani e'tiborsiz qoldiradi.
> Izohlarni [3-bobda][comments]<!-- ignore --> batafsilroq muhokama qilamiz.

Taxmin qilish o'yin dasturiga qaytsak, endi bilasizki, `let mut taxmin` `taxmin` nomli o'zgaruvchan o'zgaruvchini kiritadi. Teng belgisi (`=`) Rustga biz hozir biror narsani oʻzgaruvchiga bogʻlamoqchi ekanligimizni bildiradi. Tenglik belgisining o'ng tomonida `taxmin` bog'langan qiymat joylashgan bo'lib, u `String::new` funksiyasini chaqirish natijasidir, bu `String`ning yangi nusxasini qaytaradi.
[String][string]<!-- ignore --> standart kutubxona tomonidan taqdim etilgan satr turi bo'lib, u rivojlantirib boriladigan, UTF-8 kodlangan matn bitidir.

`::new` qatoridagi `::` sintaksisi `new` `String` tipidagi bog`langan funksiya ekanligini bildiradi. *Assosiatsiyalangan funksiya* bu funksiya
turida amalga oshiriladi, bu holda `String`. Ushbu `new` funksiya yangi, bo'sh qatorni yaratadi. Siz ko'p turdagi `new` funksiyani topasiz, chunki u qandaydir yangi qiymatni yaratadigan funksiyaning umumiy nomi.

To'liq `let mut taxmin = String::new();` qatori hozirda `String` ning yangi, bo'sh nusxasiga bog'langan o'zgaruvchan o'zgaruvchini yaratadi.

### Foydalanuvchi ma'lumotlarini qabul qilish

Eslatib o'tamiz, biz dasturning birinchi qatoriga `use std::io;` bilan standart kutubxonadan kiritish/chiqarish funksiyasini kiritgan edik. Endi biz `io` modulidan `stdin` funksiyasini chaqiramiz, bu bizga foydalanuvchi kiritishini boshqarish imkonini beradi:

```rust,ignore
    io::stdin()
        .read_line(&mut taxmin)
```

Agar biz dasturning boshida `use std::io;` bilan `io` kutubxonasini import qilmagan bo'lsak, biz ushbu funktsiya chaqiruvini `std::io::stdin` sifatida yozish orqali funksiyadan foydalanishimiz xam mumkin. `stdin` funksiyasi [`std::io::Stdin`][iostdin]<!-- ignore --> misolini qaytaradi, bu sizning terminalingiz uchun standart kirish uchun asosni ifodalovchi tur.

Keyinchalik, `.read_line(&mut guess)` qatori foydalanuvchidan ma'lumot olish uchun standart kiritish nuqtasidagi [`read_line`][read_line]<!--
ignore --> usulini chaqiradi.
Shuningdek, foydalanuvchi kiritgan maʼlumotlarni qaysi qatorda saqlash kerakligini aytish uchun `read_line` ga argument sifatida `&mut taxmin` ni oʻtkazamiz. `read_line` ning toʻliq vazifasi foydalanuvchi nima yozganidan qatʼiy nazar standart kiritishga olish va uni satrga qoʻshishdir (uning mazmunini qayta yozmasdan), shuning uchun biz bu qatorni argument sifatida beramiz. String argumenti o'zgaruvchan bo'lishi kerak, shuning uchun usul string tarkibini o'zgartirishi mumkin.

`&` bu argumentning *ma'lumotnoma* ekanligini bildiradi, bu sizga kodingizning bir nechta qismlariga ushbu ma'lumotni xotiraga bir necha marta nusxalash kerak bo'lmasdan bitta ma'lumotga kirish imkonini beradi. Ma'lumotnomalar murakkab xususiyat bo'lib, Rustning asosiy afzalliklaridan biri havolalardan foydalanish qanchalik xavfsiz va oson ekanligidir. Ushbu dasturni tugatish uchun ko'p bilimlrga ega bo'lishingiz shart emas. Hozircha siz bilishingiz kerak bo'lgan narsa shundaki, o'zgaruvchilar singari, havolalar ham standard bo'yicha o'zgarmasdir. Demak, uni oʻzgaruvchan qilish uchun `&taxmin` oʻrniga `&mut taxmin` yozish kerak. (4-bobda havolalar ko'proq va yaxshiroq tushuntiriladi)

<!-- Old heading. Do not remove or links may break. -->
<a id="handling-potential-failure-with-the-result-type"></a>

### Potensial nosozlikni `Result` turi bilan hal qilish

Biz hali ham ushbu kod qatori ustida ishlayapmiz. Biz hozir matnning uchinchi qatorini muhokama qilmoqdamiz, lekin u hali ham bitta mantiqiy kod qatorining bir qismi ekanligini unutmang. Keyingi qism bu method:

```rust,ignore
        .expect("Satrni o‘qib bo‘lmadi");
```

Biz ushbu kodni quyidagicha yozishimiz mumkin edi:

```rust,ignore
io::stdin().read_line(&mut taxmin).expect("Satrni o‘qib bo‘lmadi");
```

Biroq, bitta uzun qatorni o'qish qiyin, shuning uchun uni bo'lish yaxshidir. `.method_name()` sintaksisi bilan methodni chaqirganda uzun qatorlarni ajratishga yordam berish uchun yangi qator va boshqa bo'shliqlarni kiritish ko'pincha oqilona. Endi bu kod nima qilishini muhokama qilaylik.

Yuqorida aytib o'tilganidek, `read_line` foydalanuvchi kiritgan narsani biz unga o'tkazadigan qatorga qo'yadi, lekin u `Result` qiymatini ham qaytaradi. [`Result`][result]<!-- ignore --> - ko'pincha *enum* deb ataladigan [*enumeration*][enums]<!-- ignore -->, bu bir nechta mumkin bo'lgan holatlardan birida bo'lishi mumkin bo'lgan tur. Har bir mumkin bo'lgan holatni *variant* deb ataymiz.

[6-bobda][enums]<!-- ignore --> enumlar batafsilroq yoritiladi. Ushbu `Result` turlarining maqsadi xatolarni qayta ishlash ma'lumotlarini kodlashdir.

`Result` variantlari `Ok` va `Err`. `Ok` varianti operatsiya muvaffaqiyatli bo'lganligini bildiradi va `Ok` ichida muvaffaqiyatli yaratilgan qiymat.
`Err` varianti operatsiya bajarilmaganligini bildiradi va `Err` operatsiya qanday yoki nima uchun bajarilmagani haqida maʼlumotni oʻz ichiga oladi.

`Result` turidagi qiymatlar, har qanday turdagi qiymatlar kabi, ularda aniqlangan usullarga ega. `Result` misolida siz murojat qilishingiz mumkin bo'lgan [`expect` methodi][expect]<!-- ignore --> mavjud. Agar `Result` ning ushbu namunasi `Err` qiymati bo'lsa, `expect` dasturning ishlamay qolishiga olib keladi va `expect` ga argument sifatida siz uzatgan xabarni ko'rsatadi. Agar `read_line` usuli `Err`ni qaytarsa, bu asosiy operatsion tizimdan kelgan xato natijasi bo'lishi mumkin.

Agar `Result`ning ushbu namunasi `Ok` qiymati bo‘lsa, `expect` `Ok` ushlab turgan qaytarish qiymatini oladi va siz undan foydalanishingiz uchun aynan shu qiymatni sizga qaytaradi.
Bunday holda, bu qiymat foydalanuvchi kiritishidagi baytlar soni.

Agar siz `expect` ga murojat qilmasangiz, dastur kompilyatsiya qilinadi, lekin siz ogohlantirish olasiz:

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&taxmin guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust warns that you haven’t used the `Result` value returned from `read_line`,
indicating that the program hasn’t handled a possible error.

The right way to suppress the warning is to actually write error-handling code,
but in our case we just want to crash this program when a problem occurs, so we
can use `expect`. You’ll learn about recovering from errors in [Chapter
9][recover]<!-- ignore -->.

### Printing Values with `println!` Placeholders

Aside from the closing curly bracket, there’s only one more line to discuss in
the code so far:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

This line prints the string that now contains the user’s input. The `{}` set of
curly brackets is a placeholder: think of `{}` as little crab pincers that hold
a value in place. When printing the value of a variable, the variable name can
go inside the curly brackets. When printing the result of evaluating an
expression, place empty curly brackets in the format string, then follow the
format string with a comma-separated list of expressions to print in each empty
curly bracket placeholder in the same order. Printing a variable and the result
of an expression in one call to `println!` would look like this:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

This code would print `x = 5 and y = 12`.

### Testing the First Part

Let’s test the first part of the guessing game. Run it using `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

At this point, the first part of the game is done: we’re getting input from the
keyboard and then printing it.

## Generating a Secret Number

Next, we need to generate a secret number that the user will try to guess. The
secret number should be different every time so the game is fun to play more
than once. We’ll use a random number between 1 and 100 so the game isn’t too
difficult. Rust doesn’t yet include random number functionality in its standard
library. However, the Rust team does provide a [`rand` crate][randcrate] with
said functionality.

### Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files. The project
we’ve been building is a *binary crate*, which is an executable. The `rand`
crate is a *library crate*, which contains code that is intended to be used in
other programs and can’t be executed on its own.

Cargo’s coordination of external crates is where Cargo really shines. Before we
can write code that uses `rand`, we need to modify the *Cargo.toml* file to
include the `rand` crate as a dependency. Open that file now and add the
following line to the bottom, beneath the `[dependencies]` section header that
Cargo created for you. Be sure to specify `rand` exactly as we have here, with
this version number, or the code examples in this tutorial may not work:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

In the *Cargo.toml* file, everything that follows a header is part of that
section that continues until another section starts. In `[dependencies]` you
tell Cargo which external crates your project depends on and which versions of
those crates you require. In this case, we specify the `rand` crate with the
semantic version specifier `0.8.5`. Cargo understands [Semantic
Versioning][semver]<!-- ignore --> (sometimes called *SemVer*), which is a
standard for writing version numbers. The specifier `0.8.5` is actually
shorthand for `^0.8.5`, which means any version that is at least 0.8.5 but
below 0.9.0.

Cargo considers these versions to have public APIs compatible with version
0.8.5, and this specification ensures you’ll get the latest patch release that
will still compile with the code in this chapter. Any version 0.9.0 or greater
is not guaranteed to have the same API as what the following examples use.

Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Listing 2-2: The output from running `cargo build` after
adding the rand crate as a dependency</span>

You may see different version numbers (but they will all be compatible with the
code, thanks to SemVer!) and different lines (depending on the operating
system), and the lines may be in a different order.

When we include an external dependency, Cargo fetches the latest versions of
everything that dependency needs from the *registry*, which is a copy of data
from [Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem
post their open source Rust projects for others to use.

After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates listed that aren’t already downloaded. In this case,
although we only listed `rand` as a dependency, Cargo also grabbed other crates
that `rand` depends on to work. After downloading the crates, Rust compiles
them and then compiles the project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you
won’t get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven’t changed anything
about them in your *Cargo.toml* file. Cargo also knows that you haven’t changed
anything about your code, so it doesn’t recompile that either. With nothing to
do, it simply exits.

If you open the *src/main.rs* file, make a trivial change, and then save it and
build again, you’ll only see two lines of output:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

These lines show that Cargo only updates the build with your tiny change to the
*src/main.rs* file. Your dependencies haven’t changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those.

#### Ensuring Reproducible Builds with the *Cargo.lock* File

Cargo has a mechanism that ensures you can rebuild the same artifact every time
you or anyone else builds your code: Cargo will use only the versions of the
dependencies you specified until you indicate otherwise. For example, say that
next week version 0.8.6 of the `rand` crate comes out, and that version
contains an important bug fix, but it also contains a regression that will
break your code. To handle this, Rust creates the *Cargo.lock* file the first
time you run `cargo build`, so we now have this in the *guessing_game*
directory.

When you build a project for the first time, Cargo figures out all the versions
of the dependencies that fit the criteria and then writes them to the
*Cargo.lock* file. When you build your project in the future, Cargo will see
that the *Cargo.lock* file exists and will use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at 0.8.5 until you explicitly upgrade, thanks to the *Cargo.lock* file.
Because the *Cargo.lock* file is important for reproducible builds, it’s often
checked into source control with the rest of the code in your project.

#### Updating a Crate to Get a New Version

When you *do* want to update a crate, Cargo provides the command `update`,
which will ignore the *Cargo.lock* file and figure out all the latest versions
that fit your specifications in *Cargo.toml*. Cargo will then write those
versions to the *Cargo.lock* file. Otherwise, by default, Cargo will only look
for versions greater than 0.8.5 and less than 0.9.0. If the `rand` crate has
released the two new versions 0.8.6 and 0.9.0, you would see the following if
you ran `cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```

Cargo ignores the 0.9.0 release. At this point, you would also notice a change
in your *Cargo.lock* file noting that the version of the `rand` crate you are
now using is 0.8.6. To use `rand` version 0.9.0 or any version in the 0.9.*x*
series, you’d have to update the *Cargo.toml* file to look like this instead:

```toml
[dependencies]
rand = "0.9.0"
```

The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.

There’s a lot more to say about [Cargo][doccargo]<!-- ignore --> and [its
ecosystem][doccratesio]<!-- ignore -->, which we’ll discuss in Chapter 14, but
for now, that’s all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.

### Generating a Random Number

Let’s start using `rand` to generate a number to guess. The next step is to
update *src/main.rs*, as shown in Listing 2-3.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Listing 2-3: Adding code to generate a random
number</span>

First we add the line `use rand::Rng;`. The `Rng` trait defines methods that
random number generators implement, and this trait must be in scope for us to
use those methods. Chapter 10 will cover traits in detail.

Next, we’re adding two lines in the middle. In the first line, we call the
`rand::thread_rng` function that gives us the particular random number
generator we’re going to use: one that is local to the current thread of
execution and is seeded by the operating system. Then we call the `gen_range`
method on the random number generator. This method is defined by the `Rng`
trait that we brought into scope with the `use rand::Rng;` statement. The
`gen_range` method takes a range expression as an argument and generates a
random number in the range. The kind of range expression we’re using here takes
the form `start..=end` and is inclusive on the lower and upper bounds, so we
need to specify `1..=100` to request a number between 1 and 100.

> Note: You won’t just know which traits to use and which methods and functions
> to call from a crate, so each crate has documentation with instructions for
> using it. Another neat feature of Cargo is that running the `cargo doc
> --open` command will build documentation provided by all your dependencies
> locally and open it in your browser. If you’re interested in other
> functionality in the `rand` crate, for example, run `cargo doc --open` and
> click `rand` in the sidebar on the left.

The second new line prints the secret number. This is useful while we’re
developing the program to be able to test it, but we’ll delete it from the
final version. It’s not much of a game if the program prints the answer as soon
as it starts!

Try running the program a few times:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

You should get different random numbers, and they should all be numbers between
1 and 100. Great job!

## Comparing the Guess to the Secret Number

Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won’t compile just yet, as we will
explain.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>

First we add another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. The `Ordering` type
is another enum and has the variants `Less`, `Greater`, and `Equal`. These are
the three outcomes that are possible when you compare two values.

Then we add five new lines at the bottom that use the `Ordering` type. The
`cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing `guess` to `secret_number`. Then it returns a variant of the
`Ordering` enum we brought into scope with the `use` statement. We use a
[`match`][match]<!-- ignore --> expression to decide what to do next based on
which variant of `Ordering` was returned from the call to `cmp` with the values
in `guess` and `secret_number`.

A `match` expression is made up of *arms*. An arm consists of a *pattern* to
match against, and the code that should be run if the value given to `match`
fits that arm’s pattern. Rust takes the value given to `match` and looks
through each arm’s pattern in turn. Patterns and the `match` construct are
powerful Rust features: they let you express a variety of situations your code
might encounter and they make sure you handle them all. These features will be
covered in detail in Chapter 6 and Chapter 18, respectively.

Let’s walk through an example with the `match` expression we use here. Say that
the user has guessed 50 and the randomly generated secret number this time is
38.

When the code compares 50 to 38, the `cmp` method will return
`Ordering::Greater` because 50 is greater than 38. The `match` expression gets
the `Ordering::Greater` value and starts checking each arm’s pattern. It looks
at the first arm’s pattern, `Ordering::Less`, and sees that the value
`Ordering::Greater` does not match `Ordering::Less`, so it ignores the code in
that arm and moves to the next arm. The next arm’s pattern is
`Ordering::Greater`, which *does* match `Ordering::Greater`! The associated
code in that arm will execute and print `Too big!` to the screen. The `match`
expression ends after the first successful match, so it won’t look at the last
arm in this scenario.

However, the code in Listing 2-4 won’t compile yet. Let’s try it:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let mut guess = String::new()`, Rust was able to infer that `guess` should be
a `String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few of Rust’s number types can have a value between 1
and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a
64-bit number; as well as others. Unless otherwise specified, Rust defaults to
an `i32`, which is the type of `secret_number` unless you add type information
elsewhere that would cause Rust to infer a different numerical type. The reason
for the error is that Rust cannot compare a string and a number type.

Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it numerically to the secret number. We do
so by adding this line to the `main` function body:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

The line is:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but helpfully Rust allows us to shadow the
previous value of `guess` with a new one. *Shadowing* lets us reuse the `guess`
variable name rather than forcing us to create two unique variables, such as
`guess_str` and `guess`, for example. We’ll cover this in more detail in
[Chapter 3][shadowing]<!-- ignore -->, but for now, know that this feature is
often used when you want to convert a value from one type to another type.

We bind this new variable to the expression `guess.trim().parse()`. The `guess`
in the expression refers to the original `guess` variable that contained the
input as a string. The `trim` method on a `String` instance will eliminate any
whitespace at the beginning and end, which we must do to be able to compare the
string to the `u32`, which can only contain numerical data. The user must press
<span class="keystroke">enter</span> to satisfy `read_line` and input their
guess, which adds a newline character to the string. For example, if the user
types <span class="keystroke">5</span> and presses <span
class="keystroke">enter</span>, `guess` looks like this: `5\n`. The `\n`
represents “newline.” (On Windows, pressing <span
class="keystroke">enter</span> results in a carriage return and a newline,
`\r\n`.) The `trim` method eliminates `\n` or `\r\n`, resulting in just `5`.

The [`parse` method on strings][parse]<!-- ignore --> converts a string to
another type. Here, we use it to convert from a string to a number. We need to
tell Rust the exact number type we want by using `let guess: u32`. The colon
(`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust has a
few built-in number types; the `u32` seen here is an unsigned, 32-bit integer.
It’s a good default choice for a small positive number. You’ll learn about
other number types in [Chapter 3][integers]<!-- ignore -->.

Additionally, the `u32` annotation in this example program and the comparison
with `secret_number` means Rust will infer that `secret_number` should be a
`u32` as well. So now the comparison will be between two values of the same
type!

The `parse` method will only work on characters that can logically be converted
into numbers and so can easily cause errors. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in [“Handling Potential Failure with
`Result`”](#handling-potential-failure-with-result)<!-- ignore-->). We’ll treat
this `Result` the same way by using the `expect` method again. If `parse`
returns an `Err` `Result` variant because it couldn’t create a number from the
string, the `expect` call will crash the game and print the message we give it.
If `parse` can successfully convert the string to a number, it will return the
`Ok` variant of `Result`, and `expect` will return the number that we want from
the `Ok` value.

Let’s run the program now:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: guess the number correctly,
guess a number that is too high, and guess a number that is too low.

We have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!

## Allowing Multiple Guesses with Looping

The `loop` keyword creates an infinite loop. We’ll add a loop to give users
more chances at guessing the number:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

As you can see, we’ve moved everything from the guess input prompt onward into
a loop. Be sure to indent the lines inside the loop another four spaces each
and run the program again. The program will now ask for another guess forever,
which actually introduces a new problem. It doesn’t seem like the user can quit!

The user could always interrupt the program by using the keyboard shortcut
<span class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster, as mentioned in the `parse` discussion in [“Comparing the
Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!--
ignore -->: if the user enters a non-number answer, the program will crash. We
can take advantage of that to allow the user to quit, as shown here:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Typing `quit` will quit the game, but as you’ll notice, so will entering any
other non-number input. This is suboptimal, to say the least; we want the game
to also stop when the correct number is guessed.

### Quitting After a Correct Guess

Let’s program the game to quit when the user wins by adding a `break` statement:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.

### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess`
is converted from a `String` to a `u32`, as shown in Listing 2-5.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>

We switch from an `expect` call to a `match` expression to move from crashing
on an error to handling the error. Remember that `parse` returns a `Result`
type and `Result` is an enum that has the variants `Ok` and `Err`. We’re using
a `match` expression here, as we did with the `Ordering` result of the `cmp`
method.

If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resultant number. That `Ok` value will
match the first arm’s pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we’re creating.

If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catchall value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So the program will
execute the second arm’s code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So, effectively, the
program ignores all errors that `parse` might encounter!

Now everything in the program should work as expected. Let’s try it:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Awesome! With one tiny final tweak, we will finish the guessing game. Recall
that the program is still printing the secret number. That worked well for
testing, but it ruins the game. Let’s delete the `println!` that outputs the
secret number. Listing 2-6 shows the final code.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Listing 2-6: Complete guessing game code</span>

At this point, you’ve successfully built the guessing game. Congratulations!

## Summary

This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, functions, the use of external crates, and more. In the next
few chapters, you’ll learn about these concepts in more detail. Chapter 3
covers concepts that most programming languages have, such as variables, data
types, and functions, and shows how to use them in Rust. Chapter 4 explores
ownership, a feature that makes Rust different from other languages. Chapter 5
discusses structs and method syntax, and Chapter 6 explains how enums work.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
