## Structlarni aniqlash va yaratish

Structlar [“Tuple turi“][tuples]<!-- ignore --> bo'limida muhokama qilingan tuplelarga o'xshaydi, chunki ikkalasi ham bir-biriga bog'liq bo'lgan bir nechta qiymatlarga ega.
Tuplelar singari, structning qismlari ham har xil turdagi bo'lishi mumkin. Tuplelardan farqli o'laroq, structda siz har bir ma'lumot qismini nomlaysiz, shunda qiymatlar nimani anglatishini tushunasiz. Ushbu nomlarni qo'shish structlar tuplelardan ko'ra moslashuvchanroq ekanligini anglatadi: misol qiymatlarini belgilash yoki ularga kirish uchun ma'lumotlar tartibiga ishonishingiz shart emas.

Structni aniqlash uchun biz `struct` kalit so`zini kiritamiz va butun tuzilishga nom beramiz. Struct nomi birgalikda guruhlangan ma'lumotlar bo'laklarining ahamiyatini tavsiflashi kerak. Keyin, jingalak qavslar ichida biz *maydonlar* deb ataydigan ma'lumotlar qismlarining nomlari va turlarini aniqlaymiz. Masalan, 5-1 ro'yxati foydalanuvchi hisobi haqidagi ma'lumotlarni saqlaydigan structni ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-1: `Foydalanuvchi` structi ta'rifi</span>

Struct aniqlangandan so'ng, tegishli ma'lumotlar turiga ega bo'lgan har bir maydonga ma'lum bir qiymat berish orqali uni yaratish mumkin. Biz structning nomini ko'rsatish orqali misol yaratamiz va keyin *kalit: qiymat*(key: value) juftlarini o'z ichiga olgan jingalak qavslarni qo'shamiz, bu erda kalitlar maydonlarning nomlari va qiymatlar biz o'sha maydonlarda saqlamoqchi bo'lgan ma'lumotlardir.Biz maydonlarni structda e'lon qilgan tartibda ko'rsatishimiz shart emas. Boshqacha qilib aytganda, structning ta'rifi tur uchun umumiy shablonga o'xshaydi va misollar tur qiymatlarini yaratish uchun ushbu shablonni ma'lum ma'lumotlar bilan to'ldiradi. Misol uchun, biz 5-2 ro'yxatda ko'rsatilganidek, ma'lum bir foydalanuvchini e'lon qilishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-2: `Foydalanuvchi` nusxasini yaratish
structi</span>

Structdan ma'lum bir qiymat olish uchun biz nuqta belgilaridan foydalanamiz. Masalan, ushbu foydalanuvchining elektron pochta manziliga kirish uchun biz `foydalanuvchi1.email` dan foydalanamiz. Agar misol o'zgaruvchan bo'lsa, biz nuqta belgisi yordamida qiymatni o'zgartirishimiz va ma'lum bir maydonga belgilashimiz mumkin. 5-3 ro'yxatda o'zgaruvchan `Foydalanuvchi` misolining `email` maydonidagi qiymatni qanday o'zgartirish mumkinligi ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-3: `Foydalanuvchi` misolining `email` maydonidagi qiymatni o'zgartirish</span>

E'tibor bering, butun misol o'zgaruvchan bo'lishi kerak; Rust bizga faqat ma'lum maydonlarni o'zgaruvchan deb belgilashga ruxsat bermaydi. Har qanday ifodada bo'lgani kabi, biz ushbu yangi misolni bilvosita qaytarish uchun funksiya tanasidagi oxirgi ifoda sifatida structning yangi nusxasini qurishimiz mumkin.

5-4 roʻyxatda berilgan email va foydalanuvchi nomi bilan `Foydalanuvchi` misolini qaytaruvchi `foydalanuvchi_yaratish` funksiyasi koʻrsatilgan. `faollik` maydoni `true` qiymatini, `kirish_hisobi` esa `1` qiymatini oladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Roʻyxat 5-4: `foydalanuvchi_yaratish` funksiyasi email va foydalanuvchi nomini oladi va `Foydalanuvchi` misolini qaytaradi</span>

Funksiya parametrlarini struct maydonlari bilan bir xil nom bilan nomlash mantiqan to‘g‘ri keladi, lekin `email` va `foydalanuvchi` maydon nomlari va o‘zgaruvchilarini takrorlash biroz zerikarli. Agar structda ko'proq maydonlar bo'lsa, har bir nomni takrorlash yanada zerikarli bo'ladi. Yaxshiyamki, qulay Shorthand bor!
<!-- Old heading. Do not remove or links may break. -->
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Field Init Shorthandan foydalanish

Parametr nomlari va struct maydonlarining nomlari 5-4 ro'yxatda aynan bir xil bo'lgani uchun, `foydalanuvchi_yaratish`ni qayta yozish uchun *init shorthand* sintaksisidan foydalanishimiz mumkin, shuning uchun u xuddi shunday ishlaydi, lekin `foydalanuvchi` va `email`ni takrorlamaydi, 5-5 ro'yxatda ko'rsatilganidek.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Roʻyxat 5-5: `foydalanuvchi` va `email` parametrlari struct maydonlari bilan bir xil nomga ega boʻlgani uchun init shorthanddan foydalanadigan `foydalanuvchi_yaratish` funksiyasi</span>

Bu yerda biz `Foydalanuvchi` structining yangi nusxasini yaratmoqdamiz, unda `email` deb nomlangan maydon mavjud. Biz `email` maydonining qiymatini `foydalanuvchi_yaratish` funksiyasining `email` parametridagi qiymatga o‘rnatmoqchimiz. `email` maydoni va `email` parametri bir xil nomga ega bo'lgani uchun biz `email: email` emas, balki `email` yozishimiz kerak.

### Structni update sintaksisidan foydalanib, boshqa tuzilma nusxasidan tuzilma namunasini yaratish

Ko'pincha boshqa namunadagi qiymatlarning ko'pini o'z ichiga olgan, lekin ularning ba'zilarini o'zgartiradigan structning yangi nusxasini yaratish foydali bo'ladi. Buni *struct update sintaksisi* yordamida amalga oshirishingiz mumkin.

Birinchidan, 5-6 ro'yxatda biz yangilanish sintaksisisiz muntazam ravishda `foydalanuvchi2` da yangi `Foydalanuvchi` misolini qanday yaratishni ko'rsatamiz. Biz `email` uchun yangi qiymat o‘rnatdik, lekin aks holda 5-2 ro‘yxatda yaratgan `foydalanuvchi1` qiymatidan foydalanamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Roʻyxat 5-6: foydalanuvchi1 qiymatlaridan biri yordamida yangi `Foydalanuvchi` namunasini yaratish</span>

Strukturani yangilash sintaksisidan foydalanib, biz 5-7 ro'yxatda ko'rsatilganidek, kamroq kod bilan bir xil effektga erishishimiz mumkin. `..` sintaksisi qolgan maydonlar aniq o'rnatilganligini va belgilangan namunadagi qiymatlarga ega bo'lishi kerakligini ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-7: `Foydalanuvchi` misoli uchun yangi `email` qiymatini o'rnatish, lekin `foydalanuvchi1` dagi qolgan qiymatlardan foydalanish uchun structni yangilash sintaksisidan foydalanish</span>

The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` but has the same values for the `username`,
`active`, and `sign_in_count` fields from `user1`. The `..user1` must come last
to specify that any remaining fields should get their values from the
corresponding fields in `user1`, but we can choose to specify values for as
many fields as we want in any order, regardless of the order of the fields in
the struct’s definition.

Note that the struct update syntax uses `=` like an assignment; this is because
it moves the data, just as we saw in the [“Variables and Data Interacting with
Move”][move]<!-- ignore --> section. In this example, we can no longer use
`user1` as a whole after creating `user2` because the `String` in the
`username` field of `user1` was moved into `user2`. If we had given `user2` new
`String` values for both `email` and `username`, and thus only used the
`active` and `sign_in_count` values from `user1`, then `user1` would still be
valid after creating `user2`. Both `active` and `sign_in_count` are types that
implement the `Copy` trait, so the behavior we discussed in the [“Stack-Only
Data: Copy”][copy]<!-- ignore --> section would apply.

### Using Tuple Structs Without Named Fields to Create Different Types

Rust also supports structs that look similar to tuples, called *tuple structs*.
Tuple structs have the added meaning the struct name provides but don’t have
names associated with their fields; rather, they just have the types of the
fields. Tuple structs are useful when you want to give the whole tuple a name
and make the tuple a different type from other tuples, and when naming each
field as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here we define and use two
tuple structs named `Color` and `Point`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

Note that the `black` and `origin` values are different types because they’re
instances of different tuple structs. Each struct you define is its own type,
even though the fields within the struct might have the same types. For
example, a function that takes a parameter of type `Color` cannot take a
`Point` as an argument, even though both types are made up of three `i32`
values. Otherwise, tuple struct instances are similar to tuples in that you can
destructure them into their individual pieces, and you can use a `.` followed
by the index to access an individual value.

### Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called
*unit-like structs* because they behave similarly to `()`, the unit type that
we mentioned in [“The Tuple Type”][tuples]<!-- ignore --> section. Unit-like
structs can be useful when you need to implement a trait on some type but don’t
have any data that you want to store in the type itself. We’ll discuss traits
in Chapter 10. Here’s an example of declaring and instantiating a unit struct
named `AlwaysEqual`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

To define `AlwaysEqual`, we use the `struct` keyword, the name we want, and
then a semicolon. No need for curly brackets or parentheses! Then we can get an
instance of `AlwaysEqual` in the `subject` variable in a similar way: using the
name we defined, without any curly brackets or parentheses. Imagine that later
we’ll implement behavior for this type such that every instance of
`AlwaysEqual` is always equal to every instance of any other type, perhaps to
have a known result for testing purposes. We wouldn’t need any data to
implement that behavior! You’ll see in Chapter 10 how to define traits and
implement them on any type, including unit-like structs.

> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want each instance of this struct to own all of its data and for
> that data to be valid for as long as the entire struct is valid.
>
> It’s also possible for structs to store references to data owned by something
> else, but to do so requires the use of *lifetimes*, a Rust feature that we’ll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let’s say you try to store a reference
> in a struct without specifying lifetimes, like the following; this won’t work:
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> The compiler will complain that it needs lifetime specifiers:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> In Chapter 10, we’ll discuss how to fix these errors so you can store
> references in structs, but for now, we’ll fix errors like these using owned
> types like `String` instead of references like `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
