## Referencelarni lifetime bilan tekshirish

Lifetimelar - biz allaqachon uchratgan generiklarning yana bir turi. Turning biz xohlagan xatti-harakatga ega bo'lishini ta'minlash o'rniga, lifetime referencelar biz uchun kerak bo'lganda haqiqiyligini ta'minlaydi.

4-bobdagi [“Referencelar va Borrowing”]([references-and-borrowing]<!-- ignore -->) bo‘limida biz muhokama qilmagan bir tafsilot shundan iboratki, Rust-dagi har bir referenceda o‘sha referencening amal qilish doirasi *lifetime* bo‘ladi. Ko'pincha, lifetimelar yashirin va inferred bo'ladi,
ko'p hollarda bo'lgani kabi, turlar ham inferred qilinadi.Biz faqat bir nechta tur mumkin bo'lganda turlarga izoh berishimiz kerak. Shunga o'xshab, biz referencelarning lifetime bir necha xil yo'llar bilan bog'lanishi mumkin bo'lgan lifetimelarini izohlashimiz kerak. Rust bizdan runtimeda ishlatiladigan haqiqiy referencelar haqiqiy bo'lishini ta'minlash uchun generik lifetime parametrlaridan foydalangan holda munosabatlarga izoh berishimizni talab qiladi.

Lifetimeni izohlash boshqa dasturlash tillarining ko'pchiligida mavjud bo'lgan tushuncha ham emas, shuning uchun bu notanish tuyuladi. Garchi biz ushbu bobda lifetimeni to'liq qamrab olmasak ham, kontseptsiyadan qulay bo'lishingiz uchun lifetime sintaksisga duch kelishingiz mumkin bo'lgan umumiy usullarni muhokama qilamiz.

### Lifetimeda dangling referencelarni oldini olish

Lifetimening asosiy maqsadi dasturga reference qilish uchun mo'ljallangan ma'lumotlardan boshqa ma'lumotlarga reference qilishiga olib keladigan *dangling referencelar* ning oldini olishdir.
10-16 ro'yxatdagi dasturni ko'rib chiqing, uning tashqi va ichki ko'lami(tashqi va ichki ishlash doirasi) bor.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

<span class="caption">Ro'yxat 10-16: Qiymati ishlash doiradan chiqib ketgan referencedan foydalanishga urinish</span>

> Eslatma: 10-16, 10-17 va 10-23 ro'yxatlardagi misollar o'zgaruvchilarni
> ularga boshlang'ich qiymat bermasdan e'lon qiladi, shuning uchun o'zgaruvchi nomi
> tashqi doirada mavjud. Bir qarashda, bu Rustning null qiymatlari yo'qligiga zid
> bo'lib tuyulishi mumkin. Biroq, agar biz o'zgaruvchiga qiymat berishdan oldin
> foydalanmoqchi bo'lsak, biz kompilyatsiya vaqtida xatoga duch kelamiz, bu Rust
> haqiqatan ham null qiymatlarga ruxsat bermasligini ko'rsatadi.

Tashqi qamrov boshlang‘ich qiymati bo‘lmagan `r` nomli o‘zgaruvchini, ichki qamrov esa boshlang‘ich qiymati 5 bo‘lgan `x` nomli o‘zgaruvchini e’lon qiladi. Ichki doirada(qamrov) biz `x` ga reference sifatida `r` qiymatini belgilashga harakat qilamiz. Keyin ichki qamrov tugaydi va biz qiymatni `r` da chop etishga harakat qilamiz. Ushbu kod kompilyatsiya qilinmaydi, chunki biz undan foydalanishga urinishdan oldin `r` qiymati ko'rib chiqilmaydi. Mana xato xabari:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

`x` o'zgaruvchisi "yetarlicha uzoq umr ko'rmaydi". Sababi, 7-qatorda ichki qamrov tugashi bilan `x` amaldan tashqarida bo'ladi. Lekin `r` tashqi doira uchun hamon amal qiladi; uning qamrovi kengroq bo'lgani uchun biz uni "uzoq yashaydi" deymiz. Agar Rust ushbu kodning ishlashiga ruxsat bergan bo'lsa, `r` `x` doiradan chiqib ketganda ajratilgan xotiraga reference bo'ladi va biz `r` bilan qilishga uringan har qanday narsa to'g'ri ishlamaydi. Xo'sh, Rust bu kodning yaroqsizligini qanday aniqlaydi?
Bu borrow(qarz) tekshiruvidan foydalanadi.

### Borrow tekshiruvchisi

Rust kompilyatorida barcha borrowlar to'g'ri yoki yo'qligini aniqlash uchun ko'lamlarni taqqoslaydigan *borrow tekshiruvi(borrow checker)* mavjud. 10-17 ro'yxat 10-16 ro'yxati bilan bir xil kodni ko'rsatadi, ammo o'zgaruvchilarning lifetime(ishlash muddatini) ko'rsatadigan izohlar bilan.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

<span class="caption">Roʻyxat 10-17: `r` va `x` ning mos ravishda `a` va `b` nomlari bilan ishlash lifetimening izohlari</span>

Bu yerda biz `r`ning lifetimeni `a` bilan va `x`ning lifetimeni `b` bilan izohladik. Ko'rib turganingizdek, ichki `b` bloki tashqi `'a` lifetime blokdan ancha kichik. Kompilyatsiya vaqtida Rust ikki lifetimening o'lchamini solishtiradi va `r` ning lifetime `'a` ekanligini, lekin u `'b` lifetime(umr bo'yi) xotiraga ishora qilishini ko'radi. Dastur rad etildi, chunki `'b` `'a` dan qisqaroq: reference mavzusi reference kabi uzoq vaqt yashamaydi.

Ro'yxat 10-18 kodni tuzatadi, shuning uchun u dangling referencega ega emas va hech qanday xatosiz kompilyatsiya qilinadi.

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

<span class="caption">Ro'yxat 10-18: To'g'ri reference, chunki referencelar mos yozuvlardan ko'ra uzoqroq lifetimega ega</span>

Bu erda `x` `'b` muddatiga ega, bu holda `'a` dan kattaroqdir. Bu `r` `x` ga murojaat qilishi mumkin degan ma'noni anglatadi, chunki Rust `r` dagi reference har doim `x` amalda bo`lishini biladi.

Endi siz referencelarning amal qilish muddati qayerda ekanligini va referencelar har doim haqiqiy boʻlishini taʼminlash uchun Rust lifetimeni qanday tahlil qilishini bilganingizdan soʻng, keling, funksiyalar kontekstida parametrlarning generik lifetime va qiymatlarni qaytarishni koʻrib chiqaylik.

### Funksiyalarning generik lifetime

Biz ikkita satr bo'lagining uzunligini qaytaradigan funksiyani yozamiz. Bu funksiya ikkita satr bo'lagini oladi va bitta satr bo'lagini qaytaradi. `eng_uzun` funksiyasini amalga oshirganimizdan so'ng, 10-19 ro'yxatdagi kod `Eng uzun satr - abcd` ni chop etishi kerak.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

<span class="caption">Ro'yxat 10-19: Ikki qator boʻlagining uzunini topish uchun `eng_uzun` funksiyani chaqiruvchi `main` funksiya</span>

E'tibor bering, biz funksiya satrlarni emas, referencelar bo'lgan satr bo'laklarini olishni xohlaymiz, chunki biz `eng_uzun` funksiya uning parametrlariga egalik qilishni xohlamaymiz. 10 19 roʻyxatda biz foydalanadigan parametrlar nima uchun biz xohlagan parametrlar ekanligi haqida koʻproq muhokama qilish uchun 4-bobdagi [“String slicelari parametr sifatida”][string-slices-as-parameters]<!-- ignore --> boʻlimiga qarang.

Agar biz 10-20 ro'yxatda ko'rsatilganidek, `eng_uzun` funksiyasini amalga oshirishga harakat qilsak, u kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-20: Ikki qatorli boʻlakning uzunroq qismini qaytaradigan, lekin hali kompilyatsiya qilinmagan `eng_uzun` funksiyaning amalga oshirilishi</span>

Buning o'rniga biz lifetime haqida gapiradigan quyidagi xatoni olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Yordam matni shuni ko'rsatadiki, return(qaytarish) turiga umumiy lifetime parametri kerak, chunki Rust qaytarilayotgan reference `x` yoki `y` ga tegishli ekanligini aniqlay olmaydi. Aslida, biz ham bilmaymiz, chunki bu funksiyaning asosiy qismidagi `if` bloki `x` ga referenceni, `else` bloki esa `y` ga referenceni qaytaradi!

Ushbu funksiyani aniqlaganimizda, biz ushbu funksiyaga o'tadigan aniq qiymatlarni bilmaymiz, shuning uchun `if` yoki `else` ishi bajarilishini bilmaymiz. Shuningdek, biz uzatiladigan referencelarning aniq amal qilish muddatini bilmaymiz, shuning uchun biz qaytaradigan(return) lifetime har doim haqiqiy bo'lishini aniqlash uchun 10-17 va 10-18 ro'yxatlarda bo'lgani kabi qamrovni ko'rib chiqa olmaymiz. Borrow tekshiruvchisi buni ham aniqlay olmaydi, chunki u `x` va `y` ning ishlash lifetime qaytarilgan qiymatning lifetime(ishlash muddati) bilan qanday bog'liqligini bilmaydi. Ushbu xatoni tuzatish uchun biz referencelar o'rtasidagi munosabatni aniqlaydigan umumiy lifetime parametrlarini qo'shamiz, shunda borrow tekshiruvi tahlilini amalga oshirishi mumkin.

### Lifetime annotation sintaksisi

Lifetime annotationlar referencelarning qancha yashashini ko'rishini o'zgartirmaydi. Aksincha, ular lifetimega ta'sir qilmasdan, bir-biriga ko'plab murojaatlarning umrbod lifetimelar munosabatlarini tasvirlaydi. Imzo generik turdagi parametrni ko'rsatsa, funksiyalar har qanday turni qabul qilishi mumkin bo'lgani kabi, funksiyalar ham umumiy lifetime parametrini belgilash orqali har qanday xizmat muddati bilan murojaatlarni qabul qilishi mumkin.

Lifetime annotationlar biroz noodatiy sintaksisga ega: lifetime parametrlarining nomlari apostrof (`'`) bilan boshlanishi kerak va odatda generik turlar kabi kichik va juda qisqa bo'ladi. Ko'pchilik lifetime annotation birinchi izoh uchun `'a` nomidan foydalanadi. Annotationi reference turidan ajratish uchun boʻsh joydan foydalanib, biz lifetime parametr annotationlarini referencening `&` belgisidan keyin joylashtiramiz.

Mana bir nechta misollar: lifetime parametri bo'lmagan `i32` ga reference, `'a` nomli lifetime parametriga ega `i32` ga reference va lifetime `'a` bo'lgan `i32` ga o'zgaruvchan reference.

```rust,ignore
&i32        // reference
&'a i32     // aniq lifetimega ega bo'lgan reference
&'a mut i32 // aniq lifetimega ega o'zgaruvchan reference
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. Let’s examine how the lifetime annotations relate to each other in the context of the `longest` function.

### Lifetime Annotations in Function Signatures

To use lifetime annotations in function signatures, we need to declare the
generic *lifetime* parameters inside angle brackets between the function name
and the parameter list, just as we did with generic *type* parameters.

We want the signature to express the following constraint: the returned
reference will be valid as long as both the parameters are valid. This is the
relationship between lifetimes of the parameters and the return value. We’ll
name the lifetime `'a` and then add it to each reference, as shown in Listing
10-21.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<span class="caption">Listing 10-21: The `longest` function definition
specifying that all the references in the signature must have the same lifetime
`'a`</span>

This code should compile and produce the result we want when we use it with the
`main` function in Listing 10-19.

The function signature now tells Rust that for some lifetime `'a`, the function
takes two parameters, both of which are string slices that live at least as
long as lifetime `'a`. The function signature also tells Rust that the string
slice returned from the function will live at least as long as lifetime `'a`.
In practice, it means that the lifetime of the reference returned by the
`longest` function is the same as the smaller of the lifetimes of the values
referred to by the function arguments. These relationships are what we want
Rust to use when analyzing this code.

Remember, when we specify the lifetime parameters in this function signature,
we’re not changing the lifetimes of any values passed in or returned. Rather,
we’re specifying that the borrow checker should reject any values that don’t
adhere to these constraints. Note that the `longest` function doesn’t need to
know exactly how long `x` and `y` will live, only that some scope can be
substituted for `'a` that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function
signature, not in the function body. The lifetime annotations become part of
the contract of the function, much like the types in the signature. Having
function signatures contain the lifetime contract means the analysis the Rust
compiler does can be simpler. If there’s a problem with the way a function is
annotated or the way it is called, the compiler errors can point to the part of
our code and the constraints more precisely. If, instead, the Rust compiler
made more inferences about what we intended the relationships of the lifetimes
to be, the compiler might only be able to point to a use of our code many steps
away from the cause of the problem.

When we pass concrete references to `longest`, the concrete lifetime that is
substituted for `'a` is the part of the scope of `x` that overlaps with the
scope of `y`. In other words, the generic lifetime `'a` will get the concrete
lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because
we’ve annotated the returned reference with the same lifetime parameter `'a`,
the returned reference will also be valid for the length of the smaller of the
lifetimes of `x` and `y`.

Let’s look at how the lifetime annotations restrict the `longest` function by
passing in references that have different concrete lifetimes. Listing 10-22 is
a straightforward example.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<span class="caption">Listing 10-22: Using the `longest` function with
references to `String` values that have different concrete lifetimes</span>

In this example, `string1` is valid until the end of the outer scope, `string2`
is valid until the end of the inner scope, and `result` references something
that is valid until the end of the inner scope. Run this code, and you’ll see
that the borrow checker approves; it will compile and print `The longest string
is long string is long`.

Next, let’s try an example that shows that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We’ll move the
declaration of the `result` variable outside the inner scope but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Then we’ll move the `println!` that uses `result` to outside the
inner scope, after the inner scope has ended. The code in Listing 10-23 will
not compile.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<span class="caption">Listing 10-23: Attempting to use `result` after `string2`
has gone out of scope</span>

When we try to compile this code, we get this error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

The error shows that for `result` to be valid for the `println!` statement,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values using the same lifetime parameter `'a`.

As humans, we can look at this code and see that `string1` is longer than
`string2` and therefore `result` will contain a reference to `string1`.
Because `string1` has not gone out of scope yet, a reference to `string1` will
still be valid for the `println!` statement. However, the compiler can’t see
that the reference is valid in this case. We’ve told Rust that the lifetime of
the reference returned by the `longest` function is the same as the smaller of
the lifetimes of the references passed in. Therefore, the borrow checker
disallows the code in Listing 10-23 as possibly having an invalid reference.

Try designing more experiments that vary the values and lifetimes of the
references passed in to the `longest` function and how the returned reference
is used. Make hypotheses about whether or not your experiments will pass the
borrow checker before you compile; then check to see if you’re right!

### Thinking in Terms of Lifetimes

The way in which you need to specify lifetime parameters depends on what your
function is doing. For example, if we changed the implementation of the
`longest` function to always return the first parameter rather than the longest
string slice, we wouldn’t need to specify a lifetime on the `y` parameter. The
following code will compile:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

We’ve specified a lifetime parameter `'a` for the parameter `x` and the return
type, but not for the parameter `y`, because the lifetime of `y` does not have
any relationship with the lifetime of `x` or the return value.

When returning a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter for one of the parameters. If
the reference returned does *not* refer to one of the parameters, it must refer
to a value created within this function. However, this would be a dangling
reference because the value will go out of scope at the end of the function.
Consider this attempted implementation of the `longest` function that won’t
compile:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

Here, even though we’ve specified a lifetime parameter `'a` for the return
type, this implementation will fail to compile because the return value
lifetime is not related to the lifetime of the parameters at all. Here is the
error message we get:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

The problem is that `result` goes out of scope and gets cleaned up at the end
of the `longest` function. We’re also trying to return a reference to `result`
from the function. There is no way we can specify lifetime parameters that
would change the dangling reference, and Rust won’t let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so the calling function is then responsible for
cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various
parameters and return values of functions. Once they’re connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.

### Lifetime Annotations in Struct Definitions

So far, the structs we’ve defined all hold owned types. We can define structs to
hold references, but in that case we would need to add a lifetime annotation on
every reference in the struct’s definition. Listing 10-24 has a struct named
`ImportantExcerpt` that holds a string slice.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

<span class="caption">Listing 10-24: A struct that holds a reference, requiring
a lifetime annotation</span>

This struct has the single field `part` that holds a string slice, which is a
reference. As with generic data types, we declare the name of the generic
lifetime parameter inside angle brackets after the name of the struct so we can
use the lifetime parameter in the body of the struct definition. This
annotation means an instance of `ImportantExcerpt` can’t outlive the reference
it holds in its `part` field.

The `main` function here creates an instance of the `ImportantExcerpt` struct
that holds a reference to the first sentence of the `String` owned by the
variable `novel`. The data in `novel` exists before the `ImportantExcerpt`
instance is created. In addition, `novel` doesn’t go out of scope until after
the `ImportantExcerpt` goes out of scope, so the reference in the
`ImportantExcerpt` instance is valid.

### Lifetime Elision

You’ve learned that every reference has a lifetime and that you need to specify
lifetime parameters for functions or structs that use references. However, in
Chapter 4 we had a function in Listing 4-9, shown again in Listing 10-25, that
compiled without lifetime annotations.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

<span class="caption">Listing 10-25: A function we defined in Listing 4-9 that
compiled without lifetime annotations, even though the parameter and return
type are references</span>

The reason this function compiles without lifetime annotations is historical:
in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because
every reference needed an explicit lifetime. At that time, the function
signature would have been written like this:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

After writing a lot of Rust code, the Rust team found that Rust programmers
were entering the same lifetime annotations over and over in particular
situations. These situations were predictable and followed a few deterministic
patterns. The developers programmed these patterns into the compiler’s code so
the borrow checker could infer the lifetimes in these situations and wouldn’t
need explicit annotations.

This piece of Rust history is relevant because it’s possible that more
deterministic patterns will emerge and be added to the compiler. In the future,
even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the
*lifetime elision rules*. These aren’t rules for programmers to follow; they’re
a set of particular cases that the compiler will consider, and if your code
fits these cases, you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference. If Rust deterministically
applies the rules but there is still ambiguity as to what lifetimes the
references have, the compiler won’t guess what the lifetime of the remaining
references should be. Instead of guessing, the compiler will give you an error
that you can resolve by adding the lifetime annotations.

Lifetimes on function or method parameters are called *input lifetimes*, and
lifetimes on return values are called *output lifetimes*.

The compiler uses three rules to figure out the lifetimes of the references
when there aren’t explicit annotations. The first rule applies to input
lifetimes, and the second and third rules apply to output lifetimes. If the
compiler gets to the end of the three rules and there are still references for
which it can’t figure out lifetimes, the compiler will stop with an error.
These rules apply to `fn` definitions as well as `impl` blocks.

The first rule is that the compiler assigns a lifetime parameter to each
parameter that’s a reference. In other words, a function with one parameter gets
one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two
parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32,
y: &'b i32)`; and so on.

The second rule is that, if there is exactly one input lifetime parameter, that
lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32)
-> &'a i32`.

The third rule is that, if there are multiple input lifetime parameters, but
one of them is `&self` or `&mut self` because this is a method, the lifetime of
`self` is assigned to all output lifetime parameters. This third rule makes
methods much nicer to read and write because fewer symbols are necessary.

Let’s pretend we’re the compiler. We’ll apply these rules to figure out the
lifetimes of the references in the signature of the `first_word` function in
Listing 10-25. The signature starts without any lifetimes associated with the
references:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Then the compiler applies the first rule, which specifies that each parameter
gets its own lifetime. We’ll call it `'a` as usual, so now the signature is
this:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

The second rule applies because there is exactly one input lifetime. The second
rule specifies that the lifetime of the one input parameter gets assigned to
the output lifetime, so the signature is now this:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Now all the references in this function signature have lifetimes, and the
compiler can continue its analysis without needing the programmer to annotate
the lifetimes in this function signature.

Let’s look at another example, this time using the `longest` function that had
no lifetime parameters when we started working with it in Listing 10-20:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Let’s apply the first rule: each parameter gets its own lifetime. This time we
have two parameters instead of one, so we have two lifetimes:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

You can see that the second rule doesn’t apply because there is more than one
input lifetime. The third rule doesn’t apply either, because `longest` is a
function rather than a method, so none of the parameters are `self`. After
working through all three rules, we still haven’t figured out what the return
type’s lifetime is. This is why we got an error trying to compile the code in
Listing 10-20: the compiler worked through the lifetime elision rules but still
couldn’t figure out all the lifetimes of the references in the signature.

Because the third rule really only applies in method signatures, we’ll look at
lifetimes in that context next to see why the third rule means we don’t have to
annotate lifetimes in method signatures very often.

### Lifetime Annotations in Method Definitions

When we implement methods on a struct with lifetimes, we use the same syntax as
that of generic type parameters shown in Listing 10-11. Where we declare and
use the lifetime parameters depends on whether they’re related to the struct
fields or the method parameters and return values.

Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct’s name, because those lifetimes are part
of the struct’s type.

In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct’s fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren’t necessary in method signatures. Let’s look at some examples using the
struct named `ImportantExcerpt` that we defined in Listing 10-24.

First, we’ll use a method named `level` whose only parameter is a reference to
`self` and whose return value is an `i32`, which is not a reference to anything:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

The lifetime parameter declaration after `impl` and its use after the type name
are required, but we’re not required to annotate the lifetime of the reference
to `self` because of the first elision rule.

Here is an example where the third lifetime elision rule applies:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.

### The Static Lifetime

One special lifetime we need to discuss is `'static`, which denotes that the
affected reference *can* live for the entire duration of the program. All
string literals have the `'static` lifetime, which we can annotate as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the program’s binary, which
is always available. Therefore, the lifetime of all string literals is
`'static`.

You might see suggestions to use the `'static` lifetime in error messages. But
before specifying `'static` as the lifetime for a reference, think about
whether the reference you have actually lives the entire lifetime of your
program or not, and whether you want it to. Most of the time, an error message
suggesting the `'static` lifetime results from attempting to create a dangling
reference or a mismatch of the available lifetimes. In such cases, the solution
is fixing those problems, not specifying the `'static` lifetime.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

Let’s briefly look at the syntax of specifying generic type parameters, trait
bounds, and lifetimes all in one function!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

This is the `longest` function from Listing 10-21 that returns the longer of
two string slices. But now it has an extra parameter named `ann` of the generic
type `T`, which can be filled in by any type that implements the `Display`
trait as specified by the `where` clause. This extra parameter will be printed
using `{}`, which is why the `Display` trait bound is necessary. Because
lifetimes are a type of generic, the declarations of the lifetime parameter
`'a` and the generic type parameter `T` go in the same list inside the angle
brackets after the function name.

## Summary

We covered a lot in this chapter! Now that you know about generic type
parameters, traits and trait bounds, and generic lifetime parameters, you’re
ready to write code without repetition that works in many different situations.
Generic type parameters let you apply the code to different types. Traits and
trait bounds ensure that even though the types are generic, they’ll have the
behavior the code needs. You learned how to use lifetime annotations to ensure
that this flexible code won’t have any dangling references. And all of this
analysis happens at compile time, which doesn’t affect runtime performance!

Believe it or not, there is much more to learn on the topics we discussed in
this chapter: Chapter 17 discusses trait objects, which are another way to use
traits. There are also more complex scenarios involving lifetime annotations
that you will only need in very advanced scenarios; for those, you should read
the [Rust Reference][reference]. But next, you’ll learn how to write tests in
Rust so you can make sure your code is working the way it should.

[references-and-borrowing]:
ch04-02-references-and-borrowing.html#references-and-borrowing
[string-slices-as-parameters]:
ch04-03-slices.html#string-slices-as-parameters
[reference]: ../reference/index.html
