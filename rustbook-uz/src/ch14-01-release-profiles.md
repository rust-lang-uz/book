## Reliz profillari bilan buildarni customizatsiya qilish

Rust-da *release profillari* turli xil konfiguratsiyalarga ega bo'lgan oldindan belgilangan va sozlanishi mumkin bo'lgan profillar bo'lib, ular dasturchiga kodni kompilyatsiya qilish uchun turli xil variantlarni ko'proq nazorat qilish imkonini beradi. Har bir profil boshqalardan mustaqil ravishda configuratsiya qilingan.

Cargo ikkita asosiy profilga ega: `cargo build`ni ishga tushirganingizda `dev` cargo profili va `cargo build --release`ni ishga tushirganingizda `release` cargo profilidan foydalanadi. `dev` profili ishlab chiqish(development) uchun yaxshi standart sozlamalar bilan belgilangan va `release` profili relizlar uchun yaxshi standart parametrlarga ega.

Ushbu profil nomlari sizning buildlaringiz natijalaridan tanish bo'lishi mumkin:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

`dev` va `release` - bu kompilyator tomonidan ishlatiladigan turli xil profillar.

Loyihaning *Cargo.toml* fayliga `[profile.*]` boʻlimlarini aniq qoʻshmagan boʻlsangiz, Cargo har bir profil uchun standart(default) sozlamalarga ega.
Moslashtirmoqchi(customizatsiya) boʻlgan har qanday profil uchun `[profile.*]` boʻlimlarini qoʻshish orqali siz default sozlamalarning har qanday quyi toʻplamini bekor qilasiz. Masalan, `dev` va `release` profillari uchun `opt-level` sozlamalari uchun default qiymatlar:

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` sozlama Rust kodingizga qo'llaniladigan optimallashtirishlar sonini nazorat qiladi, 0 dan 3 gacha. Ko'proq optimallashtirishni qo'llash kompilyatsiya vaqtini uzaytiradi, shuning uchun agar siz tez-tez ishlab chiqayotgan bo'lsangiz va kodingizni kompilyatsiya qilsangiz, natijada olingan kod sekinroq ishlayotgan bo'lsa ham, tezroq kompilyatsiya qilishni kamroq optimallashtirishni xohlaysiz. Shunday qilib, `dev` uchun default `opt-level` `0` dir. When you’re
ready to release your code, it’s best to spend more time compiling. You’ll only
compile in release mode once, but you’ll run the compiled program many times,
so release mode trades longer compile time for code that runs faster. That is
why the default `opt-level` for the `release` profile is `3`.

You can override a default setting by adding a different value for it in
*Cargo.toml*. For example, if we want to use optimization level 1 in the
development profile, we can add these two lines to our project’s *Cargo.toml*
file:

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

This code overrides the default setting of `0`. Now when we run `cargo build`,
Cargo will use the defaults for the `dev` profile plus our customization to
`opt-level`. Because we set `opt-level` to `1`, Cargo will apply more
optimizations than the default, but not as many as in a release build.

For the full list of configuration options and defaults for each profile, see
[Cargo’s documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).
