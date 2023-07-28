## UTF-8 kodlangan matnni String bilan saqlash

We talked about strings in Chapter 4, but we’ll look at them in more depth now. New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These factors combine in a way that can seem difficult when you’re coming from other programming languages.

We discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text. In this section, we’ll talk about the operations on `String` that every collection type has, such as creating, updating, and reading. We’ll also discuss the ways in which `String` is different from the other collections, namely how indexing into a `String` is complicated by the differences between how people and computers interpret `String` data.

### String nima?

We’ll first define what we mean by the term *string*. Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. In Chapter 4, we talked about *string slices*, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they might be referring to either the `String` or the string slice `&str` types, not just one of those types. Although this section is largely about `String`, both types are used heavily in Rust’s standard library, and both `String` and string slices are UTF-8 encoded.

### Yangi String yaratish

Many of the same operations available with `Vec<T>` are available with `String` as well, because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. An example of a function that works the same way with `Vec<T>` and `String` is the `new` function to create an instance, shown in Listing 8-11.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-11: Yangi, bo'sh `String` yaratish</span>

This line creates a new empty string called `s`, which we can then load data into. Often, we’ll have some initial data that we want to start the string with. For that, we use the `to_string` method, which is available on any type that implements the `Display` trait, as string literals do. Listing 8-12 shows two examples.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-12: string literalidan `String` yaratish uchun `to_string` metodidan foydalanish</span>

Bu kod `dastlabki tarkib`ni o‘z ichiga olgan stringni yaratadi.

We can also use the function `String::from` to create a `String` from a string literal. The code in Listing 8-13 is equivalent to the code from Listing 8-12 that uses `to_string`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-13: string literalidan `String` yaratish uchun `String::from` funksiyasidan foydalanish</span>

Because strings are used for so many things, we can use many different generic APIs for strings, providing us with a lot of options. Some of them can seem redundant, but they all have their place! In this case, `String::from` and `to_string` do the same thing, so which you choose is a matter of style and readability.

Yodda tutingki, stringlar UTF-8 bilan kodlangan, shuning uchun 8-14 ro'yxatda ko'rsatilganidek, biz ularga har qanday to'g'ri kodlangan ma'lumotlarni kiritishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-14: Salom so'zini turli tillarda stringlarda saqlash</span>

Bularning barchasi yaroqli `String` qiymatlari.

### Stringni yangilash

A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

#### `push_str` va `push` yordamida stringga biriktirish

Biz 8-15 roʻyxatda koʻrsatilganidek, string boʻlagini qoʻshish uchun `push_str` metodidan foydalanib `String`ni kengaytirishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```


<span class="caption">Roʻyxat 8-15: `push_str` metodi yordamida `String` ga satr boʻlagini qoʻshish</span>

After these two lines, `s` will contain `foobar`. The `push_str` method takes a string slice because we don’t necessarily want to take ownership of the parameter. For example, in the code in Listing 8-16, we want to be able to use `s2` after appending its contents to `s1`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-16: Tarkibni `String` ga qo'shgandan so'ng, string bo'lagidan foydalanish</span>

If the `push_str` method took ownership of `s2`, we wouldn’t be able to print its value on the last line. However, this code works as we’d expect!

The `push` method takes a single character as a parameter and adds it to the `String`. Listing 8-17 adds the letter “l” to a `String` using the `push` method.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```


<span class="caption">Roʻyxat 8-17: `push` yordamida `String` qiymatiga bitta belgi qoʻshish</span>

Natijada, `s` tarkibida `suv` bo'ladi.

#### operatori yoki `format!` makrosidan foydalanib satrlarni birlashtirish

Often, you’ll want to combine two existing strings. One way to do so is to use the `+` operator, as shown in Listing 8-18.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```


<span class="caption">Roʻyxat 8-18: Ikkita `String` qiymatini yangi `String` qiymatiga birlashtirish uchun `+` operatoridan foydalanish</span>

The string `s3` will contain `Hello, world!`. The reason `s1` is no longer valid after the addition, and the reason we used a reference to `s2`, has to do with the signature of the method that’s called when we use the `+` operator. The `+` operator uses the `add` method, whose signature looks something like this:

```rust,ignore
fn add(self, s: &str) -> String {
```

In the standard library, you'll see `add` defined using generics and associated types. Here, we’ve substituted in concrete types, which is what happens when we call this method with `String` values. We’ll discuss generics in Chapter 10. This signature gives us the clues we need to understand the tricky bits of the `+` operator.

First, `s2` has an `&`, meaning that we’re adding a *reference* of the second string to the first string. This is because of the `s` parameter in the `add` function: we can only add a `&str` to a `String`; we can’t add two `String` values together. But wait—the type of `&s2` is `&String`, not `&str`, as specified in the second parameter to `add`. So why does Listing 8-18 compile?

The reason we’re able to use `&s2` in the call to `add` is that the compiler can *coerce* the `&String` argument into a `&str`. When we call the `add` method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`. We’ll discuss deref coercion in more depth in Chapter 15. Because `add` does not take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation.

Second, we can see in the signature that `add` takes ownership of `self`, because `self` does *not* have an `&`. This means `s1` in Listing 8-18 will be moved into the `add` call and will no longer be valid after that. So although `let s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

Agar biz bir nechta satrlarni birlashtirishimiz kerak bo'lsa, `+` operatorining xatti-harakati noqulay bo'ladi:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"` characters, it’s difficult to see what’s going on. For more complicated string combining, we can instead use the `format!` macro:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

This code also sets `s` to `tic-tac-toe`. The `format!` macro works like `println!`, but instead of printing the output to the screen, it returns a `String` with the contents. The version of the code using `format!` is much easier to read, and the code generated by the `format!` macro uses references so that this call doesn’t take ownership of any of its parameters.

### Stringlarni indekslash

In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a `String` using indexing syntax in Rust, you’ll get an error. Consider the invalid code in Listing 8-19.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-19: String bilan indekslash sintaksisidan foydalanishga urinish</span>

Ushbu kod quyidagi xatoga olib keladi:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

The error and the note tell the story: Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.

#### Ichki vakillik

A `String` is a wrapper over a `Vec<u8>`. Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14. First, this one:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

In this case, `len` will be 4, which means the vector storing the string “Hola” is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8. The following line, however, may surprise you. (Note that this string begins with the capital Cyrillic letter Ze, not the number 3.)

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Asked how long the string is, you might say 12. In fact, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value. To demonstrate, consider this invalid Rust code:

```rust,ignore,does_not_compile
let salom = "Здравствуйте";
let javob = &salom[0];
```

You already know that `answer` will not be `З`, the first letter. When encoded in UTF-8, the first byte of `З` is `208` and the second is `151`, so it would seem that `answer` should in fact be `208`, but `208` is not a valid character on its own. Returning `208` is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even if the string contains only Latin letters: if `&"hello"[0]` were valid code that returned the byte value, it would return `104`, not `h`.

Javob shundaki, kutilmagan qiymatni qaytarmaslik va darhol topilmasligi mumkin bo'lgan xatolarni keltirib chiqarmaslik uchun Rust ushbu kodni umuman kompilyatsiya qilmaydi va ishlab chiqish jarayonida tushunmovchiliklarning oldini oladi.

#### Bytes and Scalar Values and Grapheme Clusters! Oh My!

UTF-8 bilan bog'liq yana bir nuqta shundaki, Rust nuqtai nazaridan satrlarga qarashning uchta mos usuli mavjud: baytlar, skalyar qiymatlar va grafema klasterlari (biz *harflar* deb ataydigan narsaga eng yaqin narsa).

Devanagari skriptida yozilgan hindcha “नमस्ते” so'ziga qarasak, u `u8` qiymatlari vektori sifatida saqlanadi, bu quyidagicha ko'rinadi:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s `char` type is, those bytes look like this:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

```text
["न", "म", "स्", "ते"]
```

Rust kompyuterlar saqlaydigan ma'lumotlar qatorini talqin qilishning turli usullarini taqdim etadi, shunda ma'lumotlar qaysi inson tilida bo'lishidan qat'i nazar, har bir dastur kerakli talqinni tanlashi mumkin.

A final reason Rust doesn’t allow us to index into a `String` to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a `String`, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

### String bo'laklari

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.

Bitta raqam bilan `[]` yordamida indeksatsiya qilish o'rniga, muayyan baytlarni o'z ichiga olgan string bo'laklarini yaratish uchun diapazon bilan `[]` dan foydalanishingiz mumkin:

```rust
let salom = "Здравствуйте";

let s = &salom[0..4];
```

Here, `s` will be a `&str` that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means `s` will be `Зд`.

Agar biz `&salom[0..1]` kabi belgi baytlarining faqat bir qismini kesishga harakat qilsak, Rust ish runtimeda xuddi vektordagi yaroqsiz indeksga kirish kabi panic qo'yadi:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Ehtiyotkorlik bilan string bo'laklarni yaratish uchun diapazonlardan foydalanishingiz kerak, chunki bu dasturni buzishi mumkin.

### Stringlarni takrorlash usullari

The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes. For individual Unicode scalar values, use the `chars` method. Calling `chars` on “Зд” separates out and returns two values of type `char`, and you can iterate over the result to access each element:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Ushbu kod quyidagilarni chop etadi:

```text
З
д
```

Shu bilan bir qatorda, `bytes` metodi boshqa domenga mos kelishi mumkin bo'lgan har bir baytni qaytaradi:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Ushbu kod ushbu satrni tashkil etuvchi to'rt baytni chop etadi:

```text
208
151
208
180
```

Lekin esda tutingki, joriy Unicode skalyar qiymatlari 1 baytdan ortiq bo'lishi mumkin.

Getting grapheme clusters from strings as with the Devanagari script is complex, so this functionality is not provided by the standard library. Crates are available on [crates.io](https://crates.io/)<!-- ignore --> if this is the functionality you need.

### Stringlar unchalik oddiy emas

To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of `String` data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

The good news is that the standard library offers a lot of functionality built off the `String` and `&str` types to help handle these complex situations correctly. Be sure to check out the documentation for useful methods like `contains` for searching in a string and `replace` for substituting parts of a string with another string.

Keling, biroz murakkabroq narsaga o'taylik: HashMap!
