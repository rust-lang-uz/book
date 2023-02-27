## Vectorlar bilan qiymatlar ro'yxatini saqlash

Biz ko'rib chiqadigan birinchi to'plam turi `Vec<T>` bo'lib, u *vector* sifatida ham tanilgan.
Vectorlar xotirada barcha qiymatlarni yonma-yon joylashtirgan yagona ma'lumotlar strukturasida bir nechta qiymatlarni saqlash imkonini beradi. Vectorlar faqat bir xil turdagi qiymatlarni saqlashi mumkin. Ular sizda fayldagi matn satrlari yoki xarid qilish savatidagi narsalarning narxlari kabi elementlar ro'yxatiga ega bo'lsangiz foydali bo'ladi.

### Yangi vector yaratish

Yangi bo'sh vector yaratish uchun biz 8-1 ro'yxatda ko'rsatilganidek, `Vec::new` funksiyasini chaqiramiz.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-1: `i32` turidagi qiymatlarni saqlash uchun yangi, boʻsh vector yaratish</span>

E'tibor bering, biz bu erda annation tur qo'shdik. Biz ushbu vectorga hech qanday qiymat kiritmayotganimiz sababli, Rust biz qanday elementlarni saqlashni xohlayotganimizni bilmaydi. Bu muhim nuqta. Vectorlar generiklar yordamida amalga oshiriladi; Biz 10-bobda o'zingizning turlaringiz bilan generiklardan qanday foydalanishni ko'rib chiqamiz. Hozircha shuni bilingki, standart kutubxona tomonidan taqdim etilgan `Vec<T>` turi har qanday turni sig'dira oladi.
Muayyan turni ushlab turish uchun vector yaratganimizda, burchakli qavslar([]) ichida turni belgilashimiz mumkin. 8-1 roʻyxatida biz Rustga `v`dagi `Vec<T>` `i32` turidagi elementlarni saqlashini aytdik.

Ko'pincha siz boshlang'ich qiymatlari bilan `Vec<T>` ni yaratasiz va Rust siz saqlamoqchi bo'lgan qiymat turini aniqlaydi, shuning uchun kamdan-kam hollarda bu turdagi annotionni bajarishingiz kerak bo'ladi. Rust qulay tarzda `vec!` makrosini taqdim etadi, bu esa siz bergan qiymatlarni saqlaydigan yangi vectorni yaratadi. 8-2 roʻyxati `1`, `2` va `3` qiymatlariga ega boʻlgan yangi `Vec<i32>`ni yaratadi. Butun son turi `i32` dir, chunki bu standart butun son turi, biz 3-bobning ["Ma'lumotlar turlari"][data-types]<!-- ignore --> bo'limida muhokama qilganimizdek.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-2: qiymatlarni o'z ichiga olgan yangi vector yaratish</span>

Biz boshlang‘ich `i32` qiymatlarini berganimiz sababli, Rust `v` turi `Vec<i32>` ekanligini va tur izohi shart emas degan xulosaga kelishi mumkin. Keyinchalik vectorni qanday o'zgartirishni ko'rib chiqamiz.

### Vectorni yangilash

Vector yaratish va unga elementlar qo'shish uchun biz 8-3 ro'yxatda ko'rsatilganidek, `push` metodidan foydalanishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-3: vectorga qiymatlar qo'shish uchun `push` metodidan foydalanish</span>

Har qanday o'zgaruvchida bo'lgani kabi, agar biz uning qiymatini o'zgartirish imkoniyatiga ega bo'lishni istasak, 3-bobda muhokama qilinganimizdek, `mut` kalit so'zidan foydalanib, uni o'zgaruvchan qilishimiz kerak. Biz joylashtirgan raqamlarning barchasi `i32` turiga kiradi va Rust buni maʼlumotlardan chiqaradi, shuning uchun bizga `Vec<i32>` annotationi kerak emas.

### Vector elementlarini o'qish

Vectorda saqlangan qiymatga murojaat qilishning ikki yo'li mavjud: indekslash yoki `get` metodi yordamida. Quyidagi misollarda biz qo'shimcha aniqlik uchun ushbu funksiyalardan qaytariladigan qiymatlar turlarini izohladik.

8-4 ro'yxatda indekslash sintaksisi va `get` metodi bilan vectordagi qiymatga kirishning ikkala usuli ko'rsatilgan.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-4: Vectordagi elementga kirish uchun indekslash sintaksisi yoki `get` metodidan foydalanish</span>

Bu erda bir nechta detallarga e'tibor bering. Uchinchi elementni olish uchun `2` indeks qiymatidan foydalanamiz, chunki vectorlar noldan boshlab raqamlar boʻyicha indekslanadi. `&` va `[]` dan foydalanish bizga indeks qiymatidagi elementga reference beradi. Argument sifatida berilgan indeks bilan `get` metodidan  foydalansak, biz `match` bilan foydalanishimiz mumkin bo'lgan `Option<&T>`ni olamiz.

Rust elementga reference qilishning ushbu ikki usulini taqdim etishining sababi shundaki, siz mavjud elementlar doirasidan tashqarida indeks qiymatidan foydalanmoqchi bo'lganingizda dastur qanday harakat qilishini tanlashingiz mumkin. Misol sifatida, keling, besh elementli vectorga ega bo'lganimizda nima sodir bo'lishini ko'rib chiqamiz va keyin 8-5 ro'yxatda ko'rsatilganidek, har bir texnikada 100 indeksidagi elementga kirishga harakat qilamiz.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-5: besh elementni o'z ichiga olgan vectorda 100 indeksidagi elementga kirishga urinish</span>

Ushbu kodni ishga tushirganimizda, birinchi `[]` metodi dasturda panic chiqaradi, chunki u mavjud bo'lmagan elementga murojaat qiladi. Ushbu usul vector oxiridan o'tgan elementga kirishga urinish bo'lsa, dasturingiz ishdan chiqishini xohlasangiz yaxshi qo'llaniladi.

`get` metodi vektordan tashqaridagi indeksdan o'tganda, panic qo'ymasdan  `None`ni qaytaradi. Vector doirasidan tashqaridagi elementga kirish vaqti-vaqti bilan oddiy sharoitlarda sodir bo'lishi mumkin bo'lsa, siz ushbu usuldan foydalanasiz. Keyin sizning kodingiz 6-bobda muhokama qilinganidek, `Some(&element)`  yoki `None`ga ega bo'lish mantiqiga ega bo'ladi.Misol uchun, indeks raqamni kiritgan odamdan kelib chiqishi mumkin. Agar ular tasodifan juda katta raqamni kiritsa va dastur  `None` qiymatiga ega bo'lsa, siz foydalanuvchiga joriy vectorda nechta element borligini aytishingiz va ularga to'g'ri qiymat kiritish uchun yana bir imkoniyat berishingiz mumkin.Bu imlo xatosi tufayli dasturni buzishdan ko'ra foydalanuvchilar uchun qulayroq bo'lar edi!

Dasturda tegishli reference mavjud bo'lsa, borrow tekshiruvi ushbu reference va vector mazmuniga boshqa har qanday referencelar haqiqiyligini ta'minlash uchun ownership va borrowing qoidalarini (4-bobda ko'rsatilgan) amalga oshiradi. Bir xil doirada o'zgaruvchan va o'zgarmas referencelarga ega bo'lolmaysiz degan qoidani eslang. Ushbu qoida 8-6 ro'yxatda qo'llaniladi, bu yerda biz vectordagi birinchi elementga o'zgarmas referenceni ushlab turamiz va elementni oxiriga qo'shishga harakat qilamiz. Agar biz ushbu elementga keyinroq funksiyada murojaat qilsak, bu dastur ishlamaydi:


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Listing 8-6: Attempting to add an element to a vector
while holding a reference to an item</span>

Compiling this code will result in this error:


```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

The code in Listing 8-6 might look like it should work: why should a reference
to the first element care about changes at the end of the vector? This error is
due to the way vectors work: because vectors put the values next to each other
in memory, adding a new element onto the end of the vector might require
allocating new memory and copying the old elements to the new space, if there
isn’t enough room to put all the elements next to each other where the vector
is currently stored. In that case, the reference to the first element would be
pointing to deallocated memory. The borrowing rules prevent programs from
ending up in that situation.

> Note: For more on the implementation details of the `Vec<T>` type, see [“The
> Rustonomicon”][nomicon].

### Iterating over the Values in a Vector

To access each element in a vector in turn, we would iterate through all of the
elements rather than use indices to access one at a time. Listing 8-7 shows how
to use a `for` loop to get immutable references to each element in a vector of
`i32` values and print them.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Listing 8-7: Printing each element in a vector by
iterating over the elements using a `for` loop</span>

We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-8
will add `50` to each element.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Listing 8-8: Iterating over mutable references to
elements in a vector</span>

To change the value that the mutable reference refers to, we have to use the
`*` dereference operator to get to the value in `i` before we can use the `+=`
operator. We’ll talk more about the dereference operator in the [“Following the
Pointer to the Value with the Dereference Operator”][deref]<!-- ignore -->
section of Chapter 15.

Iterating over a vector, whether immutably or mutably, is safe because of the
borrow checker's rules. If we attempted to insert or remove items in the `for`
loop bodies in Listing 8-7 and Listing 8-8, we would get a compiler error
similar to the one we got with the code in Listing 8-6. The reference to the
vector that the `for` loop holds prevents simultaneous modification of the
whole vector.

### Using an Enum to Store Multiple Types

Vectors can only store values that are the same type. This can be inconvenient;
there are definitely use cases for needing to store a list of items of
different types. Fortunately, the variants of an enum are defined under the
same enum type, so when we need one type to represent elements of different
types, we can define and use an enum!

For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and all the enum variants will be considered the same type: that
of the enum. Then we can create a vector to hold that enum and so, ultimately,
holds different types. We’ve demonstrated this in Listing 8-9.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Listing 8-9: Defining an `enum` to store values of
different types in one vector</span>

Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. We
must also be explicit about what types are allowed in this vector. If Rust
allowed a vector to hold any type, there would be a chance that one or more of
the types would cause errors with the operations performed on the elements of
the vector. Using an enum plus a `match` expression means that Rust will ensure
at compile time that every possible case is handled, as discussed in Chapter 6.

If you don’t know the exhaustive set of types a program will get at runtime to
store in a vector, the enum technique won’t work. Instead, you can use a trait
object, which we’ll cover in Chapter 17.

Now that we’ve discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api]<!-- ignore --> for all the many
useful methods defined on `Vec<T>` by the standard library. For example, in
addition to `push`, a `pop` method removes and returns the last element.

### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-10.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Listing 8-10: Showing where the vector and its elements
are dropped</span>

When the vector gets dropped, all of its contents are also dropped, meaning the
integers it holds will be cleaned up. The borrow checker ensures that any
references to contents of a vector are only used while the vector itself is
valid.

Let’s move on to the next collection type: `String`!

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
