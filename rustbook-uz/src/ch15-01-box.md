## Heapdagi ma'lumotlarni ko'rsatish uchun `Box<T>` dan foydalanish

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

Birinchi holatni
[“Rekursiv turlarni Boxlar bilan qo'llash”](#rekursiv-turlarni-boxlar-bilan-qollash)<!-- ignore -->
bo‘limida ko‘rsatamiz. Ikkinchi holatda, katta hajmdagi ma'lumotlarga egalik
huquqini o'tkazish uzoq vaqt talab qilishi mumkin, chunki ma'lumotlar stackdan
ko'chiriladi. Bunday vaziyatda ishlashni yaxshilash uchun biz katta hajmdagi
ma'lumotlarni heapda box ichida saqlashimiz mumkin. Shundan so'ng, pointer
ma'lumotlarining faqat kichik miqdori stackdan ko'chiriladi, heapdagi u
reference qilingan ma'lumotlar esa bir joyda qoladi. Uchinchi holat *trait object*
sifatida tanilgan va butun 17-bob shu mavzuga bag'ishlangan,
[“Turli turdagi qiymatlarga ruxsat beruvchi Trait Objectlaridan foydalanish”][trait-objects]<!-- ignore -->
o'sha mavzu. Shunday qilib, bu erda o'rgangan narsalaringizni 17-bobda yana
qo'llaysiz!

### Heapda ma'lumotlarni saqlash uchun `Box<T>` dan foydalanish

`Box<T>` uchun heap xotiradan foydalanish holatini muhokama qilishdan oldin, biz
sintaksisni va `Box<T>` ichida saqlangan qiymatlar bilan qanday o'zaro aloqa
qilishni ko'rib chiqamiz.

15-1 ro'yxatda `i32` qiymatini heapda saqlash uchun boxdan qanday foydalanish
ko'rsatilgan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

<span class="caption">Ro'yxat 15-1: `i32` qiymatini box yordamida heapda
saqlash</span>

Biz `b` o'zgaruvchini heapda joylashgan `5`ga point qiluvchi `Box` qiymatiga ega
bo'lishi uchun e'lon qilamiz. Ushbu dastur `b = 5` ni chop etadi; bu holda biz
boxdagi ma'lumotlarga, stackda bo'lgani kabi kirishimiz mumkin. `main`ning
oxiridagi `b` kabi boxlar scopedan chiqib ketganda, xuddi egalik qilingan
qiymatlarga o'xshab, u ham xotiradan o'chiriladi. O'chirilish ham box (stackda
saqlanuvchi) uchun va u point qiluvchi ma'lumotlar (heapda saqlanuvchi) uchun
ham sodir bo'ladi.

Heapda bitta qiymat saqlash unchalik foydali emas, shuning uchun boxlarni o'zini
bu tarzda ko'pincha ishlatmaysiz. Ko'pincha holatlarda bitta `i32` kabi
qiymatlarni stackda saqlash maqsadga muvofiq bo'ladi. Keling, boxlar, agar bizda
boxlar bo'lmasa, ruxsat berilmaydigan turlarni e'lon qilishga imkon beradigan
holatni ko'rib chiqaylik.

### Rekursiv turlarni Boxlar bilan qo'llash

*Rekursiv tur*ning qiymati o'zining bir qismi sifatida bir xil turdagi boshqa
qiymatga ega bo'lishi mumkin. Rekursiv turlar muammo tug'diradi, chunki
kompilyatsiya vaqtida Rust tur qancha joy egallashini bilishi kerak. Biroq,
rekursiv turdagi qiymatlarni joylashtirish nazariy jihatdan cheksiz davom etishi
mumkin, shuning uchun Rust qiymat uchun qancha joy kerakligini bilmaydi. Boxlar
ma'lum o'lchamga ega bo'lganligi sababli, biz rekursiv tur ta'rifiga box
kiritish orqali rekursiv turlarni qo'llashimiz mumkin.

Rekursiv turga misol sifatida keling, *cons list*ni o'rganamiz. Bu funktsional
dasturlash tillarida keng tarqalgan ma'lumotlar turi hisoblanadi. Biz e'lon
qiladigan cons list turi rekursiyadan tashqari sodda; shuning uchun biz
ishlaydigan misoldagi tushunchalar rekursiv turlarni o'z ichiga olgan murakkab
vaziyatlarga tushganingizda foydali bo'ladi.

#### Cons List haqida batafsil ma'lumot

*Cons list* Lisp dasturlash tili va uning dialektlaridan kelib chiqqan va
ichma-ich juftliklardan tashkil topgan maʼlumotlar strukturasi boʻlib, linked
listning Lispdagi versiyasi hisoblanadi. Uning nomi Lispdagi `cons`
funktsiyasidan (“construct function” uchun qisqartma) kelib chiqqan bo'lib,
uning ikkita argumentidan yangi juftlik yaratadi. Qiymat va boshqa juftlikdan
iborat bo'lgan juftlikda `cons` ni chaqirish orqali biz rekursiv juftliklardan
iborat bo'lgan cons list tuzishimiz mumkin.

Misol uchun, bu yerda 1, 2, 3 ro'yxatini o'z ichiga olgan cons listining
psevdokod ko'rinishi, har bir juft qavs ichida:

```text
(1, (2, (3, Nil)))
```

Cons listdagi har bir element ikkita elementni o'z ichiga oladi: shu elementning
qiymati va keyingi element. Ro'yxatning oxirgi elementida keyingi elementsiz
faqat `Nil` deb nomlangan qiymatdan iborat bo'ladi. Cons list `cons`
funksiyasini rekursiv chaqirish orqali hosil qilinadi. Rekursiyaning tubidagi
holatini bildiruvchi qoidaga aylangan nom `Nil` hisoblanadi. E'tibor bering, bu
6-bobdagi “null” yoki “nil” tushunchasi bilan bir xil emas, ya'ni noto'g'ri yoki
yo'q qiymat.

Cons list ma'lumotlar tuzilmasi Rust-da tez-tez ishlatilmaydi. Ko'pincha Rust-da
sizga elementlar ro'yxati kerak bo'lsa, `Vec<T>` foydalanish uchun yaxshiroq
tanlovdir. Boshqa, murakkabroq rekursiv *ma'lumot turlaridan* foyladanish bir
qancha vaziyatlarda foydalidir, ammo ushbu bobdagi cons listdan boshlab, boxlar
qanday qilib, rekursiv ma'lumot turini e'lon qilishga imkon berishini
o'rganishimiz mumkin.

Ro'yxat 15-2 cons list uchun enum ko'rinishini o'z ichiga oladi. E'tibor bering,
ushbu kod hali kompilyatsiya qilinmaydi, chunki `List` turi ma'lum hajmga ega
emas, biz buni tushuntiramiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-2: `i32` qiymatlarining cons list ma'lumotlar
tuzilmasida ifodalash uchun enumni e'lon qilishga birinchi urish</span>

> Eslatma: Biz ushbu misol maqsadlari uchun faqat `i32` qiymatlarini o'z ichiga
> olgan cons listni amalga oshirmoqdamiz. Biz 10-bobda muhokama qilganimizdek,
> har qanday turdagi qiymatlarni saqlashi mumkin bo'lgan cons list turini
> generiklar yordamida e'lon qilishimiz mumkin edi.

`List` turidan foydalanib `1, 2, 3` roʻyxatini saqlash 15-3 ro'yxat kabi
bo'ladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

<span class="caption">15-3 roʻyxat: `1, 2, 3` roʻyxatini saqlash uchun `List`
enumidan foydalanish</span>

Birinchi `Cons` qiymati `1` va boshqa `List` qiymatiga ega. Bu `List` qiymati
`2` va boshqa `List` qiymatiga ega bo'lgan boshqa `Cons` qiymatidir. Ushbu
`List` qiymati `3` ni o'z ichiga olgan yana bitta `Cons` qiymati va `List`
qiymati, nihoyat `Nil`, ro'yxat oxirini bildiruvchi rekursiv bo'lmagan variant.

Agar biz 15-3 ro'yxatdagi kodni kompilyatsiya qilishga harakat qilsak, biz 15-4
ro'yxatda ko'rsatilgan xatoni olamiz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

<span class="caption">Ro'yxat 15-4: Rekursiv enumni e'lon qilishga urinishda
yuzaga keladigan xato</span>

Xato ushbu tur “cheksiz o'lchamga ega” ekanligini ko'rsatadi. Buning sababi
shundaki, biz `List`ni rekursiv variant bilan e'lon qildik: u bevosita o'zining
boshqa qiymatini saqlaydi. Natijada, Rust `List` qiymatini saqlash uchun qancha
joy kerakligini aniqlay olmaydi. Keling, nima uchun bu xatoga duch kelganimizni
qismlarga ajratamiz. Birinchidan, Rust rekursiv bo'lmagan turdagi qiymatni
saqlash uchun qancha joy kerakligini qanday hal qilishini ko'rib chiqamiz.

#### Rekursiv bo'lmagan turning o'lchamini hisoblash

6-bobda enum ta'riflarini muhokama qilganimizdagi 6-2 ro'yxatda e'lon qilingan
`Xabar` enumini eslang:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

`Xabar` qiymati uchun qancha joy ajratish kerakligini aniqlash uchun Rust har
bir variantni ko'rib chiqadi va qaysi variantga ko'proq joy kerakligini
aniqlaydi. Rust `Xabar::Chiqish` uchun hech qanday joy kerak emasligini,
`Xabar::Kochirish` ikkita `i32` qiymatini saqlash uchun yetarli joy kerakligini
aniqlaydi va hokazo. Faqat bitta variant qo'llanilishi sababli, `Xabar`
qiymatiga kerak bo'ladigan eng ko'p joy-bu uning eng katta variantini saqlash
uchun zarur bo'lgan joy hisoblanadi.

Rust 15-2 roʻyxatdagi `List` enum kabi rekursiv turga qancha boʻsh joy
kerakligini aniqlashga harakat qilganda nima sodir boʻlishini bu bilan
taqqoslang. Kompilyator `i32` turidagi qiymat va `List` turidagi qiymatga ega
bo'lgan `Cons` variantini ko'rib chiqishdan boshlaydi. Shuning uchun, `Cons`
uchun `i32` va `List` o'lchamiga teng bo'sh joy kerak bo'ladi. `List` turiga
qancha xotira kerakligini aniqlash uchun kompilyator `Cons` variantidan boshlab
variantlarni ko'rib chiqadi. `Cons` variantida `i32` turidagi qiymat va `List`
turidagi qiymat mavjud va bu jarayon 15-1-rasmda ko'rsatilganidek, cheksiz davom
etadi.

<img alt="Cheksiz Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">15-1-rasm: Cheksiz `Cons` variantlaridan iborat cheksiz
`List`</span>

#### O'lchami ma'lum bo'lgan rekursiv turni e'lon qilish uchun `Box<T>` dan foydalanish

Rust rekursiv e'lon qilingan turlar uchun qancha joy ajratish kerakligini
aniqlay olmaganligi sababli, kompilyator ushbu foydali taklif bilan xatolik
beradi:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

Ushbu taklifda "indirection" qiymatni to'g'ridan-to'g'ri saqlash o'rniga, biz
pointerni qiymatga saqlash orqali qiymatni bilvosita saqlash uchun ma'lumotlar
strukturasini o'zgartirishimiz kerakligini anglatadi.

`Box<T>` pointer bo'lgani uchun Rust har doim `Box<T>` uchun qancha joy
kerakligini biladi: pointerning o'lchami u ko'rsatayotgan ma'lumotlar miqdoriga
qarab o'zgarmaydi. Bu shuni anglatadiki, biz to'g'ridan-to'g'ri boshqa `List`
qiymati o'rniga `Cons` variantiga `Box<T>` qo'yishimiz mumkin. `Box<T>` keyingi
`List` qiymatiga ishora qiladi, bu qiymat `Cons` varianti ichida emas, balki
heapda bo'ladi. G'oyaga ko'ra, bizda hali ham boshqa ro'yxatlarni o'z ichiga
olgan ro'yxatlar bilan yaratilgan ro'yxat mavjud, ammo bu amalga oshirish endi
elementlarni bir-birining ichiga emas, balki bir-birining yoniga joylashtirishga
o'xshaydi.

We can change the definition of the `List` enum in Listing 15-2 and the usage of
the `List` in Listing 15-3 to the code in Listing 15-5, which will compile:

Biz 15-2 ro'yxatidagi `List` enumni va 15-3 ro'yxatidagi `List`ning
qo'llanishini 15-5 ro'yxatidagi kodga o'zgartirishimiz mumkin, bu kompilyatsiya
bo'ladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

<span class="caption">Ro'yxat 15-5: Maʼlum oʻlchamga ega boʻlish uchun `Box<T>`
dan foydalanadigan `List`ni e'lon qilinishi</span>

`Cons` variantiga `i32` o'lchamiga va boxdagi pointer ma'lumotlarini saqlash
uchun bo'sh joy kerak. `Nil` varianti hech qanday qiymatlarni saqlamaydi,
shuning uchun u `Cons` variantiga qaraganda kamroq joy talab qiladi. Endi
bilamizki, har qanday `List` qiymati `i32` o‘lchamini va boxdagi pointer
ma’lumotlari hajmini egallaydi. Boxdan foydalanib, biz cheksiz, rekursiv
zanjirni buzdik, shuning uchun kompilyator `List` qiymatini saqlash uchun
kerakli hajmni aniqlay oladi. 15-2-rasmda `Cons` varianti hozir qanday
ko'rinishi ko'rsatilgan.

<img alt="Cheksiz bo'lmagan Cons list" src="img/trpl15-02.svg" class="center" />

<span class="caption">15-2-rasm: Cheksiz o'lchamli bo'lmagan `List`, chunki
`Cons` endi `Box` saqlaydi</span>

Boxlar faqat bilvosita va heapda joylashuvni ta'minlaydi; ularda biz boshqa
smart pointerlarda ko'radigan boshqa maxsus imkoniyatlar yo'q. Ular, shuningdek,
ushbu maxsus imkoniyatlarga ega bo'lgan ishlash xarajatlariga ega emaslar,
shuning uchun ular bizga kerak bo'lgan yagona xususiyat bo'lgan cons list kabi
holatlarda foydali bo'lishi mumkin. Biz 17-bobda boxlar uchun ko'proq
foydalanish holatlarini ko'rib chiqamiz.

`Box<T>` turi smart pointerdir, chunki u `Deref` xususiyatini amalga oshiradi,
bu esa `Box<T>` qiymatlariga reference kabi qarashga imkonini beradi. `Box<T>`
qiymati scopedan chiqib ketganda, box ko'rsatayotgan heapdagi ma'lumotlar ham
`Drop` xususiyatini amalga oshirish tufayli tozalanadi. Ushbu ikki xususiyat biz
ushbu bobning qolgan qismida muhokama qiladigan boshqa smart pointer turlari
tomonidan taqdim etilgan funksionallik uchun yanada muhimroq bo'ladi. Keling,
ushbu ikki xususiyatni batafsil ko'rib chiqaylik.

[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types