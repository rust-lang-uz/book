## Traitlar: umumiy xatti-harakatni aniqlash

*trait* ma'lum bir turga ega bo'lgan va boshqa turlar bilan bo'lishishi mumkin bo'lgan funksionallikni belgilaydi. Biz umumiy xatti-harakatni mavhum tarzda aniqlash uchun traitlardan foydalanishimiz mumkin. Generik tur ma'lum xatti-harakatlarga ega bo'lgan har qanday tur bo'lishi mumkinligini aniqlash uchun *trait (bound)chegaralari* dan foydalanishimiz mumkin.

> Eslatma: Traitlar ba'zi farqlarga ega bo'lsa-da, ko'pincha boshqa tillarda
> *interfeyslar* deb ataladigan xususiyatga o'xshaydi.

### Traitni aniqlash

Turning xatti-harakati biz ushbu turga murojaat qilishimiz mumkin bo'lgan metodlardan iborat. Agar biz ushbu turlarning barchasida bir xil metodlarni chaqira olsak, har xil turlar bir xil xatti-harakatlarga ega. Trait ta'riflari - bu qandaydir maqsadga erishish uchun zarur bo'lgan xatti-harakatlar to'plamini aniqlash uchun metod imzolarini birgalikda guruhlash usuli.

Misol uchun, bizda turli xil va hajmdagi matnlarni o'z ichiga olgan bir nechta structlar mavjud deylik: ma'lum bir joyda joylashtirilgan yangiliklarni o'z ichiga olgan `YangiMaqola` structi va eng ko'pi 280 belgidan iborat bo'lishi mumkin bo'lgan `Maqola` yangi post, retpost yoki boshqa postga javob ekanligini ko'rsatadigan metama'lumotlar.

Biz `YangiMaqola` yoki `Maqola` misolida saqlanishi mumkin bo‘lgan ma’lumotlarning qisqacha mazmunini ko‘rsata oladigan `aggregator` nomli media agregator kutubxonasini yaratmoqchimiz. Buni amalga oshirish uchun bizga har bir tur bo'yicha xulosa kerak bo'ladi va biz ushbu xulosani misolda `umumiy_xulosa` metodini chaqirish orqali so'raymiz. 10-12 ro'yxatda ushbu xatti-harakatni ifodalovchi umumiy `Xulosa` traitining ta'rifi ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">Roʻyxat 10-12: `umumiy_xulosa` metodi bilan taʼminlangan xatti-harakatlardan iborat `Xulosa` traiti</span>

Bu yerda biz `trait` kalit so'zidan foydalanib traitni e'lon qilamiz, so'ngra belgi nomi, bu holda `Xulosa`. Shuningdek, biz ushbu traitni `pub` deb e’lon qildik, shunda bu cratega bog‘liq bo‘lgan cratelar ham bu traitdan foydalanishi mumkin, buni bir necha misollarda ko‘ramiz. Jingalak qavslar ichida biz ushbu traitni amalga oshiradigan turlarning xatti-harakatlarini tavsiflovchi metod imzolarini e'lon qilamiz, bu holda `fn umumiy_xulosa(&self) -> String`.

Metod imzosidan so'ng, jingalak qavslar ichida amalga oshirish o'rniga, biz nuqta-verguldan foydalanamiz. Ushbu traitni amalga oshiradigan har bir tur metod tanasi uchun o'ziga xos xatti-harakatni ta'minlashi kerak. Kompilyator `Xulosa` traitiga ega boʻlgan har qanday turda aynan shu imzo bilan aniqlangan `umumiy_xulosa` metodi boʻlishini talab qiladi.

Traitining tanasida bir nechta metodlar bo'lishi mumkin: metod imzolari har bir satrda bittadan ko'rsatilgan va har bir satr nuqtali vergul bilan tugaydi.

### Turga xos traitni amalga oshirish

Endi biz `Xulosa` traiti metodlarining kerakli imzolarini aniqlaganimizdan so‘ng, uni media agregatorimizdagi turlarga qo‘llashimiz mumkin. 10-13 roʻyxat sarlavhadan foydalanadigan `YangiMaqola` structidagi `Xulosa` traitining amalga oshirilishini koʻrsatadi, muallif va `umumiy_xulosa` qaytish qiymatini yaratish uchun joy. `Maqola` structi uchun biz `umumiy_xulosa`ni foydalanuvchi nomi va undan keyin maqolaning butun matni sifatida belgilaymiz, maqola mazmuni allaqachon 280 belgi bilan cheklangan deb hisoblaymiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">Roʻyxat 10-13: `Xulosa` traitini `YangiMaqola` va `Maqola` turlariga joriy qilish</span>

Traitni turga tatbiq etish odatiy usullarni amalga oshirishga o'xshaydi. Farqi shundaki, `impl` dan so'ng biz amalga oshirmoqchi bo'lgan trait nomini qo'yamiz, so'ng `for` kalit so'zidan foydalanamiz va keyin traitni amalga oshirmoqchi bo'lgan tur nomini belgilaymiz. `impl` blokida biz trait ta'rifi belgilagan metod imzolarini qo'yamiz. Har bir imzodan keyin nuqta-vergul qo'yish o'rniga, biz jingalak qavslardan foydalanamiz va metod tanasini o'ziga xos xatti-harakat bilan to'ldiramiz, biz traitning metodlari ma'lum bir turga ega bo'lishini xohlaymiz.

Kutubxona `YangiMaqola` va `Maqola`da `Xulosa` traitini joriy qilganligi sababli, crate foydalanuvchilari `YangiMaqola` va `Maqola` misollaridagi xususiyat metodlarini biz odatdagi metodlar deb ataganimizdek chaqirishlari mumkin. Yagona farq shundaki, foydalanuvchi o'ziga xos traitni turlari bilan bir qatorda qamrab olishi kerak. Binary crate bizning `aggregator` kutubxonamiz cratesidan qanday foydalanishi mumkinligiga misol:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Bu kod `1 ta yangi xabar: ismoilovdev: Rust kitobi juda foydali ekan, men juda ko'p bilimlarni o'zlashtirdim` chop etadi.

`aggregator` cratesiga bog'liq bo'lgan boshqa cratelar ham `Xulosa` traitini o'z turlari bo'yicha `Xulosa`ni amalga oshirish uchun qamrab olishi mumkin. E'tiborga olish kerak bo'lgan cheklashlardan biri shundaki, biz trait yoki turning hech bo'lmaganda bittasi bizning cratemiz uchun mahalliy(local) bo'lsa, biz traitni turga qo'llashimiz mumkin. Misol uchun, biz `Maqola` kabi maxsus turdagi `Display` kabi standart kutubxona traitlarini `aggregator` crate funksiyamizning bir qismi sifatida amalga oshirishimiz mumkin, chunki `Maqola` turi `aggregator` cratemiz uchun mahalliydir. Shuningdek, biz  `Vec<T>` da `Xulosa`ni `aggregator` cratemizda ham qo‘llashimiz mumkin, chunki `Xulosa` traiti `aggregator` cratemiz uchun mahalliydir.

Ammo biz tashqi turlarga tashqi traitlarni amalga oshira olmaymiz. Masalan, biz `aggregator` cratemiz ichida `Vec<T>` da `Display` traitini amalga oshira olmaymiz, chunki `Display` va `Vec<T>` ikkalasi ham standart kutubxonada belgilangan va bizning `aggregator` cratemiz uchun mahalliy emas. Bu cheklash *kogerentlik(coherence)* deb nomlangan xususiyatning bir qismi va aniqrog'i *yetim qoidasi(orphan rule)*, chunki ota-ona turi mavjud emasligi sababli shunday nomlangan. Bu qoida boshqa odamlarning kodi sizning kodingizni buzmasligini ta'minlaydi va aksincha. Qoidalarsiz ikkita crate bir xil turdagi bir xil traitni amalga oshirishi mumkin edi va Rust qaysi dasturdan foydalanishni bilmaydi.

### Standart ilovalar

Ba'zan har bir turdagi barcha metodlarni amalga oshirishni talab qilish o'rniga, traitdagi ba'zi yoki barcha metodlar uchun standart xatti-harakatlarga ega bo'lish foydali bo'ladi.
Keyin, biz traitni ma'lum bir turga qo'llaganimizda, har bir metodning standart xatti-harakatlarini saqlab qolishimiz yoki bekor qilishimiz mumkin.

Roʻyxat 10-14da biz 10-12 ro'yxatda bo'lgani kabi faqat metod imzosini belgilash o'rniga `Xulosa` traitining `umumiy_xulosa` metodi uchun standart qatorni belgilaymiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">Roʻyxat 10-14: `Xulosa` traitini `umumiy_xulosa` metodini standart boʻyicha amalga oshirish bilan aniqlash</span>

`YangiMaqola` misollarini umumlashtirish uchun standart ilovadan foydalanish uchun biz bo'sh `impl` blokini `impl Xulosa for YangiMaqola {}` bilan belgilaymiz.

Biz `YangiMaqola`da to‘g‘ridan-to‘g‘ri `umumiy_xulosa` metodini endi aniqlamasak ham, biz standart bo‘yicha dasturni taqdim etdik va `YangiMaqola` `Xulosa` traitini amalga oshirishini belgilab oldik. Natijada, biz hali ham `YangiMaqola` misolida `umumiy_xulosa` metodini quyidagicha chaqirishimiz mumkin:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Bu kod `Yangi maqola mavjud! (Batafsil...)`ni chop etadi.

Standart dasturni yaratish bizdan 10-13 roʻyxatdagi `Maqola`dagi `Xulosa`ni amalga oshirish haqida biror narsani oʻzgartirishimizni talab qilmaydi. Buning sababi, standart dasturni bekor qilish sintaksisi standart dasturga ega bo'lmagan trait metodini amalga oshirish sintaksisi bilan bir xil.

Standart ilovalar bir xil traitga ega bo'lgan boshqa metodlarni chaqirishi mumkin, hatto bu boshqa metodlarda standart dastur bo'lmasa ham. Shunday qilib, trait juda ko'p foydali funksiyalarni taqdim etishi mumkin va amalga oshiruvchilardan faqat uning kichik qismini ko'rsatishni talab qiladi. Misol uchun, biz `Xulosa` traitini amalga oshirish zarur bo'lgan `muallif_haqida` metodiga ega bo'lish uchun belgilashimiz va keyin `muallif_haqida` metodini chaqiradigan standart amalga oshirishga ega bo'lgan `umumiy_xulosa` metodini belgilashimiz mumkin:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

`Xulosa` ning ushbu versiyasidan foydalanish uchun biz faqat bir turdagi traitni amalga oshirganimizda `muallif_haqida` ni aniqlashimiz kerak:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

`muallif_haqida` ni aniqlaganimizdan so'ng, biz `Maqola` structi misollarida `umumiy_xulosa` deb atashimiz mumkin va `umumiy_xulosa` standart bajarilishi biz taqdim etgan `muallif_haqida` ta'rifini chaqiradi. Biz `muallif_haqida` ni qo'llaganimiz sababli, `Xulosa` traiti bizga boshqa kod yozishni talab qilmasdan `umumiy_xulosa` metodining harakatini berdi.

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Bu kod `1 ta yangi xabar: (Batafsil: @ismoilovdev...)` ni chop etadi.

Shuni esda tutingki, xuddi shu metodni bekor qilish orqali standart dasturni chaqirish mumkin emas.

### Traitlar parametr sifatida

Now that you know how to define and implement traits, we can explore how to use
traits to define functions that accept many different types. We'll use the
`Summary` trait we implemented on the `NewsArticle` and `Tweet` types in
Listing 10-13 to define a `notify` function that calls the `summarize` method
on its `item` parameter, which is of some type that implements the `Summary`
trait. To do this, we use the `impl Trait` syntax, like this:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

Instead of a concrete type for the `item` parameter, we specify the `impl`
keyword and the trait name. This parameter accepts any type that implements the
specified trait. In the body of `notify`, we can call any methods on `item`
that come from the `Summary` trait, such as `summarize`. We can call `notify`
and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the
function with any other type, such as a `String` or an `i32`, won’t compile
because those types don’t implement `Summary`.

<!-- Old headings. Do not remove or links may break. -->
<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax
sugar for a longer form known as a *trait bound*; it looks like this:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous section but is
more verbose. We place trait bounds with the declaration of the generic type
parameter after a colon and inside angle brackets.

The `impl Trait` syntax is convenient and makes for more concise code in simple
cases, while the fuller trait bound syntax can express more complexity in other
cases. For example, we can have two parameters that implement `Summary`. Doing
so with the `impl Trait` syntax looks like this:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Using `impl Trait` is appropriate if we want this function to allow `item1` and
`item2` to have different types (as long as both types implement `Summary`). If
we want to force both parameters to have the same type, however, we must use a
trait bound, like this:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

The generic type `T` specified as the type of the `item1` and `item2`
parameters constrains the function such that the concrete type of the value
passed as an argument for `item1` and `item2` must be the same.

#### Specifying Multiple Trait Bounds with the `+` Syntax

We can also specify more than one trait bound. Say we wanted `notify` to use
display formatting as well as `summarize` on `item`: we specify in the `notify`
definition that `item` must implement both `Display` and `Summary`. We can do
so using the `+` syntax:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

The `+` syntax is also valid with trait bounds on generic types:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

With the two trait bounds specified, the body of `notify` can call `summarize`
and use `{}` to format `item`.

#### Clearer Trait Bounds with `where` Clauses

Using too many trait bounds has its downsides. Each generic has its own trait
bounds, so functions with multiple generic type parameters can contain lots of
trait bound information between the function’s name and its parameter list,
making the function signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after the function
signature. So instead of writing this:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a `where` clause, like this:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

This function’s signature is less cluttered: the function name, parameter list,
and return type are close together, similar to a function without lots of trait
bounds.

### Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a
value of some type that implements a trait, as shown here:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

By using `impl Summary` for the return type, we specify that the
`returns_summarizable` function returns some type that implements the `Summary`
trait without naming the concrete type. In this case, `returns_summarizable`
returns a `Tweet`, but the code calling this function doesn’t need to know that.

The ability to specify a return type only by the trait it implements is
especially useful in the context of closures and iterators, which we cover in
Chapter 13. Closures and iterators create types that only the compiler knows or
types that are very long to specify. The `impl Trait` syntax lets you concisely
specify that a function returns some type that implements the `Iterator` trait
without needing to write out a very long type.

However, you can only use `impl Trait` if you’re returning a single type. For
example, this code that returns either a `NewsArticle` or a `Tweet` with the
return type specified as `impl Summary` wouldn’t work:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions
around how the `impl Trait` syntax is implemented in the compiler. We’ll cover
how to write a function with this behavior in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> section of Chapter 17.

### Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an `impl` block that uses generic type parameters,
we can implement methods conditionally for types that implement the specified
traits. For example, the type `Pair<T>` in Listing 10-15 always implements the
`new` function to return a new instance of `Pair<T>` (recall from the
[“Defining Methods”][methods]<!-- ignore --> section of Chapter 5 that `Self`
is a type alias for the type of the `impl` block, which in this case is
`Pair<T>`). But in the next `impl` block, `Pair<T>` only implements the
`cmp_display` method if its inner type `T` implements the `PartialOrd` trait
that enables comparison *and* the `Display` trait that enables printing.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

<span class="caption">Listing 10-15: Conditionally implementing methods on a
generic type depending on trait bounds</span>

We can also conditionally implement a trait for any type that implements
another trait. Implementations of a trait on any type that satisfies the trait
bounds are called *blanket implementations* and are extensively used in the
Rust standard library. For example, the standard library implements the
`ToString` trait on any type that implements the `Display` trait. The `impl`
block in the standard library looks similar to this code:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` trait on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this because integers implement `Display`:

```rust
let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the
“Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
correct behavior. In dynamically typed languages, we would get an error at
runtime if we called a method on a type which didn’t define the method. But Rust
moves these errors to compile time so we’re forced to fix the problems before
our code is even able to run. Additionally, we don’t have to write code that
checks for behavior at runtime because we’ve already checked at compile time.
Doing so improves performance without having to give up the flexibility of
generics.

[using-trait-objects-that-allow-for-values-of-different-types]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[methods]: ch05-03-method-syntax.html#defining-methods
