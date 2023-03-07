## `Result` bilan tiklanadigan xatolar

Ko'pgina xatolar dasturni butunlay to'xtatishni talab qiladigan darajada jiddiy emas.
Ba'zan, funksiya bajarilmasa, bu siz osongina talqin qilishingiz va javob berishingiz mumkin bo'lgan sababdir. Misol uchun, agar siz faylni ochishga urinib ko'rsangiz va fayl mavjud bo'lmagani uchun bu operatsiya bajarilmasa, jarayonni tugatish o'rniga faylni yaratishni xohlashingiz mumkin.

2-bobdagi [“Potentsial muvaffaqiyatsizlikni `Result` bilan hal qilish”][handle_failure]<!-- ignore --> bo'limini eslang: biz u yerda muvaffaqiyatsizliklarni hal qilish uchun ikkita variantga ega bo'lgan `Ok` va `Err` varianti bo'lgan `Result` enumidan foydalanganmiz. Enumning o'zi quyidagicha aniqlanadi:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` va `E` umumiy turdagi parametrlardir: biz generiklarni 10-bobda batafsilroq muhokama qilamiz. Siz hozir bilishingiz kerak bo'lgan narsa shundaki, `T` `Ok` variantida muvaffaqiyatli holatda qaytariladigan qiymat turini bildiradi, va `E` `Err`(Xato) variantida xatolik holatida qaytariladigan xato turini ifodalaydi. `Result`da ushbu umumiy turdagi parametrlar mavjud bo'lganligi sababli, biz qaytarmoqchi bo'lgan muvaffaqiyatli qiymat va xato qiymati har xil bo'lishi mumkin bo'lgan turli vaziyatlarda `Result` turidan va unda belgilangan funksiyalardan foydalanishimiz mumkin.

Keling, `Result` qiymatini qaytaruvchi funksiyani chaqiraylik, chunki funksiya bajarilmasligi mumkin. 9-3 ro'yxatda biz faylni ochishga harakat qilamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Ro'yxat 9-3: Faylni ochish</span>

`File::open` return(qaytish) turi `Result<T, E>`dir. `File::open`ni amalga oshirishdagi umumiy `T` turi muvaffaqiyatli qabul qilingan qiymat turiga, `std::fs::File`ga, ya'ni fayl deskriptoriga mos keladi. Xato qiymatida ishlatiladigan `E` turi `std::io::Error`. Qaytish(return) turi `File::open` ga chaqiruv muvaffaqiyatli bo'lishi va biz o'qishimiz yoki yozishimiz mumkin bo'lgan fayl handleni qaytarishi mumkinligini anglatadi. Funksiya chaqiruvi ham muvaffaqiyatsiz bo'lishi mumkin: masalan, fayl mavjud bo'lmasligi yoki faylga kirish uchun ruxsatimiz bo'lmasligi mumkin. `File::open` funksiyasi muvaffaqiyatli yoki muvaffaqiyatsiz bo'lganligini va bir vaqtning o'zida bizga fayl identifikatori yoki xato haqida ma'lumot beradigan metodga ega bo'lishi kerak. Ushbu ma'lumot aynan `Result` enumini bildiradi.

Agar `File::open` muvaffaqiyatli bo'lsa, `fayl_ochish` qiymati fayl identifikatorini o'z ichiga olgan `Ok` misoli bo'ladi.
Muvaffaqiyatsiz bo'lgan taqdirda, `fayl_ochish` dagi qiymat `Err` misoli bo'lib, sodir bo'lgan xato turi haqida qo'shimcha ma'lumotni o'z ichiga oladi.

`File::open` qiymatiga qarab turli amallarni bajarish uchun 9-3-Ro'yxatdagi kodga o'zgartirishimiz kerak. 9-4 ro'yxatda biz 6-bobda muhokama qilgan asosiy tool - `match` ifodasi yordamida `Result` ni boshqarishning bir usuli ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Roʻyxat 9-4: Qaytarilishi mumkin boʻlgan `Result` variantlarini boshqarish uchun `match` ifodasidan foydalanish</span>

E'tibor bering, `Option` enumi kabi, `Result` enumi va uning variantlari avtomatik import (prelude) orqali kiritilgan, shuning uchun biz `match`  qatoridagi `Ok` va `Err`  variantlaridan oldin `Result::` ni belgilashimiz shart emas.

Natija `Ok` bo'lsa, bu kod `Ok` variantidan ichki `file` qiymatini qaytaradi va biz ushbu faylni ishlov berish qiymatini `fayl_ochish` o'zgaruvchisiga tayinlaymiz. `match`dan so'ng biz o'qish yoki yozish uchun fayl boshqaruvidan foydalanishimiz mumkin.

`match`ning boshqa qismi `File::open` dan `Err` qiymatini oladigan holatni boshqaradi. Ushbu misolda biz `panic!`  makrosini tanladik. Agar joriy jildimizda *olma.txt* nomli fayl bo‘lmasa va biz ushbu kodni ishga tushirsak, biz `panic!` makrosidan quyidagi natijani ko‘ramiz:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Odatdagidek, bu chiqish bizga nima noto'g'ri ketganligini aniq aytadi.

### Turli xil xatolarga moslashish

9-4 roʻyxatdagi kod nima uchun `File::open` muvaffaqiyatsiz boʻlishidan qatʼiy nazar `panic!` qoʻyadi.
Biroq, biz turli sabablarga ko'ra turli xil harakatlarni amalga oshirishni xohlaymiz: agar fayl mavjud bo'lmagani uchun `File::open` muvaffaqiyatsiz bo'lsa, biz faylni yaratmoqchimiz va fayl boshqaruvini yangi faylga qaytaramiz. Agar `File::open` boshqa sabablarga ko'ra, masalan, faylni ochishga ruxsatimiz yo'qligi sababli muvaffaqiyatsiz bo'lsa, biz hali ham kodga 9-4 ro'yxatdagi kabi `panic!` qo'yishini xohlaymiz. Buning uchun biz 9-5 ro'yxatda ko'rsatilgan ichki `match` ifodasini qo'shamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Ro'yxat 9-5: Har xil turdagi xatolarni turli yo'llar bilan hal qilish</span>

`File::open` `Err` variantida qaytaradigan qiymat turi `io::Error` bo'lib, bu standart kutubxona tomonidan taqdim etilgan strukturadir. Ushbu strukturada `io::ErrorKind` qiymatini olish uchun chaqirishimiz mumkin bo'lgan `kind` metodi mavjud. `io::ErrorKind` enumi standart kutubxona tomonidan taqdim etilgan va `io` operatsiyasi natijasida yuzaga kelishi mumkin bo'lgan turli xil xatolarni ifodalovchi variantlarga ega. Biz foydalanmoqchi boʻlgan variant `ErrorKind::NotFound` boʻlib, biz ochmoqchi boʻlgan fayl hali mavjud emasligini bildiradi. Shunday qilib, biz `fayl_ochish` bo'yicha mos kelamiz, lekin bizda `error.kind()` da ichki match ham bor.

Biz ichki matchni tekshirmoqchi bo'lgan shart - `error.kind()` tomonidan qaytarilgan qiymat `ErrorKind` enumining `NotFound` variantidir. Agar shunday bo'lsa, biz faylni `File::create` yordamida yaratishga harakat qilamiz. Biroq, `File::create` ham muvaffaqiyatsiz bo'lishi mumkinligi sababli, bizga ichki `match` ifodasida ikkinchi arm kerak. Faylni yaratib bo'lmaganda, boshqa xato xabari chop etiladi. Tashqi `match` ning ikkinchi armi bir xil bo'lib qoladi, shuning uchun dastur yetishmayotgan fayl xatosidan tashqari har qanday xato haqida panic qo'yadi.

> ### `Result<T, E>` bilan `match` dan foydalanishning muqobillari
>
> Bu juda ko'p `match`! `match` ifodasi juda foydali, lekin ayni paytda
> juda primitivdir. 13-bobda siz `Result<T, E>` da belgilangan koʻplab
> metodlarda qoʻllaniladigan yopilishlar(closures) haqida bilib olasiz. Ushbu
> metodlar kodingizdagi `Result<T, E>` qiymatlari bilan ishlashda `match` 
> dan foydalanishdan ko'ra qisqaroq bo'lishi mumkin.
>
> Misol uchun, 9-5 ro'yxatda ko'rsatilgan mantiqni yozishning yana bir
> usuli, bu safar closures va `unwrap_or_else` metodi yordamida:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let fayl_ochish = File::open("olma.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("olma.txt").unwrap_or_else(|error| {
>                 panic!("Fayl yaratishda muammo: {:?}", error);
>             })
>         } else {
>             panic!("Faylni ochishda muammo: {:?}", error);
>         }
>     });
> }
> ```
>
> Garchi bu kod 9-5 roʻyxatdagi kabi harakatga ega boʻlsa-da, unda
> `match` iboralari mavjud emas va oʻqish uchun qulayroq. 13-bobni o‘qib bo‘lgach,
> ushbu misolga qayting va standart kutubxona hujjatlarida `unwrap_or_else`
> metodini qidiring. Ushbu metodlarning ko'pchiligi xatolar bilan
> shug'ullanayotganda katta o'rinli `match`  iboralarni tozalashi mumkin.

### Xatoda panic uchun yorliqlar: `unwrap` va `expect`

Using `match` works well enough, but it can be a bit verbose and doesn’t always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various, more specific tasks. The `unwrap` method is a
shortcut method implemented just like the `match` expression we wrote in
Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

If we run this code without a *hello.txt* file, we’ll see an error message from
the `panic!` call that the `unwrap` method makes:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

Similarly, the `expect` method lets us also choose the `panic!` error message.
Using `expect` instead of `unwrap` and providing good error messages can convey
your intent and make tracking down the source of a panic easier. The syntax of
`expect` looks like this:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here’s what it looks like:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

In production-quality code, most Rustaceans choose `expect` rather than
`unwrap` and give more context about why the operation is expected to always
succeed. That way, if your assumptions are ever proven wrong, you have more
information to use in debugging.

### Propagating Errors

When a function’s implementation calls something that might fail, instead of
handling the error within the function itself, you can return the error to the
calling code so that it can decide what to do. This is known as *propagating*
the error and gives more control to the calling code, where there might be more
information or logic that dictates how the error should be handled than what
you have available in the context of your code.

For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called the function.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `match`</span>

This function can be written in a much shorter way, but we’re going to start by
doing a lot of it manually in order to explore error handling; at the end,
we’ll show the shorter way. Let’s look at the return type of the function
first: `Result<String, io::Error>`. This means the function is returning a
value of the type `Result<T, E>` where the generic parameter `T` has been
filled in with the concrete type `String`, and the generic type `E` has been
filled in with the concrete type `io::Error`.

If this function succeeds without any problems, the code that calls this
function will receive an `Ok` value that holds a `String`—the username that
this function read from the file. If this function encounters any problems, the
calling code will receive an `Err` value that holds an instance of `io::Error`
that contains more information about what the problems were. We chose
`io::Error` as the return type of this function because that happens to be the
type of the error value returned from both of the operations we’re calling in
this function’s body that might fail: the `File::open` function and the
`read_to_string` method.

The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value with a `match` similar to the `match` in Listing 9-4.
If `File::open` succeeds, the file handle in the pattern variable `file`
becomes the value in the mutable variable `username_file` and the function
continues. In the `Err` case, instead of calling `panic!`, we use the `return`
keyword to return early out of the function entirely and pass the error value
from `File::open`, now in the pattern variable `e`, back to the calling code as
this function’s error value.

So if we have a file handle in `username_file`, the function then creates a new
`String` in variable `username` and calls the `read_to_string` method on
the file handle in `username_file` to read the contents of the file into
`username`. The `read_to_string` method also returns a `Result` because it
might fail, even though `File::open` succeeded. So we need another `match` to
handle that `Result`: if `read_to_string` succeeds, then our function has
succeeded, and we return the username from the file that’s now in `username`
wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the
same way that we returned the error value in the `match` that handled the
return value of `File::open`. However, we don’t need to explicitly say
`return`, because this is the last expression in the function.

The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. It’s
up to the calling code to decide what to do with those values. If the calling
code gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upward for
it to handle appropriately.

This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.

#### A Shortcut for Propagating Errors: the `?` Operator

Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as in Listing 9-6, but this implementation uses the
`?` operator.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: A function that returns errors to the
calling code using the `?` operator</span>

The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.

There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert values from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent
all the ways a function might fail, even if parts might fail for many different
reasons.

For example, we could change the `read_username_from_file` function in Listing
9-7 to return a custom error type named `OurError` that we define. If we also
define `impl From<io::Error> for OurError` to construct an instance of
`OurError` from an `io::Error`, then the `?` operator calls in the body of
`read_username_from_file` will call `from` and convert the error types without
needing to add any more code to the function.

In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `username_file`. If an error
occurs, the `?` operator will return early out of the whole function and give
any `Err` value to the calling code. The same thing applies to the `?` at the
end of the `read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: Chaining method calls after the `?`
operator</span>

We’ve moved the creation of the new `String` in `username` to the beginning of
the function; that part hasn’t changed. Instead of creating a variable
`username_file`, we’ve chained the call to `read_to_string` directly onto the
result of `File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing `username`
when both `File::open` and `read_to_string` succeed rather than returning
errors. The functionality is again the same as in Listing 9-6 and Listing 9-7;
this is just a different, more ergonomic way to write it.

Listing 9-9 shows a way to make this even shorter using `fs::read_to_string`.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: Using `fs::read_to_string` instead of
opening and then reading the file</span>

Reading a file into a string is a fairly common operation, so the standard
library provides the convenient `fs::read_to_string` function that opens the
file, creates a new `String`, reads the contents of the file, puts the contents
into that `String`, and returns it. Of course, using `fs::read_to_string`
doesn’t give us the opportunity to explain all the error handling, so we did it
the longer way first.

#### Where The `?` Operator Can Be Used

The `?` operator can only be used in functions whose return type is compatible
with the value the `?` is used on. This is because the `?` operator is defined
to perform an early return of a value out of the function, in the same manner
as the `match` expression we defined in Listing 9-6. In Listing 9-6, the
`match` was using a `Result` value, and the early return arm returned an
`Err(e)` value. The return type of the function has to be a `Result` so that
it’s compatible with this `return`.

In Listing 9-10, let’s look at the error we’ll get if we use the `?` operator
in a `main` function with a return type incompatible with the type of the value
we use `?` on:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Listing 9-10: Attempting to use the `?` in the `main`
function that returns `()` won’t compile</span>

This code opens a file, which might fail. The `?` operator follows the `Result`
value returned by `File::open`, but this `main` function has the return type of
`()`, not `Result`. When we compile this code, we get the following error
message:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result`, `Option`, or another type that implements
`FromResidual`.

To fix the error, you have two choices. One choice is to change the return type
of your function to be compatible with the value you’re using the `?` operator
on as long as you have no restrictions preventing that. The other technique is
to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T,
E>` in whatever way is appropriate.

The error message also mentioned that `?` can be used with `Option<T>` values
as well. As with using `?` on `Result`, you can only use `?` on `Option` in a
function that returns an `Option`. The behavior of the `?` operator when called
on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`:
if the value is `None`, the `None` will be returned early from the function at
that point. If the value is `Some`, the value inside the `Some` is the
resulting value of the expression and the function continues. Listing 9-11 has
an example of a function that finds the last character of the first line in the
given text:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Listing 9-11: Using the `?` operator on an `Option<T>`
value</span>

This function returns `Option<char>` because it’s possible that there is a
character there, but it’s also possible that there isn’t. This code takes the
`text` string slice argument and calls the `lines` method on it, which returns
an iterator over the lines in the string. Because this function wants to
examine the first line, it calls `next` on the iterator to get the first value
from the iterator. If `text` is the empty string, this call to `next` will
return `None`, in which case we use `?` to stop and return `None` from
`last_char_of_first_line`. If `text` is not the empty string, `next` will
return a `Some` value containing a string slice of the first line in `text`.

The `?` extracts the string slice, and we can call `chars` on that string slice
to get an iterator of its characters. We’re interested in the last character in
this first line, so we call `last` to return the last item in the iterator.
This is an `Option` because it’s possible that the first line is the empty
string, for example if `text` starts with a blank line but has characters on
other lines, as in `"\nhi"`. However, if there is a last character on the first
line, it will be returned in the `Some` variant. The `?` operator in the middle
gives us a concise way to express this logic, allowing us to implement the
function in one line. If we couldn’t use the `?` operator on `Option`, we’d
have to implement this logic using more method calls or a `match` expression.

Note that you can use the `?` operator on a `Result` in a function that returns
`Result`, and you can use the `?` operator on an `Option` in a function that
returns `Option`, but you can’t mix and match. The `?` operator won’t
automatically convert a `Result` to an `Option` or vice versa; in those cases,
you can use methods like the `ok` method on `Result` or the `ok_or` method on
`Option` to do the conversion explicitly.

So far, all the `main` functions we’ve used return `()`. The `main` function is
special because it’s the entry and exit point of executable programs, and there
are restrictions on what its return type can be for the programs to behave as
expected.

Luckily, `main` can also return a `Result<(), E>`. Listing 9-12 has the
code from Listing 9-10 but we’ve changed the return type of `main` to be
`Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end. This
code will now compile:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Listing 9-12: Changing `main` to return `Result<(), E>`
allows the use of the `?` operator on `Result` values</span>

The `Box<dyn Error>` type is a *trait object*, which we’ll talk about in the
[“Using Trait Objects that Allow for Values of Different
Types”][trait-objects]<!-- ignore --> section in Chapter 17. For now, you can
read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result`
value in a `main` function with the error type `Box<dyn Error>` is allowed,
because it allows any `Err` value to be returned early. Even though the body of
this `main` function will only ever return errors of type `std::io::Error`, by
specifying `Box<dyn Error>`, this signature will continue to be correct even if
more code that returns other errors is added to the body of `main`.

When a `main` function returns a `Result<(), E>`, the executable will
exit with a value of `0` if `main` returns `Ok(())` and will exit with a
nonzero value if `main` returns an `Err` value. Executables written in C return
integers when they exit: programs that exit successfully return the integer
`0`, and programs that error return some integer other than `0`. Rust also
returns integers from executables to be compatible with this convention.

The `main` function may return any types that implement [the
`std::process::Termination` trait][termination]<!-- ignore -->, which contains
a function `report` that returns an `ExitCode`. Consult the standard library
documentation for more information on implementing the `Termination` trait for
your own types.

Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: ../std/process/trait.Termination.html
