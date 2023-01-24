## Ilova A: Kalit so'zlar

Quyidagi ro'yxatda Rust tili tomonidan joriy yoki kelajakda foydalanish uchun ajratilgan kalit so'zlar mavjud.
Shunday qilib, ularni identifikator sifatida ishlatib bo'lmaydi (raw identifikatorlar bo'limida muhokama qilinadigan “[Raw identifikatorlardan][raw-identifiers]<!-- ignore -->” tashqari)
Identifikatorlar - bu funksiyalar, o'zgaruvchilar, parametrlar, struct field, modullar, cratelar, konstantalar, macroslar, statik qiymatlar, atributlar, turlar, traitlar yoki lifetimelarining nomlari.

[raw-identifiers]: #raw-identifiers

### Hozirda foydalanilayotgan kalit so'zlar

Quyida hozirda foydalanilayotgan kalit so‘zlar ro‘yxati keltirilgan, ularning funksiyalari tasvirlangan.

* `as` - primitiv castingni amalga oshiring, elementni o'z ichiga olgan o'ziga xos traitni ajrating yoki `use` statementaridagi elementlarning nomini o'zgartiring
* `async` -  joriy threadni bloklash o'rniga `Future` ni return qiling
* `await` - `Future` natijasi tayyor bo'lgunga qadar executionni to'xtatib turing
* `break` - zudlik bilan loopdan chiqing
* `const` - konstanta elementlarni yoki konstanta raw pointerlarni aniqlang
* `continue` - keyingi sikl iteratsiyasiga davom eting
* `crate` - modul yo'lida, crate rootga ishora qiladi
* `dyn` - trait objectga dinamik jo'natish
* `else` - `if` va `if let` uchun zaxira control flow konstruksiyalari
* `enum` - enumerationni aniqlash
* `extern` - tashqi funksiya yoki o‘zgaruvchini bog‘lash
* `false` - Boolean  false(noto'g'ri) so'z
* `fn` - funksiya yoki funktsiya pointer turini aniqlang
* `for` - iteratordagi elementlarni ko'rib chiqing, implement  trait yoki yuqori darajali  lifetimeni belgilang
* `if` - shartli expression natijasiga asoslangan branch
* `impl` - o'ziga inherent yoki trait funksionalligini implement
* `in` - `for` sikl sintaksisining bir qismi
* `let` - o'zgaruvchini bog'lash
* `loop` - shartsiz loop
* `match` - qiymatni patternlarga moslashtirish
* `mod` - modulni aniqlash
* `move` - make a closure take ownership of all its captures
* `mut` - reference, raw pointerlar yoki pattern bindingdagi o'zgaruvchanlikni bildiradi
* `pub` - struct fieldlarida, `impl` bloklarida yoki modullarda ommaviy ko'rinishni bildiradi
* `ref` - reference orqali bog'lash
* `return` - funksiyadan qaytish(return)
* `Self` - biz belgilayotgan yoki implement qilayotgan tur uchun turdagi alias
* `self` - method mavzusi yoki joriy modul
* `static` - global o'zgaruvchi yoki butun dasturning bajarilishi uchun lifetime
* `struct` - structurani aniqlash
* `super` - joriy modulning parent moduli
* `trait` - traitni aniqlash
* `true` - Boolean true(to'g'ri) so'z
* `type` - turdagi alias yoki associated turni aniqlash
* `union` - define a [union][union]<!-- ignore -->; is only a keyword when used
  in a union declaration
* `unsafe` - xavfli kod, funksiyalar, traitlar yoki implementationlarni bildiradi
* `use` - bring symbols into scope
* `where` - denote clauses that constrain a type
* `while` - expression natijasi asosida shartli ravishda sikl

[union]: ../reference/items/unions.html

### Kelajakda foydalanish uchun ajratilgan kalit so'zlar

Quyidagi kalit so'zlar hali hech qanday funksiyaga ega emas, lekin kelajakda foydalanish uchun Rust tomonidan zahiralangan.

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### Raw identifikatorlar

*Raw identifikatorlar* odatda ruxsat berilmaydigan kalit so'zlardan foydalanish imkonini beruvchi sintaksisdir. Kalit so‘z oldiga `r#` qo‘yish orqali raw identifikatordan foydalanasiz.

Masalan, `match` kalit so'zdir. Agar siz o'z nomi sifatida `match` dan foydalanadigan quyidagi funksiyani kompilyatsiya qilmoqchi bo'lsangiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

you’ll get this error:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Xato, funksiya identifikatori sifatida `match` kalit so‘zidan foydalana olmasligingizni ko‘rsatadi. Funksiya nomi sifatida `match` dan foydalanish uchun siz raw identifikator sintaksisidan foydalanishingiz kerak, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Ushbu kod hech qanday xatosiz kompilyatsiya qilinadi. Funksiya nomidagi `r#` prefiksi uning taʼrifida, shuningdek, funksiya `main`da chaqirilgan joyiga eʼtibor bering.

Raw identifiers allow you to use any word you choose as an identifier, even if
that word happens to be a reserved keyword. This gives us more freedom to
choose identifier names, as well as lets us integrate with programs written in
a language where these words aren’t keywords. In addition, raw identifiers
allow you to use libraries written in a different Rust edition than your crate
uses. For example, `try` isn’t a keyword in the 2015 edition but is in the 2018
edition. If you depend on a library that’s written using the 2015 edition and
has a `try` function, you’ll need to use the raw identifier syntax, `r#try` in
this case, to call that function from your 2018 edition code. See [Appendix
E][appendix-e]<!-- ignore --> for more information on editions.

[appendix-e]: appendix-05-editions.html
