## UTF-8 kodlangan matnni String bilan saqlash

Biz 4-bobda stringlar haqida gapirgan edik, ammo hozir ularni batafsil ko'rib chiqamiz.
Yangi Rustaceanlar odatda uchta sababning kombinatsiyasiga ko'ra stringlarga yopishib qolishadi: Rustning mumkin bo'lgan xatolarni ochishga moyilligi, stringlar ko'plab dasturchilar tushunganidan ko'ra murakkabroq ma'lumotlar tuzilishi va UTF-8. Bu omillar shunday birlashadiki, agar siz boshqa dasturlash tillaridan kelgan bo'lsangiz, mavzu murakkab ko'rinishi mumkin.

To'plamlar kontekstida stringlarni muhokama qilish foydalidir, chunki stringlar baytlar to'plami sifatida amalga oshiriladi, shuningdek, bu baytlar matn sifatida talqin qilinganda foydali funksiyalarni ta'minlashning ba'zi usullari. Ushbu bo'limda biz `String` bo'yicha har bir to'plam turiga ega bo'lgan yaratish, yangilash va o'qish kabi operatsiyalar haqida gapiramiz. Shuningdek, biz `String` ning boshqa to'plamlardan qanday farq qilishini, ya'ni `String` ga indekslash odamlar va kompyuterlarning `String` ma'lumotlarini qanday talqin qilishlari o'rtasidagi farqlar tufayli qanday murakkablashishini muhokama qilamiz.

### String nima?

Biz birinchi navbatda *string* atamasi bilan nimani nazarda tutayotganimizni aniqlaymiz. Rust asosiy tilda faqat bitta string turiga ega, bu `str` qator slice boʻlib, odatda uning `&str` shaklida koʻrinadi. 4-bobda biz boshqa joyda saqlangan ba'zi UTF-8 kodlangan string ma'lumotlariga referencelar bo'lgan *string slicelar* haqida gaplashdik. Masalan, satr literallari dasturning binary tizimida saqlanadi va shuning uchun satr slicedir.

Rust standart kutubxonasi tomonidan taqdim etilgan `String` turi asosiy tilga o'rnatilmagan va kengaytiriladigan, o'zgaruvchan, ega bo'lgan, UTF-8 kodlangan string turidir. Rustaceanlar Rust tilidagi "stringlar" ga murojaat qilganda, ular bu turlardan birini emas, balki `String` yoki string slice `&str` turlarini nazarda tutishi mumkin. Garchi bu bo'lim asosan String haqida bo'lsa-da, ikkala tur ham Rust standart kutubxonasida ko'p qo'llaniladi, `String` va string slicelari UTF-8 da kodlangan.

### Yangi String yaratish

`Vec<T>` bilan mavjud bo'lgan bir xil amallarning ko'pchiligi `String` bilan ham mavjud, chunki `String` aslida qo'shimcha kafolatlar, cheklovlar va imkoniyatlarga ega baytlar vectori atrofida o'rash sifatida amalga oshiriladi. `Vec<T>` va `String` bilan bir xil ishlaydigan funksiyaga misol qilib, 8-11 ro'yxatda ko'rsatilgan yangi turdagi misolni yaratuvchi `new` funksiyadir.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-11: Yangi, bo'sh `String` yaratish</span>

Ushbu satr `s` deb nomlangan yangi bo'sh qatorni yaratadi, biz keyin unga ma'lumotlarni yuklashimiz mumkin. Ko'pincha, biz stringni boshlamoqchi bo'lgan dastlabki ma'lumotlarga ega bo'lamiz. Buning uchun biz string literallari kabi `Display` traittini amalga oshiradigan har qanday turda mavjud bo'lgan `to_string` metotidan foydalanamiz. Ro'yxat 8-12 ikkita misolni ko'rsatadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-12: string literalidan `String` yaratish uchun `to_string` metodidan foydalanish</span>

Bu kod `dastlabki tarkib`ni o‘z ichiga olgan stringni yaratadi.

Satr literalidan `String` yaratish uchun `String::from` funksiyasidan ham foydalanishimiz mumkin. 8-13 ro'yxatdagi kod `to_string` funksiyasidan foydalanadigan 8-12 ro'yxatdagi kodga teng:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-13: string literalidan `String` yaratish uchun `String::from` funksiyasidan foydalanish</span>

Stringlar juda ko'p narsalar uchun ishlatilganligi sababli, biz stringlar uchun juda ko'p turli xil umumiy API'lardan foydalanishimiz mumkin, bu bizga juda ko'p imkoniyatlarni taqdim etadi. Ulardan ba'zilari ortiqcha bo'lib tuyulishi mumkin, ammo ularning barchasi o'z joylariga ega! Bunday holda, `String::from` va `to_string` bir xil ishni bajaradi, shuning uchun tanlov sizga eng yoqqan uslubga bog'liq.

Yodda tutingki, stringlar UTF-8 bilan kodlangan, shuning uchun 8-14 ro'yxatda ko'rsatilganidek, biz ularga har qanday to'g'ri kodlangan ma'lumotlarni kiritishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-14: Salom so'zini turli tillarda stringlarda saqlash</span>

Bularning barchasi yaroqli `String` qiymatlari.

### Stringni yangilash

Agar siz unga ko'proq ma'lumot kiritsangiz, `String` hajmi kattalashishi mumkin va uning tarkibi `Vec<T>` tarkibidagi kabi o'zgarishi mumkin. Bundan tashqari, `String` qiymatlarini birlashtirish uchun `+` operatori yoki `format!` makrosidan qulay foydalanish mumkin.

#### `push_str` va `push` yordamida stringga biriktirish

Biz 8-15 roʻyxatda koʻrsatilganidek, string boʻlagini qoʻshish uchun `push_str` metodidan foydalanib `String`ni kengaytirishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-15: `push_str` metodi yordamida `String` ga satr boʻlagini qoʻshish</span>

Ushbu ikki qatordan keyin `s` tarkibida `dasturchi` bo'ladi. `push_str` metodi string bo'lagini oladi, chunki biz parametrga egalik qilishni xohlamaymiz. Masalan, 8-16 roʻyxatdagi kodda biz uning mazmunini `s1` ga qoʻshgandan soʻng `s2` dan foydalanish imkoniyatiga ega boʻlishni xohlaymiz.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-16: Tarkibni `String` ga qo'shgandan so'ng, string bo'lagidan foydalanish</span>

Agar `push_str` metodi `s2` ga egalik qilgan bo‘lsa, biz uning qiymatini oxirgi satrda chop eta olmaymiz. Biroq, bu kod biz kutgandek ishlaydi!

The `push` method takes a single character as a parameter and adds it to the
`String`. Listing 8-17 adds the letter “l” to a `String` using the `push`
method.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">Listing 8-17: Adding one character to a `String` value
using `push`</span>

As a result, `s` will contain `lol`.

#### Concatenation with the `+` Operator or the `format!` Macro

Often, you’ll want to combine two existing strings. One way to do so is to use
the `+` operator, as shown in Listing 8-18.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">Listing 8-18: Using the `+` operator to combine two
`String` values into a new `String` value</span>

The string `s3` will contain `Hello, world!`. The reason `s1` is no longer
valid after the addition, and the reason we used a reference to `s2`, has to do
with the signature of the method that’s called when we use the `+` operator.
The `+` operator uses the `add` method, whose signature looks something like
this:

```rust,ignore
fn add(self, s: &str) -> String {
```

In the standard library, you'll see `add` defined using generics and associated
types. Here, we’ve substituted in concrete types, which is what happens when we
call this method with `String` values. We’ll discuss generics in Chapter 10.
This signature gives us the clues we need to understand the tricky bits of the
`+` operator.

First, `s2` has an `&`, meaning that we’re adding a *reference* of the second
string to the first string. This is because of the `s` parameter in the `add`
function: we can only add a `&str` to a `String`; we can’t add two `String`
values together. But wait—the type of `&s2` is `&String`, not `&str`, as
specified in the second parameter to `add`. So why does Listing 8-18 compile?

The reason we’re able to use `&s2` in the call to `add` is that the compiler
can *coerce* the `&String` argument into a `&str`. When we call the `add`
method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`.
We’ll discuss deref coercion in more depth in Chapter 15. Because `add` does
not take ownership of the `s` parameter, `s2` will still be a valid `String`
after this operation.

Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does *not* have an `&`. This means `s1` in Listing 8-18 will be
moved into the `add` call and will no longer be valid after that. So although
`let s3 = s1 + &s2;` looks like it will copy both strings and create a new one,
this statement actually takes ownership of `s1`, appends a copy of the contents
of `s2`, and then returns ownership of the result. In other words, it looks
like it’s making a lot of copies but isn’t; the implementation is more
efficient than copying.

If we need to concatenate multiple strings, the behavior of the `+` operator
gets unwieldy:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"`
characters, it’s difficult to see what’s going on. For more complicated string
combining, we can instead use the `format!` macro:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

This code also sets `s` to `tic-tac-toe`. The `format!` macro works like
`println!`, but instead of printing the output to the screen, it returns a
`String` with the contents. The version of the code using `format!` is much
easier to read, and the code generated by the `format!` macro uses references
so that this call doesn’t take ownership of any of its parameters.

### Indexing into Strings

In many other programming languages, accessing individual characters in a
string by referencing them by index is a valid and common operation. However,
if you try to access parts of a `String` using indexing syntax in Rust, you’ll
get an error. Consider the invalid code in Listing 8-19.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">Listing 8-19: Attempting to use indexing syntax with a
String</span>

This code will result in the following error:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

The error and the note tell the story: Rust strings don’t support indexing. But
why not? To answer that question, we need to discuss how Rust stores strings in
memory.

#### Internal Representation

A `String` is a wrapper over a `Vec<u8>`. Let’s look at some of our properly
encoded UTF-8 example strings from Listing 8-14. First, this one:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

In this case, `len` will be 4, which means the vector storing the string “Hola”
is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8. The
following line, however, may surprise you. (Note that this string begins with
the capital Cyrillic letter Ze, not the Arabic number 3.)

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Asked how long the string is, you might say 12. In fact, Rust’s answer is 24:
that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because
each Unicode scalar value in that string takes 2 bytes of storage. Therefore,
an index into the string’s bytes will not always correlate to a valid Unicode
scalar value. To demonstrate, consider this invalid Rust code:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

You already know that `answer` will not be `З`, the first letter. When encoded
in UTF-8, the first byte of `З` is `208` and the second is `151`, so it would
seem that `answer` should in fact be `208`, but `208` is not a valid character
on its own. Returning `208` is likely not what a user would want if they asked
for the first letter of this string; however, that’s the only data that Rust
has at byte index 0. Users generally don’t want the byte value returned, even
if the string contains only Latin letters: if `&"hello"[0]` were valid code
that returned the byte value, it would return `104`, not `h`.

The answer, then, is that to avoid returning an unexpected value and causing
bugs that might not be discovered immediately, Rust doesn’t compile this code
at all and prevents misunderstandings early in the development process.

#### Bytes and Scalar Values and Grapheme Clusters! Oh My!

Another point about UTF-8 is that there are actually three relevant ways to
look at strings from Rust’s perspective: as bytes, scalar values, and grapheme
clusters (the closest thing to what we would call *letters*).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is
stored as a vector of `u8` values that looks like this:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

That’s 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust’s `char` type is, those
bytes look like this:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters:
they’re diacritics that don’t make sense on their own. Finally, if we look at
them as grapheme clusters, we’d get what a person would call the four letters
that make up the Hindi word:

```text
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.

A final reason Rust doesn’t allow us to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). But it isn’t possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.

### Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the
return type of the string-indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. If you really need to use
indices to create string slices, therefore, Rust asks you to be more specific.

Rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first 4 bytes of the string.
Earlier, we mentioned that each of these characters was 2 bytes, which means
`s` will be `Зд`.

If we were to try to slice only part of a character’s bytes with something like
`&hello[0..1]`, Rust would panic at runtime in the same way as if an invalid
index were accessed in a vector:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

You should use ranges to create string slices with caution, because doing so
can crash your program.

### Methods for Iterating Over Strings

The best way to operate on pieces of strings is to be explicit about whether
you want characters or bytes. For individual Unicode scalar values, use the
`chars` method. Calling `chars` on “Зд” separates out and returns two values
of type `char`, and you can iterate over the result to access each element:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

This code will print the following:

```text
З
д
```

Alternatively, the `bytes` method returns each raw byte, which might be
appropriate for your domain:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

This code will print the four bytes that make up this string:

```text
208
151
208
180
```

But be sure to remember that valid Unicode scalar values may be made up of more
than 1 byte.

Getting grapheme clusters from strings as with the Devanagari script is
complex, so this functionality is not provided by the standard library. Crates
are available on [crates.io](https://crates.io/)<!-- ignore --> if this is the
functionality you need.

### Strings Are Not So Simple

To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which means programmers have to put more thought into
handling UTF-8 data upfront. This trade-off exposes more of the complexity of
strings than is apparent in other programming languages, but it prevents you
from having to handle errors involving non-ASCII characters later in your
development life cycle.

The good news is that the standard library offers a lot of functionality built
off the `String` and `&str` types to help handle these complex situations
correctly. Be sure to check out the documentation for useful methods like
`contains` for searching in a string and `replace` for substituting parts of a
string with another string.

Let’s switch to something a bit less complex: hash maps!
