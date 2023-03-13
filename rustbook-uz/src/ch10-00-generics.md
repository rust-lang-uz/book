# Generik turlar, Traitlar va Lifetimelar

Har bir dasturlash tilida kontseptsiyalarning takrorlanishini samarali boshqarish vositalari mavjud. Rustda bunday vositalardan biri *generiklar*: concrete  turlari yoki boshqa xususiyatlar uchun mavhum stendlar. Kodni kompilyatsiya qilish va ishga tushirishda ularning o'rnida nima bo'lishini bilmasdan, biz generiklarning xatti-harakatlarini yoki ularning boshqa generiklar bilan qanday bog'liqligini ifodalashimiz mumkin.

Funktsiyalar `i32` yoki `String` kabi aniq turdagi o'rniga ba'zi umumiy turdagi parametrlarni olishi mumkin, xuddi shu tarzda funksiya bir xil kodni bir nechta aniq qiymatlarda ishlatish uchun noma'lum qiymatlarga ega parametrlarni oladi. Aslida, biz 6-bobda `Option<T>`, 8-bobda `Vec<T>` va `HashMap<K, V>` va 9-bobda `Result<T, E>` bilan generiklardan allaqachon foydalanganmiz. Ushbu bobda siz o'zingizning turlaringizni, funksiyalaringizni va metodlaringizni generiklar bilan qanday aniqlashni o'rganasiz!

Birinchidan, kodning takrorlanishini kamaytirish uchun funksiyani qanday chiqarishni ko'rib chiqamiz. Keyin biz bir xil texnikadan faqat parametrlari turida farq qiladigan ikkita funksiyadan umumiy funksiyani yaratamiz. Shuningdek, biz struct va enum ta'riflarida generik turlardan qanday foydalanishni tushuntiramiz.

Keyin xulq-atvorni umumiy tarzda aniqlash uchun *traitlar* dan qanday foydalanishni o'rganasiz. Har qanday turdan farqli o'laroq, faqat ma'lum bir xatti-harakatga ega bo'lgan turlarni qabul qilish uchun umumiy turni cheklash uchun traitlarni umumiy turlar bilan birlashtira olasiz.

Va nihoyat, biz *lifetimelar* haqida gaplashamiz: kompilyatorga referencelar bir-biriga qanday bog'liqligi haqida ma'lumot beradigan turli xil generiklar. Lifetimelar kompilyatorga olingan qiymatlar haqida yetarli ma'lumot berishga imkon beradi, shunda u murojaatlar bizning yordamimizsiz ko'proq holatlarda haqiqiy bo'lishini ta'minlaydi.

## Funksiyani ajratib olish orqali takrorlanishni olib tashlash

Generiklar bizga kodning takrorlanishini olib tashlash uchun bir nechta turlarni ifodalovchi maxsus turlarni to'ldiruvchi bilan almashtirishga imkon beradi. Generik sintaksisga kirishdan oldin, keling, birinchi navbatda, ma'lum qiymatlarni bir nechta qiymatlarni ifodalovchi to'ldiruvchi bilan almashtiradigan funksiyani chiqarib, generik turlarni o'z ichiga olmaydigan tarzda takrorlashni qanday olib tashlashni ko'rib chiqaylik. Keyin generik funksiyani chiqarish uchun xuddi shu texnikani qo'llaymiz! Funksiyaga chiqarishingiz mumkin bo'lgan takrorlangan kodni qanday tanib olishni ko'rib chiqsangiz, generiklardan foydalanishi mumkin bo'lgan takrorlangan kodni taniy boshlaysiz.

Biz ro'yxatdagi eng katta raqamni topadigan 10-1 ro'yxatidagi qisqa dasturdan boshlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-1: Raqamlar ro'yxatidagi eng katta raqamni topish</span>

Biz butun sonlar roʻyxatini `raqamlar_listi` oʻzgaruvchisida saqlaymiz va roʻyxatdagi birinchi raqamga referenceni `eng_katta` nomli oʻzgaruvchiga joylashtiramiz. Keyin biz roʻyxatdagi barcha raqamlarni takrorlaymiz va agar joriy raqam `eng_katta`da saqlangan raqamdan katta boʻlsa, ushbu oʻzgaruvchidagi referenceni almashtiramiz.
Biroq, agar joriy raqam hozirgacha ko'rilgan eng katta raqamdan kichik yoki unga teng bo'lsa, o'zgaruvchi o'zgarmaydi va kod ro'yxatdagi keyingi raqamga o'tadi. Ro'yxatdagi barcha raqamlarni ko'rib chiqqandan so'ng, `eng_katta` eng katta raqamga ishora qilishi kerak, bu holda bu 100 ga teng.

Bizga endi ikki xil raqamlar ro‘yxatidagi eng katta raqamni topish vazifasi qo‘yildi. Buning uchun biz 10-1 roʻyxatdagi kodni takrorlashni tanlashimiz va 10-2 roʻyxatda koʻrsatilganidek, dasturning ikki xil joyida bir xil mantiqdan foydalanishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

<span class="caption">Ro'yxat 10-2: *ikkita* raqamlar roʻyxatidagi eng katta raqamni topish uchun kod</span>

Ushbu kod ishlayotgan bo'lsa-da, kodni takrorlash zerikarli va xatolarga moyil. Shuningdek, biz kodni o'zgartirmoqchi bo'lganimizda uni bir nechta joyda yangilashni unutmasligimiz kerak.

Ushbu takrorlanishni bartaraf qilish uchun biz parametrda berilgan butun sonlar ro'yxatida ishlaydigan funktsiyani aniqlash orqali abstraksiya yaratamiz. Ushbu yechim bizning kodimizni aniqroq qiladi va bizga ro'yxatdagi eng katta raqamni topish tushunchasini mavhum tarzda ifodalash imkonini beradi.

10-3 ro'yxatda biz eng katta raqamni topadigan kodni `eng_katta` deb nomlangan funksiyaga chiqaramiz. Keyin biz 10-2 ro'yxatdagi ikkita ro'yxatdagi eng katta raqamni topish uchun funksiyani chaqiramiz. Bundan tashqari, biz kelajakda ega bo'lishi mumkin bo'lgan `i32` qiymatlarining boshqa ro'yxatida ham funksiyadan foydalanishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-3: Ikkita roʻyxatdagi eng katta raqamni topish uchun abstrakt kod</span>

`eng_katta` funksiya `list` deb nomlangan parametrga ega bo'lib, biz funktsiyaga o'tkazishimiz mumkin bo'lgan `i32` qiymatlarining har qanday aniq qismini ifodalaydi. Natijada, biz funksiyani chaqirganimizda, kod biz kiritadigan maxsus qiymatlarda ishlaydi.

Xulosa qilib aytganda, biz kodni kodni 10-2-ro'yxadan 10-3-ro'yxaga oʻzgartirish uchun qilgan qadamlarimiz:

1. Ikki nusxadagi kodni aniqlang.
2. Ikki nusxadagi kodni funktsiya tanasiga chiqarib oling va ushbu kodning kirish va qaytish qiymatlarini funktsiya imzosida belgilang.
3. Buning o'rniga funktsiyani chaqirish uchun ikki nusxadagi kodning ikkita nusxasini yangilang.

Keyinchalik, kodning takrorlanishini kamaytirish uchun generiklar bilan bir xil qadamlardan foydalanamiz. Xuddi shu tarzda, funksiya tanasi ma'lum qiymatlar o'rniga mavhum `list` bo'yicha ishlay oladi, generiklar kodni mavhum turlarda ishlashga imkon beradi.

Misol uchun, bizda ikkita funksiya bor edi deylik: biri `i32` qiymatlari bo‘limidagi eng katta elementni topadigan va ikkinchisi `char` qiymatlari bo‘limidagi eng katta elementni topadigan. Bu takroriylikni qanday yo'q qilamiz? Keling, bilib olaylik!
