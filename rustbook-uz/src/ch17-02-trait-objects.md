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

Ushbu misolda to‘laqonli grafik interfeyslik (GUI) kutubxona yozilmaydi lekin
qismlar bir-biri bilan qanday ulanishini ko‘rsatiladi. Kutubxona yozish vaqtida
boshqa dasturchilar nima va qanday qilib yozishini oldindan bilib bo‘lmaydi.
Lekin bilamizki, `gui` imkon qadar ko‘p turlar qiymatidan xabardor bo‘lishi
kerak, va u `draw` (ya’ni chizish) metodini ana shu turlarning har birida
chaqirishi lozim. `draw` metodini chaqirgan vaqtida aynan nima ish sodir
bo‘lishini `gui` bilishi kerak emas, faqatgina `draw` metodi chaqirish uchun
mavjudligini biladi xolos.

Obyektga yo‘naltirilgan tillarda (misol uchun Java, C# va h.k.), `Component` nomli
klass ichida `draw` nomli metod bilan ifoda etiladi. `Button`, `Image` va
`SelectBox` kabi klasslar `Component`dan nasil olishadi va shu tufayli ular
ham `draw` metodini ifodalashadi. Ular, albatta, o‘zgacha `draw` metodini
e’lon qilishlari mumkin lekin dasturlash tili ularni xuddi `Component`dek
ko‘rishadi va `draw`ni chaqira olishadi. Rust dasturlash tilida nasl olish
imkoniyati yo‘q, vaholanki `gui` kutubxonasi foydalanuvchilari uni
kengaytira olishi uchun kutubxona boshqacha tuzilishi lozim.

### Defining a Trait for Common Behavior

To implement the behavior we want `gui` to have, we’ll define a trait named
`Draw` that will have one method named `draw`. Then we can define a vector that
takes a _trait object_. A trait object points to both an instance of a type
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


Rust dasturlash tilida structlar va enumlar “obyekt” deb atalmaydi. 
Bunday yondashuv, ularni boshqa dasturlash tillaridagi obyekt tushunchasidan 
farqlash maqsadida qo‘llaniladi. Rust tilida struct yoki enum tarkibidagi 
ma’lumotlar (ya’ni, maydonlar) va xatti-harakatlar `impl` bloklarida 
alohida saqlanadi. Aksariyat boshqa dasturlash tillarida esa ma’lumotlar va 
xatti-harakatlar yagona tuzilma sifatida birlashtirilib, odatda “obyekt” deb 
ataladi. Biroq trait obyektlar (trait objects) boshqa dasturlash tillaridagi 
obyektlarga o‘xshashlik kasb etadi. Chunki ular ma’lumot va xatti-harakatni 
birgalikda ifodalash imkonini beradi. Shunga qaramay, trait obyektlar an’anaviy 
obyektlardan farq qiladi: ular tarkibiga yangi ma’lumotlar qo‘shishga imkon 
bermaydi. Shu bois, trait obyektlar boshqa tillardagi obyektlar kabi keng 
maqsadlarda emas, balki faqat umumiy xatti-harakatni abstraktsiyalash, 
ya’ni umumiy funksionallik asosida turli obyektlar bilan ishlash imkoniyatini 
yaratish uchun qo‘llaniladi.

Listing 17-3 shows how to define a trait named `Draw` with one method named
`draw`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>

This syntax should look familiar from our discussions on how to define traits
in Chapter 10. Next comes some new syntax: Listing 17-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<dyn Draw>`, which is a trait object; it’s a stand-in for any type inside
a `Box` that implements the `Draw` trait.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

<span class="caption">Listing 17-4: Definition of the `Screen` struct with a
`components` field holding a vector of trait objects that implement the `Draw`
trait</span>

On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 17-5:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<span class="caption">Listing 17-5: A `run` method on `Screen` that calls the
`draw` method on each component</span>

This works differently from defining a struct that uses a generic type
parameter with trait bounds. A generic type parameter can only be substituted
with one concrete type at a time, whereas trait objects allow for multiple
concrete types to fill in for the trait object at runtime. For example, we
could have defined the `Screen` struct using a generic type and a trait bound
as in Listing 17-6:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<span class="caption">Listing 17-6: An alternate implementation of the `Screen`
struct and its `run` method using generics and trait bounds</span>

This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you’ll only ever have homogeneous
collections, using generics and trait bounds is preferable because the
definitions will be monomorphized at compile time to use the concrete types.

On the other hand, with the method using trait objects, one `Screen` instance
can hold a `Vec<T>` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let’s look at how this works, and then we’ll talk about the
runtime performance implications.

### Traitni amalga oshirish

Endi `Draw` traitini amalga oshiradigan ba'zi turlarni qo‘shamiz. `Button` 
turini taqdim etamiz. Yana, haqiqiy GUI kutubxonasini yaratish kitobimiz 
doirasidan tashqarida, shuning uchun `draw` metodi tanasida hech qanday 
foydali amalga oshirish bo‘lmaydi. Amalga oshirish qanday ko‘rinishi 
mumkinligini tasavvur qilish uchun, `Button` tuzilmasi `width` (kenglik), 
`height` (bo‘yi) va `label` (yorliq) kabi maydonlarga ega bo‘lishi mumkin, 
bu 17-7 ro'yxatdada ko‘rsatilgan:

<span class="filename">Faylnomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-7: `Draw` traitini amalga oshiradigan `Button` strukti</span>

`Button`ning `width`, `height` va `label` maydonlari boshqa komponentlarga
nisbatan farq qiladi; misol uchun `TextField`ning turi avvalgi maydonlar va
qo‘shimcha `placeholder` maydonidan tashkil topgan bo‘lishi mumkin. Har bir
ekranga chizilishi kerak bo‘lgan turlar `Draw` trait’ini joriy qilishadi ammo
ularning `draw` metodlari bir-birlaridan farq qiladi chunki har bir turning
o‘ziga xos shakli yoki boshqa xususiyati chizilishi mumkin. Misol uchun
`Button` bosganda sodir bo‘ladigan metod `impl` bloki ichida qo‘shimcha
kiritilishi mumkin. `TextField` uchun esa bunday funksional talab etilmaydi.

`width`, `height` va `options` maydonlardan tashkil topgan `SelectBox`ni
amalga oshirmoqchi bo‘lgan dasturchi `SelectBox` uchun `Draw` trait’ini
ham Ro‘yxat 17-8 da ko‘rsatilgandek yozishi kerak bo‘ladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<span class="caption">Ro‘yxat 17-8: `gui`dan foydalanuvchi va undagi `Draw`
trait’ini `SelectBox` struct’i uchun amalga oshiradigan crate</span>

Kutubxona foydalanuvchisi endi o‘z `main` funksiyasi ichida `Screen` nusxasini
yaratish imkoniga ega. `Screen` nusxasiga foydalanuvchi `SelectBox` va `Button`
nusxalarini har birini `Box<T>` ichiga joylash orqali ularni trait’ga o‘girib
qo‘shib chiqishi mumkin. Undan keyin `Screen` nusxasida `run` metodi chaqirgan
vaqtda har bir ichki komponentlarning `draw` metodi birma-bir chaqiriladi.
Ro‘yxat 17-9 xuddi shuni namoyish etadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<span class="caption">Ro‘yxat 17-9: Bir trait’ni amalga oshiruvchi turli xil
turlarni saqlash uchun trait obyektlar ishlatilishi</span>

When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.

Bu tushuncha — ya’ni, qiymatning aniq tipi emas, balki qanday xabarlarga 
javob bera olishi muhim bo‘lishi — dinamik tiplangan tillardagi *duck typing* 
tushunchasiga o‘xshaydi: agar u o‘rdakdek yursa va o‘rdakdek ovoz chiqarsa, 
demak u o‘rdak! 17-5-ro‘yxatdagi `Screen` uchun `run` funksiyasi 
implementatsiyasida `run` har bir komponentning aniq tipi nima ekanini 
bilishga muhtoj emas. Komponent `Button` yoki `SelectBox` ekanligini 
tekshirmaydi, shunchaki uning `draw` metodini chaqiradi. `components` vektoridagi 
qiymatlar turi sifatida `Box<dyn Draw>`ni ko‘rsatish orqali,`Screen`dan `draw` 
metodini chaqira olishimiz mumkin bo‘lgan qiymatlarni talab qiladigan qilib belgiladik.

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
monomorphization is doing _static dispatch_, which is when the compiler knows
what method you’re calling at compile time. This is opposed to _dynamic
dispatch_, which is when the compiler can’t tell at compile time which method
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

[performance-of-code-using-generics]: ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
