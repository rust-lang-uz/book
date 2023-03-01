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

`push` metodi parametr sifatida bitta belgini oladi va uni `String` ga qo'shadi. 8-17 ro'yxatda `push` metodi yordamida `String` ga `v` harfi qo'shiladi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-17: `push` yordamida `String` qiymatiga bitta belgi qoʻshish</span>

Natijada, `s` tarkibida `suv` bo'ladi.

#### `+` operatori yoki `format!` makrosidan foydalanib satrlarni birlashtirish

Ko'pincha siz ikkita mavjud satrni birlashtirishni xohlaysiz. Buning usullaridan biri 8-18 ro'yxatda ko'rsatilganidek, `+` operatoridan foydalanishdir.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-18: Ikkita `String` qiymatini yangi `String` qiymatiga birlashtirish uchun `+` operatoridan foydalanish</span>

`s3` qatorida `Salom, Rust!` bo'ladi. Qo‘shishdan keyin `s1` ning endi haqiqiy emasligi va `s2`ga referenceni qo‘llaganimiz sababi `+` operatoridan foydalanganda chaqirilayotgan metodning imzosi bilan bog‘liq.
`+` operatori `add` metodidan foydalanadi, uning imzosi quyidagicha ko'rinadi:

```rust,ignore
fn add(self, s: &str) -> String {
```

Standart kutubxonada siz umumiy va tegishli turlar yordamida aniqlangan `add`ni ko'rasiz. Bu erda biz aniq turlarni almashtirdik, bu metodni `String` qiymatlari bilan chaqirganimizda sodir bo'ladi. Biz 10-bobda generiklarni muhokama qilamiz.
Ushbu imzo bizga `+` operatorining murakkab bitlarini tushunishimiz kerak bo'lgan maslahatlarni beradi.

Birinchidan, `s2` `&` belgisiga ega, ya'ni biz birinchi satrga ikkinchi satrning *reference*ni qo'shmoqdamiz. Buning sababi `add` funksiyasidagi `s` parametri: biz faqat `String`ga `&str` qo'shishimiz mumkin; biz ikkita `String` qiymatini qo'sha olmaymiz. Lekin kuting – `&s2` turi `add` uchun ikkinchi parametrda ko‘rsatilganidek, `&str` emas, `&String`dir. Xo'sh, nima uchun 8-18 ro'yxatdagi kod kompilyatsiya bo'ladi?

`add` chaqiruvida `&s2` dan foydalanishimiz sababi shundaki, kompilyator `&String` argumentini `&str` ga *majburlashi(coerce)* mumkin. Biz `add` metodini chaqirganimizda Rust *deref coercion* dan foydalanadi, bu erda `&s2` ni `&s2[..]` ga aylantiradi.
Biz 15-bobda coercion haqida batafsilroq gaplashamiz. `add` `s` parametriga egalik qilmaganligi sababli, `s2` bu amaldan keyin ham haqiqiy `String` bo'lib qoladi.

Ikkinchidan, imzoda `add` `self` egalik qilishini ko'rishimiz mumkin, chunki `self`da `&` *yo'q*. Bu shuni anglatadiki, 8-18-sonli ro'yxatdagi `s1` `add` chaqiruviga o'tkaziladi va bundan keyin endi yaroqsiz bo'ladi. Shunday qilib, `let s3 = s1 + &s2;` har ikkala satrdan nusxa ko'chiradigan va yangisini yaratadiganga o'xshasa-da, bu statement aslida `s1`ga egalik qiladi va `s2` mazmunining nusxasini qo'shadi, va keyin natijaga egalik huquqini qaytaradi. Boshqacha qilib aytganda, u juda ko'p nusxa ko'chirayotganga o'xshaydi, lekin unday emas; implement qilish nusxalashdan ko'ra samaraliroq.

Agar biz bir nechta satrlarni birlashtirishimiz kerak bo'lsa, `+` operatorining xatti-harakati noqulay bo'ladi:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Bu vaqtda `s` `tic-tac-toe` bo'ladi. Barcha `+` va `"` belgilar bilan nima sodir bo'layotganini ko'rish qiyin. Murakkab qatorlarni birlashtirish uchun biz `format!` makrosidan foydalanishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Ushbu kod `s` ni `tic-tac-toe` ga ham o'rnatadi. `format!` makrosi `println!` kabi ishlaydi, lekin natijani ekranga chop etish o'rniga mazmuni bilan `String`ni qaytaradi. Kodning `format!` dan foydalanilgan versiyasini o‘qish ancha oson va `format!` makrosi tomonidan yaratilgan kod bu chaqiruv uning parametrlaridan birortasiga egalik qilmasligi uchun havolalardan foydalanadi.

### Stringlarni indekslash

Ko'pgina boshqa dasturlash tillarida stringdagi alohida belgilarga indeks bo'yicha murojaat qilish orqali kirish to'g'ri va keng tarqalgan operatsiya hisoblanadi. Biroq, agar siz Rust-da indekslash sintaksisidan foydalanib, `String` qismlariga kirishga harakat qilsangiz, xatoga duch kelasiz. 8-19 ro'yxatdagi noto'g'ri kodni ko'rib chiqing.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-19: String bilan indekslash sintaksisidan foydalanishga urinish</span>

Ushbu kod quyidagi xatoga olib keladi:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

Xato va eslatma Rust indekslashni qo'llab-quvvatlamasligini aytadi. Lekin nega yo'q? Bu savolga javob berish uchun Rust stringlarni xotirada qanday saqlashini muhokama qilishimiz kerak.

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
