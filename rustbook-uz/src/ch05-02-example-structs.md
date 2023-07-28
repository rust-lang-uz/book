## Structlar yordamida namunaviy dastur

To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. We’ll start by using single variables, and then refactor the program until we’re using structs instead.

Let’s make a new binary project with Cargo called *rectangles* that will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle. Listing 5-8 shows a short program with one way of doing exactly that in our project’s *src/main.rs*.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```


<span class="caption">Ro'yxat 5-8: Alohida kenglik va balandlik o'zgaruvchilari bilan belgilangan to'rtburchaklar maydonini hisoblash</span>

Endi ushbu dasturni `cargo run` yordamida ishga tushiring:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Ushbu kod har bir o'lcham bilan `area` funksiyasini chaqirish orqali to'rtburchakning maydonini aniqlashga muvaffaq bo'ladi, ammo biz ushbu kodni aniq va o'qilishi uchun ko'proq narsani qilishimiz mumkin.

Ushbu kod bilan bog'liq muammo `area` signaturesida aniq ko'rinadi:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

The `area` function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related. It would be more readable and more manageable to group width and height together. We’ve already discussed one way we might do that in [“The Tuple Type”][the-tuple-type]<!-- ignore --> section of Chapter 3: by using tuples.

### Tuplelar yordamida Refaktoring

5-9 ro'yxatda tuplelardan foydalanadigan dasturimizning boshqa versiyasi ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```


<span class="caption">Ro'yxat 5-9: To'rtburchakning kengligi va balandligini tuple yordamida aniqlash</span>

In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.

Mixing up the width and height wouldn’t matter for the area calculation, but if we want to draw the rectangle on the screen, it would matter! We would have to keep in mind that `width` is the tuple index `0` and `height` is the tuple index `1`. This would be even harder for someone else to figure out and keep in mind if they were to use our code. Because we haven’t conveyed the meaning of our data in our code, it’s now easier to introduce errors.

### Struktuctlar bilan Refaktoring: ko'proq ma'no qo'shish

We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts, as shown in Listing 5-10.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">Ro'yxat 5-10: `Kvadrat` strukturasini aniqlash</span>

Here we’ve defined a struct and named it `Rectangle`. Inside the curly brackets, we defined the fields as `width` and `height`, both of which have type `u32`. Then, in `main`, we created a particular instance of `Rectangle` that has a width of `30` and a height of `50`.

Our `area` function is now defined with one parameter, which we’ve named `rectangle`, whose type is an immutable borrow of a struct `Rectangle` instance. As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, `main` retains its ownership and can continue using `rect1`, which is the reason we use the `&` in the function signature and where we call the function.

The `area` function accesses the `width` and `height` fields of the `Rectangle` instance (note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs). Our function signature for `area` now says exactly what we mean: calculate the area of `Rectangle`, using its `width` and `height` fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of `0` and `1`. This is a win for clarity.

### Olingan Traitlar bilan foydali funksionallikni qo'shish

It’d be useful to be able to print an instance of `Rectangle` while we’re debugging our program and see the values for all its fields. Listing 5-11 tries using the [`println!` macro][println]<!-- ignore --> as we have used in previous chapters. This won’t work, however.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```


<span class="caption">Ro'yxat 5-11: `Kvadrat`ni chop etishga urinish misoli</span>

Ushbu kodni kompilyatsiya qilishda, biz ushbu asosiy xabar bilan xatoga duch kelamiz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption. The primitive types we’ve seen so far implement `Display` by default because there’s only one way you’d want to show a `1` or any other primitive type to a user. But with structs, the way `println!` should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of `Display` to use with `println!` and the `{}` placeholder.

Agar xatolarni o'qishda davom etsak, biz ushbu foydali eslatmani topamiz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Let’s try it! The `println!` macro call will now look like `println!("rect1 is
{:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. The `Debug` trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

Compile the code with this change. Drat! We still get an error:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Ammo yana, kompilyator bizga foydali eslatma beradi:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust *does* include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition, as shown in Listing 5-12.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```


<span class="caption">Ro'yxat 5-12: `Debug` traitini olish uchun atribut qo‘shish va debug formatlash yordamida `Kvadrat` misolini chop etish</span>

Endi dasturni ishga tushirganimizda, biz hech qanday xatolikka yo'l qo'ymaymiz va biz quyidagi natijani ko'ramiz:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string. In this example, using the `{:#?}` style will output the following:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Another way to print out a value using the `Debug` format is to use the [`dbg!` macro][dbg]<!-- ignore -->, which takes ownership of an expression (as opposed to `println!`, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

> Note: Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!`, which prints to the standard output console stream (`stdout`). We’ll talk more about `stderr` and `stdout` in the [“Writing Error Messages to Standard Error Instead of Standard Output” section in Chapter 12][err]<!-- ignore -->.

Mana bizni `kenglik` maydoniga tayinlanadigan qiymat, shuningdek `kvadrat1`dagi butun structning qiymati qiziqtiradigan misol:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

We can put `dbg!` around the expression `30 * scale` and, because `dbg!` returns ownership of the expression’s value, the `width` field will get the same value as if we didn’t have the `dbg!` call there. We don’t want `dbg!` to take ownership of `rect1`, so we use a reference to `rect1` in the next call. Here’s what the output of this example looks like:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

We can see the first bit of output came from *src/main.rs* line 10 where we’re debugging the expression `30 * scale`, and its resultant value is `60` (the `Debug` formatting implemented for integers is to print only their value). The `dbg!` call on line 14 of *src/main.rs* outputs the value of `&rect1`, which is the `Rectangle` struct. This output uses the pretty `Debug` formatting of the `Rectangle` type. The `dbg!` macro can be really helpful when you’re trying to figure out what your code is doing!

In addition to the `Debug` trait, Rust has provided a number of traits for us to use with the `derive` attribute that can add useful behavior to our custom types. Those traits and their behaviors are listed in [Appendix C][app-c]<!--
ignore -->. We’ll cover how to implement these traits with custom behavior as well as how to create your own traits in Chapter 10. There are also many attributes other than 

`derive`; for more information, see [the “Attributes” section of the Rust Reference][attributes].

Our `area` function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our `Rectangle` struct because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the `area` function into an `area` *method* defined on our `Rectangle` type.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
