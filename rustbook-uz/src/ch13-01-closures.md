<!-- Old heading. Do not remove or links may break. -->
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Closurelar: Environmentni qamrab oladigan anonim funksiyalar

Rustning closureri - bu o'zgaruvchida saqlashingiz yoki boshqa funksiyalarga argument sifatida o'tishingiz mumkin bo'lgan anonim funktsiyalar. Closureni bir joyda yaratishingiz va keyin uni boshqa kontekstda baholash uchun boshqa joyga murojaat qilishingiz mumkin. Funksiyalardan farqli o'laroq, closurelar ular belgilangan doiradagi qiymatlarni olishlari mumkin.
Ushbu closure xususiyatlari kodni qayta ishlatish va xatti-harakatlarni moslashtirishga(behavior customization) qanday imkon berishini ko'rsatamiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Environmentni closurelar bilan qo'lga olish

Avvalo, keyinchalik foydalanish uchun ular belgilangan muhitdan(environment) qiymatlarni olish uchun closurelardan qanday foydalanishimiz mumkinligini ko'rib chiqamiz.Bu senariy: Ko'pincha bizning futbolka kompaniyamiz reklama ro'yxatidagi kimgadir eksklyuziv, cheklangan nashrdagi futbolkani sovg'a sifatida taqdim etadi. Pochta ro'yxatidagi odamlar ixtiyoriy ravishda o'z profillariga sevimli ranglarini qo'shishlari mumkin. Agar bepul futbolka uchun tanlangan kishi o'zining sevimli ranglar to'plamiga ega bo'lsa, u rangdagi futbolkani oladi. Agar biror kishi sevimli rangni ko'rsatmagan bo'lsa, u kompaniyada eng ko'p bo'lgan rangni oladi.

Buni amalga oshirishning ko'plab usullari mavjud. Ushbu misol uchun biz `Qizil` va `Moviy` variantlariga ega `FutbolkaRangi` nomli enumdan foydalanamiz (oddiylik uchun mavjud ranglar sonini cheklaydi). Biz kompaniya inventarini `Inventarizatsiya` strukturasi bilan ifodalaymiz, unda `futbolkalar` deb nomlangan maydon mavjud bo‘lib, unda hozirda mavjud bo‘lgan futbolka ranglarini ifodalovchi `Vec<FutbolkaRangi>` mavjud.
`Inventarizatsiya` da belgilangan `yutuq` metodi bepul futbolka g‘olibining ixtiyoriy futbolka rangini afzal ko‘radi va odam oladigan futbolka rangini qaytaradi. Ushbu sozlash 13-1 ro'yxatda ko'rsatilgan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Ro'yxat 13-1: Futbolka kompaniyasining sovg'a holati</span>

`main` boʻlimida belgilangan `dokon` ikkita moviy futbolka va bitta qizil futbolka qolgan. Qizil ko'ylakni afzal ko'rgan foydalanuvchi va hech qanday imtiyozsiz foydalanuvchi uchun `yutuq` metodini chaqiramiz.

Shunga qaramay, ushbu kod ko'p jihatdan amalga oshirilishi mumkin va bu yerda, closurelarga e'tibor qaratish uchun biz siz allaqachon o'rgangan tushunchalarga yopishib oldik, closuredan foydalanadigan `yutuq` metodidan tashqari. `yutuq` metodida biz `Option<FutbolkaRangi>` turidagi parametr sifatida foydalanuvchi imtiyozini olamiz va `foydalanuvchi_afzalligi` da `unwrap_or_else` metodini chaqiramiz. [`Option<T>` da `unwrap_or_else`][unwrap-or-else]<!-- ignore --> metodi standart kutubxona tomonidan aniqlanadi. Buning uchun bitta argument kerak bo‘ladi: `T` qiymatini qaytaruvchi hech qanday argumentsiz closure (`Option<T>` enumning `Some` variantida, bizning holatimizda `FutbolkaRangi`da tugaydigan qiymat turiga aylantiriladi). Agar `Option<T>` `Some` varianti bo'lsa, `unwrap_or_else` qiymatini `Some` ichidan qaytaradi. Agar `Option<T>` `None` varianti bo'lsa, `unwrap_or_else` closureni chaqiradi va closure orqali qaytarilgan qiymatni qaytaradi.

Biz closure ifodasini belgilaymiz `|| self.most_stocked()`ni `unwrap_or_else` argumenti sifatida. Bu hech qanday parametrlarni o'zi qabul qilmaydigan closuredir (agar closure parametrlari bo'lsa, ular ikkita vertikal chiziq orasida paydo bo'ladi). Closurening asosiy qismi `self.most_stocked()` ni chaqiradi. Biz bu yerda closureni aniqlayapmiz va `unwrap_or_else` ni amalga oshirish, agar natija kerak bo‘lsa, keyinroq closureni baholaydi.

Ushbu kodni ishga tushirsak quyidagi natijani chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Qiziqarli tomoni shundaki, biz joriy `Inventarizatsiya` misolida `self.most_stocked()` deb nomlanuvchi closuredan o‘tdik. Standart kutubxona biz belgilagan `Inventarizatsiya` yoki `FutbolkaRangi` turlari yoki biz ushbu senariyda foydalanmoqchi bo'lgan mantiq haqida hech narsa bilishi shart emas edi. Closure `self`  `Inventarizatsiya` misoliga o'zgarmas(immutable) referenceni oladi va uni biz belgilagan kod bilan `unwrap_or_else` metodiga uzatadi. Funksiyalar esa o'z muhitini(environmentini) shu tarzda ushlab tura olmaydi.

### Closure typi Inference va Annotation

Funksiyalar va closurelar o'rtasida ko'proq farqlar mavjud. Closurelar odatda parametrlar turlarini yoki `fn` funksiyalari kabi qaytarish qiymatini(return value) izohlashni talab qilmaydi. Funksiyalar uchun tur annotationlari talab qilinadi, chunki turlar foydalanuvchilarga ochiq interfeysning bir qismidir. Ushbu interfeysni qat'iy belgilash, har bir kishi funksiya qanday turdagi qiymatlardan foydalanishi va qaytarishi(return) haqida kelishib olishini ta'minlash uchun muhimdir. Boshqa tomondan, closurelar bu kabi ochiq interfeysda ishlatilmaydi: ular o'zgaruvchilarda saqlanadi va ularni nomlamasdan va kutubxonamiz foydalanuvchilariga ko'rsatmasdan foydalaniladi.

Closurelar odatda qisqa va har qanday ixtiyoriy senariyda emas, faqat tor kontekstda tegishli. Ushbu cheklangan kontekstlarda kompilyator ko'pgina o'zgaruvchilarning turlarini qanday aniqlashga qodir bo'lganiga o'xshab, parametrlarning turlarini va qaytish turini taxmin qilishi mumkin (kompilyatorga closure turi annotationlari ham kerak bo'lgan kamdan-kam holatlar mavjud).

O'zgaruvchilarda bo'lgani kabi, agar biz aniqlik va ravshanlikni oshirishni xohlasak, zarur bo'lgandan ko'ra batafsilroq bo'lish uchun turdagi annotationlarni qo'shishimiz mumkin. Closure uchun turlarga izoh(annotation) qo'yish 13-2 ro'yxatda ko'rsatilgan definitionga o'xshaydi. Ushbu misolda biz closureni aniqlaymiz va uni 13-1 ro'yxatda bo'lgani kabi argument sifatida topshirgan joyda closureni belgilash o'rniga uni o'zgaruvchida saqlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-2: Ixtiyoriy turdagi annotationlarni qo'shish
closureda parametr va qaytariladigan qiymat turlari</span>

Turga annotationlar qoʻshilishi bilan closure sintaksisi funksiyalar sintaksisiga koʻproq oʻxshaydi. Bu yerda biz taqqoslash uchun parametriga 1 qo'shadigan funksiyani va bir xil xatti-harakatlarga ega bo'lgan closureni aniqlaymiz. Tegishli qismlarni bir qatorga qo'yish uchun bir nechta bo'shliqlar qo'shdik. Bu pipelardan foydalanish va ixtiyoriy bo'lgan sintaksis miqdori bundan mustasno, closure sintaksisi funksiya sintaksisiga qanchalik o'xshashligini ko'rsatadi:

```rust,ignore
fn  bitta_v1_qoshish    (x: u32) -> u32 { x + 1 }
let bitta_v2_qoshish =  |x: u32| -> u32 { x + 1 };
let bitta_v3_qoshish =  |x|             { x + 1 };
let bitta_v4_qoshish =  |x|               x + 1  ;
```

Birinchi qatorda funksiya taʼrifi(definition), ikkinchi qatorda esa toʻliq izohlangan closure definitioni koʻrsatilgan. Uchinchi qatorda biz closure definitiondan turdagi annotationlarni olib tashlaymiz. To'rtinchi qatorda biz qavslarni olib tashlaymiz, ular ixtiyoriy, chunki closure tanas(body) faqat bitta ifodaga(expression) ega. Bularning barchasi to'g'ri definitionlar bo'lib, ular chaqirilganda bir xil xatti-harakatlarni keltirib chiqaradi. `bitta_v3_qoshish` va `bitta_v4_qoshish` qatorlari kompilyatsiya qilish uchun closurelarni baholashni talab qiladi, chunki turlar ulardan foydalanishdan kelib chiqadi. Bu `let v = Vec::new();` ga o'xshash bo'lib, Rust turini aniqlay olishi uchun `Vec` ga turiga izohlar(annotation) yoki ba'zi turdagi qiymatlar kiritilishi kerak.

Closure definitionlari uchun kompilyator ularning har bir parametri va ularning qaytish(return) qiymati uchun bitta aniq turdagi xulosa chiqaradi. Masalan, 13-3 ro'yxatda parametr sifatida qabul qilingan qiymatni qaytaradigan qisqa closure definitioni ko'rsatilgan. Ushbu closure ushbu misol maqsadlaridan tashqari juda foydali emas. E'tibor bering, biz definitionga hech qanday annotation qo'shmaganmiz.
Hech qanday turdagi annotationlar mavjud emasligi sababli, biz bu yerda birinchi marta `String` bilan qilgan har qanday turdagi closureni chaqirishimiz mumkin. Agar biz `namuna_closure` ni butun(integer) son bilan chaqirishga harakat qilsak, xatoga yo'l qo'yamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-3: Ikki xil turga ega bo'lgan closureni chaqirishga urinish</span>

Kompilyator bizga quyidagi xatoni beradi:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

Birinchi marta `namuna_closure` `String` qiymati bilan chaqirilganda, kompilyator `x` turini va  closurening qaytish turini `String` deb hisoblaydi. Keyin bu turlar(type) `namuna_closure` bo'limida yopiladi va biz bir xil closure(yopilish) bilan boshqa turdan foydalanishga uringanimizda xatoga duch kelamiz.

### Malumot olish yoki Egalik(Ownership) huquqini ko'chirish

Closurelar o'z muhitidan qiymatlarni uchta usulda olishlari mumkin, ular to'g'ridan-to'g'ri funksiya parametr olishi mumkin bo'lgan uchta usulga mos keladi: immutably borrowing (o'zgarmas borrowing(qarz olish)), mutably borrowing (o'zgaruvchan borrowing(qarz olish)) va egalik qilish(ownership). Closure funksiya tanasi(body) olingan qiymatlar bilan nima qilishiga qarab ulardan qaysi birini ishlatishni hal qiladi.

13-4 ro'yxatda biz `list` deb nomlangan vectorga immutable(o'zgarmas) referencei qamrab oluvchi closureni aniqlaymiz, chunki u qiymatni chop etish uchun faqat immutable referencega muhtoj:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">Ro'yxat 13-4: Buni closureni aniqlash va chaqirish
immutable referenceni ushlaydi</span>

Ushbu misol, shuningdek, o'zgaruvchining closure definitioniga bog'lanishi mumkinligini ko'rsatadi va biz keyinchalik o'zgaruvchi nomi va qavslar yordamida o'zgaruvchi nomi funksiya nomiga o'xshab yopishni chaqirishimiz mumkin.

Biz bir vaqtning o'zida bir nechta immutable(o'zgarmas) referencelarga ega bo'lishimiz mumkin bo'lgan `list` uchun, `list` closure definitionidan oldin, closure definitionidan keyin, lekin closure chaqirilishidan oldin va closure chaqirilgandan keyin hali ham koddan foydalanish mumkin. Ushbu kod kompilyatsiya bo'ladi, ishlaydi va chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Keyinchalik, 13-5 ro'yxatda biz closure bodysini `list` vectoriga element qo'shishi uchun o'zgartiramiz. Closure endi mutable(o'zgaruvchan) referenceni oladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">Ro'yxat 13-5: Mutable(o'zgaruvchan) referenceni ushlaydigan closureni aniqlash va chaqirish</span>

Ushbu kod kompilyatsiya bo'ladi, ishlaydi va chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Note that there’s no longer a `println!` between the definition and the call of
the `borrows_mutably` closure: when `borrows_mutably` is defined, it captures a
mutable reference to `list`. We don’t use the closure again after the closure
is called, so the mutable borrow ends. Between the closure definition and the
closure call, an immutable borrow to print isn’t allowed because no other
borrows are allowed when there’s a mutable borrow. Try adding a `println!`
there to see what error message you get!

If you want to force the closure to take ownership of the values it uses in the
environment even though the body of the closure doesn’t strictly need
ownership, you can use the `move` keyword before the parameter list.

This technique is mostly useful when passing a closure to a new thread to move
the data so that it’s owned by the new thread. We’ll discuss threads and why
you would want to use them in detail in Chapter 16 when we talk about
concurrency, but for now, let’s briefly explore spawning a new thread using a
closure that needs the `move` keyword. Listing 13-6 shows Listing 13-4 modified
to print the vector in a new thread rather than in the main thread:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<span class="caption">Listing 13-6: Using `move` to force the closure for the
thread to take ownership of `list`</span>

We spawn a new thread, giving the thread a closure to run as an argument. The
closure body prints out the list. In Listing 13-4, the closure only captured
`list` using an immutable reference because that's the least amount of access
to `list` needed to print it. In this example, even though the closure body
still only needs an immutable reference, we need to specify that `list` should
be moved into the closure by putting the `move` keyword at the beginning of the
closure definition. The new thread might finish before the rest of the main
thread finishes, or the main thread might finish first. If the main thread
maintained ownership of `list` but ended before the new thread did and dropped
`list`, the immutable reference in the thread would be invalid. Therefore, the
compiler requires that `list` be moved into the closure given to the new thread
so the reference will be valid. Try removing the `move` keyword or using `list`
in the main thread after the closure is defined to see what compiler errors you
get!

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Moving Captured Values Out of Closures and the `Fn` Traits

Once a closure has captured a reference or captured ownership of a value from
the environment where the closure is defined (thus affecting what, if anything,
is moved *into* the closure), the code in the body of the closure defines what
happens to the references or values when the closure is evaluated later (thus
affecting what, if anything, is moved *out of* the closure). A closure body can
do any of the following: move a captured value out of the closure, mutate the
captured value, neither move nor mutate the value, or capture nothing from the
environment to begin with.

The way a closure captures and handles values from the environment affects
which traits the closure implements, and traits are how functions and structs
can specify what kinds of closures they can use. Closures will automatically
implement one, two, or all three of these `Fn` traits, in an additive fashion,
depending on how the closure’s body handles the values:

1. `FnOnce` applies to closures that can be called once. All closures implement
   at least this trait, because all closures can be called. A closure that
   moves captured values out of its body will only implement `FnOnce` and none
   of the other `Fn` traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their
   body, but that might mutate the captured values. These closures can be
   called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body
   and that don’t mutate captured values, as well as closures that capture
   nothing from their environment. These closures can be called more than once
   without mutating their environment, which is important in cases such as
   calling a closure multiple times concurrently.

Let’s look at the definition of the `unwrap_or_else` method on `Option<T>` that
we used in Listing 13-1:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Recall that `T` is the generic type representing the type of the value in the
`Some` variant of an `Option`. That type `T` is also the return type of the
`unwrap_or_else` function: code that calls `unwrap_or_else` on an
`Option<String>`, for example, will get a `String`.

Next, notice that the `unwrap_or_else` function has the additional generic type
parameter `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.

The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called once, take no arguments, and return a `T`.
Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` is only going to call `f` at most one time. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts the most different kinds
of closures and is as flexible as it can be.

> Note: Functions can implement all three of the `Fn` traits too. If what we
> want to do doesn’t require capturing a value from the environment, we can use
> the name of a function rather than a closure where we need something that
> implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value,
> we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the
> value is `None`.

Now let’s look at the standard library method `sort_by_key` defined on slices,
to see how that differs from `unwrap_or_else` and why `sort_by_key` uses
`FnMut` instead of `FnOnce` for the trait bound. The closure gets one argument
in the form of a reference to the current item in the slice being considered,
and returns a value of type `K` that can be ordered. This function is useful
when you want to sort a slice by a particular attribute of each item. In
Listing 13-7, we have a list of `Rectangle` instances and we use `sort_by_key`
to order them by their `width` attribute from low to high:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">Listing 13-7: Using `sort_by_key` to order rectangles by
width</span>

This code prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn’t capture, mutate, or move out anything from its environment, so
it meets the trait bound requirements.

In contrast, Listing 13-8 shows an example of a closure that implements just
the `FnOnce` trait, because it moves a value out of the environment. The
compiler won’t let us use this closure with `sort_by_key`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">Listing 13-8: Attempting to use an `FnOnce` closure with
`sort_by_key`</span>

This is a contrived, convoluted way (that doesn’t work) to try and count the
number of times `sort_by_key` gets called when sorting `list`. This code
attempts to do this counting by pushing `value`—a `String` from the closure’s
environment—into the `sort_operations` vector. The closure captures `value`
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn’t work because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can’t be moved out of the closure because the closure must
implement `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn’t
move values out of the environment. To count the number of times `sort_by_key`
is called, keeping a counter in the environment and incrementing its value in
the closure body is a more straightforward way to calculate that. The closure
in Listing 13-9 works with `sort_by_key` because it is only capturing a mutable
reference to the `num_sort_operations` counter and can therefore be called more
than once:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">Listing 13-9: Using an `FnMut` closure with `sort_by_key`
is allowed</span>

The `Fn` traits are important when defining or using functions or types that
make use of closures. In the next section, we’ll discuss iterators. Many
iterator methods take closure arguments, so keep these closure details in mind
as we continue!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
