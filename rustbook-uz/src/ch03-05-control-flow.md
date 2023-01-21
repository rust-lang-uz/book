## Control Flow

Shartning `true` yoki yo'qligiga qarab ba'zi kodlarni ishga tushirish va shart `true` bo'lganda ba'zi kodlarni qayta-qayta ishga tushirish qobiliyati ko'pchilik dasturlash tillarida asosiy building bloklari hisoblanadi. Rust kodining bajarilishini nazorat qilish imkonini beruvchi eng keng tarqalgan konstruksiyalar `if` expressionlari va looplaridir.

### `if` ifodalari

`if` ifodasi shartlarga qarab kodingizni branchga ajratish imkonini beradi. Siz shartni taqdim etasiz va keyin shunday deb aytasiz: “Agar bu shart bajarilsa, ushbu kod blokini ishga tushiring. Agar shart bajarilmasa, ushbu kod blokini ishga tushirmang."

`If` ifodasini oʻrganish uchun *loyihalar* jildingizda *branchlar* nomli yangi loyiha yarating. *src/main.rs* faylida quyidagilarni kiriting:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Barcha `if` expressionlari `if` kalit so‘zidan boshlanadi, undan keyin shart keladi. Bunday holda, shart `raqam` o'zgaruvchisi 5 dan kichik qiymatga ega yoki yo'qligini tekshiradi. Agar shart `true` bo'lsa, biz kod blokini shartdan keyin darhol jingalak qavslar ichiga joylashtiramiz.
`if` expressionlaridagi shartlar bilan bog‘langan kod bloklari ba’zan *arms* deb ataladi, xuddi biz 2-bobning [“Tahminni maxfiy raqam bilan solishtirish”][comparing-the-guess-to-the-secret-number]<!--ignore --> bo‘limida muhokama qilgan `match` expressionlaridagi qurollar kabi.

Ixtiyoriy ravishda, agar shart `false` deb baholansa, dasturga bajarilishi uchun muqobil kod blokini berish uchun biz tanlagan `else` expressionini ham kiritishimiz mumkin. Agar `else` ifodasini bermasangiz va shart `false` bo‘lsa, dastur shunchaki `if` blokini o‘tkazib yuboradi va kodning keyingi bitiga o‘tadi.

Ushbu kodni ishga tushirishga harakat qiling; quyidagi chiqishni ko'rishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Keling, nima sodir bo'lishini ko'rish uchun `raqam` qiymatini shartni `false` qiladigan qiymatga o'zgartirib ko'raylik:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Dasturni qayta ishga tushiring va natijaga qarang:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Shuni ham ta'kidlash kerakki, ushbu koddagi shart `bool` bo'lishi kerak. Agar shart `bool` bo'lmasa, biz xatoga yo'l qo'yamiz. Masalan, quyidagi kodni ishga tushirishga harakat qiling:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

`if` sharti bu safar `3` qiymatiga teng bo'ladi va Rust xato qiladi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Xato shuni ko'rsatadiki, Rust `bool` kutgan, lekin integer(butun) son olgan. Ruby va JavaScript kabi tillardan farqli o'laroq, Rust boolean bo'lmagan turlarni boolean tilga o'zgartirishga avtomatik ravishda urinmaydi. Siz aniq bo'lishingiz va har doim `if` ni mantiqiy shart sifatida ko'rsatishingiz kerak. Agar biz `if` kod bloki faqat raqam `0` ga teng bo‘lmaganda ishlashini istasak, masalan, `if` ifodasini quyidagiga o‘zgartirishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Running this code will print `number was something other than zero`.

#### Handling Multiple Conditions with `else if`

You can use multiple conditions by combining `if` and `else` in an `else if`
expression. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Ushbu dasturda to'rtta yo'l bor. Uni ishga tushirgandan so'ng siz quyidagi chiqishni ko'rishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Ushbu dastur bajarilganda, u har bir `if` expressionni navbatma-navbat tekshiradi va shart `true` deb baholanadigan birinchi tanani bajaradi. E'tibor bering 6, 2 ga bo'linsa ham, biz `son 2 ga bo'linmaydi` chiqishini ko'rmayapmiz va `else` blokidagi `raqam 4, 3 yoki 2 ga bo'linmaydi` matnini ko'rmaymiz.Buning sababi, Rust faqat birinchi `true` shart uchun blokni bajaradi va bir marta topilsa, qolganlarini ham tekshirmaydi.
Juda ko'p `else if` expressionlaridan foydalanish kodingizni buzishi mumkin, shuning uchun sizda bir nechta bo'lsa, kodingizni qayta tahrirlashni xohlashingiz mumkin. 6-bobda bu holatlar uchun `match` deb nomlangan kuchli Rust tarmoqli konstruksiyasi tasvirlangan.

#### `let` statementida `if` dan foydalanish

`if` expression bo‘lganligi sababli, biz 3-2-listdagi kabi natijani o‘zgaruvchiga belgilash uchun `let` statementining o‘ng tomonida foydalanishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Ro'yxat 3-2: `if` expressioni natijasini o‘zgaruvchiga tayinlash</span>

`raqam` o'zgaruvchisi `if` expressioni natijasiga asoslangan qiymatga bog'lanadi. Nima sodir bo'lishini ko'rish uchun ushbu kodni ishga tushiring:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Esda tutingki, kod bloklari ulardagi oxirgi expressiongacha evaluate qilianadi va raqamlar o'zlari ham expressionlardir. Bu holda butun `if` expressionning qiymati qaysi kod bloki bajarilishiga bog'liq. Bu `if` ning har bir armidan result bo'lish potentsialiga ega bo'lgan qiymatlar bir xil turdagi bo'lishi kerakligini anglatadi; 3-2 ro'yxatda `if` va `else` armllarining natijalari `i32` butun sonlari edi. Agar turlar mos kelmasa(mismatched), quyidagi misolda bo'lgani kabi, biz xatoga duch kelamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Ushbu kodni kompilyatsiya qilmoqchi bo'lganimizda, biz xatoga duch kelamiz. `if` va `else` armllari mos kelmaydigan qiymat turlariga ega va Rust muammoni dasturda qayerdan topish mumkinligini aniq ko'rsatadi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` blokidagi expression butun songa, `else` blokidagi expression esa satrga baholanadi. Bu ishlamaydi, chunki oʻzgaruvchilar bitta turga ega boʻlishi kerak va Rust kompilyatsiya vaqtida `raqam` oʻzgaruvchisi qaysi turini aniq bilishi kerak. `raqam` turini bilish kompilyatorga ushbu tur biz `raqam` ishlatadigan hamma joyda haqiqiyligini tekshirish imkonini beradi. Agar `raqam` turi faqat runtimeda aniqlangan bo'lsa, Rust buni qila olmaydi; kompilyator murakkabroq bo'lar edi va agar u har qanday o'zgaruvchi uchun bir nechta gipotetik turlarni kuzatib borishi kerak bo'lsa, kod haqida kamroq kafolatlar beradi.

### Looplar bilan takrorlash

Ko'pincha kod blokini bir necha marta bajarish foydali bo'ladi. Ushbu vazifani bajarish uchun Rust bir nechta *looplarni* taqdim etadi, ular tsikl tanasi ichidagi kod orqali oxirigacha ishlaydi va keyin darhol boshida boshlanadi. Looplar bilan tajriba o'tkazish uchun keling, *looplar* deb nomlangan yangi loyiha yarataylik.

Rustda uch xil looplar mavjud: `loop`, `while` va `for`. Keling, har birini sinab ko'raylik.

#### Kodni `loop` bilan takrorlash

`loop` kalit so'zi Rustga kod blokini abadiy qayta-qayta bajarishni yoki uni to'xtatishni aniq aytmaguningizcha bajarishni aytadi.

Misol tariqasida, *looplar* jildingizdagi *src/main.rs* faylini quyidagicha o'zgartiring:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Ushbu dasturni ishga tushirganimizda, dasturni qo'lda to'xtatmagunimizcha, `yana!` so'zi doimiy ravishda chop etilishini ko'ramiz.Aksariyat terminallar uzluksiz siklda ishlab qolgan dasturni to'xtatish uchun <span class="keystroke">ctrl-c</span>  klaviatura yorliqlarini qo'llab-quvvatlaydi.
Sinab ko'ring:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/looplar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/looplar`
yana!
yana!
yana!
yana!
^Cyana!
```

`^C` belgisi <span class="keystroke">ctrl-c</span> tugmalarini bosgan joyni bildiradi. Kod uzilish signalini qabul qilganda siklning qayerda bo'lganiga qarab, `^C` dan keyin chop etilgan `yana!` so'zini ko'rishingiz yoki ko'rmasligingiz mumkin.

Yaxshiyamki, Rust kod yordamida loopdan chiqish yo'lini ham taqdim etadi. Siz dasturga siklni bajarishni qachon to'xtatish kerakligini aytish uchun `break` kalit so'zini siklga qo'yishingiz mumkin. 
Eslatib o'tamiz, biz buni 2-bobning [”To'g'ri taxmindan keyin chiqish”][quitting-after-a-correct-guess]<!-- ignore --> bo'limidagi taxminiy o'yinda, foydalanuvchi to'g'ri raqamni taxmin qilish orqali o'yinda g'alaba qozonganida dasturdan chiqish uchun qilganmiz.

Shuningdek, biz taxmin qilish o'yinida `continue` dan foydalandik, bu siklda dasturga siklning ushbu iteratsiyasida qolgan har qanday kodni o'tkazib yuborish va keyingi iteratsiyaga o'tishni aytadi.

#### Returning Values from Loops

One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job. You might also need to pass
the result of that operation out of the loop to the rest of your code. To do
this, you can add the value you want returned after the `break` expression you
use to stop the loop; that value will be returned out of the loop so you can
use it, as shown here:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Before the loop, we declare a variable named `counter` and initialize it to
`0`. Then we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add `1` to the `counter` variable,
and then check whether the `counter` is equal to `10`. When it is, we use the
`break` keyword with the value `counter * 2`. After the loop, we use a
semicolon to end the statement that assigns the value to `result`. Finally, we
print the value in `result`, which in this case is `20`.

#### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, `break` and `continue` apply to the innermost
loop at that point. You can optionally specify a *loop label* on a loop that
you can then use with `break` or `continue` to specify that those keywords
apply to the labeled loop instead of the innermost loop. Loop labels must begin
with a single quote. Here’s an example with two nested loops:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

The outer loop has the label `'counting_up`, and it will count up from 0 to 2.
The inner loop without a label counts down from 10 to 9. The first `break` that
doesn’t specify a label will exit the inner loop only. The `break
'counting_up;` statement will exit the outer loop. This code prints:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Conditional Loops with `while`

A program will often need to evaluate a condition within a loop. While the
condition is `true`, the loop runs. When the condition ceases to be `true`, the
program calls `break`, stopping the loop. It’s possible to implement behavior
like this using a combination of `loop`, `if`, `else`, and `break`; you could
try that now in a program, if you’d like. However, this pattern is so common
that Rust has a built-in language construct for it, called a `while` loop. In
Listing 3-3, we use `while` to loop the program three times, counting down each
time, and then, after the loop, print a message and exit.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Listing 3-3: Using a `while` loop to run code while a
condition holds true</span>

This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it’s clearer. While a condition
evaluates to `true`, the code runs; otherwise, it exits the loop.

#### Looping Through a Collection with `for`

You can choose to use the `while` construct to loop over the elements of a
collection, such as an array. For example, the loop in Listing 3-4 prints each
element in the array `a`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Listing 3-4: Looping through each element of a collection
using a `while` loop</span>

Here, the code counts up through the elements in the array. It starts at index
`0`, and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer `true`). Running this code will print every
element in the array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.

However, this approach is error prone; we could cause the program to panic if
the index value or test condition is incorrect. For example, if you changed the
definition of the `a` array to have four elements but forgot to update the
condition to `while index < 4`, the code would panic. It’s also slow, because
the compiler adds runtime code to perform the conditional check of whether the
index is within the bounds of the array on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like the code in Listing 3-5.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Listing 3-5: Looping through each element of a collection
using a `for` loop</span>

When we run this code, we’ll see the same output as in Listing 3-4. More
importantly, we’ve now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items.

Using the `for` loop, you wouldn’t need to remember to change any other code if
you changed the number of values in the array, as you would with the method
used in Listing 3-4.

The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, provided by the standard library, which generates
all numbers in sequence starting from one number and ending before another
number.

Here’s what the countdown would look like using a `for` loop and another method
we’ve not yet talked about, `rev`, to reverse the range:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

This code is a bit nicer, isn’t it?

## Summary

You made it! This was a sizable chapter: you learned about variables, scalar
and compound data types, functions, comments, `if` expressions, and loops! To
practice with the concepts discussed in this chapter, try building programs to
do the following:

* Convert temperatures between Fahrenheit and Celsius.
* Generate the *n*th Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
  taking advantage of the repetition in the song.

When you’re ready to move on, we’ll talk about a concept in Rust that *doesn’t*
commonly exist in other programming languages: ownership.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
