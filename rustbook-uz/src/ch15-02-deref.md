## Smart Pointerlarni `Deref` Xususiyati Bilan Oddiy Havolalar Kabi Ishlatish

`Deref` xususiyatini qo'llash, *dereference operatori*ning `*` (ko'paytirish
yoki glob operatori bilan adashtirmaslik kerak) xulq-atvorini sozlashga imkon
beradi. Smart pointerlarni `Deref` xususiyati bilan oddiy havolalar kabi
qo'llasangiz, siz havolalar ustida ishlaydigan kod yozishingiz, shuningdek,
ushbu kodni smart pointerlar bilan ishlatishingiz mumkin bo'ladi.

Keling, avvalo, dereference operatori oddiy havolalar bilan qanday ishlashini
ko'rib chiqaylik. Keyin biz `Box<T>` kabi maxsus turni e'lon qilishga harakat
qilamiz va dereference operatori nega bizning yangi e'lon qilgan turimizdagi
havola kabi ishlamayotganini ko'ramiz. Biz `Deref` xususiyatini amalga oshirish
smart pointerlarning havolalarga o'xshash tarzda ishlashiga qanday imkon
berishini ko'rib chiqamiz. Keyin biz Rustning *deref coercion* xususiyatini va
u bizga havolalar yoki smart pointerlar bilan ishlashga qanday imkon berishini
ko'rib chiqamiz.

> Eslatma: biz qurmoqchi bo'lgan `MyBox<T>` turi va haqiqiy `Box<T>` o‘rtasida
> bitta katta farq bor: bizning versiyamiz o‘z ma’lumotlarini heapda saqlamaydi.
> Biz ushbu misolda e'tiborimizni `Deref`ga qaratmoqdamiz, shuning uchun 
> ma'lumotlarning qayerda saqlanishi pointerga o'xshash xatti-harakatlardan 
> kamroq ahamiyatga ega.

<!-- Old link, do not remove -->
<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>

### Pointerni Qiymatga bog'lash 

Muntazam havola pointerning bir turi bo'lib, pointerni boshqa joyda saqlangan
qiymatga o'q kabi tasavvur qilishning bir usuli. 15-6 ro'yxatda biz `i32`
qiymatiga havola yaratamiz va keyin qiymatga havolani bog'lash uchun dereference
operatoridan foydalanamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

<span class="caption">Ro'yxat 15-6: `i32` qiymatiga havola orqali murojat qilish
uchun dereference operatoridan foydalanish</span>

`x` o'zgaruvchisi `i32` turidagi `5` qiymatiga ega. Biz `y` ni `x` ning
havolasiga tenglashtiramiz. Biz `x` `5` ga teng ekanligini solishtirishimiz
mumkin. Ammo, agar biz `y` dagi qiymatni solishtirmoqchi bo'lsak, kompilyator
haqiqiy qiymatni solishtirishi uchun `*y` dan foydalanib, u havola qilgan
qiymatga (ya'ni, *dereference*) murojaat qilishimiz kerak. `y` da dereference
qo'llaganimizdan so'ng, `y` ishora qilib turgan butun son qiymatiga kirish
imkoniga ega bo'lamiz, bu `5` bilan solishtirishimizga imkon beradi.

Agar `assert_eq!(5, y);` yozishga harakat qilganimizda, ushbu kompilyatsiya
xatoligini olgan bo'lar edik:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Raqam va raqamga havola bilan solishtirishga yo'l qo'yilmaydi, chunki ular har
xil turlar. Biz havola qilingan qiymatga murojaat qilish uchun dereference
operatoridan foydalanishimiz kerak.

### `Box<T>` ni Havola Kabi Ishlatish

15-6 ro'yxatdagi kodni havola o'rniga `Box<T>` ishlatgan holda qayta yozishimiz
mumkin; 15-7 ro'yxatdagi funksiyalarida `Box<T>` da ishlatiladigan dereference
operatori 15-6 ro'yxatidagi havolada ishlatilgan dereference operatori
bilan bir xil tarzda ishlatiladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

<span class="caption">Ro'yxat 15-7: `Box<i32>` da dereference operatorini
ishlatish</span>

15-7 va 15-6 ro'yxat o'rtasidagi asosiy farq shundaki, biz bu yerda `y` ni `x`
qiymatiga havola emas, balki `x` ning ko'chirilgan qiymatiga ishora qiluvchi
`Box<T>` ning misoli qilib belgiladik. Oxirgi solishtiruvda biz dereference
operatoridan `Box<T>` ko'rsatgichiga murojat qilish uchun xuddi `y` havola
bo'lganida qilganimizdek bajarishimiz mumkin. Keyin biz `Box<T>` ning o'ziga xos
xususiyatlarini o'rganamiz, bu bizga o'z turimizni e'lon qilish orqali
dereference operatoridan foydalanishga imkon beradi. 

### O'zimizning Aqlli Ko'rsatgichimizni E'lon Qilish

Keling, aqlli ko'rsatgichlar havolalardan qanday farq qilishini bilish uchun
standart kutubxona tomonidan taqdim etilgan `Box<T>` turiga o'xshash aqlli
ko'rsatgichni yarataylik. Keyin biz dereference operatoridan foydalanish
qobiliyatini qanday qo'shishni ko'rib chiqamiz.

`Box<T>` turi oxir-oqibat bitta elementga ega bo'lgan tuple struct sifatida
aniqlanadi, 15-8 ro'yxatda xuddi shu tarzda `MyBox<T>` turini belgilaydi.
Shuningdek, `Box<T>` da belgilangan `new` funksiyaga mos keladigan `new`
funksiyani aniqlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-8: `MyBox<T>` turini aniqlash</span>

Biz `MyBox` nomli structni aniqlaymiz va `T` generic parametrini e'lon qilamiz,
chunki biz turimiz istalgan turdagi qiymatlarni ushlab turishini xohlaymiz.
`MyBox` turi `T` turidagi bitta elementga ega bo'lgan tuple structdir.
`MyBox::new` funksiyasi `T` turidagi bitta parametrni oladi va berilgan qiymatni
ushlab turuvchi `MyBox` misolini qaytaradi.

15-7 ro'yxatdagi `main` funksiyasini 15-8 ro'yxatiga qo'shib, `Box<T>` o'rniga
biz belgilagan `MyBox<T>`turidan foydalanish uchun o'zgartirishga harakat
qilaylik. 15-9 ro'yxatdagi kod kompilyatsiya qilinmaydi, chunki Rust `MyBox` ni
qanday qilib dereference qilishni bilmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-9: `MyBox<T>` dan xuddi havolalar va `Box<T>`
dan foydalanganimiz kabi foydalanishga urinish</span>

Natijada kompilyatsiya xatosi:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

Bizning `MyBox<T>` turini dereference qilib bo'lmaydi, chunki biz bu qobiliyatni
o'z turimizda qo'llamaganmiz. `*` operatori yordamida dereference qilishni
yoqish uchun biz `Deref` traitini qo`llaymiz.

### `Deref` Traitni Amalga Oshirish Orqali Turga Havola Kabi Munosabatda Bo'lish

10-bobning [“Turga xos Traitni amalga
oshirish”][impl-trait]<!-- ignore --> boʻlimida muhokama
qilinganidek, traitni amalga oshirish uchun biz traitning talab qilinadigan
usullarini amalga oshirishimiz kerak. Standart kutubxona tomonidan taqdim
etilgan `Deref` xususiyati bizdan `self` qarz oladigan va ichki ma'lumotlarga
havolani qaytaradigan `deref` nomli metodni qo'llashimizni talab qiladi. 15-10
ro'yxat `MyBox` ta'rifiga qo'shish uchun `Deref` amalga oshirilishini o'z ichiga
oladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-10: `MyBox<T>` uchun `Deref` ni amalga
oshirish</span>

`Type Target = T;` sintaksisi foydalanish uchun `Deref` xususiyati uchun
bog`langan turni belgilaydi. Bog'langan turlar generic parametrni e'lon
qilishning biroz boshqacha usulidir, ammo hozircha ular haqida
tashvishlanishingiz shart emas; biz ularni 19-bobda batafsil yoritamiz.

Biz `deref` metodining tanasini `&self.0` bilan to'ldiramiz, shuning uchun
`deref` biz `*` operatori bilan kirmoqchi bo'lgan qiymatga havolani qaytaradi;
5-bobning [“Har xil turlarni yaratish uchun nomli maydonlarsiz tuplelardan
foydalanish”][tuple-structs]<!-- ignore --> boʻlimidan `.0` tuple structidagi
birinchi qiymatga kirishini esga oling. `MyBox<T>` qiymatida `*` ni chaqiruvchi
15-9 ro'yxatdagi `main` funksiya endi kompilyatsiya qilinadi va solishtiruvlar
o`tadi!

`Deref` traitisiz kompilyator faqat `&` havolalarini dereference qilishi mumkin.
`deref` metodi kompilyatorga `Deref` ni qo'llaydigan har qanday turdagi qiymatni
olish va `deref` usulini chaqirish va `&` havolasini olish imkoniyatini beradi.

15-9 ro'yxatda `*y` ga kirganimizda, Rust sahna ortida ushbu kodni ishga
tushirdi:

```rust,ignore
*(y.deref())
```

Rust `*` operatorini `deref` metodini chaqirish va keyin oddiy dereference bilan
almashtiradi, shuning uchun `deref` metodini chaqirish kerakligi haqida
o'ylamasligimiz kerak. Ushbu Rust xususiyati bizga oddiy havola yoki `Deref` ni
qo'llaydigan turga ega bo'ladimi, bir xil ishlaydigan kod yozish imkonini
beradi.

`deref` metodi qiymatga havolani qaytarishining sababi va `*(y.deref())` qavslar
tashqarisidagi oddiy dereference hali ham zarur bo'lishi ownership tizimi bilan
bog'liq. Agar `deref` usuli qiymatga havola o'rniga to'g'ridan-to'g'ri qiymatni
qaytargan bo'lsa, qiymat `o'zidan` o'chiriladi. Biz `MyBox<T>` ichidagi ichki
qiymatga egalik qilishni istamaymiz, bu holatda yoki ko'p hollarda biz
dereference operatoridan foydalanamiz.

Esda tutingki, `*` operatori `deref` metodini chaqirish va keyin `*` operatorini
faqat bir marta chaqirish bilan almashtiriladi, har safar kodimizda `*` dan
foydalanamiz. `*` operatorini almashtirish cheksiz takrorlanmasligi sababli, biz
15-9 ro'yxatdagi `assert_eq!` dagi `5` ga mos keladigan `i32` turidagi
ma'lumotlarga ega bo`lamiz.

### Funksiya va Metodlar bilan Yashirin Deref Coercion'lar

*Deref coercion* havolani `Deref` xususiyatini boshqa turga havolada amalga
oshiradigan turga aylantiradi. Masalan, deref coercion `&String` ni `&str` ga
aylantirishi mumkin, chunki `String` `Deref` traitini amalga oshiradi va u
`&str` ni qaytaradi. Deref coercion - bu Rust funksiyalar va metodlarga
argumentlar bo'yicha bajaradigan qulaylik va faqat `Deref` traitini amalga
oshiradigan turlarda ishlaydi. Bu funksiya yoki metod ta'rifidagi parametr turiga
mos kelmaydigan funksiya yoki metodga argument sifatida ma'lum bir turdagi
qiymatga havolani uzatganimizda avtomatik ravishda sodir boʻladi. `Deref`
metodiga chaqiruvlar ketma-ketligi biz taqdim etgan turni parametr kerak
bo'lgan turga aylantiradi.

Rustga deref coercion qo'shildi, shuning uchun dasturchilar funktsiya va metod
chaqiruvlarini yozish uchun `&` va `*` bilan ko'p aniq havolalar va
dereferencelarni qo'shishlari shart emas. Deref coercion xususiyati bizga
havolalar yoki aqlli ko'rsatkichlar uchun ishlashi mumkin bo'lgan ko'proq kod
yozish imkonini beradi. 

Deref coercionni amalda ko'rish uchun biz 15-8 ro'yxatda belgilagan `MyBox<T>`
turini hamda 15-10 ro'yxatiga qo'shgan `Deref` ni amalga oshirishdan
foydalanamiz. Ro'yxat 15-11 string slice parametriga ega bo'lgan funksiyaning
ta'rifini ko'rsatadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-11: `&str` tipidagi `nom` parametriga ega
`salom` funksiyasi</span>

Biz `salom` funksiyasini argument sifatida string slice bilan chaqirishimiz
mumkin, masalan, `salom("Rust");`. Deref coercion 15-12 ro'yxatda
ko'rsatilganidek, `MyBox<String>` turidagi qiymatga havola bilan `salom` ni
chaqirish imkonini beradi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-12: `MyBox<String>` qiymatiga havola bilan
`salom` deb chaqirish, bu deref coercion tufayli ishlaydi</span>

Bu yerda biz `&m` argumenti bilan `salom` funksiyasini chaqiramiz, bu
`MyBox<String>` qiymatiga havola. Biz `Deref` traitini `MyBox<T>` uchun
15-10 ro'yxatda amalga oshirganimiz uchun Rust `deref` ni chaqirish orqali
`&MyBox<String>` ni `&String` ga aylantirishi mumkin. Standart kutubxona `String`
da `Deref` ning amalga oshirilishini ta'minlaydi, bu string slice qaytaradi va
bu `Deref` uchun API hujjatlarida. Rust `&String` ni `&str` ga aylantirish uchun
yana `deref` ni chaqiradi, bu `salom` funksiyasi ta`rifiga mos keladi.

Agar Rust deref coercionni amalga oshirmagan bo'lganida, biz `&MyBox<String>`
tipidagi qiymat bilan `salom` ni chaqirish uchun 15-12 ro'yxatdagi kod
o'rniga 15-13 ro'yxatdagi kodini yozishimiz kerak edi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

<span class="caption">Ro'yxat 15-13: Agar Rustda deref coercion bo'lmaganida,
biz yozishimiz kerak bo'lgan kod</span>

`(*m)` `MyBox<String>` ni `String` ga yo'naltiradi. Keyin `&` va `[..]` `salom`
belgisiga mos kelishi uchun butun stringga teng bo'lgan `String` string sliceni
oladi. Deref coercionsiz ushbu kodni o'qish, yozish va tushunish ushbu belgilar
bilan qiyinroq. Deref coercion Rustga ushbu konversiyalarni biz uchun avtomatik
tarzda boshqarishga imkon beradi.

"Deref" traiti jalb qilingan turlar uchun aniqlanganda, Rust turlarni tahlil
qiladi va parametr turiga mos keladigan havolani olish uchun kerak bo'lganda
`Deref::deref` dan foydalanadi. `Deref::deref` qo'shilishi kerak bo'lgan vaqtlar
soni kompilyatsiya vaqtida hal qilinadi, shuning uchun deref coerciondan
foydalanganlik uchun ishga tushirish vaqtida jarima yo'q!

### Deref Coercion O'zgaruvchanlik bilan Qanday O'zaro Ta'sir Qilishi

O'zgarmas havolalarda `*` operatorini rad qilish uchun `Deref` traitidan
foydalanishga o'xshab, o'zgaruvchan havolalarda `*` operatorini rad
qilish uchun `DerefMut` traitidan foydalanishingiz mumkin.

Rust ushbu uchta holatda tur va traitni amalga oshirishlarni topsa, deref
coercionni amalga oshiradi

* `&T` dan `&U` gacha, `T: Deref<Target=U>`
* `&mut T` dan `&mut U` gacha `T: DerefMut<Target=U>`
* `&mut T` dan `&U` gacha `T: Deref<Target=U>`

Birinchi ikkita holat bir-biri bilan bir xil, faqat ikkinchisi o'zgaruvchanlikni
amalga oshiradi. Birinchi holatda aytilishicha, agar sizda `&T` bo'lsa va `T`
`Deref` ni `U` turiga qo'llasa, shaffof tarzda `&U` ni olishingiz mumkin.
Ikkinchi holatda aytilishicha, xuddi shunday deref coercion o'zgaruvchan
havolalar uchun sodir bo'ladi.

Uchinchi holat qiyinroq: Rust o'zgarmasga o'zgaruvchan havolani ham majbur
qiladi. Ammo buning teskarisi *mumkin emas*: o'zgarmas havolalar hech qachon
o'zgaruvchan havolalarga majburlamaydi. Qarz olish qoidalari tufayli, agar sizda
o'zgaruvchan havola bo'lsa, bu o'zgaruvchan havola ma'lumot uchun yagona havola
bo'lishi kerak (aks holda dastur kompilyatsiya qilinmaydi). Bitta o'zgaruvchan
havolani bitta o'zgarmas havolaga aylantirish hech qachon qarz olish qoidalarini
buzmaydi. O'zgarmas havolani o'zgaruvchan havolaga aylantirish uchun dastlabki
o'zgarmas havola ushbu ma'lumotga yagona o'zgarmas havola bo'lishini talab
qiladi, ammo qarz olish qoidalari bunga kafolat bermaydi. Shu sababli, Rust
o'zgarmas havolani o'zgaruvchan havolaga aylantirish mumkin deb taxmin qila
olmaydi.

[impl-trait]: ch10-02-traits.html#turga-xos-traitni-amalga-oshirish
[tuple-structs]: ch05-01-defining-structs.html#har-xil-turlarni-yaratish-uchun-nomli-maydonlarsiz-tuplelardan-foydalanish
