### Turli Turlarga Ruxsat Beruvchi Trait Ob'ektlarni Ishlatish

8-bobda biz vektorlarning faqat bitta turdagi elementlarni saqlashi mumkinligini aytib o'tgan edik. 8-9 ro'yxatda biz `SpreadsheetCell` enumini yaratdik,
unda butun sonlar, o'nliklar va matnlarni saqlash uchun variantlar mavjud edi. Bu har bir katakda turli xil ma'lumot turlarini saqlash imkonini berdi va
biz hali ham satrni ifodalovchi vektorni saqlay olishimiz mumkin edi. Bu yechim o'zgaruvchan elementlarimiz o'z kodimiz kompilyatsiya qilinayotgan paytda
ma'lum bo'lgan aniq turdagi to'plam bo'lsa yaxshi ishlaydi.

Biroq, ba'zan biz kutubxonamiz foydalanuvchisi ma'lum bir vaziyatda ruxsat etilgan turlar to'plamini kengaytirish imkoniyatiga ega bo'lishini xohlaymiz.
Buni qanday amalga oshirishni ko'rsatish uchun, biz ekranlarga chizish uchun `draw` metodini chaqiradigan elementlar ro'yxatida yuradigan grafik
foydalanuvchi interfeysi (GUI) vositasining misolini yaratamiz. Biz `gui` deb nomlangan kutubxona yaratuvchisiz, bu GUI kutubxonasining tuzilishini o'z
ichiga oladi. Bu kutubxona foydalanuvchilar foydalanishi uchun `Button` yoki `TextField` kabi ba'zi turlarni o'z ichiga olishi mumkin. Bundan tashqari,
`gui` foydalanuvchilari o'z turlarini yaratmoqchi bo'lishi mumkin: masalan, bir dasturchi `Image` qo'shishi mumkin, boshqasi esa `SelectBox` qo'shishi
mumkin.

Biz ushbu misolda to'liq GUI kutubxonasini amalga oshirmaymiz, lekin qismlarning qanday birgalikda ishlashini ko'rsatamiz. Kutubxonani yozish vaqtida biz
boshqa dasturchilar yaratishi mumkin bo'lgan barcha turlarni bilib, belgilab qo'yishimiz mumkin emas. Lekin biz `gui` ko'p turdagi qiymatlarni kuzatib
borishi va har bir turdagi qiymatlar uchun `draw` metodini chaqirishi kerakligini bilamiz. `draw` metodini chaqirganda nima bo'lishini aniq bilishimiz
shart emas, faqat qiymat bu metodni chaqirish uchun mavjud bo'lishi kerak.

Agar biz meros olishni qo'llaydigan tilni ishlatsak, `Component` nomli sinfni yaratib, unga `draw` metodini qo'shishimiz mumkin. Boshqa sinflar, masalan,
`Button`, `Image`, va `SelectBox`, `Component` sinfidan meros olishadi va shu bilan `draw` metodini meros qilib olishadi. Ular `draw` metodini o'z
xususiy xatti-harakatlarini aniqlash uchun o'zgartirishlari mumkin, lekin framework barcha turlarga `Component` misollari sifatida qarashi va ularga
`draw` metodini chaqirishi mumkin. Ammo Rustda meros olish mavjud emas, shuning uchun `gui` kutubxonasini yangi turlar bilan kengaytirishga imkon
beradigan boshqa usulni yaratishimiz kerak.

### Umumiy Xatti-Harakatlar Uchun Traitni Aniqlash

`gui` kutubxonasida kerakli xatti-harakatlarni amalga oshirish uchun biz `Draw` nomli traitni yaratamiz, unda `draw` nomli bir metod bo'ladi. Keyin biz
trait ob'ekti sifatida vektorni aniqlaymiz. Trait ob'ekti bizning belgilangan traitimizni amalga oshirgan turdagi bir misolga va trait metodlarini ushbu
turda runtime paytida qidirish uchun ishlatiladigan jadvalga ishora qiladi. Trait ob'ektini yaratish uchun biz `&` havola yoki `Box<T>` aqlli ko'rsatkich
kabi pointerni, so'ngra `dyn` kalit so'zini va tegishli traitni belgilashimiz kerak. (Trait ob'ektlari pointerlardan foydalanishi kerakligi haqida
19-bobda “Dinamik O'lchamli Turlar va `Sized` Trait” bo'limida gaplashamiz.) Biz trait ob'ektlarini generik yoki aniq turni almashtirish uchun
ishlatishimiz mumkin. Trait ob'ekti ishlatiladigan joyda, Rustning tur tizimi kompilyatsiya vaqtida har qanday qiymat bu trait ob'ekti traitini amalga
oshirishi kerakligini ta'minlaydi. Natijada, biz kompilyatsiya vaqtida barcha mumkin bo'lgan turlardan xabardor bo'lishimiz shart emas.

Rustda biz struct va enumlarni “obyektlar” deb atashdan qochamiz, chunki ular boshqa tillardagi obyektlardan farq qiladi. Struct yoki enumda struct
maydonlaridagi ma'lumot va `impl` bloklaridagi xatti-harakatlar alohida-alohida mavjud, boshqacha tillarda ma'lumot va xatti-harakatlarni bir tushunchaga
birlashtirilgan narsa ko'pincha obyekt deb ataladi. Biroq, trait ob'ektlari boshqa tillardagi obyektlarga o'xshashroq, chunki ular ma'lumot va
xatti-harakatlarni birlashtiradi. Ammo trait ob'ektlari an'anaviy obyektlardan farq qiladi, chunki trait ob'ektiga ma'lumot qo'shishimiz mumkin emas.
Trait ob'ektlar boshqa tillardagi obyektlarga nisbatan umumiy foydalidir: ularning maxsus maqsadi umumiy xatti-harakatlarni abstraktsiya qilishdir.

17-3 ro'yxatda `draw` nomli metodga ega `Draw` traitini qanday aniqlash ko'rsatilgan:

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

Bu sintaksis traitlarni qanday aniqlash bo'yicha 10-bobdagi muhokamadan tanish bo'lishi kerak. Keyin yangi sintaksis keladi: 17-4 ro'yxatda `Screen`
nomli structni aniqlaymiz, unda `components` nomli vektor mavjud. Ushbu vektor `Box<dyn Draw>` turida, ya'ni trait ob'ekti; bu `Draw` traitini amalga
oshirgan har qanday turdagi `Box` ichidagi turga o'rnatuvchi hisoblanadi.

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

`Screen` structida har bir `components`dagi element uchun `draw` metodini chaqiradigan `run` metodini aniqlaymiz, 17-5 ro'yxatda ko'rsatilgan:

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

Bu generik tur parametrlar bilan trait chegaralari yordamida aniqlangan structdan farq qiladi. Generik tur parametri faqat bir konkret tur bilan
almashtirilishi mumkin, trait ob'ektlari esa runtime paytida trait ob'ekti o'rniga bir nechta konkret turlarni qo'llashga imkon beradi. Masalan, `Screen`
structini generik tur va trait chegarasi yordamida quyidagicha aniqlash mumkin edi:

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

Bu bizni faqat `Button` yoki `TextField` turidagi komponentlar bilan `Screen` misoliga cheklaydi. Agar siz faqat gomogen to'plamlarni ishlatsangiz,
generiklar va trait chegaralarini ishlatish ma'qul, chunki aniqlashlar kompilyatsiya vaqtida konkret turlarni ishlatish uchun monomorfizatsiyalanadi.

Boshqa tomondan, trait ob'ektlari bilan ishlaydigan metod bilan bitta `Screen` misoli `Box<Button>` va `Box<TextField>` ni o'z ichiga olgan `Vec<T>` ni
saqlashi mumkin. Bu qanday ishlashini ko'rib chiqamiz va runtime performans ta'sirini muhokama qilamiz.

### Traitni Amalga Oshirish

Endi `Draw` traitini amalga oshiruvchi turlarni qo'shamiz. Biz `Button` turini taqdim etamiz. Yana, to'liq GUI kutubxonasini yaratish ushbu kitob
doirasidan tashqarida, shuning uchun `draw` metodining tanasida hech qanday foydali kod bo'lmaydi. Amalga oshirilishi qanday ko'rinishi mumkinligini
tasavvur qilish uchun, `Button` structida `width`, `height`, va `label` maydonlari bo'lishi mumkin, 17-7 ro'yxatda ko'rsatilgan:

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

`Button

` maydonlari boshqa komponentlardagi maydonlardan farq qiladi; masalan, `TextField` turi ham `width`, `height`, va `label` maydonlariga, shuningdek,
`placeholder` maydoniga ega bo'lishi mumkin. Ekranda chizishimiz kerak bo'lgan har bir tur `Draw` traitini amalga oshiradi, lekin `draw` metodida turli
xil kodlardan foydalanadi, masalan, `Button`da ko'rsatilgan (haqiqiy GUI kodi bilan emas). `Button` turi qo'shimcha `impl` blokiga ega bo'lishi mumkin,
bunda foydalanuvchi tugmani bosganda nima bo'lishi bilan bog'liq metodlar mavjud bo'lishi mumkin. Bu turdagi metodlar `TextField` kabi turlarga tatbiq
etilmaydi.

Agar bizning kutubxonamizni ishlatadigan kishi `SelectBox` structini yaratib, `width`, `height`, va `options` maydonlari bo'lsa, u `Draw` traitini
`SelectBox` turida amalga oshiradi, 17-8 ro'yxatda ko'rsatilgan:

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

Kutubxonamiz foydalanuvchisi endi `Screen` misolini yaratib, `main` funksiyasini yozishi mumkin. `Screen` misoliga `SelectBox` va `Button` qo'shishi
mumkin, har birini `Box<T>` ga joylashtirib, trait ob'ektiga aylantirishi mumkin. Keyin `Screen` misolida `run` metodini chaqirishi mumkin, bu metod har
bir komponentda `draw` metodini chaqiradi. 17-9 ro'yxat bu amalga oshirishni ko'rsatadi:

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

Kutubxonani yozishda biz `SelectBox` turini qo'shishi mumkinligini bilmasdik, lekin bizning `Screen` implementatsiyamiz yangi tur bilan ishlay olishiga
va uni chizishga imkon berdi, chunki `SelectBox` `Draw` traitini amalga oshiradi, demak `draw` metodini amalga oshiradi.

Ushbu tushuncha—qiymatning konkret turini bilmasdan, faqat qiymat javob beradigan xabarlarga e'tibor berish—dinamik ravishda tiplashgan tillarda *qush
tiplash* konseptiga o'xshaydi: agar u qush kabi yuradi va qush kabi qichqirsa, demak u qushdir! 17-5 ro'yxatdagi `Screen` ustida `run` metodini amalga
oshirishda, `run` har bir komponentning konkret turini bilishi shart emas. U komponentning `Button` yoki `SelectBox` misoli ekanligini tekshirmaydi,
faqat `draw` metodini chaqiradi. `components` vektoridagi qiymatlar turi sifatida `Box<dyn Draw>` ni belgilash orqali biz `Screen` ni `draw` metodini
chaqira olishimiz uchun kerakli qiymatlar bilan aniqladik.

Trait ob'ektlari va Rustning tur tizimidan foydalanish, qush tiplashni qo'llagan kodga o'xshash kod yozishning afzalligi shundaki, biz hech qachon
runtime vaqtida qiymatning ma'lum bir metodni amalga oshirishi tekshirilishi yoki metodni amalga oshirmagan qiymatni chaqirishda xatolarni olish haqida
xavotir olmamız shart emas. Rust kodimizni trait ob'ektlari kerak bo'lgan traitlarni amalga oshirmagan qiymatlar bilan kompilyatsiya qilmaydi.

Masalan, 17-10 ro'yxatda `String`ni komponent sifatida ishlatishga harakat qilsak nima bo'lishini ko'rsatadi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

`String` `Draw` traitini amalga oshirmaydi, shuning uchun biz ushbu xatoni olamiz:

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

Bu xato bizga `Screen`ga o'tkazayotganimizni xohlamagan turdagi biror narsa berayotganimizni yoki `String`da `Draw` traitini amalga oshirishimiz
kerakligini bildiradi, shunda `Screen` uni `draw` metodini chaqira oladi.

### Trait Ob'ektlari Dinamik Dispatchni Amalga Oshiradi

10-bobdagi [“Generiklar bilan Kodning Ishlashiga Ta'siri”][performance-of-code-using-generics] bo'limida trait chegaralari bilan generiklarni ishlatishda
kompilyator tomonidan amalga oshiriladigan monomorfizatsiya jarayonini muhokama qilganimizni eslaymiz: kompilyator generik tur parametrlarini o'rnini
bosadigan har bir konkret tur uchun generik bo'lmagan implementatsiyalarni yaratadi. Monomorfizatsiya natijasida hosil bo'lgan kod *statik dispatch*
qiladi, bu kompilyator kompilyatsiya vaqtida qaysi metodni chaqirayotganini biladi. Bu *dinamik dispatch* ga qarshi bo'lib, kompilyator kompilyatsiya
vaqtida qaysi metodni chaqirayotganini bilmaydi. Dinamik dispatch holatlarida, kompilyator runtime paytida qaysi metodni chaqirishni aniqlash uchun kodni
chiqaradi.

Trait ob'ektlarini ishlatishda Rust dinamik dispatchni ishlatishi kerak. Kompilyator trait ob'ektlari bilan ishlatiladigan barcha turlarni bilmaydi,
shuning uchun qaysi metodni qaysi turda chaqirishni bilmaydi. Buning o'rniga, runtime paytida Rust trait ob'ekti ichidagi ko'rsatkichlarni qaysi metodni
chaqirishni bilish uchun ishlatadi. Bu qidiruv runtime xarajatlarni keltirib chiqaradi, bu statik dispatch bilan yuz bermaydi. Dinamik dispatch
shuningdek kompilyatorning metod kodini inline qilish imkoniyatini cheklaydi, bu ba'zi optimizatsiyalarni amalga oshirishga to'sqinlik qiladi. Biroq,
17-5 va 17-9 ro'yxatlarda yozgan kodimizda biz qo'shimcha moslashuvchanlikka ega bo'ldik, shuning uchun bu trade-offni hisobga olish kerak.

[performance-of-code-using-generics]:
ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
