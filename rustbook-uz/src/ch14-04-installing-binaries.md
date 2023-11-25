<!-- Old link, do not remove -->
<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## Binary(ikkilik) fayllarni `cargo install` bilan o'rnatish

`cargo install` buyrug'i binary cratelarni mahalliy(local) sifatida o'rnatish va ishlatish imkonini beradi. Bu tizim paketlarini almashtirish uchun mo'ljallanmagan; Bu Rust dasturchilari uchun [crates.io](https://crates.io/)<!-- ignore --> saytida boshqalar baham ko'rgan toollarni o'rnatishning qulay usuli bo'lishi kerak. E'tibor bering, siz faqat binary targetlarga ega bo'lgan paketlarni o'rnatishingiz mumkin. *binary target* bu o'z-o'zidan ishga tushirilmaydigan, lekin mos bo'lgan kutubxona(library) targetdidan farqli o'laroq, src/main.rs fayli yoki cratening bir qismi sifatida bajariladigan boshqa faylni o'z ichiga olgan bajariladigan dastur. Boshqa dasturlarga kiritish uchun. Odatda, cratelar *README* faylida crate va kutubxona ekanligi, binary targetli yoki har ikkalasi haqida ma'lumotga ega.

`cargo install` bilan o'rnatilgan barcha binary fayllar o'rnatish ildizining(root) *bin* jildida saqlanadi. Rust-ni *rustup.rs* yordamida o'rnatgan bo'lsangiz va hech qanday maxsus konfiguratsiyaga ega bo'lmasangiz, bu jild *$HOME/.cargo/bin* bo'ladi. `cargo install` bilan oʻrnatgan dasturlarni ishga tushirish uchun jildingiz `$PATH`da ekanligiga ishonch hosil qiling.

Masalan, 12-bobda biz fayllarni qidirish uchun `ripgrep` deb nomlangan `grep` toolining Rust ilovasi mavjudligini eslatib o'tdik. Keling `ripgrep` ni o'rnatish uchun biz quyidagilarni ishga tushirishimiz mumkin:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v13.0.0
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v13.0.0
--snip--
   Compiling ripgrep v13.0.0
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

Chiqishning ikkinchidan oxirgi qatori o'rnatilgan binary faylning joylashuvi va nomini ko'rsatadi, bu `ripgrep` holatida `rg`dir. O'rnatish jildi `$PATH` da bo'lsa, avval aytib o'tilganidek, siz `rg --help` ni ishga tushirishingiz va fayllarni qidirish uchun tezroq, rustda kuchidan foydalanib yozilgan tooldan foydalanishni boshlashingiz mumkin!
