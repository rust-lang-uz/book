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
rm -rf add_one
cargo new add_one --lib
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

#### Depending on an External Package in a Workspace

Notice that the workspace has only one *Cargo.lock* file at the top level,
rather than having a *Cargo.lock* in each crate’s directory. This ensures that
all crates are using the same version of all dependencies. If we add the `rand`
package to the *adder/Cargo.toml* and *add_one/Cargo.toml* files, Cargo will
resolve both of those to one version of `rand` and record that in the one
*Cargo.lock*. Making all crates in the workspace use the same dependencies
means the crates will always be compatible with each other. Let’s add the
`rand` crate to the `[dependencies]` section in the *add_one/Cargo.toml* file
so we can use the `rand` crate in the `add_one` crate:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Fayl nomi: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

We can now add `use rand;` to the *add_one/src/lib.rs* file, and building the
whole workspace by running `cargo build` in the *add* directory will bring in
and compile the `rand` crate. We will get one warning because we aren’t
referring to the `rand` we brought into scope:

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
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

The top-level *Cargo.lock* now contains information about the dependency of
`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* files as well. For example, if we add `use rand;`
to the *adder/src/main.rs* file for the `adder` package, we’ll get an error:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

To fix this, edit the *Cargo.toml* file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo has ensured that every
crate in every package in the workspace using the `rand` package will be using
the same version, saving us space and ensuring that the crates in the workspace
will be compatible with each other.

#### Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:

<span class="filename">Fayl nomi: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Now run `cargo test` in the top-level *add* directory. Running `cargo test` in
a workspace structured like this one will run the tests for all the crates in
the workspace:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The first section of the output shows that the `it_works` test in the `add_one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add_one` crate.

We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This output shows `cargo test` only ran the tests for the `add_one` crate and
didn’t run the `adder` crate tests.

If you publish the crates in the workspace to [crates.io](https://crates.io/),
each crate in the workspace will need to be published separately. Like `cargo
test`, we can publish a particular crate in our workspace by using the `-p`
flag and specifying the name of the crate we want to publish.

For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!

As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between crates easier if they
are often changed at the same time.
