## Crateni Crates.io-ga nashr qilish

Biz [crates.io](https://crates.io/)<!-- ignore --> paketlaridan loyihamizga dependency sifatida foydalandik, lekin siz oʻz paketlaringizni nashr(publish) qilish orqali kodingizni boshqa odamlar bilan ham baham koʻrishingiz mumkin. [crates.io](https://crates.io/)<!-- ignore --> saytidagi crate registri paketlaringizning manba kodini tarqatadi, shuning uchun u birinchi navbatda open source kodni saqlaydi.

Rust va Cargoda publish etilgan paketingizni odamlar topishi va undan foydalanishini osonlashtiradigan funksiyalar mavjud. Biz ushbu xususiyatlarning ba'zilari haqida keyin gaplashamiz va keyin paketni qanday nashr(publish) qilishni tushuntiramiz.

### Foydali hujjatlarga(documentation) sharhlar(comment) qo'yish

Paketlaringizni to'g'ri hujjatlashtirish boshqa foydalanuvchilarga ulardan qanday va qachon foydalanishni bilishga yordam beradi, shuning uchun texnik hujjatlarni yozish uchun vaqt sarflashga arziydi. 3-bobda biz Rust kodini ikkita slash `//` yordamida qanday izohlashni(comment) muhokama qildik. Rust shuningdek, HTML hujjatlarini yaratadigan *documentation comment* deb nomlanuvchi hujjatlar uchun o'ziga xos izohga ega. HTML sizning cratengiz qanday *impelemnent qilinganidan* farqli o'laroq, sizning cratengizdan qanday *foydalanishni* bilishga qiziqqan dasturchilar uchun mo'ljallangan umumiy API elementlari uchun hujjat sharhlari mazmunini ko'rsatadi.

Hujjatlarga sharhlar ikkita o'rniga uchta slashdan foydalaniladi, `///` va matnni formatlash uchun Markdown notationni qo'llab-quvvatlaydi. Hujjatlarga sharhlarni ular hujjatlashtirilayotgan element oldiga qo'ying. 14-1 Ro'yxatda `my_crate` nomli cratedagi `bir_qoshish` funksiyasi uchun hujjat sharhlari ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

<span class="caption">Ro'yxat 14-1: Funksiya uchun hujjat sharhi(documentation comment</span>

Bu yerda biz `bir_qoshish` funksiyasi nima qilishini tavsiflab beramiz, `Misollar` sarlavhasi bilan bo‘limni boshlaymiz, so‘ngra `bir_qoshish`  funksiyasidan qanday foydalanishni ko‘rsatadigan kodni taqdim etamiz. Biz ushbu hujjat sharhidan HTML hujjatlarini `cargo doc`ni ishga tushirish orqali yaratishimiz mumkin. Bu buyruq Rust bilan tarqatilgan `rustdoc` toolini ishga tushiradi va yaratilgan HTML hujjatlarini *target/doc* jildiga joylashtiradi.

Qulaylik uchun `cargo doc --open` ni ishga tushirish joriy crate hujjatlari uchun HTML-ni yaratadi (shuningdek, cratengizning barcha dependencilari uchun hujjatlar) va natijani veb-brauzerda ochadi. `bir_qoshish` funksiyasiga o‘ting va 14-1-rasmda ko‘rsatilganidek, hujjat sharhlaridagi matn qanday ko‘rsatilishini ko‘rasiz:

<img alt="Rendered HTML documentation for the `bir_qoshish` function of `my_crate`" src="img/trlpuz1.png" class="center" />

<span class="caption">14-1-Rasm: `bir_qoshish` funksiyasi uchun HTML hujjatlari</span>

#### Tez-tez ishlatiladigan bo'limlar

Biz HTML-da `Misollar` sarlavhali bo'lim yaratish uchun 14-1 ro'yxatdagi `# Misollar` Markdown sarlavhasidan foydalandik. Mualliflar o'z hujjatlarida tez-tez foydalanadigan boshqa bo'limlar:

* **Panics**: Hujjat yozilayotan funksiya senariylari panic qo'zg'atishi mumkin. O'z dasturlari panic qo'zg'ashini istamaydigan funksiyaning murojaat qiluvchilari bunday holatlarda funksiyani chaqirmasliklariga ishonch hosil qilishlari kerak.
* **Errors**: Agar funksiya `Result` ni qaytarsa, yuzaga kelishi mumkin bo'lgan xatolar turlarini tavsiflash va bu xatolar qaytarilishiga qanday sharoitlar sabab bo'lishi mumkinligi murojaat qiluvchilar uchun foydali bo'lishi mumkin, shuning uchun ular turli xil xatolarni turli yo'llar bilan hal qilish uchun kod yozishlari mumkin.
* **Safety**: Agar funksiya murojaat qilish uchun `unsafe`  bo'lsa (biz 19-bobda xavfsizlikni muhokama qilamiz), funksiya nima uchun xavfli ekanligini tushuntiruvchi bo'lim bo'lishi kerak va funksiya murojaat qiluvchilar qo'llab-quvvatlashini kutayotgan o'zgarmaslarni qamrab oladi.

Ko'pgina hujjatlar sharhlari ushbu bo'limlarning barchasiga muhtoj emas, ammo bu sizning kodingiz foydalanuvchilari bilishni qiziqtiradigan jihatlarni eslatish uchun yaxshi nazorat ro'yxati.

#### Texnik hujjatlarga sharhlar test sifatida

Hujjatlarga sharhlaringizga(documentation comment) misol kod bloklarini qo'shish kutubxonangizdan(library) qanday foydalanishni ko'rsatishga yordam beradi va bu qo'shimcha bonusga ega bo'ladi: `cargo test` ishga tushirish hujjatlaringizdagi kod misollarini test sifatida ishga tushiradi! Hech narsa misollar bilan hujjatlashtirishdan yaxshiroq emas. Lekin hech narsa ishlamaydigan misollardan ko'ra yomonroq emas, chunki hujjatlar yozilgandan beri kod o'zgargan. Agar biz 14-1 roʻyxatdagi `bir_qoshish` funksiyasi uchun hujjatlar bilan `cargo test` oʻtkazsak, test natijalarida quyidagi boʻlimni koʻramiz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - bir_qoshish (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

Endi funksiyani yoki misolni, misoldagi `assert_eq!` panic qo'zg'atadigan tarzda o'zgartirsak va yana `cargo test` ishga tushirsak, hujjat testlari misol va kod bir-biri bilan sinxronlanmaganligini aniqlaymiz.!

#### O'z ichiga olgan elementlarni sharhlash

Hujjat sharhining uslubi `//!` hujjatni sharhlardan keyingi elementlarga emas, balki sharhlarni o'z ichiga olgan elementga qo'shadi. Biz odatda bu doc izohlaridan cratening ildiz(root) faylida (odatda *src/lib.rs*) yoki modul ichida crateni yoki butun modulni hujjatlash uchun foydalanamiz.

Masalan, `bir_qoshish` funksiyasini o'z ichiga olgan `my_crate` cratesi maqsadini tavsiflovchi hujjatlarni qo'shish uchun biz *src/lib.rs* faylining boshiga `//!` bilan boshlanadigan hujjat sharhlarini qo`shamiz, 14-2 ro'yxatda ko'rsatilganidek:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 14-2: Umuman olganda, `my_crate` cratesi uchun hujjatlar</span>

E'tibor bering, `//!` bilan boshlanadigan oxirgi qatordan keyin hech qanday kod yo'q. Fikrlarni `///` o'rniga `//!` bilan boshlaganimiz sababli, biz ushbu sharhdan keyingi elementni emas, balki ushbu sharhni o'z ichiga olgan elementni hujjatlashtirmoqdamiz. Bunday holda, bu element crate ildizi(root) bo'lgan *src/lib.rs* faylidir. Ushbu sharhlar butun crateni tasvirlaydi.

`cargo doc --open`ni ishga tushirganimizda, bu izohlar 14-2-rasmda ko‘rsatilganidek, `my_crate` hujjatlarining birinchi sahifasida cratedagi public itemlar ro‘yxati ustida ko'rsatiladi:

<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trlpuz2.png" class="center" />

<span class="caption">14-2-rasm: `my_crate` uchun taqdim etilgan hujjatlar, jumladan, crateni bir butun sifatida tavsiflovchi sharh</span>

Elementlar ichidagi hujjat sharhlari, ayniqsa, cratelar va modullarni tavsiflash uchun foydalidir. Foydalanuvchilarga cratening tashkil etilishini tushunishlariga yordam berish uchun konteynerning umumiy maqsadini tushuntirish uchun ulardan foydalaning.

### `pub use` bilan qulay Public APIni eksport qilish

Public API structi crateni nashr qilishda muhim ahamiyatga ega. Sizning cratengizdan foydalanadigan odamlar structureni sizdan ko'ra kamroq bilishadi va agar sizning cratengiz katta modul ierarxiyasiga ega bo'lsa, ular foydalanmoqchi bo'lgan qismlarni topishda qiyinchiliklarga duch kelishlari mumkin.

7-bobda biz `pub` kalit so‘zi yordamida itemlarni qanday qilib hammaga ochiq(public) qilish va `use` kalit so‘zi bilan obyektlarni qamrovga(scope) kiritishni ko‘rib chiqdik.Biroq, crateni ishlab chiqishda sizga mantiqiy bo'lgan structure foydalanuvchilaringiz uchun unchalik qulay bo'lmasligi mumkin. Siz structlaringizni bir nechta darajalarni o'z ichiga olgan ierarxiyada tartibga solishni xohlashingiz mumkin, ammo keyin siz ierarxiyada chuqur aniqlagan turdan foydalanmoqchi bo'lgan odamlar ushbu tur mavjudligini aniqlashda muammolarga duch kelishlari mumkin.
Ular, shuningdek, `use` `my_crate::FoydaliTur;` o'rniga `use` ``my_crate::biror_modul::boshqa_modul::FoydaliTur;`` ni kiritishlari kerakligidan bezovtalanishi mumkin.

Yaxshi xabar shundaki, agar sturcture boshqa kutubxonadan(library) foydalanishi uchun *qulay bo'lmasa*, ichki organizationgizni o'zgartirishingiz shart emas: Buning o'rniga, `pub use` dan foydalanib, private structuredan farq qiladigan public structure yaratish uchun itemlarni qayta eksport qilishingiz mumkin. Qayta eksport qilish public ob'ektni bir joyda oladi va uni boshqa joyda hammaga ochiq(public) qiladi, go'yo u boshqa joyda aniqlangandek.

Masalan, badiiy tushunchalarni modellashtirish uchun `rassom` nomli kutubxona(library) yaratdik, deylik.
Ushbu kutubxona ichida ikkita modul mavjud: 14-3 roʻyxatda koʻrsatilganidek, `AsosiyRang` va `IkkilamchiRang` nomli ikkita raqamni oʻz ichiga olgan `turlar` moduli va `aralashtirish` nomli funksiyani oʻz ichiga olgan `yordamchi` moduli:
<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

<span class="caption">Roʻyxat 14-3: `turlar` va `yordamchi` modullariga ajratilgan `rassom` kutubxonasi</span>

14-3-rasmda `cargo doc` tomonidan yaratilgan ushbu crate uchun hujjatlarning bosh sahifasi qanday ko'rinishi ko'rsatilgan:

<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />

<span class="caption">14-3-rasm: `turlar` va `yordamchi` modullari ro‘yxati keltirilgan `rassom` hujjatlarining bosh sahifasi</span>

E'tibor bering, `AsosiyRang` va `IkkilamchiRang` turlari birinchi sahifada ko'rsatilmagan va `aralashtirish` funksiyasi ham mavjud emas. Ularni ko'rish uchun `turlar` va `yordamchi` ni bosishimiz kerak.

Ushbu kutubxonaga bog'liq bo'lgan boshqa cratega `rassom` dan elementlarni qamrab oladigan, hozirda aniqlangan modul stryucturedan ko'rsatadigan `use` statementlari kerak bo'ladi. 14-4 roʻyxatda `rassom` cratesidagi `AsosiyRang` va `aralashtirish` elementlaridan foydalanadigan crate misoli koʻrsatilgan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

<span class="caption">Roʻyxat 14-4: `rassom` crate itemlaridan foydalanilgan, ichki stuctureni eksport qilingan crate</span>

`rassom` cratesidan foydalanadigan 14-4-Ro'yxatdagi kod muallifi `AsosiyRang` `turlar` modulida, `aralashtirish` esa `yordamchi` modulida ekanligini aniqlashi kerak edi. `rassom` cratening modul stucturesi undan foydalanadiganlarga qaraganda `rassom` crate ustida ishlayotgan developerlarga ko'proq mos keladi. The internal
structure doesn’t contain any useful information for someone trying to
understand how to use the `art` crate, but rather causes confusion because
developers who use it have to figure out where to look, and must specify the
module names in the `use` statements.
Ichki stuctureda `rassom` cratesidan qanday foydalanishni tushunishga urinayotganlar uchun foydali ma'lumotlar mavjud emas, aksincha, chalkashliklarga sabab bo'ladi, chunki undan foydalanadigan developerlar qayerga qarash kerakligini aniqlashlari kerak va `use` statementlarida modul nomlarini ko'rsatishi kerak.

Ichki organizationni public API’dan olib tashlash uchun biz 14-5 ro‘yxatda ko‘rsatilganidek, top leveldagi elementlarni qayta eksport qilish uchun `pub use` statementlarini qo‘shish uchun 14-3 ro‘yxatdagi `rassom` crate kodini o‘zgartirishimiz mumkin:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 14-5: Elementlarni qayta eksport qilish uchun `pub use` statementlarini qo'shish</span>

The API documentation that `cargo doc` generates for this crate will now list
and link re-exports on the front page, as shown in Figure 14-4, making the
`PrimaryColor` and `SecondaryColor` types and the `mix` function easier to find.

<img alt="Rendered documentation for the `art` crate with the re-exports on the front page" src="img/trpl14-04.png" class="center" />

<span class="caption">Figure 14-4: The front page of the documentation for `art`
that lists the re-exports</span>

The `art` crate users can still see and use the internal structure from Listing
14-3 as demonstrated in Listing 14-4, or they can use the more convenient
structure in Listing 14-5, as shown in Listing 14-6:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

<span class="caption">Listing 14-6: A program using the re-exported items from
the `art` crate</span>

In cases where there are many nested modules, re-exporting the types at the top
level with `pub use` can make a significant difference in the experience of
people who use the crate. Another common use of `pub use` is to re-export
definitions of a dependency in the current crate to make that crate's
definitions part of your crate’s public API.

Creating a useful public API structure is more of an art than a science, and
you can iterate to find the API that works best for your users. Choosing `pub
use` gives you flexibility in how you structure your crate internally and
decouples that internal structure from what you present to your users. Look at
some of the code of crates you’ve installed to see if their internal structure
differs from their public API.

### Setting Up a Crates.io Account

Before you can publish any crates, you need to create an account on
[crates.io](https://crates.io/)<!-- ignore --> and get an API token. To do so,
visit the home page at [crates.io](https://crates.io/)<!-- ignore --> and log
in via a GitHub account. (The GitHub account is currently a requirement, but
the site might support other ways of creating an account in the future.) Once
you’re logged in, visit your account settings at
[https://crates.io/me/](https://crates.io/me/)<!-- ignore --> and retrieve your
API key. Then run the `cargo login` command with your API key, like this:

```console
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

This command will inform Cargo of your API token and store it locally in
*~/.cargo/credentials*. Note that this token is a *secret*: do not share it
with anyone else. If you do share it with anyone for any reason, you should
revoke it and generate a new token on [crates.io](https://crates.io/)<!-- ignore
-->.

### Adding Metadata to a New Crate

Let’s say you have a crate you want to publish. Before publishing, you’ll need
to add some metadata in the `[package]` section of the crate’s *Cargo.toml*
file.

Your crate will need a unique name. While you’re working on a crate locally,
you can name a crate whatever you’d like. However, crate names on
[crates.io](https://crates.io/)<!-- ignore --> are allocated on a first-come,
first-served basis. Once a crate name is taken, no one else can publish a crate
with that name. Before attempting to publish a crate, search for the name you
want to use. If the name has been used, you will need to find another name and
edit the `name` field in the *Cargo.toml* file under the `[package]` section to
use the new name for publishing, like so:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

Even if you’ve chosen a unique name, when you run `cargo publish` to publish
the crate at this point, you’ll get a warning and then an error:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
```

This errors because you’re missing some crucial information: a description and
license are required so people will know what your crate does and under what
terms they can use it. In *Cargo.toml*, add a description that's just a
sentence or two, because it will appear with your crate in search results. For
the `license` field, you need to give a *license identifier value*. The [Linux
Foundation’s Software Package Data Exchange (SPDX)][spdx] lists the identifiers
you can use for this value. For example, to specify that you’ve licensed your
crate using the MIT License, add the `MIT` identifier:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

If you want to use a license that doesn’t appear in the SPDX, you need to place
the text of that license in a file, include the file in your project, and then
use `license-file` to specify the name of that file instead of using the
`license` key.

Guidance on which license is appropriate for your project is beyond the scope
of this book. Many people in the Rust community license their projects in the
same way as Rust by using a dual license of `MIT OR Apache-2.0`. This practice
demonstrates that you can also specify multiple license identifiers separated
by `OR` to have multiple licenses for your project.

With a unique name, the version, your description, and a license added, the
*Cargo.toml* file for a project that is ready to publish might look like this:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[Cargo’s documentation](https://doc.rust-lang.org/cargo/) describes other
metadata you can specify to ensure others can discover and use your crate more
easily.

### Publishing to Crates.io

Now that you’ve created an account, saved your API token, chosen a name for
your crate, and specified the required metadata, you’re ready to publish!
Publishing a crate uploads a specific version to
[crates.io](https://crates.io/)<!-- ignore --> for others to use.

Be careful, because a publish is *permanent*. The version can never be
overwritten, and the code cannot be deleted. One major goal of
[crates.io](https://crates.io/)<!-- ignore --> is to act as a permanent archive
of code so that builds of all projects that depend on crates from
[crates.io](https://crates.io/)<!-- ignore --> will continue to work. Allowing
version deletions would make fulfilling that goal impossible. However, there is
no limit to the number of crate versions you can publish.

Run the `cargo publish` command again. It should succeed now:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

Congratulations! You’ve now shared your code with the Rust community, and
anyone can easily add your crate as a dependency of their project.

### Publishing a New Version of an Existing Crate

When you’ve made changes to your crate and are ready to release a new version,
you change the `version` value specified in your *Cargo.toml* file and
republish. Use the [Semantic Versioning rules][semver] to decide what an
appropriate next version number is based on the kinds of changes you’ve made.
Then run `cargo publish` to upload the new version.

<!-- Old link, do not remove -->
<a id="removing-versions-from-cratesio-with-cargo-yank"></a>

### Deprecating Versions from Crates.io with `cargo yank`

Although you can’t remove previous versions of a crate, you can prevent any
future projects from adding them as a new dependency. This is useful when a
crate version is broken for one reason or another. In such situations, Cargo
supports *yanking* a crate version.

Yanking a version prevents new projects from depending on that version while
allowing all existing projects that depend on it to continue. Essentially, a
yank means that all projects with a *Cargo.lock* will not break, and any future
*Cargo.lock* files generated will not use the yanked version.

To yank a version of a crate, in the directory of the crate that you’ve
previously published, run `cargo yank` and specify which version you want to
yank. For example, if we've published a crate named `guessing_game` version
1.0.1 and we want to yank it, in the project directory for `guessing_game` we'd
run:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

By adding `--undo` to the command, you can also undo a yank and allow projects
to start depending on a version again:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

A yank *does not* delete any code. It cannot, for example, delete accidentally
uploaded secrets. If that happens, you must reset those secrets immediately.

[spdx]: http://spdx.org/licenses/
[semver]: http://semver.org/
