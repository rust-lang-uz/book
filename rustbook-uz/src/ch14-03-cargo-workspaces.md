## Cargo Workspaselar

12-bobda biz bianry crate va kutubxona cratesini o'z ichiga olgan paketni yaratdik. Loyihangiz rivojlanib borgan sari, kutubxona(library) cratesi kattalashib borishini va paketingizni bir nechta kutubxona cratelariga bo'lishni xohlayotganingizni ko'rishingiz mumkin. Cargo tandemda ishlab chiqilgan bir nechta tegishli paketlarni boshqarishga yordam beradigan *workspaces* deb nomlangan xususiyatni taklif etadi.

### Workspace yaratish

*workspace* - bu bir xil *Cargo.lock* va output(chiqish) jildiga ega bo'lgan paketlar to'plami. Keling, workspcedan foydalangan holda loyiha yarataylik - biz workspacening tuzilishiga e'tibor qaratishimiz uchun arzimas koddan foydalanamiz. Workspaceni tuzishning bir necha yo'li mavjud, shuning uchun biz faqat bitta umumiy usulni ko'rsatamiz. Binary(ikkilik) va ikkita kutubxonani o'z ichiga olgan workspacega ega bo'lamiz. Asosiy funksionallikni ta'minlaydigan binary ikkita kutubxonaga bog'liq bo'ladi. Bitta kutubxona `bitta_qoshish` funksiyasini, ikkinchi kutubxona esa `ikkita_qoshish` funksiyasini taqdim etadi.
Ushbu uchta crate bir xil workspacening bir qismi bo'ladi. Biz workspaceni uchun yangi jild yaratishdan boshlaymiz:

```console
$ mkdir qoshish
$ cd qoshish
```

Keyinchalik, *qoshuvchi* jildida biz butun workspaceni sozlaydigan *Cargo.toml* faylini yaratamiz. Bu faylda `[package]` boʻlimi boʻlmaydi.
Buning o'rniga, u binary(ikkilik) crate yordamida paketga yo'lni ko'rsatib, workspacega a'zolar qo'shish imkonini beruvchi `[workspace]` bo'limidan boshlanadi; bu holda, bu yo'l *qoshuvchi*:

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/qoshish/Cargo.toml}}
```

Keyin, *qoshuvchi* jilida `cargo new` ni ishga tushirish orqali `qoshuvchi` binary cratesini yaratamiz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new qoshuvchi
     Created binary (application) `qoshuvchi` package
```

Ushbu nuqtada biz `cargo build` ni ishga tushirish orqali worksoaceni qurishimiz mumkin. Sizning *qoshuvchi* jildingiz quyidagicha ko'rinishi kerak:

```text
├── Cargo.lock
├── Cargo.toml
├── qoshuvchi
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Workspaceda kompilyatsiya qilingan artefaktlar joylashtiriladigan top leveldagi bitta *target* jildi mavjud; `qoshuvchi` paketi o'zining *target* jildiga ega emas. Agar biz *qoshuvchi* jildi ichidan `cargo build`ni ishga tushirsak ham, kompilyatsiya qilingan artefaktlar hali ham *qoshish/qoshuvchi/target* emas, balki *qoshish/target* da tugaydi. Cargo workspacedagi *target* jildini shunday tuzadi, chunki workspacedagi cratelar bir-biriga bog'liq bo'lishi kerak. Agar har bir crate o'zining *target* jildiga ega bo'lsa, har bir crate artefaktlarni o'zining *target* jildiga joylashtirish uchun workspacedagi boshqa cratelarning har birini qayta kompilyatsiya qilishi kerak edi. Bitta *target* jildini baham ko'rish(share) orqali cratelar keraksiz rebuildingdan qochishi mumkin.

### Workspaceda ikkinchi paketni yaratish

Keyinchalik, workspaceda boshqa a'zolar(member) paketini yaratamiz va uni `bitta_qoshish` deb nomlaymiz. `members` ro'yxatida *bitta_qoshish* yo'lini belgilash uchun top leveldagi *Cargo.toml* ni o'zgartiring:

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/qoshish/Cargo.toml}}
```

Keyin `bitta_qoshish` nomli yangi kutubxonalibrary cratesini yarating:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf bitta_qoshish
cargo new bitta_qoshish --lib
copy output below
-->

```console
$ cargo new bitta_qoshish --lib
     Created library `bitta_qoshish` package
```

Sizning *qoshish* jildingizda endi quyidagi jild va fayllar bo‘lishi kerak:

```text
├── Cargo.lock
├── Cargo.toml
├── bitta_qoshish
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── qoshuvchi
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

*bitta_qoshish/src/lib.rs* fayliga `bitta_qoshish` funksiyasini qo'shamiz:

<span class="filename">Fayl nomi: bitta_qoshish/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/qoshish/bitta_qoshish/src/lib.rs}}
```

Endi biz kutubxonamizga ega bo'lgan `bitta_qoshish` paketiga bog'liq bo'lgan `qoshuvchi` paketiga ega bo'lishimiz mumkin. Birinchidan, biz *qoshuvchi/Cargo.toml* ga `bitta_qoshish` yo'liga bog'liqlikni qo'shishimiz kerak.

<span class="filename">Fayl nomi: qoshuvchi/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/qoshish/qoshuvchi/Cargo.toml:6:7}}
```

Cargo workspacedagi cratelar bir-biriga bog'liq bo'ladi deb o'ylamaydi, shuning uchun biz qaramlik munosabatlari(relationship) haqida aniq bo'lishimiz kerak.

Keyin, keling, `qoshuvchi` cratedagi `bitta_qoshish` funksiyasidan (`bitta_qoshish` cratesidan) foydalanamiz. *qoshuvchi/src/main.rs* faylini oching va yangi `bitta_qoshish` kutubxona cratesini qamrab olish uchun tepaga `use` qatorini qo'shing. Keyin 14-7 roʻyxatdagi kabi `bitta_qoshish` funksiyasini chaqirish uchun `main` funksiyani oʻzgartiring.

<span class="filename">Fayl nomi: qoshuvchi/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/qoshish/qoshuvchi/src/main.rs}}
```

<span class="caption">Roʻyxat 14-7: `bitta_qoshish` kutubxonasi cratesidan `qoshish` cratesidan foydalanish</span>

Keling, yuqori darajadagi *qoshish* jildida `cargo build`ni ishga tushirish orqali workspaceni build qilaylik!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling bitta_qoshish v0.1.0 (file:///projects/qoshish/bitta_qoshish)
   Compiling qoshuvchi v0.1.0 (file:///projects/qoshish/qoshuvchi)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

Binary crateni *qoshish* jildidan ishga tushirish uchun biz `-p` argumenti va `cargo run` bilan paket nomidan foydalanib workspaceda qaysi paketni ishga tushirishni belgilashimiz mumkin:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p qoshuvchi
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/qoshuvchi`
Hello, world! 10 plus one is 11!
```

Bu kodni *qoshuvchi/src/main.rs* da ishga tushiradi, bu `bitta_qoshish` cratesiga bog'liq.

#### Workspacedagi tashqi(external) paketga bog'liqlik

E'tibor bering, workspaceda har bir crate jildida *Cargo.lock* emas, balki top leveldagi faqat bitta *Cargo.lock* fayli mavjud. Bu barcha cratelar barcha depencilarning(bog'liqlik) bir xil versiyasidan foydalanishini ta'minlaydi. Agar biz *qoshuvchi/Cargo.toml* va *bitta_qoshish/Cargo.toml* fayllariga `rand` paketini qo'shsak, Cargo ikkalasini ham `rand` ning bitta versiyasida hal qiladi va buni bitta *Cargo.lock*da qayd etadi. Workspacedagi barcha cratelarni bir xil depensilardan foydalanishga aylantirish, cratelarning har doim bir-biriga mos kelishini anglatadi. Keling, *bitta_qoshish/Cargo.toml* faylidagi `[dependencies]` bo'limiga `rand` cratesini qo'shamiz, shunda biz `bitta_qoshish` cratesida `rand` cratesidan foydalanishimiz mumkin:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Fayl nomi: bitta_qoshish/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/qoshish/bitta_qoshish/Cargo.toml:6:7}}
```

Endi biz *bitta_qoshish/src/lib.rs* fayliga `use rand;` ni qo'shishimiz mumkin va *qoshish* jildida `cargo build`-ni ishga tushirish orqali butun workspaceni build qilish `rand` cratesini olib keladi va kompilyatsiya qiladi. Biz bitta ogohlantirish olamiz, chunki biz qamrab olgan `rand` ni nazarda tutmayapmiz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling bitta_qoshish v0.1.0 (file:///projects/qoshish/bitta_qoshish)
warning: unused import: `rand`
 --> bitta_qoshish/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `bitta_qoshish` (lib) generated 1 warning
   Compiling qoshuvchi v0.1.0 (file:///projects/qoshish/qoshuvchi)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

Top leveldagi *Cargo.lock* endi `bitta_qoshish` `rand` ga bog'liqligi(dependency) haqida ma'lumotni o'z ichiga oladi. Biroq, workspacening biror joyida `rand` ishlatilsa ham, ularning *Cargo.toml* fayllariga `rand` qo'shmagunimizcha, biz uni workspacedagi boshqa cratelarda ishlata olmaymiz. Masalan, agar biz `qoshuvchi` paketi uchun *qoshuvchi/src/main.rs* fayliga `use rand;` qo'shsak, xatoga duch kelamiz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling qoshuvchi v0.1.0 (file:///projects/qoshish/qoshuvchi)
error[E0432]: unresolved import `rand`
 --> qoshuvchi/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Buni tuzatish uchun `qoshuvchi` paketi uchun *Cargo.toml* faylini tahrirlang va `rand` ham unga dependency(bog'liqligini) ekanligini ko'rsating. `qoshuvchi` paketini yaratish *Cargo.lock* dagi `qoshuvchi` uchun depensiar ro'yxatiga `rand` qo'shadi, lekin `rand` ning qo'shimcha nusxalari yuklab olinmaydi. Cargo `rand` paketidan foydalangan holda workspacedagi har bir cratedagi har bir crate bir xil versiyadan foydalanishini taʼminladi, bu bizga joyni tejaydi va workspacedagi cratelar bir-biriga mos kelishini taʼminlaydi.

#### Workspacega test qo'shish

Yana bir yaxshilanish uchun, keling, `bitta_qoshish` cratesidagi `bitta_qoshish::bitta_qoshish` funksiyasi testini qo'shamiz:

<span class="filename">Fayl nomi: bitta_qoshish/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/qoshish/bitta_qoshish/src/lib.rs}}
```

Top-leveldagi *qoshish* jildida `cargo test`-ni ishga tushiring. Shunga o'xshash tuzilgan workspaceda `cargo test` ni o'tkazish workspacedagi barcha cratelar uchun testlarni o'tkazadi:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling bitta_qoshish v0.1.0 (file:///projects/qoshish/bitta_qoshish)
   Compiling adder v0.1.0 (file:///projects/qoshish/qoshuvchi)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/bitta_qoshish-f0253159197f7841)

running 1 test
test tests::ishlamoqda ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/qoshuvchi-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests bitta_qoshish

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Chiqishning(output) birinchi qismida `bitta_qoshish` cratesidagi `ishlamoqda` testi muvaffaqiyatli o'tganligi ko'rsatilgan. Keyingi bo'limda `qoshuvchi` cratesida nol testlar topilganligi ko'rsatilgan, so'ngra oxirgi bo'lim `bitta_qoshish` cratesida nol hujjat testlari topilganligini ko'rsatadi.

Bundan tashqari, biz top leveldagi jilddan `-p` flagidan foydalanib va biz test qilib ko'rmoqchi bo'lgan crate nomini ko'rsatib, workspacedagi ma'lum bir crate uchun testlarni o'tkazishimiz mumkin:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p bitta_qoshish
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p bitta_qoshish
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/bitta_qoshish-b3235fea9a156f74)

running 1 test
test tests::ishlamoqda ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests bitta_qoshish

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Bu chiqishda `cargo test` koʻrsatilgan, faqat `bitta_qoshish` cratesi uchun stestlar oʻtkazilgan va `qoshuvchi` cratesi testlari oʻtkazilmagan.

Agar siz workspacedagi cratelarni [crates.io](https://crates.io/)-ga nashr(publish) qilsangiz, workspacedagi har bir crate alohida nashr etilishi kerak bo'ladi.
`cargo test` singari, biz `p` flagidan foydalanib va nashr qilmoqchi bo'lgan crate nomini ko'rsatib, workspacemizda ma'lum bir crateni nashr qilishimiz mumkin.

Qo'shimcha mashq qilish uchun ushbu workspacega `bitta_qoshish` cratesga o'xshash `ikkita_qoshish` cratesini qo'shing!

Loyihangiz o'sib ulg'aygan sayin, workspacedan foydalanishni o'ylab ko'ring: bitta katta kod blokidan ko'ra kichikroq, individual komponentlarni tushunish osonroq. Bundan tashqari, cratelarni workspaceda saqlash, agar ular bir vaqtning o'zida tez-tez almashtirilsa, cratelar orasidagi muvofiqlashtirishni osonlashtirishi mumkin.