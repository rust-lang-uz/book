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

The help text mentions `std::cmp::PartialOrd`, which is a *trait*, and we’re
going to talk about traits in the next section. For now, know that this error
states that the body of `largest` won’t work for all possible types that `T`
could be. Because we want to compare values of type `T` in the body, we can
only use types whose values can be ordered. To enable comparisons, the standard
library has the `std::cmp::PartialOrd` trait that you can implement on types
(see Appendix C for more on this trait). By following the help text's
suggestion, we restrict the types valid for `T` to only those that implement
`PartialOrd` and this example will compile, because the standard library
implements `PartialOrd` on both `i32` and `char`.

### In Struct Definitions

We can also define structs to use a generic type parameter in one or more
fields using the `<>` syntax. Listing 10-6 defines a `Point<T>` struct to hold
`x` and `y` coordinate values of any type.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">Listing 10-6: A `Point<T>` struct that holds `x` and `y`
values of type `T`</span>

The syntax for using generics in struct definitions is similar to that used in
function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then we use the generic type
in the struct definition where we would otherwise specify concrete data types.

Note that because we’ve used only one generic type to define `Point<T>`, this
definition says that the `Point<T>` struct is generic over some type `T`, and
the fields `x` and `y` are *both* that same type, whatever that type may be. If
we create an instance of a `Point<T>` that has values of different types, as in
Listing 10-7, our code won’t compile.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">Listing 10-7: The fields `x` and `y` must be the same
type because both have the same generic data type `T`.</span>

In this example, when we assign the integer value 5 to `x`, we let the compiler
know that the generic type `T` will be an integer for this instance of
`Point<T>`. Then when we specify 4.0 for `y`, which we’ve defined to have the
same type as `x`, we’ll get a type mismatch error like this:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters. For example, in
Listing 10-8, we change the definition of `Point` to be generic over types `T`
and `U` where `x` is of type `T` and `y` is of type `U`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">Listing 10-8: A `Point<T, U>` generic over two types so
that `x` and `y` can be values of different types</span>

Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. If you're finding you need lots of generic types in
your code, it could indicate that your code needs restructuring into smaller
pieces.

### In Enum Definitions

As we did with structs, we can define enums to hold generic data types in their
variants. Let’s take another look at the `Option<T>` enum that the standard
library provides, which we used in Chapter 6:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This definition should now make more sense to you. As you can see, the
`Option<T>` enum is generic over type `T` and has two variants: `Some`, which
holds one value of type `T`, and a `None` variant that doesn’t hold any value.
By using the `Option<T>` enum, we can express the abstract concept of an
optional value, and because `Option<T>` is generic, we can use this abstraction
no matter what the type of the optional value is.

Enums can use multiple generic types as well. The definition of the `Result`
enum that we used in Chapter 9 is one example:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, `T` and `E`, and has two variants:
`Ok`, which holds a value of type `T`, and `Err`, which holds a value of type
`E`. This definition makes it convenient to use the `Result` enum anywhere we
have an operation that might succeed (return a value of some type `T`) or fail
(return an error of some type `E`). In fact, this is what we used to open a
file in Listing 9-3, where `T` was filled in with the type `std::fs::File` when
the file was opened successfully and `E` was filled in with the type
`std::io::Error` when there were problems opening the file.

When you recognize situations in your code with multiple struct or enum
definitions that differ only in the types of the values they hold, you can
avoid duplication by using generic types instead.

### In Method Definitions

We can implement methods on structs and enums (as we did in Chapter 5) and use
generic types in their definitions, too. Listing 10-9 shows the `Point<T>`
struct we defined in Listing 10-6 with a method named `x` implemented on it.

<span class="filename">Filename: src/main.rs</span>

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
