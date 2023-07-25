## Modullilikni va xatolarni boshqarishni yaxshilash uchun refaktoring

Dasturimizni yaxshilash uchun dastur tuzilishi va uning yuzaga kelishi mumkin bo'lgan xatolarni qanday hal qilishi bilan bog'liq bo'lgan to'rtta muammoni tuzatamiz. Birinchidan, bizning `main` funksiyamiz endi ikkita vazifani bajaradi: u argumentlarni tahlil qiladi va fayllarni o'qiydi. Dasturimiz o'sib borishi bilan `main` funksiya boshqaradigan alohida vazifalar soni ortadi. Funksiyaga mas'uliyat yuklagan sari, uning qismlaridan birini buzmasdan fikr yuritish, sinab ko'rish va o'zgartirish qiyinroq bo'ladi. Har bir funksiya bitta vazifa uchun javobgar bo'lishi uchun funksionallikni ajratish yaxshiroqdir.

Bu muammo ikkinchi muammo bilan ham bog'liq: `sorov` va `fayl_yoli` bizning dasturimiz uchun konfiguratsiya o'zgaruvchilari bo'lsa-da, dastur mantig'ini bajarish uchun `tarkib` kabi o'zgaruvchilardan foydalaniladi. `main` qancha uzun bo'lsa, biz ko'proq o'zgaruvchilarni qamrab olishimiz kerak bo'ladi; bizda qancha ko'p o'zgaruvchilar mavjud bo'lsa, ularning har birining maqsadini kuzatib borish shunchalik qiyin bo'ladi. Maqsadlari aniq bo'lishi uchun konfiguratsiya o'zgaruvchilarini bitta tuzilishga guruhlash yaxshidir.

Uchinchi muammo shundaki, biz faylni o‘qib chiqmaganda xato xabarini chop etish uchun `expect` tugmasidan foydalanganmiz, biroq xato xabari “Faylni o‘qishi kerak edi” degan yozuvni chiqaradi. Faylni o'qish bir necha usul bilan muvaffaqiyatsiz bo'lishi mumkin: masalan, fayl yetishmayotgan bo'lishi mumkin yoki bizda uni ochishga ruxsat yo'q.
Hozirda, vaziyatdan qat'i nazar, biz hamma narsa uchun bir xil xato xabarini chop qilamiz, bu esa foydalanuvchiga hech qanday ma'lumot bermaydi!

To‘rtinchidan, biz turli xil xatolarni qayta ishlash uchun `expect` dan qayta-qayta foydalanamiz va agar foydalanuvchi dasturimizni yetarlicha argumentlarni ko'rsatmasdan ishga tushirsa, Rustdan `index out of bounds`("chegaradan tashqari indeks") xatosini oladi va bu muammoni aniq tushuntirmaydi. Xatolarni qayta ishlash mantig'ini o'zgartirish kerak bo'lsa, kelajakdagi saqlovchilar(maintainerlar) kod bilan maslahatlashish uchun faqat bitta joyga ega bo'lishlari uchun barcha xatolarni qayta ishlash kodi bir joyda bo'lsa yaxshi bo'lar edi. Xatolarni qayta ishlash uchun barcha kodlar bir joyda bo'lsa, biz oxirgi foydalanuvchilarimiz uchun mazmunli bo'lgan xabarlarni chop etishimizni ta'minlaydi.

Keling, loyihamizni qayta tiklash orqali ushbu to'rtta muammoni hal qilaylik.

### Binary loyihalar uchun vazifalarni ajratish

Bir nechta vazifalar uchun javobgarlikni `main` funksiyaga taqsimlashning tashkiliy muammosi ko'plab ikkilik(binary) loyihalar uchun umumiydir. Natijada, Rust hamjamiyati `main` kattalasha boshlaganda ikkilik dasturning alohida muammolarini ajratish bo'yicha ko'rsatmalar ishlab chiqdi. Bu jarayon quyidagi bosqichlardan iborat:

* Dasturingizni *main.rs* va *lib.rs* ga bo'ling va dasturingiz mantig'ini *lib.rs* ga o'tkazing.

* Agar buyruq satrini tahlil qilish mantig'i kichik bo'lsa, u *main.rs* da qolishi mumkin.

* Buyruqlar qatorini tahlil qilish mantig'i murakkablasha boshlagach, uni *main.rs* dan chiqarib, *lib.rs* ga o'tkazing.

Ushbu jarayondan keyin `main` funksiyada qoladigan mas'uliyatlar quyidagilar bilan cheklanishi kerak:

* Argument qiymatlari bilan buyruq satrini tahlil qilish mantig'ini chaqirish
* Boshqa har qanday konfiguratsiyani sozlash
* *lib.rs* da `run` funksiyasini chaqirish
* `run` xatoni qaytarsa, xatoni hal qilish

Ushbu pattern vazifalarni ajratish bilan bog'liq: *main.rs* dasturni ishga tushirishni boshqaradi va *lib.rs* topshirilgan vazifaning barcha mantig'ini boshqaradi. `main` funksiyani toʻgʻridan-toʻgʻri test qilib koʻra olmasligingiz sababli, ushbu structura dasturingizning barcha mantig'ini *lib.rs* funksiyalariga koʻchirish orqali test qilib koʻrish imkonini beradi. *main.rs* da qolgan kod uni o'qish orqali uning to'g'riligini tekshirish uchun yetarlicha kichik bo'ladi. Keling, ushbu jarayonni kuzatib, dasturimizni qayta ishlaymiz.

#### Argument tahlilchisini(parser) chiqarish

Argumentlarni tahlil qilish(parsing qilish) funksiyasini `main` buyruq satrini tahlil qilish mantig'ini *src/lib.rs* ga ko'chirishga tayyorlash uchun chaqiradigan funksiyaga ajratamiz. Ro'yxat 12-5 `main` ning yangi boshlanishini ko'rsatadi, u `parse_config` yangi funksiyasini chaqiradi, biz buni hozircha *src/main.rs* da aniqlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-5: `main` dan `parse_config` funksiyasini chiqarish</span>

Biz hali ham buyruq qatori argumentlarini vectorga yig‘moqdamiz, lekin 1-indeksdagi argument qiymatini `sorov` o‘zgaruvchisiga va 2 indeksidagi argument qiymatini `main` funksiyasi ichidagi `fayl_yoli` o‘zgaruvchisiga belgilash o‘rniga, butun vectorni `parse_config` funksiyasiga o‘tkazamiz. Keyin `parse_config` funksiyasi qaysi argument qaysi o'zgaruvchiga kirishini aniqlaydigan mantiqni ushlab turadi va qiymatlarni `main`ga qaytaradi. Biz hali ham `sorov` va `fayl_yoli` o'zgaruvchilarini `main`da yaratamiz, lekin `main` endi buyruq qatori argumentlari va o'zgaruvchilari qanday mos kelishini aniqlash vazifasiga ega emas.

Ushbu qayta ishlash bizning kichik dasturimiz uchun ortiqcha bo'lib tuyulishi mumkin, ammo biz kichik, bosqichma-bosqich refactoring qilmoqdamiz. Ushbu o'zgartirishni amalga oshirgandan so'ng, argumentni tahlil qilish hali ham ishlayotganligini tekshirish uchun dasturni qayta ishga tushiring. Muammolar yuzaga kelganda sabablarini aniqlashga yordam berish uchun taraqqiyotingizni tez-tez tekshirib turish yaxshidir.

#### Konfiguratsiya qiymatlarini guruhlash

`parse_config` funksiyasini yanada yaxshilash uchun yana bir kichik qadam tashlashimiz mumkin.
Ayni paytda biz tupleni qaytarmoqdamiz, lekin keyin darhol bu tupleni yana alohida qismlarga ajratamiz. Bu, ehtimol, bizda hali to'g'ri mavhumlik yo'qligining belgisidir.

Yaxshilash uchun joy borligini ko'rsatadigan yana bir ko'rsatkich `parse_config` ning `config` qismidir, bu biz qaytaradigan ikkita qiymat bir-biriga bog'liqligini va ikkalasi ham bitta konfiguratsiya qiymatining bir qismi ekanligini anglatadi. Biz hozirda bu mantiqni ma'lumotlar strukturasida yetkazmayapmiz, bundan tashqari ikkita qiymatni tuplega guruhlash; Buning o'rniga biz ikkita qiymatni bitta strukturaga joylashtiramiz va har bir struktura maydoniga mazmunli nom beramiz. Buni qilish ushbu kodning kelajakdagi saqlovchilariga(maintainerlarga) turli qadriyatlar bir-biriga qanday bog'liqligini va ularning maqsadi nima ekanligini tushunishni osonlashtiradi.

12-6 ro'yxatda `parse_config` funksiyasining yaxshilanishi ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-6: `Config` strukturasining namunasini qaytarish uchun `parse_config` ni qayta tahrirlash</span>

Biz `sorov` va `fayl_yoli` nomli maydonlarga ega bo'lishi uchun aniqlangan `Config` nomli structi qo'shdik. Endi `parse_config` signaturesi `Config` qiymatini qaytarishini bildiradi. Biz `args`dagi `String` qiymatlariga reference qilingan satr bo‘laklarini qaytargan `parse_config` korpusida endi `Config` ga tegishli `String` qiymatlarini o‘z ichiga olgan holda belgilaymiz. `main`dagi `args` oʻzgaruvchisi argument qiymatlarining owneri(ega) boʻlib, faqat `parse_config` funksiyasiga ularni borrowga(qarz olish) ruxsat beradi, yaʼni `Config` `args` qiymatlariga ownership(egalik) qilmoqchi boʻlsa, Rustning borrowing(qarz olish) qoidalarini buzgan boʻlamiz.

`String` ma'lumotlarini boshqarishning bir qancha usullari mavjud; Eng oson, garchi unchalik samarasiz bo'lsa ham, route qiymatlar bo'yicha `clone` metodini chaqirishdir.
Bu `Config` nusxasi uchun ma'lumotlarning to'liq nusxasini oladi, bu esa satr(string) ma'lumotlariga referenceni saqlashdan ko'ra ko'proq vaqt va xotirani oladi. Biroq, ma'lumotlarni klonlash bizning kodimizni juda sodda qiladi, chunki biz referencelarning lifetimeni(ishlash muddati) boshqarishimiz shart emas; bu holatda, soddalikka erishish uchun ozgina ishlashdan voz kechish foydali savdodir.

> ### `clone` dan foydalanishning o'zaro kelishuvlari
>
> Ko'pgina Rustaceanlar orasida `clone` dan foydalanish vaqti xarajati tufayli ownership
> muammolarini hal qilish uchun foydalanmaslik tendentsiyasi mavjud.
> [13-bobda][ch13]<!-- ignore --> siz ushbu turdagi vaziyatda samaraliroq
> usullardan qanday foydalanishni o'rganasiz. Ammo hozircha rivojlanishni
> davom ettirish uchun bir nechta satrlarni nusxalash ma'qul, chunki siz bu nusxalarni
> faqat bir marta qilasiz va fayl yo'li va so'rovlar qatori juda kichik. Birinchi o'tishda
> kodni giperoptimallashtirishga urinishdan ko'ra, biroz samarasiz ishlaydigan dasturga
> ega bo'lish yaxshiroqdir. Rust bilan tajribangiz ortgan sayin, eng samarali
> yechimdan boshlash osonroq bo'ladi, ammo hozircha `clone` deb
> nomlash juda maqbuldir.

Biz `main`ni yangiladik, shuning uchun u `parse_config` tomonidan qaytarilgan `Config` namunasini `config` nomli o‘zgaruvchiga joylashtiradi va biz avval alohida `sorov` va `fayl_yoli` o‘zgaruvchilaridan foydalangan kodni yangiladik, shuning uchun u endi `Config` strukturasidagi maydonlardan foydalanadi.

Endi bizning kodimiz `sorov` va `fayl_yoli` bir-biriga bog'liqligini va ularning maqsadi dastur qanday ishlashini sozlash ekanligini aniqroq bildiradi. Ushbu qiymatlardan foydalanadigan har qanday kod ularni maqsadlari uchun nomlangan maydonlardagi `config` misolida topishni biladi.

#### `Config` uchun konstruktor yaratish

Hozircha biz `main` dan buyruq qatori argumentlarini tahlil qilish uchun javob beradigan mantiqni chiqarib oldik va uni `parse_config` funksiyasiga joylashtirdik. Bu bizga `sorov` va `fayl_yoli` qiymatlari o'zaro bog'liqligini va bu munosabatlar bizning kodimizda ko'rsatilishi kerakligini ko'rishga yordam berdi. Keyin biz `sorov` va `fayl_yoli` ning tegishli maqsadini nomlash va `parse_config` funksiyasidan qiymatlar nomlarini stuct maydoni nomi sifatida qaytarish uchun `Config` structini qo'shdik.

Endi `parse_config` funksiyasining maqsadi `Config` misolini yaratish bo‘lganligi sababli, biz `parse_config` ni oddiy funksiyadan `Config` structi bilan bog'langan `new` funksiyaga o‘zgartirishimiz mumkin. Ushbu o'zgarish kodni yanada idiomatik qiladi. Biz standart kutubxonada `String` kabi turlarning namunalarini `String::new` ni chaqirish orqali yaratishimiz mumkin. Xuddi shunday, `parse_config`ni `Config` bilan bog‘langan `new` funksiyaga o‘zgartirib, `Config::new` ni chaqirish orqali `Config` misollarini yaratishimiz mumkin bo‘ladi. 12-7 ro'yxat biz qilishimiz kerak bo'lgan o'zgarishlarni ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-7: `parse_config` ni `Config::new` ga o'zgartirish</span>

Biz `parse_config` deb chaqirgan `main`ni yangilab, `Config::new` deb chaqirdik. Biz `parse_config` nomini `new` ga o‘zgartirdik va uni `new` funksiyani `Config` bilan bog‘laydigan `impl` blokiga o‘tkazdik. Ishlayotganiga ishonch hosil qilish uchun ushbu kodni qayta kompilyatsiya qilib ko'ring.

### Qayta ishlash xatolarini tuzatish

Endi biz xatolarimizni tuzatish ustida ishlaymiz. Eslatib o'tamiz, `args` vectoridagi qiymatlarga 1 yoki indeks 2 da kirishga urinish vector uchtadan kam elementni o'z ichiga olgan bo'lsa, dastur panic paydo bo'ladi. Dasturni hech qanday argumentlarsiz ishga tushirishga harakat qiling; u shunday ko'rinadi:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

The line `index out of bounds: the len is 1 but the index is 1` is an error
message intended for programmers. It won’t help our end users understand what
they should do instead. Let’s fix that now.

#### Improving the Error Message

In Listing 12-8, we add a check in the `new` function that will verify that the
slice is long enough before accessing index 1 and 2. If the slice isn’t long
enough, the program panics and displays a better error message.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

<span class="caption">Listing 12-8: Adding a check for the number of
arguments</span>

This code is similar to [the `Guess::new` function we wrote in Listing
9-13][ch9-custom-types]<!-- ignore -->, where we called `panic!` when the
`value` argument was out of the range of valid values. Instead of checking for
a range of values here, we’re checking that the length of `args` is at least 3
and the rest of the function can operate under the assumption that this
condition has been met. If `args` has fewer than three items, this condition
will be true, and we call the `panic!` macro to end the program immediately.

With these extra few lines of code in `new`, let’s run the program without any
arguments again to see what the error looks like now:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

This output is better: we now have a reasonable error message. However, we also
have extraneous information we don’t want to give to our users. Perhaps using
the technique we used in Listing 9-13 isn’t the best to use here: a call to
`panic!` is more appropriate for a programming problem than a usage problem,
[as discussed in Chapter 9][ch9-error-guidelines]<!-- ignore -->. Instead,
we’ll use the other technique you learned about in Chapter 9—[returning a
`Result`][ch9-result]<!-- ignore --> that indicates either success or an error.

<!-- Old headings. Do not remove or links may break. -->
<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### Returning a `Result` Instead of Calling `panic!`

We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. We’re also
going to change the function name from `new` to `build` because many
programmers expect `new` functions to never fail. When `Config::build` is
communicating to `main`, we can use the `Result` type to signal there was a
problem. Then we can change `main` to convert an `Err` variant into a more
practical error for our users without the surrounding text about `thread
'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.

Listing 12-9 shows the changes we need to make to the return value of the
function we’re now calling `Config::build` and the body of the function needed
to return a `Result`. Note that this won’t compile until we update `main` as
well, which we’ll do in the next listing.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<span class="caption">Listing 12-9: Returning a `Result` from
`Config::build`</span>

Our `build` function returns a `Result` with a `Config` instance in the success
case and a `&'static str` in the error case. Our error values will always be
string literals that have the `'static` lifetime.

We’ve made two changes in the body of the function: instead of calling `panic!`
when the user doesn’t pass enough arguments, we now return an `Err` value, and
we’ve wrapped the `Config` return value in an `Ok`. These changes make the
function conform to its new type signature.

Returning an `Err` value from `Config::build` allows the `main` function to
handle the `Result` value returned from the `build` function and exit the
process more cleanly in the error case.

<!-- Old headings. Do not remove or links may break. -->
<a id="calling-confignew-and-handling-errors"></a>

#### Calling `Config::build` and Handling Errors

To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::build`, as shown in
Listing 12-10. We’ll also take the responsibility of exiting the command line
tool with a nonzero error code away from `panic!` and instead implement it by
hand. A nonzero exit status is a convention to signal to the process that
called our program that the program exited with an error state.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<span class="caption">Listing 12-10: Exiting with an error code if building a
`Config` fails</span>

In this listing, we’ve used a method we haven’t covered in detail yet:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method’s behavior is similar
to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value
is an `Err` value, this method calls the code in the *closure*, which is an
anonymous function we define and pass as an argument to `unwrap_or_else`. We’ll
cover closures in more detail in [Chapter 13][ch13]<!-- ignore -->. For now,
you just need to know that `unwrap_or_else` will pass the inner value of the
`Err`, which in this case is the static string `"not enough arguments"` that we
added in Listing 12-9, to our closure in the argument `err` that appears
between the vertical pipes. The code in the closure can then use the `err`
value when it runs.

We’ve added a new `use` line to bring `process` from the standard library into
scope. The code in the closure that will be run in the error case is only two
lines: we print the `err` value and then call `process::exit`. The
`process::exit` function will stop the program immediately and return the
number that was passed as the exit status code. This is similar to the
`panic!`-based handling we used in Listing 12-8, but we no longer get all the
extra output. Let’s try it:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Great! This output is much friendlier for our users.

### Extracting Logic from `main`

Now that we’ve finished refactoring the configuration parsing, let’s turn to
the program’s logic. As we stated in [“Separation of Concerns for Binary
Projects”](#separation-of-concerns-for-binary-projects)<!-- ignore -->, we’ll
extract a function named `run` that will hold all the logic currently in the
`main` function that isn’t involved with setting up configuration or handling
errors. When we’re done, `main` will be concise and easy to verify by
inspection, and we’ll be able to write tests for all the other logic.

Listing 12-11 shows the extracted `run` function. For now, we’re just making
the small, incremental improvement of extracting the function. We’re still
defining the function in *src/main.rs*.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<span class="caption">Listing 12-11: Extracting a `run` function containing the
rest of the program logic</span>

The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.

#### Returning Errors from the `run` Function

With the remaining program logic separated into the `run` function, we can
improve the error handling, as we did with `Config::build` in Listing 12-9.
Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate the logic around handling errors into `main` in a
user-friendly way. Listing 12-12 shows the changes we need to make to the
signature and body of `run`.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<span class="caption">Listing 12-12: Changing the `run` function to return
`Result`</span>

We’ve made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<dyn Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.

For the error type, we used the *trait object* `Box<dyn Error>` (and we’ve
brought `std::error::Error` into scope with a `use` statement at the top).
We’ll cover trait objects in [Chapter 17][ch17]<!-- ignore -->. For now, just
know that `Box<dyn Error>` means the function will return a type that
implements the `Error` trait, but we don’t have to specify what particular type
the return value will be. This gives us flexibility to return error values that
may be of different types in different error cases. The `dyn` keyword is short
for “dynamic.”

Second, we’ve removed the call to `expect` in favor of the `?` operator, as we
talked about in [Chapter 9][ch9-question-mark]<!-- ignore -->. Rather than
`panic!` on an error, `?` will return the error value from the current function
for the caller to handle.

Third, the `run` function now returns an `Ok` value in the success case.
We’ve declared the `run` function’s success type as `()` in the signature,
which means we need to wrap the unit type value in the `Ok` value. This
`Ok(())` syntax might look a bit strange at first, but using `()` like this is
the idiomatic way to indicate that we’re calling `run` for its side effects
only; it doesn’t return a value we need.

When you run this code, it will compile but will display a warning:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust tells us that our code ignored the `Result` value and the `Result` value
might indicate that an error occurred. But we’re not checking to see whether or
not there was an error, and the compiler reminds us that we probably meant to
have some error-handling code here! Let’s rectify that problem now.

#### Handling Errors Returned from `run` in `main`

We’ll check for errors and handle them using a technique similar to one we used
with `Config::build` in Listing 12-10, but with a slight difference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

We use `if let` rather than `unwrap_or_else` to check whether `run` returns an
`Err` value and call `process::exit(1)` if it does. The `run` function doesn’t
return a value that we want to `unwrap` in the same way that `Config::build`
returns the `Config` instance. Because `run` returns `()` in the success case,
we only care about detecting an error, so we don’t need `unwrap_or_else` to
return the unwrapped value, which would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases: we print the error and exit.

### Splitting Code into a Library Crate

Our `minigrep` project is looking good so far! Now we’ll split the
*src/main.rs* file and put some code into the *src/lib.rs* file. That way we
can test the code and have a *src/main.rs* file with fewer responsibilities.

Let’s move all the code that isn’t the `main` function from *src/main.rs* to
*src/lib.rs*:

* The `run` function definition
* The relevant `use` statements
* The definition of `Config`
* The `Config::build` function definition

The contents of *src/lib.rs* should have the signatures shown in Listing 12-13
(we’ve omitted the bodies of the functions for brevity). Note that this won’t
compile until we modify *src/main.rs* in Listing 12-14.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

<span class="caption">Listing 12-13: Moving `Config` and `run` into
*src/lib.rs*</span>

We’ve made liberal use of the `pub` keyword: on `Config`, on its fields and its
`build` method, and on the `run` function. We now have a library crate that has
a public API we can test!

Now we need to bring the code we moved to *src/lib.rs* into the scope of the
binary crate in *src/main.rs*, as shown in Listing 12-14.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<span class="caption">Listing 12-14: Using the `minigrep` library crate in
*src/main.rs*</span>

We add a `use minigrep::Config` line to bring the `Config` type from the
library crate into the binary crate’s scope, and we prefix the `run` function
with our crate name. Now all the functionality should be connected and should
work. Run the program with `cargo run` and make sure everything works
correctly.

Whew! That was a lot of work, but we’ve set ourselves up for success in the
future. Now it’s much easier to handle errors, and we’ve made the code more
modular. Almost all of our work will be done in *src/lib.rs* from here on out.

Let’s take advantage of this newfound modularity by doing something that would
have been difficult with the old code but is easy with the new code: we’ll
write some tests!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
