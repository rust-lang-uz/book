## `panic!` yoki `panic!` qo'ymaslik

Xo'sh, qachon `panic!` deb murojat qilish va qachon `Result`ni qaytarish kerakligini qanday hal qilasiz? Kodda panic paydo bo'lganda, uni tiklashning iloji yo'q. Qayta tiklashning mumkin bo'lgan yo'li bormi yoki yo'qmi, har qanday xatolik uchun `panic!` deb chaiqruv qilishingiz mumkin, lekin siz chaqiruv kodi nomidan vaziyatni tuzatib bo'lmaydi degan qarorga kelasiz. `Result` qiymatini qaytarishni tanlaganingizda, siz chaqiruv kodini tanlash imkoniyatini berasiz. Chaqiruv kodi vaziyatga mos keladigan tarzda tiklashga urinishi mumkin yoki `Err`dagi xatoni qayta tiklab bo'lmaydi, deb qaror qilishi va `panic!` qo'yishi mumkin, bu sizning tiklanadigan xatongizni tuzatib bo'lmaydiganga aylantiradi. Shuning uchun, muvaffaqiyatsiz bo'lishi mumkin bo'lgan funksiyani belgilashda `Result` ni qaytarish yaxshi standart tanlovdir.

Misollar, prototip kodi va testlar kabi holatlarda `Result`ni qaytarish o'rniga panic qo'yadigan kodni yozish maqsadga muvofiqdir. Keling, nima uchun ekanligini ko'rib chiqaylik, keyin kompilyator muvaffaqiyatsizlik mumkin emasligini ayta olmaydigan vaziyatlarni muhokama qilaylik, lekin siz inson sifatida buni qila olasiz. Bob kutubxona kodida panic qo'yish yoki yo'qligini hal qilish bo'yicha ba'zi umumiy ko'rsatmalar bilan yakunlanadi.

### Misollar, Prototip Kodi va Testlar

Ba'zi bir kontseptsiyani tasvirlash uchun misol yozayotganingizda, shuningdek, xatolarni qayta ishlash kodini o'z ichiga olgan holda, misolni kamroq tushunarli qilish mumkin. Misollarda, panic qo'zg'atishi mumkin bo'lgan `unwrap` kabi metodga murojaat qilish sizning ilovangiz xatoliklarni qanday hal qilishini xohlayotganingiz uchun to'ldiruvchi sifatida tushuniladi, bu sizning kodingizning qolgan qismi nima qilayotganiga qarab farq qilishi mumkin.

Xuddi shunday, prototiplashda xatolarni qanday hal qilishni hal qilishdan oldin `unwrap` va `expect` metodllari juda qulaydir.
Dasturingizni yanada mustahkamroq qilishga tayyor bo'lganingizda ular kodingizda aniq belgilar qoldiradilar.

Agar testda metod chaqiruvi muvaffaqiyatsiz bo'lsa, bu metod sinovdan o'tkazilayotgan funksiya bo'lmasa ham, butun test muvaffaqiyatsiz bo'lishini xohlaysiz. Chunki `panic!` – bu sinovning muvaffaqiyatsiz deb belgilanishi, `unwrap` yoki `expect` deb atalgan narsa aynan shunday bo'lishi kerak.

### Siz kompilyatordan ko'ra ko'proq ma'lumotga ega bo'lgan holatlar

Agar sizda `Result` `Ok` qiymatiga ega bo'lishini ta'minlaydigan boshqa mantiqqa ega bo'lsangiz, `unwrap` yoki `expect` ni chaqirish ham o'rinli bo'lardi, ammo mantiq kompilyator tushunadigan narsa emas. Siz hali ham `Result` qiymatiga ega bo'lasiz, uni hal qilishingiz kerak: siz murojaat qilayotgan har qanday operatsiya sizning vaziyatingizda mantiqan imkonsiz bo'lsa ham, umuman muvaffaqiyatsiz bo'lish ehtimoli bor. Agar siz kodni qo‘lda tekshirish orqali sizda hech qachon `Err` varianti bo‘lmasligiga ishonch hosil qilsangiz, `unwrap` deb nomlash juda maqbuldir, va `expect` matnida hech qachon `Err` varianti bo'lmaydi deb o'ylagan sababni hujjatlash yaxshiroqdir. Mana bir misol:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Qattiq kodlangan stringni tahlil qilish orqali `IpAddr` misolini yaratmoqdamiz. Biz `127.0.0.1` to‘g‘ri IP manzil ekanligini ko‘ramiz, shuning uchun bu yerda `expect` dan foydalanish mumkin. Biroq, qattiq kodlangan, yaroqli satrga ega bo'lish `parse` metodining qaytish turini o'zgartirmaydi: biz hali ham `Result` qiymatini olamiz va kompilyator bizni `Result` bilan ishlashga majbur qiladi, go‘yo `Err` varianti mumkin, chunki kompilyator bu satr har doim haqiqiy IP manzil ekanligini ko‘rish uchun yetarlicha aqlli emas. Agar IP-manzillar qatori dasturga qattiq kodlanganidan ko'ra foydalanuvchidan kelgan bo'lsa va shuning uchun muvaffaqiyatsizlikka uchragan bo'lsa, biz, albatta, `Result` ni yanada ishonchli tarzda boshqarishni xohlaymiz.
Ushbu IP-manzil qattiq kodlangan degan taxminni eslatib o'tsak, agar kelajakda IP-manzilni boshqa manbadan olishimiz kerak bo'lsa, bizni `expect` ni xatolarni boshqarish kodini yaxshiroq o'zgartirishga undaydi.

### Xatolarni bartaraf etish bo'yicha ko'rsatmalar

Agar kodingiz yomon holatda bo'lishi mumkin bo'lsa, kodingiz panic qo'yishi tavsiya etiladi. Shu nuqtai nazardan, *yomon holat* deganda baʼzi taxminlar(assumption), kafolatlar(guarantee), shartnomalar(contract,) yoki oʻzgarmasliklar buzilganda, masalan, notoʻgʻri qiymatlar, qarama-qarshi qiymatlar yoki yetishmayotgan qiymatlar kodingizga oʻtkazilganda, shuningdek quyidagilardan biri yoki bir nechtasi:

* The bad state is something that is unexpected, as opposed to something that
  will likely happen occasionally, like a user entering data in the wrong
  format.
* Your code after this point needs to rely on not being in this bad state,
  rather than checking for the problem at every step.
* There’s not a good way to encode this information in the types you use. We’ll
  work through an example of what we mean in the [“Encoding States and Behavior
  as Types”][encoding]<!-- ignore --> section of Chapter 17.

If someone calls your code and passes in values that don’t make sense, it’s
best to return an error if you can so the user of the library can decide what
they want to do in that case. However, in cases where continuing could be
insecure or harmful, the best choice might be to call `panic!` and alert the
person using your library to the bug in their code so they can fix it during
development. Similarly, `panic!` is often appropriate if you’re calling
external code that is out of your control and it returns an invalid state that
you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a `Result`
than to make a `panic!` call. Examples include a parser being given malformed
data or an HTTP request returning a status that indicates you have hit a rate
limit. In these cases, returning a `Result` indicates that failure is an
expected possibility that the calling code must decide how to handle.

When your code performs an operation that could put a user at risk if it’s
called using invalid values, your code should verify the values are valid first
and panic if the values aren’t valid. This is mostly for safety reasons:
attempting to operate on invalid data can expose your code to vulnerabilities.
This is the main reason the standard library will call `panic!` if you attempt
an out-of-bounds memory access: trying to access memory that doesn’t belong to
the current data structure is a common security problem. Functions often have
*contracts*: their behavior is only guaranteed if the inputs meet particular
requirements. Panicking when the contract is violated makes sense because a
contract violation always indicates a caller-side bug and it’s not a kind of
error you want the calling code to have to explicitly handle. In fact, there’s
no reasonable way for calling code to recover; the calling *programmers* need
to fix the code. Contracts for a function, especially when a violation will
cause a panic, should be explained in the API documentation for the function.

However, having lots of error checks in all of your functions would be verbose
and annoying. Fortunately, you can use Rust’s type system (and thus the type
checking done by the compiler) to do many of the checks for you. If your
function has a particular type as a parameter, you can proceed with your code’s
logic knowing that the compiler has already ensured you have a valid value. For
example, if you have a type rather than an `Option`, your program expects to
have *something* rather than *nothing*. Your code then doesn’t have to handle
two cases for the `Some` and `None` variants: it will only have one case for
definitely having a value. Code trying to pass nothing to your function won’t
even compile, so your function doesn’t have to check for that case at runtime.
Another example is using an unsigned integer type such as `u32`, which ensures
the parameter is never negative.

### Creating Custom Types for Validation

Let’s take the idea of using Rust’s type system to ensure we have a valid value
one step further and look at creating a custom type for validation. Recall the
guessing game in Chapter 2 in which our code asked the user to guess a number
between 1 and 100. We never validated that the user’s guess was between those
numbers before checking it against our secret number; we only validated that
the guess was positive. In this case, the consequences were not very dire: our
output of “Too high” or “Too low” would still be correct. But it would be a
useful enhancement to guide the user toward valid guesses and have different
behavior when a user guesses a number that’s out of range versus when a user
types, for example, letters instead.

One way to do this would be to parse the guess as an `i32` instead of only a
`u32` to allow potentially negative numbers, and then add a check for the
number being in range, like so:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

The `if` expression checks whether our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that `guess` is
between 1 and 100.

However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, having a check like this in every function would be
tedious (and might impact performance).

Instead, we can make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere. That
way, it’s safe for functions to use the new type in their signatures and
confidently use the values they receive. Listing 9-13 shows one way to define a
`Guess` type that will only create an instance of `Guess` if the `new` function
receives a value between 1 and 100.

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file requires the `rand` crate. We do want to include it for reader
experimentation purposes, but don't want to include it for rustdoc testing
purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-13/src/main.rs:here}}
```

<span class="caption">Listing 9-13: A `Guess` type that will only continue with
values between 1 and 100</span>

First, we define a struct named `Guess` that has a field named `value` that
holds an `i32`. This is where the number will be stored.

Then we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `i32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it’s between 1 and 100.
If `value` doesn’t pass this test, we make a `panic!` call, which will alert
the programmer who is writing the calling code that they have a bug they need
to fix, because creating a `Guess` with a `value` outside this range would
violate the contract that `Guess::new` is relying on. The conditions in which
`Guess::new` might panic should be discussed in its public-facing API
documentation; we’ll cover documentation conventions indicating the possibility
of a `panic!` in the API documentation that you create in Chapter 14. If
`value` does pass the test, we create a new `Guess` with its `value` field set
to the `value` parameter and return the `Guess`.

Next, we implement a method named `value` that borrows `self`, doesn’t have any
other parameters, and returns an `i32`. This kind of method is sometimes called
a *getter*, because its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It’s important that the `value` field be private so code
using the `Guess` struct is not allowed to set `value` directly: code outside
the module *must* use the `Guess::new` function to create an instance of
`Guess`, thereby ensuring there’s no way for a `Guess` to have a `value` that
hasn’t been checked by the conditions in the `Guess::new` function.

A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than an
`i32` and wouldn’t need to do any additional checks in its body.

## Summary

Rust’s error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can’t handle and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust’s type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.

Now that you’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we’ll talk about how generics work and how you
can use them in your code.

[encoding]: ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
