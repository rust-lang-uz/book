## Slice turi

*Slicelar* butun to'plamga emas, balki to'plamdagi elementlarning qo'shni ketma-ketligiga murojaat qilish imkonini beradi. Slice bir xil referencedir, shuning uchun u ownershipga ega emas.

Bu erda kichik dasturlash muammosi: bo'shliqlar bilan ajratilgan so'zlar qatorini oladigan va shu qatorda topilgan birinchi so'zni qaytaradigan funksiya yozing.
Agar funksiya satrda bo'sh joy topmasa, butun satr bitta so'zdan iborat bo'lishi kerak, shuning uchun butun satr qaytarilishi kerak.

Keling, slicelar hal qiladigan muammoni tushunish uchun ushbu funksiyaning imzosini slicelardan foydalanmasdan qanday yozishni ko'rib chiqaylik:

```rust,ignore
fn birinchi_soz(s: &String) -> ?
```

`birinchi_soz` funksiyasi parametr sifatida `&String` ga ega. Biz ownershiplik qilishni xohlamaymiz, shuning uchun bu yaxshi. Ammo biz nimani return qilishimiz kerak? Bizda satrning *qismi* haqida gapirishning methodi yo'q. Biroq, biz bo'sh joy bilan ko'rsatilgan so'z oxiri indeksini qaytarishimiz mumkin. 4-7 ro'yxatda ko'rsatilganidek, buni sinab ko'raylik.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 4-7: `String` parametriga bayt indeks qiymatini qaytaradigan `birinchi_soz` funksiyasi</span>

Biz `String` elementini element bo'yicha ko'rib chiqishimiz va qiymat bo'sh joy yoki yo'qligini tekshirishimiz kerakligi sababli, `as_bytes` usuli yordamida `String`ni baytlar arrayiga aylantiramiz.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

Keyinchalik, `iter` methodi yordamida baytlar arrayida iterator yaratamiz:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```
Biz iteratorlarni [13-bobda][ch13]<!-- ignore --> batafsilroq muhokama qilamiz.
Hozircha bilingki, `iter` to‘plamdagi har bir elementni return qiladigan va `enumerate` `iter` natijasini o‘rab, har bir elementni tuplening bir qismi sifatida return qiladigan methoddir. `enumerate` dan qaytarilgan tuplening birinchi elementi indeks, ikkinchi element esa elementga referencedir.
Bu indeksni o'zimiz hisoblashdan ko'ra biroz qulayroqdir.

`enumerate` methodi tupleni qaytarganligi sababli, biz ushbu tupleni destructure qilish uchun patternlardan foydalanishimiz mumkin. Biz [6-bobda][ch6]<!-- ignore --> patternlarni ko'proq muhokama qilamiz. `for` siklida biz tupledagi indeks uchun `i` va bitta bayt uchun `&element` ga ega bo‘lgan patternni belgilaymiz.
Biz `.iter().enumerate()` dan elementga referenceni olganimiz uchun biz patternda `&` dan foydalanamiz.

`for` sikli ichida biz bayt literal sintaksisidan foydalanib, bo'sh joyni ifodalovchi baytni qidiramiz. Agar bo'sh joy topsak, biz pozitsiyani return qilamiz.
Aks holda, `s.len()` yordamida satr uzunligini qaytaramiz.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

Endi bizda satrdagi birinchi so'zning oxirgi indeksini aniqlashning usuli bor, ammo muammo bor. Biz `usize` ni o'z-o'zidan qaytarmoqdamiz, lekin bu `&String` kontekstida faqat meaningful raqam. Boshqacha qilib aytadigan bo'lsak, bu `String` dan alohida qiymat bo'lganligi sababli, uning kelajakda ham amal qilishiga kafolat yo'q. Ro'yxat 4-8da 4-7 ro'yxatdagi `birinchi_soz` funksiyasidan foydalanadigan dasturni ko'rib chiqing.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

<span class="caption">Ro'yxat 4-8: `birinchi_soz` funksiyasini chaqirish natijasida olingan natijani saqlash va keyin `String` tarkibini o'zgartirish</span>

Bu dastur hech qanday xatosiz kompilyatsiya qiladi va agar biz `s.clear()` ga murojat qilgandan keyin `soz` ishlatgan bo'lsak ham shunday bo'lardi. Because `word` isn’t connected to the state of `s`
at all, `word` still contains the value `5`. We could use that value `5` with
the variable `s` to try to extract the first word out, but this would be a bug
because the contents of `s` have changed since we saved `5` in `word`.

Having to worry about the index in `word` getting out of sync with the data in
`s` is tedious and error prone! Managing these indices is even more brittle if
we write a `second_word` function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking a starting *and* an ending index, and we have even more
values that were calculated from data in a particular state but aren’t tied to
that state at all. We have three unrelated variables floating around that need
to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

### String Slices

A *string slice* is a reference to part of a `String`, and it looks like this:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

Rather than a reference to the entire `String`, `hello` is a reference to a
portion of the `String`, specified in the extra `[0..5]` bit. We create slices
using a range within brackets by specifying `[starting_index..ending_index]`,
where `starting_index` is the first position in the slice and `ending_index` is
one more than the last position in the slice. Internally, the slice data
structure stores the starting position and the length of the slice, which
corresponds to `ending_index` minus `starting_index`. So, in the case of `let
world = &s[6..11];`, `world` would be a slice that contains a pointer to the
byte at index 6 of `s` with a length value of `5`.

Figure 4-6 shows this in a diagram.

<img alt="Three tables: a table representing the stack data of s, which points
to the byte at index 0 in a table of the string data &quot;hello world&quot; on
the heap. The third table rep-resents the stack data of the slice world, which
has a length value of 5 and points to byte 6 of the heap data table."
src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-6: String slice referring to part of a
`String`</span>

With Rust’s `..` range syntax, if you want to start at index 0, you can drop
the value before the two periods. In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the `String`, you
can drop the trailing number. That means these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these
are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> Note: String slice range indices must occur at valid UTF-8 character
> boundaries. If you attempt to create a string slice in the middle of a
> multibyte character, your program will exit with an error. For the purposes
> of introducing string slices, we are assuming ASCII only in this section; a
> more thorough discussion of UTF-8 handling is in the [“Storing UTF-8 Encoded
> Text with Strings”][strings]<!-- ignore --> section of Chapter 8.

With all this information in mind, let’s rewrite `first_word` to return a
slice. The type that signifies “string slice” is written as `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

We get the index for the end of the word the same way we did in Listing 4-7, by
looking for the first occurrence of a space. When we find a space, we return a
string slice using the start of the string and the index of the space as the
starting and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up because the
compiler will ensure the references into the `String` remain valid. Remember
the bug in the program in Listing 4-8, when we got the index to the end of the
first word but then cleared the string so our index was invalid? That code was
logically incorrect but didn’t show any immediate errors. The problems would
show up later if we kept trying to use the first word index with an emptied
string. Slices make this bug impossible and let us know we have a problem with
our code much sooner. Using the slice version of `first_word` will throw a
compile-time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

Here’s the compiler error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

Recall from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Because `clear` needs to
truncate the `String`, it needs to get a mutable reference. The `println!`
after the call to `clear` uses the reference in `word`, so the immutable
reference must still be active at that point. Rust disallows the mutable
reference in `clear` and the immutable reference in `word` from existing at the
same time, and compilation fails. Not only has Rust made our API easier to use,
but it has also eliminated an entire class of errors at compile time!

<!-- Old heading. Do not remove or links may break. -->
<a id="string-literals-are-slices"></a>

#### String Literals as Slices

Recall that we talked about string literals being stored inside the binary. Now
that we know about slices, we can properly understand string literals:

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: it’s a slice pointing to that specific point of
the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

#### String Slices as Parameters

Knowing that you can take slices of literals and `String` values leads us to
one more improvement on `first_word`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write the signature shown in Listing 4-9
instead because it allows us to use the same function on both `&String` values
and `&str` values.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<span class="caption">Listing 4-9: Improving the `first_word` function by using
a string slice for the type of the `s` parameter</span>

If we have a string slice, we can pass that directly. If we have a `String`, we
can pass a slice of the `String` or a reference to the `String`. This
flexibility takes advantage of *deref coercions*, a feature we will cover in
[“Implicit Deref Coercions with Functions and
Methods”][deref-coercions]<!--ignore--> section of Chapter 15.

Defining a function to take a string slice instead of a reference to a `String`
makes our API more general and useful without losing any functionality:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

### Other Slices

String slices, as you might imagine, are specific to strings. But there’s a
more general slice type too. Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

Just as we might want to refer to part of a string, we might want to refer to
part of an array. We’d do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

This slice has the type `&[i32]`. It works the same way as string slices do, by
storing a reference to the first element and a length. You’ll use this kind of
slice for all sorts of other collections. We’ll discuss these collections in
detail when we talk about vectors in Chapter 8.

## Summary

The concepts of ownership, borrowing, and slices ensure memory safety in Rust
programs at compile time. The Rust language gives you control over your memory
usage in the same way as other systems programming languages, but having the
owner of data automatically clean up that data when the owner goes out of scope
means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about
these concepts further throughout the rest of the book. Let’s move on to
Chapter 5 and look at grouping pieces of data together in a `struct`.

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
