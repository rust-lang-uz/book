## Turli xildagi qiymatlarni qabul qila oladigan Trait ya'ni xususiyat obyektlaridan foydalanish

8-bobda biz vektorlarning faqat bir turdagi elementlarni saqlash
imkoniyatiga ega ekanligini ta’kidlagan edik. 8-9-ro‘yxatda
butun sonlar, kasr sonlar va matnlarni saqlash uchun variantlarga
ega bo‘lgan ’SpreadsheetCell’ nomli sanab o‘tish turini yaratib,
bu muammoni hal qilish ko'rsatilgan edi. Bu usul har bir katakda
turli xil ma’lumotlarni saqlash va shu bilan birga kataklar
qatorini ifodalovchi vektorga ega bo‘lish imkonini berdi. Agar
o‘zaro almashtirilishi mumkin bo‘lgan elementlarni kodda tuzilayotgan
paytda ma’lum bo‘lgan turlarning belgilangan to‘plamidan iborat
bo‘lsa, bu juda yaxshi yechim hisoblanadi.

Biroq, ba’zida kutubxonamiz foydalanuvchisi o‘zi uchun mos bo‘lgan, muayyan vaziyatda
ishlatilishi mumkin bo‘lgan turlar to‘plamini kengaytira olishini xohlaymiz. Bu qanday 
amalga oshirilishini ko‘rsatish uchun, grafik foydalanuvchi interfeysi (GUI) vositasi 
misolini yaratamiz. Ushbu vosita elementlar ro‘yxatidan o‘tadi va har bir element uchun 
`draw` metodini chaqiradi. Bu GUI vositalarida keng qo‘llaniladigan uslubdir.`gui` 
nomli kutubxona crate yaratiladi. Ushbuu crate GUI kutubxonasining asosiy tuzilmasini o‘z 
ichiga oladi. Unda, masalan, `Button` yoki `TextField` kabi foydalanishga tayyor ayrim 
turlarni taqdim qilishi mumkin. Shu bilan birga, `gui` foydalanuvchilari o‘zlarining 
chizilishi mumkin bo‘lgan turlarini ham yaratmoqchi bo‘lishadi: masalan, bir dasturchi
 `Image` turini qo‘shsa, boshqasi `SelectBox` turini qo‘shishi mumkin.

Ushbu misolda biz to'laqonli grafik interfeyslik (GUI) kutubxona yozmaymiz, lekin
qismlar bir-biri bilan qanday ulanishini ko'rsatamiz. Kutubxona yozish vaqtida
biz boshqa dasturchilar nima va qanday qilib yozishini oldindan bilmaymiz.
Lekin biz bilamizki, `gui` imkon qadar ko'p turlar qiymatidan xabardor bo'lishi
kerak, va u `draw` (ya'ni chizish) metodini ana shu turlarning har birida
chaqirishi lozim. Biz `draw` metodini chaqirgan vaqtimizda aynan nima ish sodir
bo'lishini `gui` bilishi kerak emas, faqatgina `draw` metodi bizga chaqirish
uchun mavjudligini biladi xolos.

Buni nasl qilib oluvchi tilda qilish uchun, biz `draw` metodiga ega `Component`
nomli tur yaratishimizga to'g'ri keladi. Boshqa `Button`, `Image` va `SelectBox`
kabi turlar esa `Component` turdan nasl qilib olish orqali `draw` metodini ham o'z
ichiga oladi. Har biri `draw` metodi hislatini o'zgartirish uchun qayta yozib chiqishlari
mumkin, lekin asl freymvork hammasini huddi `Component` turi bo'lganiday, `draw` ni
chaqirishi mumkin. Lekin Rust da nasldorlik bo'lmagani uchun, biz `gui` kutubxonasini
foydalanuvchilar o'zlari xohlashganiga kengaytirishlari uchun, boshqa usul bilan tuzib
chiqish yo'llarini ko'rib chiqishimizga to'g'ri keladi.

### Defining a Trait for Common Behavior

To implement the behavior we want `gui` to have, we’ll define a trait named
`Draw` that will have one method named `draw`. Then we can define a vector that
takes a *trait object*. A trait object points to both an instance of a type
implementing our specified trait and a table used to look up trait methods on
that type at runtime. We create a trait object by specifying some sort of
pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn`
keyword, and then specifying the relevant trait. (We’ll talk about the reason
trait objects must use a pointer in Chapter 19 in the section [“Dynamically
Sized Types and the `Sized` Trait.”][dynamically-sized]<!-- ignore -->) We can
use trait objects in place of a generic or concrete type. Wherever we use a
trait object, Rust’s type system will ensure at compile time that any value
used in that context will implement the trait object’s trait. Consequently, we
don’t need to know all the possible types at compile time.

We’ve mentioned that, in Rust, we refrain from calling structs and enums
“objects” to distinguish them from other languages’ objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks are
separated, whereas in other languages, the data and behavior combined into one
concept is often labeled an object. However, trait objects *are* more like
objects in other languages in the sense that they combine data and behavior.
But trait objects differ from traditional objects in that we can’t add data to
a trait object. Trait objects aren’t as generally useful as objects in other
languages: their specific purpose is to allow abstraction across common
behavior.

17-3 chi ro'yxat `Draw` trait'ini `draw` metodi bilan birga ta'riflash ko'rsatib beradi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<span class="caption">Ro'yxat 17-3: `Draw` trait'ining ta'rifi</span>

Ushbu sintaksis bizning 10 chi bo'limda bo'lib o'tgan Traitlarni joriy etish
suhbatimizdan keyin tanish bo'lishi kerak. Keyingisi esa yana yangi sintaksis:
17-4 chi ro'yxat `Screen` nomli `components` nomi ostidagi vekotr o'z ichiga olgan
structni ta'riflaydi. Ushbu vektor `Box<dyn Draw>` turidan, ya'ni trait obyekt (bu
`Box` ichida `Draw` tratini joriy etuvchi istalgan turga solishtiriluvchi).

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-4: `Screen` structidagi `components` maydoni
bir vektorda joylashgan va `Draw` tratini joriy etgan obyektlarni ushlab turibdi
</span>

`Screen` struktida, biz 17-5 chi ro'yxatda ko'rsatilganiday, `draw` metodini har
bir `components` ustidan chaqiradigan `run` nomli metod yaratamiz:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-5: `Screen` da har bir komponent ustidan
`draw` metodini chaqiradigan `run` metodi</span>

Bu generik tur ko'rsatkichi va trait cheklanmalardan farqli boshqacha
ishlaydi. Generik tur parametr bir vaqt o'zida faqat bitta tur qabul qiladi,
trait obyektlar esa boshqa tarafdan ko'plab konkret turlar ishlash vaqtidagi
trait obyektlarni to'ldirib berish uchun ishlatsa bo'ladi. Misol uchun,
`Screen` struktini 17-6 chi ro'yxatda ko'rsatilganiday generik tur va trait
cheklanmalari bilan ta'riflasa bo'ladi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-6: `Screen` strukti va uning `run` metodining
generik va trait cheklanmalarini ishlatgandagi alternativ ta'rifi.</span>

This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you’ll only ever have homogeneous
collections, using generics and trait bounds is preferable because the
definitions will be monomorphized at compile time to use the concrete types.

On the other hand, with the method using trait objects, one `Screen` instance
can hold a `Vec<T>` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let’s look at how this works, and then we’ll talk about the
runtime performance implications.

### Implementing the Trait

Now we’ll add some types that implement the `Draw` trait. We’ll provide the
`Button` type. Again, actually implementing a GUI library is beyond the scope
of this book, so the `draw` method won’t have any useful implementation in its
body. To imagine what the implementation might look like, a `Button` struct
might have fields for `width`, `height`, and `label`, as shown in Listing 17-7:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<span class="caption">Listing 17-7: A `Button` struct that implements the
`Draw` trait</span>

The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components; for example, a `TextField` type might have those
same fields plus a `placeholder` field. Each of the types we want to draw on
the screen will implement the `Draw` trait but will use different code in the
`draw` method to define how to draw that particular type, as `Button` has here
(without the actual GUI code, as mentioned). The `Button` type, for instance,
might have an additional `impl` block containing methods related to what
happens when a user clicks the button. These kinds of methods won’t apply to
types like `TextField`.

If someone using our library decides to implement a `SelectBox` struct that has
`width`, `height`, and `options` fields, they implement the `Draw` trait on the
`SelectBox` type as well, as shown in Listing 17-8:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<span class="caption">Listing 17-8: Another crate using `gui` and implementing
the `Draw` trait on a `SelectBox` struct</span>

Our library’s user can now write their `main` function to create a `Screen`
instance. To the `Screen` instance, they can add a `SelectBox` and a `Button`
by putting each in a `Box<T>` to become a trait object. They can then call the
`run` method on the `Screen` instance, which will call `draw` on each of the
components. Listing 17-9 shows this implementation:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<span class="caption">Listing 17-9: Using trait objects to store values of
different types that implement the same trait</span>

When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.

This concept—of being concerned only with the messages a value responds to
rather than the value’s concrete type—is similar to the concept of *duck
typing* in dynamically typed languages: if it walks like a duck and quacks
like a duck, then it must be a duck! In the implementation of `run` on `Screen`
in Listing 17-5, `run` doesn’t need to know what the concrete type of each
component is. It doesn’t check whether a component is an instance of a `Button`
or a `SelectBox`, it just calls the `draw` method on the component. By
specifying `Box<dyn Draw>` as the type of the values in the `components`
vector, we’ve defined `Screen` to need values that we can call the `draw`
method on.

The advantage of using trait objects and Rust’s type system to write code
similar to code using duck typing is that we never have to check whether a
value implements a particular method at runtime or worry about getting errors
if a value doesn’t implement a method but we call it anyway. Rust won’t compile
our code if the values don’t implement the traits that the trait objects need.

For example, Listing 17-10 shows what happens if we try to create a `Screen`
with a `String` as a component:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

<span class="caption">Listing 17-10: Attempting to use a type that doesn’t
implement the trait object’s trait</span>

We’ll get this error because `String` doesn’t implement the `Draw` trait:

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

This error lets us know that either we’re passing something to `Screen` we
didn’t mean to pass and so should pass a different type or we should implement
`Draw` on `String` so that `Screen` is able to call `draw` on it.

### Trait Objects Perform Dynamic Dispatch

Recall in the [“Performance of Code Using
Generics”][performance-of-code-using-generics]<!-- ignore --> section in
Chapter 10 our discussion on the monomorphization process performed by the
compiler when we use trait bounds on generics: the compiler generates
nongeneric implementations of functions and methods for each concrete type that
we use in place of a generic type parameter. The code that results from
monomorphization is doing *static dispatch*, which is when the compiler knows
what method you’re calling at compile time. This is opposed to *dynamic
dispatch*, which is when the compiler can’t tell at compile time which method
you’re calling. In dynamic dispatch cases, the compiler emits code that at
runtime will figure out which method to call.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t
know all the types that might be used with the code that’s using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at
runtime, Rust uses the pointers inside the trait object to know which method to
call. This lookup incurs a runtime cost that doesn’t occur with static
dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a
method’s code, which in turn prevents some optimizations. However, we did get
extra flexibility in the code that we wrote in Listing 17-5 and were able to
support in Listing 17-9, so it’s a trade-off to consider.

[performance-of-code-using-generics]:
ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
