## Funksiyalar

Funksiyalar Rust kodida keng tarqalgan. Siz allaqachon tildagi eng muhim funksiyalardan birini ko'rgansiz: ko'plab dasturlarning kirish nuqtasi bo'lgan `main` funksiya. Siz yangi funksiyalarni e'lon qilish imkonini beruvchi `fn` kalit so'zini ham ko'rdingiz.

Rust kodi funksiya va oʻzgaruvchilar nomlari uchun anʼanaviy uslub sifatida *snake case* dan foydalanadi, unda barcha harflar kichik va alohida soʻzlarning tagiga chiziladi.
Mana, misol funksiya ta'rifini o'z ichiga olgan dastur:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust-da funksiyani `fn` so'ng funksiya nomi va qavslar to'plamini kiritish orqali aniqlaymiz. Jingalak qavslar kompilyatorga funktsiya tanasi qayerda boshlanishi va tugashini bildiradi.

Biz belgilagan har qanday funktsiyani uning nomidan keyin qavslar to'plamini kiritish orqali chaqirishimiz mumkin. Dasturda `boshqa_funksiya` ni aniqlanganligi sababli uni `main` funksiya ichidan chaqirish mumkin. E'tibor bering, biz `boshqa_funksiya` ni manba kodidagi `main` funksiyadan keyin belgilaganmiz; uni avval ham belgilashimiz mumkin edi. Rust sizning funksiyalaringizni qayerda belgilashingizning ahamiyati yo'q, faqat ular so'rov yuboruvchi tomonidan ko'rinadigan doirada aniqlangan.

Keling, funksiyalarni ko'proq o'rganish uchun *funktsiyalar* nomli yangi binary loyihani boshlaylik. `boshqa_funksiya` misolini *src/main.rs* ga joylashtiring va uni ishga tushiring.Quyidagi chiqishni ko'rishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Qatorlar `main` funksiyada paydo bo'ladigan tartibda bajariladi.
Avvaliga "Hello, world!" xabar chop etiladi, keyin `boshqa_funksiya` chaqiriladi va uning xabari chop etiladi.

### Parametrlar

Biz funksiyalarni *parametrlari* bo'lishi uchun belgilashimiz mumkin, ular funksiya imzosining bir qismi bo'lgan maxsus o'zgaruvchilardir. Agar funksiya parametrlarga ega bo'lsa, siz unga ushbu parametrlar uchun aniq qiymatlarni berishingiz mumkin. Texnik jihatdan aniq qiymatlar *argumentlar* deb ataladi, ammo tasodifiy suhbatda odamlar funksiya taʼrifidagi oʻzgaruvchilar yoki funksiyani chaqirganingizda qabul qilingan aniq qiymatlar uchun *parametr* va *argument* soʻzlarini bir-birining oʻrniga ishlatishga moyildirlar.

`boshqa_funksiya` ning ushbu versiyasida biz parametr qo'shamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Ushbu dasturni ishga tushirishga harakat qiling; quyidagi chiqishni olishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`boshqa_funksiya` deklaratsiyasi `x` nomli bitta parametrga ega. `x` turi `i32` sifatida belgilangan. Biz `5`ni `boshqa_funksiya`ga o‘tkazganimizda, `println!` makros `5` ni `x`ni o‘z ichiga olgan jingalak qavslar juftligi format satrida joylashgan joyga qo‘yadi.

Funksiya signaturelarda siz har bir parametr turini e'lon qilishingiz kerak. Bu Rust dizaynidagi ataylab qabul qilingan qaror: funksiya taʼriflarida turdagi izohlarni talab qilish kompilyatorga qaysi turni nazarda tutayotganingizni tushunish uchun ularni kodning boshqa joylarida ishlatishingizga deyarli hech qachon ehtiyoj sezmasligini anglatadi. Kompilyator, shuningdek, funksiya qanday turlarni kutayotganini bilsa, yanada foydali xato xabarlarini berishi mumkin.

Bir nechta parametrlarni belgilashda parametr deklaratsiyasini vergul bilan ajrating, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Ushbu misol ikkita parametrli `belgilangan_vaqt` nomli funksiyani yaratadi. Birinchi parametr `value` deb nomlangan va `i32` dir. Ikkinchisi `unit_label` deb nomlanadi va `char` turidir. Keyin funksiya `value` va ``unit_label` ni o‘z ichiga olgan matnni chop etadi.

Let’s try running this code. Replace the program currently in your *functions*
project’s *src/main.rs* file with the preceding example and run it using `cargo
run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Because we called the function with `5` as the value for `value` and `'h'` as
the value for `unit_label`, the program output contains those values.

### Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an
expression. So far, the functions we’ve covered haven’t included an ending
expression, but you have seen an expression as part of a statement. Because
Rust is an expression-based language, this is an important distinction to
understand. Other languages don’t have the same distinctions, so let’s look at
what statements and expressions are and how their differences affect the bodies
of functions.

* **Statements** are instructions that perform some action and do not return
  a value.
* **Expressions** evaluate to a resultant value. Let’s look at some examples.

We’ve actually already used statements and expressions. Creating a variable and
assigning a value to it with the `let` keyword is a statement. In Listing 3-1,
`let y = 6;` is a statement.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Listing 3-1: A `main` function declaration containing one statement</span>

Function definitions are also statements; the entire preceding example is a
statement in itself.

Statements do not return values. Therefore, you can’t assign a `let` statement
to another variable, as the following code tries to do; you’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

When you run this program, the error you’ll get looks like this:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

The `let y = 6` statement does not return a value, so there isn’t anything for
`x` to bind to. This is different from what happens in other languages, such as
C and Ruby, where the assignment returns the value of the assignment. In those
languages, you can write `x = y = 6` and have both `x` and `y` have the value
`6`; that is not the case in Rust.

Expressions evaluate to a value and make up most of the rest of the code that
you’ll write in Rust. Consider a math operation, such as `5 + 6`, which is an
expression that evaluates to the value `11`. Expressions can be part of
statements: in Listing 3-1, the `6` in the statement `let y = 6;` is an
expression that evaluates to the value `6`. Calling a function is an
expression. Calling a macro is an expression. A new scope block created with
curly brackets is an expression, for example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

This expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to `4`. That value gets bound to `y`
as part of the `let` statement. Note that the `x + 1` line doesn’t have a
semicolon at the end, which is unlike most of the lines you’ve seen so far.
Expressions do not include ending semicolons. If you add a semicolon to the end
of an expression, you turn it into a statement, and it will then not return a
value. Keep this in mind as you explore function return values and expressions
next.

### Functions with Return Values

Functions can return values to the code that calls them. We don’t name return
values, but we must declare their type after an arrow (`->`). In Rust, the
return value of the function is synonymous with the value of the final
expression in the block of the body of a function. You can return early from a
function by using the `return` keyword and specifying a value, but most
functions return the last expression implicitly. Here’s an example of a
function that returns a value:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

There are no function calls, macros, or even `let` statements in the `five`
function—just the number `5` by itself. That’s a perfectly valid function in
Rust. Note that the function’s return type is specified too, as `-> i32`. Try
running this code; the output should look like this:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

The `5` in `five` is the function’s return value, which is why the return type
is `i32`. Let’s examine this in more detail. There are two important bits:
first, the line `let x = five();` shows that we’re using the return value of a
function to initialize a variable. Because the function `five` returns a `5`,
that line is the same as the following:

```rust
let x = 5;
```

Second, the `five` function has no parameters and defines the type of the
return value, but the body of the function is a lonely `5` with no semicolon
because it’s an expression whose value we want to return.

Let’s look at another example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Running this code will print `The value of x is: 6`. But if we place a
semicolon at the end of the line containing `x + 1`, changing it from an
expression to a statement, we’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Compiling this code produces an error, as follows:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

The main error message, `mismatched types`, reveals the core issue with this
code. The definition of the function `plus_one` says that it will return an
`i32`, but statements don’t evaluate to a value, which is expressed by `()`,
the unit type. Therefore, nothing is returned, which contradicts the function
definition and results in an error. In this output, Rust provides a message to
possibly help rectify this issue: it suggests removing the semicolon, which
would fix the error.
