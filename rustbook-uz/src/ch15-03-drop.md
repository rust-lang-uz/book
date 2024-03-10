## `Drop` Trait bilan tozalash uchun kodni yuritish

Agar qiymat o‘z doirasidan chiqqanda uni o‘zgartirish imkonini beradigan ikkinchi muhim sanalgan smart pointer namunasidan biri bu `Drop`dir. Siz `Drop` traitini implementatsiya qilish uchun xohlagan turdan foydalanishingiz mumkin, va kodni fayl yoki tarmoqlarni ulash resurslarini yaratish uchun ham ishlatilishi mumkin 

`Drop`ni smart pointerlar kontekstida ishlatishimizning sababi `Drop` traiti smart pointerni implementatsiyasida deyarli har doim ishlatiladi. Masalan, qachonki `Box<T>` tashlab yuborilganda u quti ko‘rsatayotgan heapdan joy ajratadi.

Ayrim dasturlash tillarida ayrim turlar uchun dasturchi xotirani bo‘shatish uchun yoki har safar  resurslar o‘sha tur instancedan ishlatib bo‘lmagungacha kodni chaqirishi kerak. Fayl handlelari, soketlar va locklar bunga misol bo‘la oladi. Agar ular kodni chaqirishni unitsalar, tizimda haddan tashqari yuklanish yuzaga keladi va tizim ishdan chiqadi. Rustda agar qiymat o‘z doirasidan chiqqanda siz kodning ma’lum bir qismi ishga tushirishni belgilashingiz mumkin, kompilyator avtomatik ravishda kodni kiritadi. Natijada, ma’lum bir turdagi instance tugagan dasturning hamma joyiga tozalovchi kodni joylashtirishdan xavotir olmasangiz ham bo‘ladi va siz resurslarni sizib ketishini oldini olgan bo‘lasiz!

Siz `Drop` traiti implementatsiyasi yordamida agar qiymat doirasidan chiqqan holda kodni run qilish uchun belgilashingiz mumkin. `Drop` traiti sizdan `self`dan referens oluvchi `drop` nomli metodni implementatsiya qilishni talab qiladi. Rustda `drop` qachon chaqirilishini ko‘rish uchun, `drop`ni `println!` yordamida implementatsiya qilib ko‘raylik.

15-14 ni ko‘rib chiqadigan bo‘lsak, Rustda qachon `drop` funksiyasi ishlashini ko‘rish uchun faqat o‘ziga tegishli bo‘lgan`CustomSmartPointer` structi faqat agar instance o‘z doirasidan chiqqanda `Dropping CustomSmartPointer!` ni print qiladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<span class="caption">15-14ni ko'rib chiqish: `CustomSmartPointer` structi biz tozalash uchun qo’ygan kodda `Drop` traitining implementatsiyasi</span>

`Drop` traiti o‘z ichiga preludeni oladi, shuning uchun biz uni scopeni ichiga olishimiz shart emas. Biz `CustomSmartPointer`da `Drop`ni implementatsiya qilamiz va `drop` metodi implementatisyasi uchun `println!`ni chaqiramiz. `drop` funksiyasining tana (body) qismi bu sizning turdagi instance o‘z doirasidan (scope) chiqib ketgandagi ayrim bir logikaga ega koddir. Rustda qachon `drop` chaqirilishini ko‘rish uchun biz ozgina tekstni print qilamiz.

`main`da biz 2ta `CustomSmartPointer` instancelarini yaratamiz va keyin `CustomSmartPointers yaratildi`ni print qilamiz. `main`ning oxirida `CustomSmartPointer` doiradan (scope) chiqib ketadi va Rust yakuniy xabarni print qilib, biz kodga qo‘ygan `drop` metodini chaqiradi. E’tibor bering biz `drop` metodini to‘g‘ridan-to‘g‘ri chaqririshimiz shart emas.

Agar biz dasturni run qilsak, quyidagi outputni ko‘ramiz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust avtomatik ravishda bizning o‘rnimizga biz ko‘rsatgan kodni instance doiradan (scope) chiqqanda `drop`ni chaqirdi. O‘zgaruvchilar yaratilish paytida teskari tartibda tushib qoldiriladi (drop qilinadi), shuning uchun `d` `c`dan oldin tushib qoldirildi (drop qilindi). Ushbu misolning maqsadi sizga `drop` metodining qanday ishlashining vizual ko‘rinishini berishdir; odatda xabarni print qilishning o‘rniga siz sizning turingizni ishga tushirish (run qilish) uchun tozalash kodini ko‘rsatasiz. 

### `std::mem::drop` yordamida Qiymatni Erta Drop qilish

Afsuski, avtomatik `drop` funksiyasini o‘chirish oson emas.  Odatda `drop`ni o‘chirish zarur emas; `Drop`ning asosiy mohiyati uning avtomatik ravishda hal qilishidir. Ba’zi paytlarda siz qiymatni erta tozalashga duch kelishingiz mumkin. Lockalarni boshqaruvchi smart pointerlarni ishlatishga bir misol bo‘la oladi:  bir doirada (scope)da boshqa kodni olish uchun siz lockni chaqiradigan `drop` metodini majburiy ravishda ishlatishingiz mumkin. Rust sizga `Drop` traitidagi `drop` metodini qo‘lda tushurishga qo‘ymaydi; agar siz qiymatni o‘z doirani (scope) tugashidan oldin majburiy drop bo‘lishini xohlasangiz. uning uchun siz standart kutubxona tomonidan taqdim etilgan `std::mem::drop`ni ishlatishingiz mumkin.

Agar biz 15-14dagi ilovaga qo‘lda `Drop` traitining `drop` metodi yordamida `main`ga o‘zgaritirish kiratigan bo‘lsak, 15-15 ilovada ko‘rsatilgan kompilyator xatosini ko‘ramiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<span class="caption">Listing 15-15: Attempting to call the `drop` method from
the `Drop` trait manually to clean up early</span>

When we try to compile this code, we’ll get this error:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

This error message states that we’re not allowed to explicitly call `drop`. The
error message uses the term *destructor*, which is the general programming term
for a function that cleans up an instance. A *destructor* is analogous to a
*constructor*, which creates an instance. The `drop` function in Rust is one
particular destructor.

Rust doesn’t let us call `drop` explicitly because Rust would still
automatically call `drop` on the value at the end of `main`. This would cause a
*double free* error because Rust would be trying to clean up the same value
twice.

We can’t disable the automatic insertion of `drop` when a value goes out of
scope, and we can’t call the `drop` method explicitly. So, if we need to force
a value to be cleaned up early, we use the `std::mem::drop` function.

The `std::mem::drop` function is different from the `drop` method in the `Drop`
trait. We call it by passing as an argument the value we want to force drop.
The function is in the prelude, so we can modify `main` in Listing 15-15 to
call the `drop` function, as shown in Listing 15-16:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<span class="caption">Listing 15-16: Calling `std::mem::drop` to explicitly
drop a value before it goes out of scope</span>

Running this code will print the following:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

The text ```Dropping CustomSmartPointer with data `some data`!``` is printed
between the `CustomSmartPointer created.` and `CustomSmartPointer dropped
before the end of main.` text, showing that the `drop` method code is called to
drop `c` at that point.

You can use code specified in a `Drop` trait implementation in many ways to
make cleanup convenient and safe: for instance, you could use it to create your
own memory allocator! With the `Drop` trait and Rust’s ownership system, you
don’t have to remember to clean up because Rust does it automatically.

You also don’t have to worry about problems resulting from accidentally
cleaning up values still in use: the ownership system that makes sure
references are always valid also ensures that `drop` gets called only once when
the value is no longer being used.

Now that we’ve examined `Box<T>` and some of the characteristics of smart
pointers, let’s look at a few other smart pointers defined in the standard
library.
