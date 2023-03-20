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

Endi siz traitlarni qanday aniqlash va amalga oshirishni bilganingizdan so'ng, biz ko'plab turlarni qabul qiladigan funksiyalarni aniqlash uchun traitlardan qanday foydalanishni o'rganishimiz mumkin. Biz 10-13 roʻyxatdagi `YangiMaqola` va `Maqola` turlari uchun joriy qilingan `Xulosa` traitidan foydalanamiz, uning `element` parametri boʻyicha umumlashtirish metodlini chaqiradigan `xabar_berish` funksiyasini belgilaymiz, u `Xulosa` traitini amalga oshiradi. Buning uchun biz `impl Trait` sintaksisidan foydalanamiz, masalan:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

`element` parametri uchun aniq tur o'rniga biz `impl` kalit so'zini va trait nomini belgilaymiz. Ushbu parametr belgilangan traitni amalga oshiradigan har qanday turni qabul qiladi. `xabar_berish` qismida biz `Xulosa` traitidan kelib chiqadigan `element` bo‘yicha har qanday metodlarni chaqirishimiz mumkin, masalan, `umumiy_xulosa`. Biz `xabar_berish` ga chaiqruv  qilishimiz va `YangiMaqola` yoki `Maqola` ning istalgan misolida o'tishimiz mumkin. Funksiyani `String` yoki `i32` kabi boshqa har qanday turdagi chaqiruvchi kod kompilyatsiya qilinmaydi, chunki bu turlar `Xulosa` ni amalga oshirmaydi.

<!-- Old headings. Do not remove or links may break. -->
<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Traitlarni cheklash sintaksisi

`impl Trait` sintaksisi oddiy holatlar uchun ishlaydi, lekin aslida *trait bound* deb nomlanuvchi uzunroq shakl uchun sintaksis shakaridir; bu shunday ko'rinadi:

```rust,ignore
pub fn xabar_berish<T: Xulosa>(element: &T) {
    println!("Tezkor xabarlar! {}", element.umumiy_xulosa());
}
```

Ushbu uzunroq shakl oldingi bo'limdagi misolga teng, ammo batafsilroq. Trait chegaralarini ikki nuqta va ichki burchakli qavslardan keyin umumiy tur parametri e'lon qilingan holda joylashtiramiz.

`impl Trait` sintaksisi qulay va oddiy holatlarda ixchamroq kodni yaratadi, to'liqroq traitlar bilan bog'langan sintaksisi esa boshqa holatlarda ko'proq murakkablikni ifodalashi mumkin. Misol uchun, bizda `Xulosa` ni amalga oshiradigan ikkita parametr bo'lishi mumkin. Buni `impl Trait` sintaksisi bilan bajarish quyidagicha ko'rinadi:

```rust,ignore
pub fn xabar_berish(element1: &impl Xulosa, element2: &impl Xulosa) {
```

Agar biz ushbu funksiya `element1` va `element2` turli xil turlarga ega bo'lishini istasak, `impl Trait` dan foydalanish maqsadga muvofiqdir (agar ikkala tur ham `Xulosa`ni qo'llasa). Agar biz ikkala parametrni bir xil turga ega bo'lishga majburlamoqchi bo'lsak, quyidagi kabi trait bounddan foydalanishimiz kerak:

```rust,ignore
pub fn xabar_berish<T: Xulosa>(element1: &T, element2: &T) {
```

`element1` va `element2` parametrlarining turi sifatida belgilangan umumiy `T` turi funksiyani shunday cheklaydiki, `element1` va `element2` uchun argument sifatida berilgan qiymatning aniq turi bir xil bo`lishi kerak.

#### `+` sintaksisi bilan bir nechta trait chegaralarini belgilash

Bundan tashqari, biz bir nechta traitlarni belgilashimiz mumkin. Aytaylik, biz `xabar_berish` funksiyasidan display formatlash hamda `element` bo‘yicha `umumiy_xulosa`dan foydalanishni xohladik: biz `xabar_berish` ta'rifida `element` `Display` va `Xulosa` ni ham amalga oshirishi kerakligini belgilaymiz. Buni `+` sintaksisi yordamida amalga oshirishimiz mumkin:

```rust,ignore
pub fn xabar_berish(element: &(impl Xulosa + Display)) {
```

`+` sintaksisi generik turdagi belgilar chegaralari bilan ham amal qiladi:

```rust,ignore
pub fn xabar_berish<T: Xulosa+ Display>(element: &T) {
```

Belgilangan ikkita trait chegarasi bilan `xabar_berish` asosiy qismi `umumiy_xulosa` deb chaqirishi va `element`ni formatlash uchun `{}` dan foydalanishi mumkin.

#### `where` bandlari bilan aniqroq trait bounds(chegaralari)

Haddan tashqari ko'p belgilar boundlaridan foydalanish o'zining salbiy tomonlariga ega. Har bir generikning o'ziga xos trait boundlari bor, shuning uchun bir nechta umumiy turdagi parametrlarga ega funksiyalar funksiya nomi va uning parametrlar ro'yxati o'rtasida ko'plab belgilar bilan bog'liq ma'lumotlarni o'z ichiga olishi mumkin, bu funksiya imzosini o'qishni qiyinlashtiradi. Shu sababli, Rust funksiya imzosidan keyin `where` bandida trait boundlarini belgilash uchun muqobil sintaksisga ega.

```rust,ignore
fn boshqa_funksiya<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

biz `where` bandidan foydalanishimiz mumkin, masalan:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

Bu funksiya imzosi kamroq chalkash: funksiya nomi, parametrlar ro'yxati va qaytish turi bir-biriga yaqin bo'lib, ko'p trait boundlari bo'lmagan funksiyaga o'xshaydi.

### Traitlarni amalga oshiradigan Return(qaytaruvchi) turlar

Bu yerda ko'rsatilganidek, traitni amalga oshiradigan ba'zi turdagi qiymatni qaytarish(return) uchun `impl Trait` sintaksisini return holatida ham ishlatishimiz mumkin:

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
