## Ilova A: Kalit so'zlar

The following list contains keywords that are reserved for current or future use by the Rust language. As such, they cannot be used as identifiers (except as raw identifiers as we’ll discuss in the “[Raw Identifiers][raw-identifiers]<!-- ignore -->” section). Identifiers are names of functions, variables, parameters, struct fields, modules, crates, constants, macros, static values, attributes, types, traits, or lifetimes.

### Hozirda foydalanilayotgan kalit so'zlar

Quyida hozirda foydalanilayotgan kalit so‘zlar ro‘yxati keltirilgan, ularning funksiyalari tasvirlangan.

* `as` - perform primitive casting, disambiguate the specific trait containing an item, or rename items in `use` statements
* `async` -  return a `Future` instead of blocking the current thread
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
* `for` - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
* `if` - branch based on the result of a conditional expression
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
* `self` - metod mavzusi yoki joriy modul
* `static` - global o'zgaruvchi yoki butun dasturning bajarilishi uchun lifetime
* `struct` - structurani aniqlash
* `super` - joriy modulning parent moduli
* `trait` - traitni aniqlash
* `true` - Boolean true(to'g'ri) so'z
* `type` - turdagi alias yoki associated turni aniqlash
* `union` - define a [union][union]<!-- ignore -->; is only a keyword when used in a union declaration
* `unsafe` - xavfli kod, funksiyalar, traitlar yoki implementationlarni bildiradi
* `use` - bring symbols into scope
* `where` - denote clauses that constrain a type
* `while` - expression natijasi asosida shartli ravishda sikl

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

*Raw identifiers* are the syntax that lets you use keywords where they wouldn’t normally be allowed. You use a raw identifier by prefixing a keyword with `r#`.

For example, `match` is a keyword. If you try to compile the following function that uses `match` as its name:

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

The error shows that you can’t use the keyword `match` as the function identifier. To use `match` as a function name, you need to use the raw identifier syntax, like this:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

This code will compile without any errors. Note the `r#` prefix on the function name in its definition as well as where the function is called in `main`.

Raw identifiers allow you to use any word you choose as an identifier, even if that word happens to be a reserved keyword. This gives us more freedom to choose identifier names, as well as lets us integrate with programs written in a language where these words aren’t keywords. In addition, raw identifiers allow you to use libraries written in a different Rust edition than your crate uses. For example, `try` isn’t a keyword in the 2015 edition but is in the 2018 edition. If you depend on a library that’s written using the 2015 edition and has a `try` function, you’ll need to use the raw identifier syntax, `r#try` in this case, to call that function from your 2018 edition code. See [Appendix E][appendix-e]<!-- ignore --> for more information on editions.

[raw-identifiers]: #raw-identifiers

[union]: ../reference/items/unions.html

[appendix-e]: appendix-05-editions.html
