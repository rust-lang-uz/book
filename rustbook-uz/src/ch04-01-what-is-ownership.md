## Ownership Nima?

*Ownership*(Egalik) bu Rust dasturi xotirani qanday boshqarishini boshqaradigan qoidalar to'plami.
Barcha dasturlar ishlayotgan vaqtda kompyuter xotirasidan qanday foydalanishini boshqarishi kerak.
Ba'zi tillarda axlat yig'ish mavjud bo'lib, ular dastur ishlayotgan paytda ishlatilmaydigan xotirani muntazam ravishda qidiradi; boshqa tillarda dasturchi xotirani aniq ajratishi va bo'shatishi kerak. Rust uchinchi yondashuvdan foydalanadi: xotira kompilyator tekshiradigan qoidalar to'plamiga ownership tizimi orqali boshqariladi. Agar biron bir qoidalar buzilgan bo'lsa, dastur kompilatsiya qilinmaydi. Ownership xususiyatlarining hech biri dasturingiz ishlayotgan vaqtda sekinlashtirmaydi.

Ownership ko'plab dasturchilar uchun yangi tushuncha bo'lganligi sababli, unga ko'nikish uchun biroz vaqt kerak bo'ladi. Yaxshi xabar shundaki, siz Rust va ownership tizimi qoidalari bilan qanchalik tajribali bo'lsangiz, xavfsiz va samarali kodni tabiiy ravishda ishlab chiqish osonroq bo'ladi. Unda davom etamiz!

Ownershipni tushunganingizda, Rustni noyob qiladigan xususiyatlarni tushunish uchun mustahkam asosga ega bo'lasiz. Ushbu bobda, siz juda keng tarqalgan ma'lumotlar tuzilishiga qaratilgan ba'zi misollarni orqali  ownershipni ishlashini o'rganasiz: string.

> ### Stack va Heap
>
> Ko'pgina dasturlash tillari stek va heap haqida tez-tez o'ylashingizni talab qilmaydi.
> Ammo Rust kabi tizim dasturlash tilida qiymat stekda yoki heapda bo'ladimi,
> til o'zini qanday tutishiga ta'sir qiladi va nima uchun siz ma'lum qarorlar
> qabul qilishingiz kerak. Ownershipning qismlari stek va heapga nisbatan keyinchalik
> ushbu bobda tasvirlanadi, shuning uchun bu yerda tayyorgarlik jarayonida qisqacha 
> tushuntirish berilgan.
>
> Stack ham, heap ham runtimeda foydalanish uchun kodingiz uchun mavjud bo'lgan 
> xotira qismlaridir, lekin ular turli yo'llar bilan tuzilgan. Stack qiymatlarni
> ularni olgan tartibda saqlaydi va qiymatlarni teskari tartibda o'chiradi
> Bu *oxirgi kelgan, birinchi chiqqan* deb ataladi. Plitalar stackini o'ylab
> ko'ring: ko'proq plastinka qo'shsangiz, ularni qoziqning ustiga qo'yasiz va plastinka
> kerak bo'lganda, siz yuqoridan birini olib qo'yasiz. Plitalarni o'rtadan yoki pastdan
> qo'shish yoki olib tashlash ham ishlamaydi! Ma'lumotlarni qo'shish *stekga qo'shish*,
> ma'lumotlarni olib tashlash esa *stekdan o'chirish* deb ataladi. Stackda saqlangan
> barcha ma'lumotlar ma'lum, qat'iy belgilangan hajmga ega bo'lishi kerak. Kompilyatsiya vaqtida
> noma'lum o'lchamli yoki o'zgarishi mumkin bo'lgan o'lchamdagi ma'lumotlar esa heapda
> saqlanishi kerak.
>
> heap kamroq tartibga solingan: ma'lumotlarni heapga qo'yganingizda, ma'lum miqdorda
> bo'sh joy talab qilasiz. Xotira ajratuvchisi heapda etarlicha katta bo'lgan bo'sh joyni
> topadi, uni ishlatilayotgan deb belgilaydi va o'sha joyning manzili bo'lgan
> *pointerni* ni qaytaradi. Bu jarayon *heap allocating* deb ataladi va ba'zan
> faqat *allocating* deb qisqartiriladi (qiymatlarni stekga qo'shish ajratish
> hisoblanmaydi). Heapga pointer ma'lum, qat'iy o'lcham bo'lgani uchun siz
> pointerni stekda saqlashingiz mumkin, lekin haqiqiy ma'lumotlarni
> olishni istasangiz, pointergaga amal qilishingiz kerak. Restoranda o'tirganingizni
> o'ylab ko'ring. Kirish paytida siz guruhingizdagi odamlar sonini bildirasiz
> va uy egasi hammaga mos keladigan bo'sh stol topadi va sizni u yerga olib boradi.
> Agar guruhingizdagi kimdir kechikib kelsa, sizni topish uchun qayerda o'tirganingizni
> so'rashi mumkin.
>
> Stekga qo'shish heapda allocating qilishdan tezroq bo'ladi, chunki allacator hech
> qachon yangi ma'lumotlarni saqlash uchun joy izlamasligi kerak; bu joy har doim
> stackning yuqori qismida joylashgan. Nisbatan, heapda bo'sh joy ajratish ko'proq
> mehnat talab qiladi, chunki allacator avval ma'lumotlarni saqlash uchun yetarlicha
> katta joy topishi va keyingi allocatinga tayyorgarlik ko'rish uchun buxgalteriya
> hisobini amalga oshirishi kerak.
>
> Heapdagi ma'lumotlarga kirish stekdagi ma'lumotlarga kirishdan ko'ra sekinroq, chunki u yerga 
> borish uchun pointerga amal qilishingiz kerak. Zamonaviy protsessorlar xotirada
> kamroq o'tishsa, tezroq ishlaydi. O'xshashlikni davom ettirib, ko'plab jadvallardan
> buyurtmalarni qabul qiladigan restoran serverini ko'rib chiqing. Keyingi stolga o'tishdan oldin
> barcha buyurtmalarni bitta stolda olish eng samarali hisoblanadi. A jadvalidan
> buyurtma olish, keyin B jadvalidan buyurtma olish, keyin yana A dan va yana B dan bitta
> buyurtma olish ancha sekinroq jarayon bo'ladi. Xuddi shu qoidaga ko'ra,
> protsessor uzoqroqda emas (u heapda bo'lishi mumkin) emas, balki boshqa
> ma'lumotlarga yaqin (stekdagi kabi) ma'lumotlarda ishlasa, o'z ishini yaxshiroq
> bajarishi mumkin.
>
> Sizning kodingiz funksiyani chaqirganda, funksiyaga o'tgan qiymatlar (shu jumladan, potentsial,
> heapdagi ma'lumotlarga pointerlar) va funksiyaning mahalliy o'zgaruvchilari
> stekga qo'shiladi. Funktsiya tugagach, bu qiymatlar stekdan chiqariladi.
>
> Kodning qaysi qismlari heapda qaysi ma'lumotlardan foydalanayotganini kuzatib borish,
> heapdagi takroriy ma'lumotlar miqdorini minimallashtirish va bo'sh joy qolmasligi uchun
> heapdagi foydalanilmagan ma'lumotlarni tozalash - bularning barchasi ownership hal qiladigan 
> muammolardir. Ownershipni tushunganingizdan so'ng, stek va heap haqida tez-tez
> o'ylashingiz shart emas, lekin ownership qilishning asosiy maqsadi heap
> ma'lumotlarni boshqarish ekanligini bilish uning nima uchun shunday ishlashini
> tushuntirishga yordam beradi.

### Ownership qoidalari

Birinchidan, ownership qoidalarini ko'rib chiqaylik.Biz ularni ko'rsatadigan misollar bilan ishlashda ushbu qoidalarni yodda tuting:

* Rust-dagi har bir qiymat *owner*ga ega.
* Bir vaqtning o'zida faqat bitta owneri bo'lishi mumkin.
* Owneri amaldan tashqariga chiqsa, qiymat o'chiriladi.

### O'zgaruvchan Scope

Endi biz Rustning asosiy sintaksisidan o‘tganimiz uchun, biz barcha `fn main() {` kodini misollarga kiritmaymiz, shuning uchun agar kuzatib boradigan bo‘lsangiz, quyidagi misollarni `main` funksiyasiga qo‘lda kiritganingizga ishonch hosil qiling. Natijada, bizning misollarimiz biroz ixchamroq bo'ladi, bu bizga qozon kodiga emas, balki haqiqiy tafsilotlarga e'tibor berishga imkon beradi.

Ownershipning birinchi misoli sifatida biz ba'zi o'zgaruvchilarning *scope*ni ko'rib chiqamiz. Scope - dastur doirasidagi element amal qiladigan diapazon. Quyidagi o'zgaruvchini oling:

```rust
let s = "salom";
```

`s` o'zgaruvchisi satr literaliga ishora qiladi, bu yerda satr qiymati dasturimiz matniga qattiq kodlangan. O'zgaruvchi e'lon qilingan paytdan boshlab joriy *scopning* oxirigacha amal qiladi. 4-1 ro'yxatida `s` o'zgaruvchisi qayerda to'g'ri bo'lishini izohlovchi izohlar bilan dastur ko'rsatilgan.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 4-1: O'zgaruvchi va uning amal qiladigan doirasi</span>

Boshqacha qilib aytganda, bu yerda ikkita muhim nuqta bor:

* Qachonki `s` *scopega* kirsa, u amal qiladi.
* U scopedan tashqariga *chiqmaguncha* amal qiladi.

Ushbu nuqtada, scopelar va o'zgaruvchilarning haqiqiyligi o'rtasidagi munosabatlar boshqa dasturlash tillaridagiga o'xshaydi. Endi biz `String` turini joriy qilish orqali ushbu tushunchaga asoslanamiz.

### `String` turi

Ownership qoidalarini tasvirlash uchun bizga 3-bobning [”Ma'lumotlar turlari”][data-types]<!-- ignore -->
bo'limida ko'rib chiqilganlarga qaraganda murakkabroq ma'lumotlar turi kerak. Oldin ko'rib chiqilgan turlar ma'lum o'lchamga ega bo'lib, ular stekda saqlanishi va qo'llanilish doirasi tugagach, stekdan o'chirilishi mumkin va agar kodning boshqa qismi foydalanishi kerak bo'lsa yangi, mustaqil misol yaratish uchun tez va ahamiyatsiz nusxa ko'chirilishi mumkin kodning boshqa qismi bir xil qiymatni boshqa doirada ishlatishi kerak. Ammo biz heapda saqlangan ma'lumotlarni ko'rib chiqmoqchimiz va Rust bu ma'lumotlarni qachon tozalashni bilishini o'rganmoqchimiz va `String` turi ajoyib misoldir.

Biz `String` ning ownership bilan bog'liq qismlariga e'tibor qaratamiz. Ushbu jihatlar standart kutubxona tomonidan taqdim etilganmi yoki siz yaratganmi, boshqa murakkab ma'lumotlar turlariga ham tegishli.
Biz [8-bobda][ch8]<!-- ignore --> `String`ni chuqurroq muhokama qilamiz.

Biz allaqachon string literallarini ko'rdik, bu erda string qiymati bizning dasturimizga qattiq kodlangan. String literallari qulay, ammo ular biz matndan foydalanmoqchi bo'lgan har qanday vaziyatga mos kelmaydi. Buning sabablaridan biri shundaki, ular o'zgarmasdir. Yana bir narsa shundaki, biz kodni yozganimizda har bir satr qiymatini bilish mumkin emas: masalan, agar biz foydalanuvchi ma'lumotlarini olib, uni saqlamoqchi bo'lsak-chi? Bunday holatlar uchun Rust ikkinchi string turiga ega, `String`. Bu tur heapda ajratilgan ma'lumotlarni boshqaradi va shuning uchun kompilyatsiya vaqtida bizga noma'lum bo'lgan matn miqdorini saqlashi mumkin. Siz `from` funksiyasidan foydalanib satr literalidan `String` yaratishingiz mumkin, masalan:

```rust
let s = String::from("salom");
```

Ikki nuqtali `::` operatori bizga `string_from` kabi qandaydir nomdan foydalanish o'rniga `String` turi ostida ushbu `from` funksiyasini nom maydoniga qo`yish imkonini beradi.
Biz ushbu sintaksisni 5-bobning [”Method sintaksisi”][method-syntax]<!-- ignore --> bo'limida ko'proq muhokama qilamiz va 7-bobdagi [”Modul treedagi elementga murojaat qilish yo'llari”][paths-module-tree]<!-- ignore --> da modullar bilan nomlar oralig'i haqida gapiramiz.

Ushbu turdagi *string* mutatsiyaga uchragan bo'lishi mumkin:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Xo'sh, bu erda qanday farq bor? Nima uchun `String` ni mutatsiyaga solish mumkin, lekin harflarni o'zgartirish mumkin emas? Farqi bu ikki turning xotira bilan qanday munosabatda bo'lishida.

### Xotira va Taqsimlash

String literalida biz kompilyatsiya vaqtida tarkibni bilamiz, shuning uchun matn to'g'ridan-to'g'ri yakuniy bajariladigan faylga qattiq kodlangan. This is why string
literals are fast and efficient. But these properties only come from the string
literal’s immutability. Unfortunately, we can’t put a blob of memory into the
binary for each piece of text whose size is unknown at compile time and whose
size might change while running the program.

With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means:

* The memory must be requested from the memory allocator at runtime.
* We need a way of returning this memory to the allocator when we’re done with
  our `String`.

That first part is done by us: when we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages.

However, the second part is different. In languages with a *garbage collector
(GC)*, the GC keeps track of and cleans up memory that isn’t being used
anymore, and we don’t need to think about it. In most languages without a GC,
it’s our responsibility to identify when memory is no longer being used and to
call code to explicitly free it, just as we did to request it. Doing this
correctly has historically been a difficult programming problem. If we forget,
we’ll waste memory. If we do it too early, we’ll have an invalid variable. If
we do it twice, that’s a bug too. We need to pair exactly one `allocate` with
exactly one `free`.

Rust takes a different path: the memory is automatically returned once the
variable that owns it goes out of scope. Here’s a version of our scope example
from Listing 4-1 using a `String` instead of a string literal:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

There is a natural point at which we can return the memory our `String` needs
to the allocator: when `s` goes out of scope. When a variable goes out of
scope, Rust calls a special function for us. This function is called
[`drop`][drop]<!-- ignore -->, and it’s where the author of `String` can put
the code to return the memory. Rust calls `drop` automatically at the closing
curly bracket.

> Note: In C++, this pattern of deallocating resources at the end of an item’s
> lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*.
> The `drop` function in Rust will be familiar to you if you’ve used RAII
> patterns.

This pattern has a profound impact on the way Rust code is written. It may seem
simple right now, but the behavior of code can be unexpected in more
complicated situations when we want to have multiple variables use the data
we’ve allocated on the heap. Let’s explore some of those situations now.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-move"></a>

#### Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in Rust.
Let’s look at an example using an integer in Listing 4-2.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<span class="caption">Listing 4-2: Assigning the integer value of variable `x`
to `y`</span>

We can probably guess what this is doing: “bind the value `5` to `x`; then make
a copy of the value in `x` and bind it to `y`.” We now have two variables, `x`
and `y`, and both equal `5`. This is indeed what is happening, because integers
are simple values with a known, fixed size, and these two `5` values are pushed
onto the stack.

Now let’s look at the `String` version:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

This looks very similar, so we might assume that the way it works would be the
same: that is, the second line would make a copy of the value in `s1` and bind
it to `s2`. But this isn’t quite what happens.

Take a look at Figure 4-1 to see what is happening to `String` under the
covers. A `String` is made up of three parts, shown on the left: a pointer to
the memory that holds the contents of the string, a length, and a capacity.
This group of data is stored on the stack. On the right is the memory on the
heap that holds the contents.

<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">Figure 4-1: Representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span>

The length is how much memory, in bytes, the contents of the `String` are
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has received from the allocator. The difference between length and
capacity matters, but not in this context, so for now, it’s fine to ignore the
capacity.

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. In other words, the data
representation in memory looks like Figure 4-2.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-2: Representation in memory of the variable `s2`
that has a copy of the pointer, length, and capacity of `s1`</span>

The representation does *not* look like Figure 4-3, which is what memory would
look like if Rust instead copied the heap data as well. If Rust did this, the
operation `s2 = s1` could be very expensive in terms of runtime performance if
the data on the heap were large.

<img alt="Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-3: Another possibility for what `s2 = s1` might
do if Rust copied the heap data as well</span>

Earlier, we said that when a variable goes out of scope, Rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
Figure 4-2 shows both data pointers pointing to the same location. This is a
problem: when `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a *double free* error and is one of the memory
safety bugs we mentioned previously. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as
no longer valid. Therefore, Rust doesn’t need to free anything when `s1` goes
out of scope. Check out what happens when you try to use `s1` after `s2` is
created; it won’t work:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

You’ll get an error like this because Rust prevents you from using the
invalidated reference:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

If you’ve heard the terms *shallow copy* and *deep copy* while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounds like making a shallow copy. But
because Rust also invalidates the first variable, instead of being called a
shallow copy, it’s known as a *move*. In this example, we would say that `s1`
was *moved* into `s2`. So, what actually happens is shown in Figure 4-4.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">Figure 4-4: Representation in memory after `s1` has been
invalidated</span>

That solves our problem! With only `s2` valid, when it goes out of scope it
alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never
automatically create “deep” copies of your data. Therefore, any *automatic*
copying can be assumed to be inexpensive in terms of runtime performance.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-clone"></a>

#### Variables and Data Interacting with Clone

If we *do* want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We’ll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you’ve probably seen them before.

Here’s an example of the `clone` method in action:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

This works just fine and explicitly produces the behavior shown in Figure 4-3,
where the heap data *does* get copied.

When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It’s a visual indicator that something
different is going on.

#### Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code using
integers—part of which was shown in Listing 4-2—works and is valid:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

But this code seems to contradict what we just learned: we don’t have a call to
`clone`, but `x` is still valid and wasn’t moved into `y`.

The reason is that types such as integers that have a known size at compile
time are stored entirely on the stack, so copies of the actual values are quick
to make. That means there’s no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there’s no difference
between deep and shallow copying here, so calling `clone` wouldn’t do anything
different from the usual shallow copying, and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on
types that are stored on the stack, as integers are (we’ll talk more about
traits in [Chapter 10][traits]<!-- ignore -->). If a type implements the `Copy`
trait, variables that use it do not move, but rather are trivially copied,
making them still valid after assignment to another variable.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts,
has implemented the `Drop` trait. If the type needs something special to happen
when the value goes out of scope and we add the `Copy` annotation to that type,
we’ll get a compile-time error. To learn about how to add the `Copy` annotation
to your type to implement the trait, see [“Derivable
Traits”][derivable-traits]<!-- ignore --> in Appendix C.

So, what types implement the `Copy` trait? You can check the documentation for
the given type to be sure, but as a general rule, any group of simple scalar
values can implement `Copy`, and nothing that requires allocation or is some
form of resource can implement `Copy`. Here are some of the types that
implement `Copy`:

* All the integer types, such as `u32`.
* The Boolean type, `bool`, with values `true` and `false`.
* All the floating-point types, such as `f64`.
* The character type, `char`.
* Tuples, if they only contain types that also implement `Copy`. For example,
  `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

### Ownership and Functions

The mechanics of passing a value to a function are similar to those when
assigning a value to a variable. Passing a variable to a function will move or
copy, just as assignment does. Listing 4-3 has an example with some annotations
showing where variables go into and out of scope.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<span class="caption">Listing 4-3: Functions with ownership and scope
annotated</span>

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile-time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Listing 4-4 shows an example of a
function that returns some value, with similar annotations as those in Listing
4-3.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<span class="caption">Listing 4-4: Transferring ownership of return
values</span>

The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless ownership
of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every
function is a bit tedious. What if we want to let a function use a value but
not take ownership? It’s quite annoying that anything we pass in also needs to
be passed back if we want to use it again, in addition to any data resulting
from the body of the function that we might want to return as well.

Rust does let us return multiple values using a tuple, as shown in Listing 4-5.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<span class="caption">Listing 4-5: Returning ownership of parameters</span>

But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for using a value without
transferring ownership, called *references*.

[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: ../std/ops/trait.Drop.html#tymethod.drop
