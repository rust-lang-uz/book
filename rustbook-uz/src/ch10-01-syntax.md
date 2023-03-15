## Generik ma'lumotlar turlari

Funksiya imzolari yoki structlar kabi elementlar uchun definitionlarni yaratish uchun biz generik(umumiy) ma'lumotlardan foydalanamiz, keyin ularni turli xil aniq ma'lumotlar turlari bilan ishlatishimiz mumkin. Keling, avval generiklar yordamida funksiyalar, structlar, enumlar va metodlarni qanday aniqlashni ko'rib chiqaylik. Keyin biz generiklar kod ishlashiga qanday ta'sir qilishini muhokama qilamiz.

### Funksiya ta'riflarida

Generiklardan foydalanadigan funksiyani belgilashda biz generiklarni funksiya imzosiga joylashtiramiz, u yerda biz odatda parametrlarning ma'lumotlar turlarini va qiymatni qaytaramiz. Bu bizning kodimizni yanada moslashuvchan qiladi va kodning takrorlanishining oldini olish bilan birga funksiyamizni chaqiruvchilarga ko'proq funksionallik beradi.

`eng_katta` funksiyamizni davom ettirsak, 10-4 roʻyxatda ikkalasi ham boʻlakdagi eng katta qiymatni topadigan ikkita funksiya koʻrsatilgan. Keyin biz ularni generiklardan foydalanadigan yagona funksiyaga birlashtiramiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">Roʻyxat 10-4: Ikki funksiya faqat nomlari va imzolaridagi turlari bilan farqlanadi</span>

`eng_katta_i32` funksiyasi biz 10-3 roʻyxatda ajratib olingan funksiya boʻlib, u boʻlakdagi eng katta `i32`ni topadi. `eng_katta_char` funksiyasi bo‘lakdagi eng katta `char`ni topadi. Funksiya organlari bir xil kodga ega, shuning uchun bitta funksiyaga umumiy turdagi parametrni kiritish orqali takrorlanishni bartaraf qilaylik.

Yangi bitta funksiyada turlarni parametrlash uchun, biz funksiyaning qiymat parametrlari uchun qilganimiz kabi, tur parametrini nomlashimiz kerak. Tur parametri nomi sifatida istalgan identifikatordan foydalanishingiz mumkin. Lekin biz `T` dan foydalanamiz, chunki Rust-dagi parametr nomlari odatda qisqa, koʻpincha harfdan iborat boʻladi va Rustning tur nomlash konventsiyasi UpperCamelCase hisoblanadi. “type(tur)” so'zining qisqartmasi `T`, Rust dasturchilarining ko'pchiligining standart tanlovidir.

Funksiya tanasida parametrdan foydalanganda, biz imzoda parametr nomini e'lon qilishimiz kerak, shunda kompilyator bu nom nimani anglatishini biladi.
Xuddi shunday, biz funktsiya imzosida tup parametri nomini ishlatganimizda, uni ishlatishdan oldin parametr nomini e'lon qilishimiz kerak. Generik `eng_katta` funksiyani aniqlash uchun burchakli qavslar ichida `<>` nomi deklaratsiyasini funksiya nomi va parametrlar ro'yxati orasiga qo'ying, masalan:

```rust,ignore
fn eng_katta<T>(list: &[T]) -> &T {
```

Biz bu taʼrifni shunday oʻqiymiz: `eng_katta` funksiyasi `T` turiga nisbatan umumiydir. Bu funksiya `list` nomli bitta parametrga ega, bu `T` turidagi qiymatlar bo'lagidir. `eng_katta` funksiya bir xil turdagi `T` qiymatiga referenceni qaytaradi.

10-5 ro'yxatda imzodagi umumiy ma'lumotlar turidan foydalangan holda birlashtirilgan `eng_katta` funksiya ta'rifi ko'rsatilgan. list shuningdek, funktsiyani `i32` yoki `char` qiymatlari bilan qanday chaqirishimiz mumkinligini ko'rsatadi. E'tibor bering, bu kod hali kompilyatsiya qilinmaydi, ammo biz uni ushbu bobda keyinroq tuzatamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">Ro'yxat 10-5: Umumiy turdagi parametrlardan foydalangan holda `eng_katta` funksiya; bu hali kompilyatsiya qilinmagan</span>

Agar dasturni hozir kompilyatsiya qilsak, biz quyidagi xatolikni olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

Yordam matnida `std::cmp::PartialOrd` qayd etilgan, bu *trait* va biz keyingi bo'limda traitlar haqida gaplashamiz. Hozircha shuni bilingki, bu xato `eng_katta` tanasi `T` bo'lishi mumkin bo'lgan barcha mumkin bo'lgan turlar uchun ishlamasligini bildiradi. Kod tanasidagi `T` turidagi qiymatlarni solishtirmoqchi bo'lganimiz uchun biz faqat qiymatlari ordere qilinadigan turlardan foydalanishimiz mumkin. Taqqoslashni yoqish uchun standart kutubxona `std::cmp::PartialOrd` traitiga ega bo'lib, uni turlarga tatbiq etishingiz mumkin (bu trait haqida batafsil ma'lumot uchun C ilovasiga qarang). Yordam matnining taklifiga amal qilib, biz `T` uchun amal qiladigan turlarni faqat `PartialOrd`-ni qo'llaydiganlar bilan cheklaymiz va bu misol kompilyatsiya qilinadi, chunki standart kutubxona `PartialOrd`ni ham `i32` va `char` da qo'llaydi.

### Struktura Definitionlarida

Shuningdek, biz `<>` sintaksisi yordamida bir yoki bir nechta maydonlarda umumiy turdagi parametrlardan foydalanish uchun structlarni belgilashimiz mumkin. Ro'yxat 10-6 har qanday turdagi `x` va `y` koordinata qiymatlarini saqlash uchun `Point<T>` structni belgilaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">10-6 roʻyxat: `T` turidagi `x` va `y` qiymatlarini oʻz ichiga olgan `Point<T>` structi</span>

Struktura ta'riflarida generiklardan foydalanish sintaksisi funksiya ta'riflarida qo'llaniladigan sintaksisiga juda oʻxshaydi. Birinchidan, burchakli qavslar ichida strukturaning nomidan keyin tur parametrining nomini e'lon qilamiz. Keyin biz aniq ma'lumotlar turlarini ko'rsatadigan struct ta'rifida umumiy turdan foydalanamiz.

Esda tutingki, biz `Point<T>`ni aniqlash uchun faqat bitta umumiy turdan foydalanganmiz, bu taʼrifda aytilishicha, `Point<T>` structi ba'zi bir `T` turiga nisbatan umumiy boʻlib, `x` va `y` maydonlari qaysi turdagi boʻlishidan qatʼi nazar bir xil turdagi dir. Agar biz 10-7 ro'yxatdagi kabi har xil turdagi qiymatlarga ega bo'lgan `Point<T>` nusxasini yaratsak, bizning kodimiz kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">Roʻyxat 10-7: `x` va `y` maydonlari bir xil turdagi boʻlishi kerak, chunki ikkalasi ham bir xil umumiy maʼlumotlar turi `T`ga ega.</span>

Ushbu misolda, biz `x` ga 5 butun qiymatini belgilaganimizda, kompilyatorga `T` umumiy turi `Point<T>` misoli uchun butun son bo'lishini bildiramiz. Keyin biz `x` bilan bir xil turga ega ekanligini aniqlagan `y` uchun 4.0 ni belgilaganimizda, biz quyidagi turdagi nomuvofiqlik xatosini olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

`x` va `y` ikkalasi ham umumiy bo'lgan, lekin har xil turlarga ega bo'lishi mumkin bo'lgan `Point` strukturasini aniqlash uchun biz bir nechta umumiy turdagi parametrlardan foydalanishimiz mumkin. Masalan, 10-8 roʻyxatda biz `Point` taʼrifini `T` va `U` turlari boʻyicha umumiy qilib oʻzgartiramiz, bunda `x` `T` turiga, `y` esa `U` turiga tegishli.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">Roʻyxat 10-8: `x` va `y` har xil turdagi qiymatlar boʻlishi uchun ikki turdagi umumiy `Point<T, U>`.</span>

Endi ko'rsatilgan `Point` ning barcha misollariga ruxsat berilgan! Ta'rifda siz xohlagancha umumiy turdagi parametrlardan foydalanishingiz mumkin, lekin bir nechtadan ko'proq foydalanish kodingizni o'qishni qiyinlashtiradi. Agar siz kodingizda ko'plab umumiy turlar kerakligini aniqlasangiz, bu sizning kodingizni kichikroq qismlarga qayta qurish kerakligini ko'rsatishi mumkin.

### Enum Definitionlarida

Structlar bilan qilganimizdek, ularning variantlarida umumiy ma'lumotlar turlarini saqlash uchun enumlarni belgilashimiz mumkin. Biz 6-bobda foydalanilgan standart kutubxona taqdim etadigan `Option<T>` enumini yana bir ko'rib chiqamiz:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Bu ta'rif endi siz uchun yanada ma'noli bo'lishi kerak. Ko'rib turganingizdek, `Option<T>` enum `T` turiga nisbatan umumiy va ikkita variantga ega: `T` turidagi bitta qiymatga ega `Some` va hech qanday qiymatga ega bo'lmagan `None` varianti.
`Option<T>` enum yordamida biz ixtiyoriy qiymatning mavhum kontseptsiyasini ifodalashimiz mumkin va `Option<T>` umumiy bo'lgani uchun biz ixtiyoriy qiymatning turi qanday bo'lishidan qat`i nazar, bu abstraktsiyadan foydalanishimiz mumkin.

Enumlar bir nechta generik turlardan ham foydalanishi mumkin. Biz 9-bobda aytib o'tgan `Result` enumining ta'rifi ushbu foydalanishga misoldir:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` enumlari ikki xil, `T` va `E` uchun generikdir va ikkita variantga ega: `T` turidagi qiymatga ega `OK` va `E` turidagi qiymatga ega bo'lgan `Err`. Bu taʼrif `Result` enumidan bizda muvaffaqiyatli boʻlishi mumkin boʻlgan (`T` turidagi qiymatni qaytarish) yoki muvaffaqiyatsiz boʻlishi mumkin boʻlgan (`E` turidagi xatolikni qaytarish) istalgan joyda foydalanishni qulay qiladi. Aslida, biz 9-3 ro'yxatdagi faylni shunday ochar edik, bu yerda fayl muvaffaqiyatli ochilganda `T` `std::fs::File` turi bilan to'ldirilgan va faylni ochishda muammolar yuzaga kelganda `E` `std::io::Error` turi bilan to`ldirilgan.

Kodingizdagi vaziyatlarni faqat ular ega bo'lgan qiymatlar turlarida farq qiluvchi bir nechta tuzilma yoki enum ta'riflari bilan tanib olganingizda, uning o'rniga generik turlardan foydalanish orqali takrorlanishdan qochishingiz mumkin.

### Metod Definitionlarida

Biz tuzilmalar va raqamlar bo'yicha usullarni qo'llashimiz mumkin (5-bobda qilganimiz kabi) va ularning ta'riflarida umumiy turlardan ham foydalanishimiz mumkin. 10-9 ro'yxatda biz 10-6 ro'yxatda belgilagan "Point<T>" tuzilmasi ko'rsatilgan va unda "x" nomli usul qo'llaniladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">Listing 10-9: Implementing a method named `x` on the
`Point<T>` struct that will return a reference to the `x` field of type
`T`</span>

Here, we’ve defined a method named `x` on `Point<T>` that returns a reference
to the data in the field `x`.

Note that we have to declare `T` just after `impl` so we can use `T` to specify
that we’re implementing methods on the type `Point<T>`. By declaring `T` as a
generic type after `impl`, Rust can identify that the type in the angle
brackets in `Point` is a generic type rather than a concrete type. We could
have chosen a different name for this generic parameter than the generic
parameter declared in the struct definition, but using the same name is
conventional. Methods written within an `impl` that declares the generic type
will be defined on any instance of the type, no matter what concrete type ends
up substituting for the generic type.

We can also specify constraints on generic types when defining methods on the
type. We could, for example, implement methods only on `Point<f32>` instances
rather than on `Point<T>` instances with any generic type. In Listing 10-10 we
use the concrete type `f32`, meaning we don’t declare any types after `impl`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">Listing 10-10: An `impl` block that only applies to a
struct with a particular concrete type for the generic type parameter `T`</span>

This code means the type `Point<f32>` will have a `distance_from_origin`
method; other instances of `Point<T>` where `T` is not of type `f32` will not
have this method defined. The method measures how far our point is from the
point at coordinates (0.0, 0.0) and uses mathematical operations that are
available only for floating point types.

Generic type parameters in a struct definition aren’t always the same as those
you use in that same struct’s method signatures. Listing 10-11 uses the generic
types `X1` and `Y1` for the `Point` struct and `X2` `Y2` for the `mixup` method
signature to make the example clearer. The method creates a new `Point`
instance with the `x` value from the `self` `Point` (of type `X1`) and the `y`
value from the passed-in `Point` (of type `Y2`).

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">Listing 10-11: A method that uses generic types different
from its struct’s definition</span>

In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct
that has a string slice for `x` (with value `"Hello"`) and a `char` for `y`
(with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`,
which will have an `i32` for `x`, because `x` came from `p1`. The `p3` variable
will have a `char` for `y`, because `y` came from `p2`. The `println!` macro
call will print `p3.x = 5, p3.y = c`.

The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with `impl` and some are declared with the method
definition. Here, the generic parameters `X1` and `Y1` are declared after
`impl` because they go with the struct definition. The generic parameters `X2`
and `Y2` are declared after `fn mixup`, because they’re only relevant to the
method.

### Performance of Code Using Generics

You might be wondering whether there is a runtime cost when using generic type
parameters. The good news is that using generic types won't make your program run
any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code using
generics at compile time. *Monomorphization* is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled. In this process, the compiler does the opposite of the steps we used
to create the generic function in Listing 10-5: the compiler looks at all the
places where generic code is called and generates code for the concrete types
the generic code is called with.

Let’s look at how this works by using the standard library’s generic
`Option<T>` enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization. During that
process, the compiler reads the values that have been used in `Option<T>`
instances and identifies two kinds of `Option<T>`: one is `i32` and the other
is `f64`. As such, it expands the generic definition of `Option<T>` into two
definitions specialized to `i32` and `f64`, thereby replacing the generic
definition with the specific ones.

The monomorphized version of the code looks similar to the following (the
compiler uses different names than what we’re using here for illustration):

<span class="filename">Filename: src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

The generic `Option<T>` is replaced with the specific definitions created by
the compiler. Because Rust compiles generic code into code that specifies the
type in each instance, we pay no runtime cost for using generics. When the code
runs, it performs just as it would if we had duplicated each definition by
hand. The process of monomorphization makes Rust’s generics extremely efficient
at runtime.
