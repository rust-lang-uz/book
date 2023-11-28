## Ma'lumotlarni Heapga joylash uchun `Box<T>` dan foydalanish

Eng sodda smart pointer bu *box* bo'lib, uning turi `Box<T>` deb yoziladi.
Boxlar sizga ma'lumotlarni stackda emas, balki heapda saqlashga imkon beradi.
Stackda esa heapdagi ma'lumotlariga pointer qoladi. Stack va heap o'rtasidagi
farqni ko'rib chiqish uchun 4-bobga qarang.

Boxlar o'z ma'lumotlarini stackda emas, balki heapda saqlashdan tashqari,
ishlash bo'yicha qo'shimcha xarajatlarga ega emas. Lekin ularda ko'p qo'shimcha
imkoniyatlar ham yo'q. Siz ulardan ko'pincha quyidagi holatlarda foydalanasiz:

* Agar sizda kompilyatsiya vaqtida o'lchami noma'lum bo'lgan tur mavjud bo'lsa
  va siz aniq o'lchamni talab qiladigan kontekstda ushbu turdagi qiymatdan
  foydalanmoqchi bo'lsangiz
* Agar sizda katta hajmdagi maʼlumotlar mavjud boʻlsa va siz egalik huquqini
  oʻtkazganingizda maʼlumotlardan nusxa olinmasligiga ishonch hosil qilmoqchi
  bo'lsangiz
* Agar siz biror qiymatga egalik qilmoqchi bo'lsangiz va siz uni ma'lum bir
  turda bo'lishiga emas, balki ma'lum bir traitni implement qiluvchi tur
  bo'lishi haqida qayg'ursangiz

Birinchi holatni [“Rekursiv turlarni Boxlar bilan qo'llash”](#rekursiv-turlarni-boxlar-bilan-qollash)<!-- ignore --> bo‘limida ko‘rsatamiz. Ikkinchi holatda, katta hajmdagi ma'lumotlarga egalik huquqini o'tkazish uzoq vaqt talab qilishi mumkin, chunki ma'lumotlar stackdan ko'chiriladi. Bunday vaziyatda ishlashni yaxshilash uchun biz katta hajmdagi ma'lumotlarni heapda box ichida saqlashimiz mumkin. Shundan so'ng, pointer ma'lumotlarining faqat kichik miqdori stackdan ko'chiriladi, heapdagi u reference qilingan ma'lumotlar esa bir joyda qoladi. Uchinchi holat *trait object* sifatida tanilgan va butun 17-bob shu mavzuga bag'ishlangan, [“Turli turdagi qiymatlarga ruxsat beruvchi Trait Objectlaridan foydalanish”][trait-objects]<!-- ignore --> o'sha mavzu. Shunday qilib, bu erda o'rgangan narsalaringizni 17-bobda yana qo'llaysiz!

### Heapda ma'lumotlarni saqlash uchun `Box<T>` dan foydalanish

`Box<T>` uchun heap xotiradan foydalanish holatini muhokama qilishdan oldin, biz sintaksisni va `Box<T>` ichida saqlangan qiymatlar bilan qanday o'zaro aloqa qilishni ko`rib chiqamiz.

15-1 ro'yxatda `i32` qiymatini heapda saqlash uchun boxdan qanday foydalanish ko'rsatilgan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

<span class="caption">Ro'yxat 15-1: `i32` qiymatini box yordamida heapda saqlash</span>

Biz `b` o'zgaruvchini heapda joylashgan `5`ga point qiluvchi `Box` qiymatiga ega bo'lishi uchun e'lon qilamiz. Ushbu dastur `b = 5` ni chop etadi; bu holda biz boxdagi ma'lumotlarga, stackda bo'lgani kabi kirishimiz mumkin. `main`ning oxiridagi `b` kabi boxlar scopedan chiqib ketganda, xuddi egalik qilingan qiymatlarga o'xshab, u ham xotiradan o'chiriladi. O'chirilish ham box (stackda saqlanuvchi) uchun va u point qiluvchi ma'lumotlar (heapda saqlanuvchi) uchun ham sodir bo'ladi.

Heapda bitta qiymat saqlash unchalik foydali emas, shuning uchun boxlarni o'zini bu tarzda ko'pincha ishlatmaysiz. Ko'pincha holatlarda bitta `i32` kabi qiymatlarni stackda saqlash maqsadga muvofiq bo'ladi. Keling, boxlar, agar bizda boxlar bo'lmasa, ruxsat berilmaydigan turlarni e'lon qilishga imkon beradigan holatni ko'rib chiqaylik.

### Rekursiv turlarni Boxlar bilan qo'llash

*Rekursiv tur*ning qiymati o'zining bir qismi sifatida bir xil turdagi boshqa qiymatga ega bo'lishi mumkin. Rekursiv turlar muammo tug'diradi, chunki kompilyatsiya vaqtida Rust tur qancha joy egallashini bilishi kerak. Biroq, rekursiv turdagi qiymatlarni joylashtirish nazariy jihatdan cheksiz davom etishi mumkin, shuning uchun Rust qiymat uchun qancha joy kerakligini bilmaydi. Boxlar ma'lum o'lchamga ega bo'lganligi sababli, biz rekursiv tur ta'rifiga box kiritish orqali rekursiv turlarni qo'llashimiz mumkin.

Rekursiv turga misol sifatida keling, *cons list*ni o'rganamiz. Bu funktsional dasturlash tillarida keng tarqalgan ma'lumotlar turi hisoblanadi. Biz e'lon qiladigan cons list turi rekursiyadan tashqari sodda; shuning uchun biz ishlaydigan misoldagi tushunchalar rekursiv turlarni o'z ichiga olgan murakkab vaziyatlarga tushganingizda foydali bo'ladi.

#### Cons List haqida batafsil ma'lumot

*Cons list* Lisp dasturlash tili va uning dialektlaridan kelib chiqqan va ichma-ich juftliklardan tashkil topgan maʼlumotlar strukturasi boʻlib, linked listning Lispdagi versiyasi hisoblanadi. Uning nomi Lispdagi `cons` funktsiyasidan (“construct function” uchun qisqartma) kelib chiqqan bo'lib, uning ikkita argumentidan yangi juftlik yaratadi. Qiymat va boshqa juftlikdan iborat bo'lgan juftlikda `cons` ni chaqirish orqali biz rekursiv juftliklardan iborat bo'lgan cons list tuzishimiz mumkin.

Misol uchun, bu yerda 1, 2, 3 ro'yxatini o'z ichiga olgan cons listining psevdokod ko'rinishi, har bir juft qavs ichida:

```text
(1, (2, (3, Nil)))
```

Cons listdagi har bir element ikkita elementni o'z ichiga oladi: shu elementning qiymati va keyingi element. Ro'yxatning oxirgi elementida keyingi elementsiz faqat `Nil` deb nomlangan qiymatdan iborat bo'ladi. Cons list `cons` funksiyasini rekursiv chaqirish orqali hosil qilinadi. Rekursiyaning tubidagi holatini bildiruvchi qoidaga aylangan nom `Nil` hisoblanadi. E'tibor bering, bu 6-bobdagi “null” yoki “nil” tushunchasi bilan bir xil emas, ya'ni noto'g'ri yoki yo'q qiymat.

Cons list ma'lumotlar tuzilmasi Rust-da tez-tez ishlatilmaydi. Ko'pincha Rust-da sizga elementlar ro'yxati kerak bo'lsa, `Vec<T>` foydalanish uchun yaxshiroq tanlovdir. Boshqa, murakkabroq rekursiv *ma'lumot turlaridan* foyladanish bir qancha vaziyatlarda foydalidir, ammo ushbu bobdagi cons listdan boshlab, boxlar qanday qilib, rekursiv ma'lumot turini e'lon qilishga imkon berishini o'rganishimiz mumkin.

Ro'yxat 15-2 cons list uchun enum ko'rinishini o'z ichiga oladi. E'tibor bering, ushbu kod hali kompilyatsiya qilinmaydi, chunki `List` turi ma'lum hajmga ega emas, biz buni tushuntiramiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```
<span class="caption">Ro'yxat 15-2: `i32` qiymatlarining cons list ma'lumotlar tuzilmasida ifodalash uchun enumni e'lon qilishga birinchi urish</span>

> Eslatma: Biz ushbu misol maqsadlari uchun faqat `i32` qiymatlarini o'z ichiga olgan cons listni amalga oshirmoqdamiz. Biz 10-bobda muhokama qilganimizdek, har qanday turdagi qiymatlarni saqlashi mumkin bo'lgan cons list turini generiklar yordamida e'lon qilishimiz  mumkin edi.

`List` turidan foydalanib `1, 2, 3` roʻyxatini saqlash 15-3 ro'yxat kabi bo'ladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

<span class="caption">15-3 roʻyxat: `1, 2, 3` roʻyxatini saqlash uchun `List` enumidan foydalanish</span>

Birinchi `Cons` qiymati `1` va boshqa `List` qiymatiga ega. Bu `List` qiymati `2` va boshqa `List` qiymatiga ega bo'lgan boshqa `Cons` qiymatidir. Ushbu `List` qiymati `3` ni o'z ichiga olgan yana bitta `Cons` qiymati va `List` qiymati, nihoyat `Nil`, ro'yxat oxirini bildiruvchi rekursiv bo'lmagan variant.

Agar biz 15-3 ro'yxatdagi kodni kompilyatsiya qilishga harakat qilsak, biz 15-4 ro'yxatda ko'rsatilgan xatoni olamiz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

<span class="caption">Ro'yxat 15-4: Rekursiv enumni e'lon qilishga urinishda yuzaga keladigan xato</span>

Xato ushbu tur “cheksiz o'lchamga ega” ekanligini ko'rsatadi. Buning sababi shundaki, biz `List`ni rekursiv variant bilan e'lon qildik: u bevosita o'zining boshqa qiymatini saqlaydi. Natijada, Rust `List` qiymatini saqlash uchun qancha joy kerakligini aniqlay olmaydi. Keling, nima uchun bu xatoga duch kelganimizni qismlarga ajratamiz. Birinchidan, Rust rekursiv bo'lmagan turdagi qiymatni saqlash uchun qancha joy kerakligini qanday hal qilishini ko'rib chiqamiz.

#### Rekursiv bo'lmagan turning o'lchamini hisoblash

Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

To determine how much space to allocate for a `Message` value, Rust goes
through each of the variants to see which variant needs the most space. Rust
sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough
space to store two `i32` values, and so forth. Because only one variant will be
used, the most space a `Message` value will need is the space it would take to
store the largest of its variants.

Contrast this with what happens when Rust tries to determine how much space a
recursive type like the `List` enum in Listing 15-2 needs. The compiler starts
by looking at the `Cons` variant, which holds a value of type `i32` and a value
of type `List`. Therefore, `Cons` needs an amount of space equal to the size of
an `i32` plus the size of a `List`. To figure out how much memory the `List`
type needs, the compiler looks at the variants, starting with the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, and this process continues infinitely, as shown in Figure 15-1.

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-1: An infinite `List` consisting of infinite
`Cons` variants</span>

#### Using `Box<T>` to Get a Recursive Type with a Known Size

Because Rust can’t figure out how much space to allocate for recursively
defined types, the compiler gives an error with this helpful suggestion:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

In this suggestion, “indirection” means that instead of storing a value
directly, we should change the data structure to store the value indirectly by
storing a pointer to the value instead.

Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>`
needs: a pointer’s size doesn’t change based on the amount of data it’s
pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead
of another `List` value directly. The `Box<T>` will point to the next `List`
value that will be on the heap rather than inside the `Cons` variant.
Conceptually, we still have a list, created with lists holding other lists, but
this implementation is now more like placing the items next to one another
rather than inside one another.

We can change the definition of the `List` enum in Listing 15-2 and the usage
of the `List` in Listing 15-3 to the code in Listing 15-5, which will compile:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

<span class="caption">Listing 15-5: Definition of `List` that uses `Box<T>` in
order to have a known size</span>

The `Cons` variant needs the size of an `i32` plus the space to store the
box’s pointer data. The `Nil` variant stores no values, so it needs less space
than the `Cons` variant. We now know that any `List` value will take up the
size of an `i32` plus the size of a box’s pointer data. By using a box, we’ve
broken the infinite, recursive chain, so the compiler can figure out the size
it needs to store a `List` value. Figure 15-2 shows what the `Cons` variant
looks like now.

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figure 15-2: A `List` that is not infinitely sized
because `Cons` holds a `Box`</span>

Boxes provide only the indirection and heap allocation; they don’t have any
other special capabilities, like those we’ll see with the other smart pointer
types. They also don’t have the performance overhead that these special
capabilities incur, so they can be useful in cases like the cons list where the
indirection is the only feature we need. We’ll look at more use cases for boxes
in Chapter 17, too.

The `Box<T>` type is a smart pointer because it implements the `Deref` trait,
which allows `Box<T>` values to be treated like references. When a `Box<T>`
value goes out of scope, the heap data that the box is pointing to is cleaned
up as well because of the `Drop` trait implementation. These two traits will be
even more important to the functionality provided by the other smart pointer
types we’ll discuss in the rest of this chapter. Let’s explore these two traits
in more detail.

[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
