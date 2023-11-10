## Iteratorlar yordamida elementlar ketma-ketligini qayta ishlash

Iterator pattern sizga navbat bilan elementlarning ketma-ketligi bo'yicha ba'zi vazifalarni(task) bajarishga imkon beradi. Iterator har bir elementni takrorlash va ketma-ketlik qachon tugashini aniqlash mantiqi uchun javobgardir. Iteratorlardan foydalanganda, bu mantiqni(logic) o'zingiz takrorlashingiz shart emas.

Rust-da iteratorlar *dangasa*, ya'ni iteratorni ishlatish uchun ishlatadigan metodlarni chaqirmaguningizcha ular hech qanday ta'sir ko'rsatmaydi. Masalan, 13-10-Ro'yxatdagi kod `Vec<T>` da belgilangan `iter` metodini chaqirish orqali `v1` vektoridagi elementlar ustidan iterator yaratadi. Ushbu kod o'z-o'zidan hech qanday foydali ish qilmaydi.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-10: iterator yaratish</span>

Iterator `v1_iter` o'zgaruvchisida saqlanadi. Biz iteratorni yaratganimizdan so'ng, biz uni turli usullarda ishlatishimiz mumkin. 3-bobdagi 3-5 ro'yxatda biz arrayning har bir elementida ba'zi kodlarni bajarish uchun `for` loop siklidan foydalangan holda uni takrorladik. Korpus ostida bu bilvosita yaratgan va keyin iteratorni ishlatgan, ammo biz hozirgacha uning qanday ishlashini ko'rib chiqdik.

13-11 Ro'yxatdagi misolda biz iteratorni yaratishni `for` loop siklidagi iteratordan foydalanishdan ajratamiz. `for` loop sikli `v1_iter` da iterator yordamida chaqirilganda, iteratordagi har bir element loop siklning bir iteratsiyasida ishlatiladi, bu esa har bir qiymatni chop etadi.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-11: `for` loop siklida iteratordan foydalanish</span>

Standart kutubxonalari tomonidan taqdim etilgan iteratorlarga ega bo'lmagan tillarda siz xuddi shu funksiyani o'zgaruvchini 0 indeksidan boshlab yozishingiz mumkin, qiymat olish uchun vektorga indekslash uchun ushbu o'zgaruvchidan foydalanish va vektordagi elementlarning umumiy soniga yetgunga qadar sikldagi o'zgaruvchi qiymatini oshirish.

Iteratorlar siz uchun barcha mantiqni(logic) boshqaradi, siz chalkashtirib yuborishingiz mumkin bo'lgan takroriy kodni qisqartiradi. Iteratorlar vektorlar kabi indekslash mumkin bo'lgan ma'lumotlar tuzilmalari(data structure) emas, balki turli xil ketma-ketliklar(sequence) bilan bir xil mantiqdan foydalanish uchun ko'proq moslashuvchanlikni beradi. Keling, iteratorlar buni qanday qilishini ko'rib chiqaylik.

### `Iterator` traiti va `next` metodi

Barcha iteratorlar standart kutubxonada(standard library) aniqlangan `Iterator` nomli traitni implement qiladilar. Traitning definitioni quyidagicha ko'rinadi:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // default implement qilingan  metodlar bekor qilindi
}
```

Eʼtibor bering, bu definitionda baʼzi yangi sintaksislar qoʻllangan: `type Item` va `Self::Item` bu trait bilan *bogʻlangan turni(associated type)* belgilaydi. Bog'langan turlar haqida 19-bobda batafsil gaplashamiz. Hozircha siz bilishingiz kerak bo'lgan narsa shuki, ushbu kodda aytilishicha, `Iterator` traitini implement qilish uchun siz `Item` turini ham belgilashingiz kerak bo'ladi va bu `Item` turi `next` metodining qaytarish(return) turida qo'llaniladi. Boshqacha qilib aytganda, `Item` turi iteratordan qaytarilgan tur bo'ladi.

`Iterator` traiti amalga oshiruvchilardan(implementorlar) faqat bitta metodni belgilashni talab qiladi: `next` metod, u bir vaqtning o'zida `Some` ga o'ralgan(wrapped) iteratorning bir elementini qaytaradi va takrorlash(iteratsiya) tugagach, `None`ni qaytaradi.

Biz iteratorlarda `next` metodini to'g'ridan-to'g'ri chaqirishimiz mumkin; Ro'yxat 13-12 vektordan yaratilgan iteratorda `next` ga takroriy chaqiruvlardan qanday qiymatlar qaytarilishini ko'rsatadi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 13-12: iteratorda `next` metodini chaqirish</span>

Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code *consumes*, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn’t need
to make `v1_iter` mutable when we used a `for` loop because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.

Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.

### Methods that Consume the Iterator

The `Iterator` trait has a number of different methods with default
implementations provided by the standard library; you can find out about these
methods by looking in the standard library API documentation for the `Iterator`
trait. Some of these methods call the `next` method in their definition, which
is why you’re required to implement the `next` method when implementing the
`Iterator` trait.

Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-13 has a
test illustrating a use of the `sum` method:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

<span class="caption">Listing 13-13: Calling the `sum` method to get the total
of all items in the iterator</span>

We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes
ownership of the iterator we call it on.

### Methods that Produce Other Iterators

*Iterator adaptors* are methods defined on the `Iterator` trait that don’t
consume the iterator. Instead, they produce different iterators by changing
some aspect of the original iterator.

Listing 13-14 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item as the items are iterated through.
The `map` method returns a new iterator that produces the modified items. The
closure here creates a new iterator in which each item from the vector will be
incremented by 1:

<span class="filename">Filename: src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<span class="caption">Listing 13-14: Calling the iterator adaptor `map` to
create a new iterator</span>

However, this code produces a warning:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

The code in Listing 13-14 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.

To fix this warning and consume the iterator, we’ll use the `collect` method,
which we used in Chapter 12 with `env::args` in Listing 12-1. This method
consumes the iterator and collects the resulting values into a collection data
type.

In Listing 13-15, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

<span class="caption">Listing 13-15: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>

Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.

You can chain multiple calls to iterator adaptors to perform complex actions in
a readable way. But because all iterators are lazy, you have to call one of the
consuming adaptor methods to get results from calls to iterator adaptors.

### Using Closures that Capture Their Environment

Many iterator adapters take closures as arguments, and commonly the closures
we’ll specify as arguments to iterator adapters will be closures that capture
their environment.

For this example, we’ll use the `filter` method that takes a closure. The
closure gets an item from the iterator and returns a `bool`. If the closure
returns `true`, the value will be included in the iteration produced by
`filter`. If the closure returns `false`, the value won’t be included.

In Listing 13-16, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

<span class="caption">Listing 13-16: Using the `filter` method with a closure
that captures `shoe_size`</span>

The `shoes_in_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.

In the body of `shoes_in_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.

The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.

The test shows that when we call `shoes_in_size`, we get back only shoes
that have the same size as the value we specified.
