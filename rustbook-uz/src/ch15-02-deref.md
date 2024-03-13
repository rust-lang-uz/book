## Smart Pointerlarni `Deref` Xususiyati Bilan Oddiy Havolalar Kabi Ishlatish

`Deref` xususiyatini qo'llash, *dereference operatori*ning `*` (ko'paytirish
yoki glob operatori bilan adashtirmaslik kerak) xulq-atvorini sozlashga imkon
beradi. Smart pointerlarni `Deref` xususiyati bilan oddiy havolalar kabi
qo'llasangiz, siz havolalar ustida ishlaydigan kod yozishingiz, shuningdek,
ushbu kodni smart pointerlar bilan ishlatishingiz mumkin bo'ladi.

Keling, avvalo, dereference operatori oddiy havolalar bilan qanday ishlashini
ko'rib chiqaylik. Keyin biz `Box<T>` kabi maxsus turni e'lon qilishga harakat
qilamiz va dereference operatori nega bizning yangi e'lon qilgan turimizdagi
havola kabi ishlamayotganini ko'ramiz. Biz `Deref` xususiyatini amalga oshirish
smart pointerlarning havolalarga o'xshash tarzda ishlashiga qanday imkon
berishini ko'rib chiqamiz. Keyin biz Rustning *deref coercion* xususiyatini va
u bizga havolalar yoki smart pointerlar bilan ishlashga qanday imkon berishini
ko'rib chiqamiz.

> Eslatma: biz qurmoqchi bo'lgan `MyBox<T>` turi va haqiqiy `Box<T>` o‘rtasida
> bitta katta farq bor: bizning versiyamiz o‘z ma’lumotlarini heapda saqlamaydi.
> Biz ushbu misolda e'tiborimizni `Deref`ga qaratmoqdamiz, shuning uchun 
> ma'lumotlarning qayerda saqlanishi pointerga o'xshash xatti-harakatlardan 
> kamroq ahamiyatga ega.

<!-- Old link, do not remove -->
<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>

### Pointerni Qiymatga bog'lash 

Muntazam havola pointerning bir turi bo'lib, pointerni boshqa joyda saqlangan
qiymatga o'q kabi tasavvur qilishning bir usuli. 15-6 ro'yxatda biz `i32`
qiymatiga havola yaratamiz va keyin qiymatga havolani bog'lash uchun dereference
operatoridan foydalanamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

<span class="caption">Ro'yxat 15-6: `i32` qiymatiga havola orqali murojat qilish
uchun dereference operatoridan foydalanish</span>

`x` o'zgaruvchisi `i32` turidagi `5` qiymatiga ega. Biz `y` ni `x` ning
havolasiga tenglashtiramiz. Biz `x` `5` ga teng ekanligini solishtirishimiz
mumkin. Ammo, agar biz `y` dagi qiymatni solishtirmoqchi bo'lsak, kompilyator
haqiqiy qiymatni solishtirishi uchun `*y` dan foydalanib, u havola qilgan
qiymatga (ya'ni, *dereference*) murojaat qilishimiz kerak. `y` da dereference
qo'llaganimizdan so'ng, `y` ishora qilib turgan butun son qiymatiga kirish
imkoniga ega bo'lamiz, bu `5` bilan solishtirishimizga imkon beradi.

Agar `assert_eq!(5, y);` yozishga harakat qilganimizda, ushbu kompilyatsiya
xatoligini olgan bo'lar edik:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Raqam va raqamga havola bilan solishtirishga yo'l qo'yilmaydi, chunki ular har
xil turlar. Biz havola qilingan qiymatga murojaat qilish uchun dereference
operatoridan foydalanishimiz kerak.

### `Box<T>` ni Havola Kabi Ishlatish

15-6 ro'yxatdagi kodni havola o'rniga `Box<T>` ishlatgan holda qayta yozishimiz
mumkin; 15-7 ro'yxatdagi funksiyalarida `Box<T>` da ishlatiladigan dereference
operatori 15-6 ro'yxatidagi havolada ishlatilgan dereference operatori
bilan bir xil tarzda ishlatiladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

<span class="caption">Ro'yxat 15-7: `Box<i32>` da dereference operatorini
ishlatish</span>

15-7 va 15-6 ro'yxat o'rtasidagi asosiy farq shundaki, biz bu yerda `y` ni `x`
qiymatiga havola emas, balki `x` ning ko'chirilgan qiymatiga ishora qiluvchi
`Box<T>` ning misoli qilib belgiladik. Oxirgi solishtiruvda biz dereference
operatoridan `Box<T>` ko'rsatgichiga murojat qilish uchun xuddi `y` havola
bo'lganida qilganimizdek bajarishimiz mumkin. Keyin biz `Box<T>` ning o'ziga xos
xususiyatlarini o'rganamiz, bu bizga o'z turimizni e'lon qilish orqali
dereference operatoridan foydalanishga imkon beradi. 

### O'zimizning Aqlli Ko'rsatgichimizni E'lon Qilish

Keling, aqlli ko'rsatgichlar havolalardan qanday farq qilishini bilish uchun
standart kutubxona tomonidan taqdim etilgan `Box<T>` turiga o'xshash aqlli
ko'rsatgichni yarataylik. Keyin biz dereference operatoridan foydalanish
qobiliyatini qanday qo'shishni ko'rib chiqamiz.

`Box<T>` turi oxir-oqibat bitta elementga ega bo'lgan tuple struct sifatida
aniqlanadi, 15-8 ro'yxatda xuddi shu tarzda `MyBox<T>` turini belgilaydi.
Shuningdek, `Box<T>` da belgilangan `new` funksiyaga mos keladigan `new`
funksiyani aniqlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-8: `MyBox<T>` turini aniqlash</span>

Biz `MyBox` nomli structni aniqlaymiz va `T` generic parametrini e'lon qilamiz,
chunki biz turimiz istalgan turdagi qiymatlarni ushlab turishini xohlaymiz.
`MyBox` turi `T` turidagi bitta elementga ega bo'lgan tuple structdir.
`MyBox::new` funksiyasi `T` turidagi bitta parametrni oladi va berilgan qiymatni
ushlab turuvchi `MyBox` misolini qaytaradi.

15-7 ro'yxatdagi `main` funksiyasini 15-8 ro'yxatiga qo'shib, `Box<T>` o'rniga
biz belgilagan `MyBox<T>`turidan foydalanish uchun o'zgartirishga harakat
qilaylik. 15-9 ro'yxatdagi kod kompilyatsiya qilinmaydi, chunki Rust `MyBox` ni
qanday qilib dereference qilishni bilmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-9: `MyBox<T>` dan xuddi havolalar va `Box<T>`
dan foydalanganimiz kabi foydalanishga urinish</span>

Natijada kompilyatsiya xatosi:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

Bizning `MyBox<T>` turini dereference qilib bo'lmaydi, chunki biz bu qobiliyatni
o'z turimizda qo'llamaganmiz. `*` operatori yordamida dereference qilishni
yoqish uchun biz `Deref` traitini qo`llaymiz.

### `Deref` Traitni Amalga Oshirish Orqali Turga Havola Kabi Munosabatda Bo'lish

10-bobning [“Turga xos Traitni amalga
oshirish”][turga-xos-traitni-amalga-oshirish]<!-- ignore --> boʻlimida muhokama
qilinganidek, traitni amalga oshirish uchun biz traitning talab qilinadigan
usullarini amalga oshirishimiz kerak. Standart kutubxona tomonidan taqdim
etilgan `Deref` xususiyati bizdan `self` qarz oladigan va ichki ma'lumotlarga
havolani qaytaradigan `deref` nomli metodni qo'llashimizni talab qiladi. 15-10
ro'yxat `MyBox` ta'rifiga qo'shish uchun `Deref` amalga oshirilishini o'z ichiga
oladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-10: `MyBox<T>` uchun `Deref` ni amalga
oshirish</span>

The `type Target = T;` syntax defines an associated type for the `Deref`
trait to use. Associated types are a slightly different way of declaring a
generic parameter, but you don’t need to worry about them for now; we’ll cover
them in more detail in Chapter 19.

We fill in the body of the `deref` method with `&self.0` so `deref` returns a
reference to the value we want to access with the `*` operator; recall from the
[“Using Tuple Structs without Named Fields to Create Different
Types”][tuple-structs]<!-- ignore --> section of Chapter 5 that `.0` accesses
the first value in a tuple struct. The `main` function in Ro'yxat 15-9 that
calls `*` on the `MyBox<T>` value now compiles, and the assertions pass!

Without the `Deref` trait, the compiler can only dereference `&` references.
The `deref` method gives the compiler the ability to take a value of any type
that implements `Deref` and call the `deref` method to get a `&` reference that
it knows how to dereference.

When we entered `*y` in Ro'yxat 15-9, behind the scenes Rust actually ran this
code:

```rust,ignore
*(y.deref())
```

Rust substitutes the `*` operator with a call to the `deref` method and then a
plain dereference so we don’t have to think about whether or not we need to
call the `deref` method. This Rust feature lets us write code that functions
identically whether we have a regular reference or a type that implements
`Deref`.

The reason the `deref` method returns a reference to a value, and that the
plain dereference outside the parentheses in `*(y.deref())` is still necessary,
is to do with the ownership system. If the `deref` method returned the value
directly instead of a reference to the value, the value would be moved out of
`self`. We don’t want to take ownership of the inner value inside `MyBox<T>` in
this case or in most cases where we use the dereference operator.

Note that the `*` operator is replaced with a call to the `deref` method and
then a call to the `*` operator just once, each time we use a `*` in our code.
Because the substitution of the `*` operator does not recurse infinitely, we
end up with data of type `i32`, which matches the `5` in `assert_eq!` in
Ro'yxat 15-9.

### Implicit Deref Coercions with Functions and Methods

*Deref coercion* converts a reference to a type that implements the `Deref`
trait into a reference to another type. For example, deref coercion can convert
`&String` to `&str` because `String` implements the `Deref` trait such that it
returns `&str`. Deref coercion is a convenience Rust performs on arguments to
functions and methods, and works only on types that implement the `Deref`
trait. It happens automatically when we pass a reference to a particular type’s
value as an argument to a function or method that doesn’t match the parameter
type in the function or method definition. A sequence of calls to the `deref`
method converts the type we provided into the type the parameter needs.

Deref coercion was added to Rust so that programmers writing function and
method calls don’t need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.

To see deref coercion in action, let’s use the `MyBox<T>` type we defined in
Ro'yxat 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Ro'yxat 15-11 shows the definition of a function that has a string slice
parameter:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-11: A `hello` function that has the parameter
`name` of type `&str`</span>

We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");` for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Ro'yxat 15-12:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-12: Calling `hello` with a reference to a
`MyBox<String>` value, which works because of deref coercion</span>

Here we’re calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Ro'yxat 15-10, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, and this is in the API documentation
for `Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which
matches the `hello` function’s definition.

If Rust didn’t implement deref coercion, we would have to write the code in
Ro'yxat 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-13: The code we would have to write if Rust
didn’t have deref coercion</span>

The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. This code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.

When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as necessary to get a reference to
match the parameter’s type. The number of times that `Deref::deref` needs to be
inserted is resolved at compile time, so there is no runtime penalty for taking
advantage of deref coercion!

### How Deref Coercion Interacts with Mutability

Similar to how you use the `Deref` trait to override the `*` operator on
immutable references, you can use the `DerefMut` trait to override the `*`
operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three
cases:

* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same as each other except that the second
implements mutability. The first case states that if you have a `&T`, and `T`
implements `Deref` to some type `U`, you can get a `&U` transparently. The
second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is *not* possible: immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn’t compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that the
initial immutable reference is the only immutable reference to that data, but
the borrowing rules don’t guarantee that. Therefore, Rust can’t make the
assumption that converting an immutable reference to a mutable reference is
possible.

[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
