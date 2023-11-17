## I/O loyihamizni takomillashtirish

Iteratorlar haqidagi yangi bilimlar bilan biz koddagi joylarni aniqroq va ixchamroq qilish uchun iteratorlardan foydalangan holda 12-bobdagi I/O(input/output)  loyihasini yaxshilashimiz mumkin. Keling, iteratorlar `Config::build` va `qidiruv` funksiyalarini amalga implement qilishni qanday yaxshilashi mumkinligini ko'rib chiqaylik.

### Iterator yordamida `clone`ni olib tashlash

12-6 roʻyxatda biz `String` qiymatlari boʻlagini olgan kodni qoʻshdik va boʻlimga indekslash va qiymatlarni klonlash orqali `Config` strukturasining namunasini yaratdik,  `Config` strukturasiga ushbu qiymatlarga ownershiplik(egalik) qilish imkonini berdi. 13-17 ro'yxatda biz 12-23 ro'yxatdagi kabi `Config::build` funksiyasining bajarilishini takrorladik:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

<span class="caption">Ro'yxat 13-17: `Config::build` funksiyasining 12-23-Ro'yxatdan takrorlanishi</span>

O'shanda biz samarasiz `clone` chaqiruvlari(call) haqida qayg'urmaslikni aytdik, chunki kelajakda ularni olib tashlaymiz. Xo'sh, bu vaqt hozir!

Bizga bu yerda `clone` kerak edi, chunki bizda `args` parametrida  `String` elementlari bo‘lgan slice bor, lekin `build` funksiyasi `args`ga ega emas. `Config` namunasiga ownershiplikni(egalik) qaytarish uchun `Config`ning `sorov` va `fayl_yoli` maydonlaridagi qiymatlarni klonlashimiz kerak edi, shunda `Config` namunasi o‘z qiymatlariga ega bo‘lishi mumkin.

Iteratorlar haqidagi yangi bilimlarimiz bilan biz `build` funksiyasini oʻzgartirib, bir sliceni olish oʻrniga iteratorga argument sifatida ownershiplik qilishimiz mumkin.
Biz slice uzunligini tekshiradigan kod o'rniga iterator funksiyasidan foydalanamiz va ma'lum joylarga ko'rsatamiz. Bu `Config::build` funksiyasi nima qilayotganini aniqlaydi, chunki iterator qiymatlarga kira oladi.

`Config::build` iteratorga ownershiplik qilib, borrow qilingan indekslash operatsiyalaridan foydalanishni to'xtatgandan so'ng, biz `clone` deb chaqirish va yangi ajratish(allocation) o'rniga `String` qiymatlarini iteratordan `Config`ga ko'chirishimiz mumkin.

#### Qaytarilgan(return) iteratordan to'g'ridan-to'g'ri foydalanish

I/O loyihangizning *src/main.rs* faylini oching, u quyidagicha ko'rinishi kerak:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Biz birinchi navbatda 12-24-Ro'yhatdagi `main` funksiyaning boshlanishini 13-18-Ro'yxatdagi kodga almashtiramiz, bu safar iteratordan foydalanadi. Biz `Config::build`ni ham yangilamagunimizcha, bu kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-18: `env::args` ning return(qaytish) qiymatini `Config::build`` ga o'tkazish</span>

`env::args` funksiyasi iteratorni qaytaradi! Iterator qiymatlarini(value) vectorga yig'ib, keyin sliceni(bo'lak) `Config::build`  ga o'tkazish o'rniga, endi biz `env::args` dan qaytarilgan(return) iteratorga ownershiplik(egalik) huquqini to'g'ridan-to'g'ri `Config::build` ga o'tkazmoqdamiz.

Keyinchalik, `Config::build` definitioni yangilashimiz kerak. I/O loyihangizning *src/lib.rs* faylida keling, `Config::build` signaturesni 13-19-raqamli roʻyxatga oʻxshatib oʻzgartiraylik. Bu hali ham kompilyatsiya qilinmaydi, chunki biz funksiya bodysini(tanasi) yangilashimiz kerak.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

<span class="caption">Listing 13-19: Updating the signature of `Config::build`
to expect an iterator</span>

The standard library documentation for the `env::args` function shows that the
type of the iterator it returns is `std::env::Args`, and that type implements
the `Iterator` trait and returns `String` values.

We’ve updated the signature of the `Config::build` function so the parameter
`args` has a generic type with the trait bounds `impl Iterator<Item = String>`
instead of `&[String]`. This usage of the `impl Trait` syntax we discussed in
the [“Traits as Parameters”][impl-trait]<!-- ignore --> section of Chapter 10
means that `args` can be any type that implements the `Iterator` type and
returns `String` items.

Because we’re taking ownership of `args` and we’ll be mutating `args` by
iterating over it, we can add the `mut` keyword into the specification of the
`args` parameter to make it mutable.

#### Using `Iterator` Trait Methods Instead of Indexing

Next, we’ll fix the body of `Config::build`. Because `args` implements the
`Iterator` trait, we know we can call the `next` method on it! Listing 13-20
updates the code from Listing 12-23 to use the `next` method:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

<span class="caption">Listing 13-20: Changing the body of `Config::build` to use
iterator methods</span>

Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. Second, we call `next` to get the
value we want to put in the `query` field of `Config`. If `next` returns a
`Some`, we use a `match` to extract the value. If it returns `None`, it means
not enough arguments were given and we return early with an `Err` value. We do
the same thing for the `file_path` value.

### Making Code Clearer with Iterator Adaptors

We can also take advantage of iterators in the `search` function in our I/O
project, which is reproduced here in Listing 13-21 as it was in Listing 12-19:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

<span class="caption">Listing 13-21: The implementation of the `search`
function from Listing 12-19</span>

We can write this code in a more concise way using iterator adaptor methods.
Doing so also lets us avoid having a mutable intermediate `results` vector. The
functional programming style prefers to minimize the amount of mutable state to
make code clearer. Removing the mutable state might enable a future enhancement
to make searching happen in parallel, because we wouldn’t have to manage
concurrent access to the `results` vector. Listing 13-22 shows this change:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 13-22: `qidiruv` funksiyasini impelement qilishda iterator adapter metodlaridan foydalanish</span>

Eslatib o'tamiz, `qidiruv` funksiyasining maqsadi `tarkib` dagi `sorov` ni o'z ichiga olgan barcha qatorlarni qaytarishdir(return). 13-16 Roʻyxatdagi `filter` misoliga oʻxshab, bu kod `filter` adapteridan faqat `line.contains(sorov)` uchun `true` qaytaradigan satrlarni saqlash uchun foydalanadi. Keyin mos keladigan qatorlarni `collect` bilan boshqa vectorga yig'amiz. Juda oddiyroq! `harflarga_etiborsiz_qidirish` funksiyasida ham iterator metodlaridan foydalanish uchun xuddi shunday o'zgartirish kiriting.

### Looplar yoki iteratorlar o'rtasida tanlash

Keyingi mantiqiy savol - o'z kodingizda qaysi uslubni tanlashingiz kerakligi va nima uchun: 13-21-Ro'yxatdagi asl dastur yoki 13-22-Ro'yxatdagi iteratorlardan foydalangan holda versiya. Aksariyat Rust dasturchilari iterator uslubidan foydalanishni afzal ko'rishadi. Avvaliga o'rganish biroz qiyinroq, lekin siz turli xil iterator adapterlari va ular nima qilishini his qilganingizdan so'ng, iteratorlarni tushunish osonroq bo'ladi. Kod aylanishning turli bitlari va yangi vectorlarni yaratish o'rniga, loop siklning yuqori darajadagi(high-level) maqsadiga e'tibor qaratadi. Bu ba'zi oddiy kodlarni abstrakt qiladi, shuning uchun ushbu kodga xos bo'lgan tushunchalarni, masalan, iteratordagi har bir element o'tishi kerak bo'lgan filtrlash shartini ko'rish osonroq bo'ladi.

Ammo ikkita dastur haqiqattan ham ekvivalentmi? Intuitiv taxmin shundan iboratki, low-leveldagi loop tezroq bo'ladi. Keling, performance haqida gapiraylik.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
