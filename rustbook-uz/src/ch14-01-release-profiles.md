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

`opt-level` sozlama Rust kodingizga qo'llaniladigan optimallashtirishlar sonini nazorat qiladi, 0 dan 3 gacha. Ko'proq optimallashtirishni qo'llash kompilyatsiya vaqtini uzaytiradi, shuning uchun agar siz tez-tez ishlab chiqayotgan bo'lsangiz va kodingizni kompilyatsiya qilsangiz, natijada olingan kod sekinroq ishlayotgan bo'lsa ham, tezroq kompilyatsiya qilishni kamroq optimallashtirishni xohlaysiz. Shunday qilib, `dev` uchun default `opt-level` `0` dir. Kodni releasega chiqarishga tayyor bo'lganingizda, kompilyatsiya qilish uchun ko'proq vaqt sarflaganingiz ma'qul. Siz release rejimida faqat bir marta kompilyatsiya qilasiz, lekin kompilyatsiya qilingan dasturni ko'p marta ishga tushirasiz, shuning uchun release rejimi tradelari tezroq ishlaydigan kod uchun kompilyatsiya vaqtini uzaytiradi. Shuning uchun `release` profili uchun default `opt-level` `3` dir.

Siz *Cargo.toml* da boshqa qiymat qoʻshish orqali default sozlamani bekor qilishingiz mumkin. Misol uchun, agar biz development profilida optimallashtirish darajasi 1 dan foydalanmoqchi bo'lsak, loyihamizning *Cargo.toml* fayliga ushbu ikki qatorni qo'shishimiz mumkin:

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Bu kod default `0` sozlamasini bekor qiladi. Now when we run `cargo build`,
Cargo `dev` profili uchun default sozlamalardan hamda `opt-level`ga moslashtirishimizdan foydalanadi. Biz `opt-level`ni `1` ga o‘rnatganimiz sababli, Cargo defaultdan ko‘ra ko‘proq optimallashtirishni qo‘llaydi, lekin release builddagi kabi emas.

Har bir profil uchun konfiguratsiya opsiyalari va standart sozlamalarning to'liq ro'yxati uchun Cargo [texnik hujjatlariga](https://doc.rust-lang.org/cargo/reference/profiles.html) qarang.