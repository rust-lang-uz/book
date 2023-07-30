## Testga asoslangan ishlab chiqish bilan kutubxonaning funksionalligini rivojlantirish

Endi biz mantiqni *src/lib.rs* ga chiqardik va argumentlarni yig‘ish va xatolarni qayta ishlashni *src/main.rs* da qoldirdik, kodimizning asosiy funksionalligi uchun testlarni yozish ancha osonlashdi. Biz turli xil argumentlar bilan funksiyalarni to'g'ridan-to'g'ri chaqirishimiz va buyruq satridan binaryga murojaat qilmasdan qaytish(return) qiymatlarini tekshirishimiz mumkin.

Ushbu bo'limda biz quyidagi bosqichlar bilan test-driven development (TDD) jarayonidan foydalangan holda `minigrep` dasturiga qidiruv mantig'ini qo'shamiz:

1. Muvaffaqiyatsiz bo'lgan testni yozing va siz kutgan sabab tufayli muvaffaqiyatsiz bo'lishiga ishonch hosil qilish uchun uni ishga tushiring.
2. Yangi testdan o'tish uchun yetarli kodni yozing yoki o'zgartiring.
3. Siz qo'shgan yoki o'zgartirgan kodni qayta tiklang(refaktoring) va testlar o'tishda davom etayotganiga ishonch hosil qiling.
4. Repeat from step 1!

Garchi bu dasturiy ta'minotni yozishning ko'p usullaridan biri bo'lsa-da, TDD kod dizaynini boshqarishga yordam beradi. Testdan o'tishni ta'minlaydigan kodni yozishdan oldin testni yozish jarayon davomida yuqori sinov qamrovini saqlashga yordam beradi.

Biz fayl tarkibidagi so'rovlar qatorini qidirishni amalga oshiradigan va so'rovga mos keladigan qatorlar ro'yxatini tuzadigan funksiyani amalga oshirishni sinovdan o'tkazamiz. Biz bu funksiyani `qidiruv` funksiyasiga qo‘shamiz.

### Muvaffaqiyatsiz test yozish

Bizga endi ular kerak emasligi sababli, dasturning harakatini tekshirish uchun foydalanilgan *src/lib.rs* va *src/main.rs* dan `println!` statementlarini olib tashlaymiz. Keyin, *src/lib.rs* da, [11-bobda][ch11-anatomy]<!-- ignore --> qilganimizdek, test funksiyasiga ega `tests` modulini qo'shing. Test funksiyasi biz `qidirish` funksiyasiga ega bo'lishini xohlagan xatti-harakatni belgilaydi: u so'rov va izlash uchun matnni oladi va u so'rovni o'z ichiga olgan matndan faqat satrlarni qaytaradi. 12-15 ro'yxatda ushbu test ko'rsatilgan, u hali kompilyatsiya bo'lmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">12-15 roʻyxat: `qidiruv` funksiyasi uchun muvaffaqiyatsiz test yaratish</span>

Bu test `marali` qatorini qidiradi.Biz izlayotgan matn uchta qatordan iborat bo‘lib, ulardan faqat bittasi `marali`ni o‘z ichiga oladi (E’tibor bering, qo‘sh qo‘shtirnoqning ochilishidan keyingi teskari chiziq Rustga ushbu satr literalining boshiga yangi qator belgisini qo‘ymaslikni bildiradi). `qidiruv` funksiyasidan qaytarilgan qiymat faqat biz kutgan qatorni o'z ichiga oladi, deb ta'kidlaymiz.

Biz hali bu testni bajara olmaymiz va uning muvaffaqiyatsizligini kuzata olmaymiz, chunki test hatto kompilyatsiya ham qilmaydi: `qidiruv` funksiyasi hali mavjud emas! TDD tamoyillariga muvofiq, biz 12-16 roʻyxatda koʻrsatilganidek, har doim boʻsh vektorni qaytaruvchi `qidiruv` funksiyasining definitionni qoʻshish orqali testni kompilyatsiya qilish va ishga tushirish uchun yetarli kodni qoʻshamiz. Keyin test kompilyatsiya qilinishi va muvaffaqiyatsiz bo'lishi kerak, chunki bo'sh vektor `"xavfsiz, tez, samarali."` qatorini o'z ichiga olgan vektorga mos kelmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 12-16: `qidiruv` funksiyasini yetarli darajada aniqlash, shuning uchun testimiz kompilyatsiya bo'ladi</span>

E'tibor bering, biz `qidiruv` signaturesida `'a` aniq lifetimeni belgilashimiz va bu lifetimeni `tarkib` argumenti va qaytarish(return) qiymati bilan ishlatishimiz kerak. [10-bobda][ch10-lifetimes]<!-- ignore -->  esda tutingki, lifetime parametrlari qaysi argumentning lifetime(ishlash muddati) qaytariladigan qiymatning lifetime bilan bog'liqligini belgilaydi. Bunday holda, qaytarilgan vektorda `tarkib` argumentining bo'laklariga (`sorov` argumenti o'rniga) reference qiluvchi string bo'laklari bo'lishi kerakligini ko'rsatamiz.

Boshqacha qilib aytganda, biz Rustga aytamizki, `qidiruv` funksiyasi tomonidan qaytarilgan maʼlumotlar `tarkib` argumentida `qidiruv` funksiyasiga oʻtgan maʼlumotlar shuncha vaqtgacha yashaydi. Bu muhim! Murojaatlar haqiqiy bo'lishi uchun bo'laklar(slice) bo'yicha reference qilingan ma'lumotlar ham haqiqiy bo'lishi kerak; agar kompilyator biz `tarkib` emas, balki `sorov` ning satr bo'laklarini(string slice) yaratmoqda deb hisoblasa, u xavfsizlik tekshiruvini noto'g'ri bajaradi.

Agar biz lifetime izohlarni(annotation) unutib, ushbu funksiyani kompilyatsiya qilishga harakat qilsak, biz ushbu xatoni olamiz:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust bizga ikkita argumenning qaysi biri kerakligini bila olmaydi, shuning uchun biz buni aniq aytishimiz kerak. `tarkib` barcha matnimizni o'z ichiga olgan argument bo'lgani uchun va biz ushbu matnning mos keladigan qismlarini qaytarmoqchi bo'lganimiz sababli, biz `tarkib` lifetime sintaksisi yordamida qaytarish qiymatiga ulanishi kerak bo'lgan argument ekanligini bilamiz.

Boshqa dasturlash tillari signaturedagi qiymatlarni qaytarish uchun argumentlarni ulashni talab qilmaydi, ammo bu amaliyot vaqt o'tishi bilan osonlashadi. Siz ushbu misolni 10-bobdagi [“Ma’lumotnomalarni lifetime bilan tekshirish”][validating-references-with-lifetimes]<!-- ignore --> bo‘limi bilan solishtirishingiz mumkin.

Endi testni bajaramiz:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

Ajoyib, test biz kutganimizdek muvaffaqiyatsiz tugadi. Keling, testdan o'tamiz!

### Testdan o'tish uchun kod yozish

Hozirda testimiz muvaffaqiyatsiz tugadi, chunki biz har doim bo'sh vektorni qaytaramiz. Buni tuzatish va `qidiruv` ni amalga oshirish uchun dasturimiz quyidagi bosqichlarni bajarishi kerak:

* `tarkib` ning har bir satrini takrorlang.
* Berilgan satrda siz izlayotgan qator mavjudligini tekshiring.
* Agar shunday bo'lsa, uni biz qaytaradigan qiymatlar ro'yxatiga qo'shing.
* Agar bunday bo'lmasa, hech narsa qilmang.
* Mos keladigan natijalar ro'yxatini qaytaring.

Keling, satrlarni takrorlashdan boshlab, har bir bosqichda ishlaylik.

#### `lines` metodi bilan qatorlar bo'ylab takrorlash

Rust 12-17 ro'yxatda ko'rsatilganidek, qulay tarzda `lines` deb nomlangan satrlarni qatorma-qator takrorlash uchun foydali metodga ega. E'tibor bering, bu hali kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 12-17: `tarkib`dagi har bir qatorni takrorlash
</span>

`lines` metodi iteratorni qaytaradi.Biz iteratorlar haqida [13-bobda][ch13-iterators]<!-- ignore --> chuqurroq gaplashamiz, lekin esda tutingki, siz iteratordan foydalanishning bunday usulini [3-5-ro'yxatda][ch3-iter]<!-- ignore --> ko'rgansiz, bu yerda biz to'plamdagi har bir elementda ba'zi kodlarni ishlatish uchun iterator bilan `for` siklidan foydalanganmiz.

#### So'rov uchun har bir qatorni qidirish

Keyinchalik, joriy qatorda so'rovlar qatori mavjudligini tekshiramiz. Yaxshiyamki, satrlarda biz uchun buni amalga oshiradigan `contains` deb nomlangan foydali metod mavjud! 12-18 roʻyxatda koʻrsatilganidek, `qidiruv` funksiyasidagi `contains` metodiga murojatni qoʻshing. E'tibor bering, bu hali kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 12-18: satrda `sorov` dagi satr mavjudligini ko'rish uchun funksiya qo'shiladi</span>

Ayni paytda biz funksionallikni yaratmoqdamiz. Uni kompilyatsiya qilish uchun biz funksiya signaturesida ko'rsatganimizdek, tanadan qiymatni qaytarishimiz kerak.

#### Mos keladigan qatorlarni saqlash

Ushbu funksiyani tugatish uchun bizga qaytarmoqchi bo'lgan mos keladigan satrlarni saqlash metodi kerak. Buning uchun biz `for` siklidan oldin o'zgaruvchan vector yasashimiz va vectorda `line`ni saqlash uchun `push` metodini chaqirishimiz mumkin. `for` siklidan so'ng, 12-19 ro'yxatda ko'rsatilganidek, vectorni qaytaramiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 12-19: Biz ularni qaytarishimiz uchun mos keladigan satrlarni saqlash</span>

Endi `qidiruv` funksiyasi faqat `sorov` ni o'z ichiga olgan qatorlarni qaytarishi kerak va bizning testimiz o'tishi kerak. Keling, testni bajaramiz:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Testimiz muvaffaqiyatli o'tdi, shuning uchun u ishlayotganini bilamiz!

Shu nuqtada, biz bir xil funksionallikni saqlab qolish uchun testlarni o'tkazgan holda qidiruv funksiyasini amalga oshirishni qayta tiklash imkoniyatlarini ko'rib chiqishimiz mumkin. Qidiruv funksiyasidagi kod juda yomon emas, lekin u iteratorlarning ba'zi foydali xususiyatlaridan foydalanmaydi. Biz [13-bobda][ch13-iterators]<!-- ignore --> ushbu misolga qaytamiz, u yerda iteratorlarni batafsil o'rganamiz va uni qanday yaxshilashni ko'rib chiqamiz.

#### `run` funksiyasidagi `qidiruv` funksiyasidan foydalanish

Endi `qidiruv` funksiyasi ishlayotgan va testdan o‘tgan bo‘lsa, `run` funksiyamizdan `qidiruv` ni chaqirishimiz kerak. Biz `config.sorov` qiymatini va fayldan o'qiydigan `tarkib`-ni  `qidiruv` funksiyasiga o'tkazishimiz kerak. Keyin `run` `qidiruv`dan qaytarilgan har bir qatorni chop etadi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

Biz `qidiruv` dan har bir qatorni qaytarish va uni chop etish uchun `for` siklidan foydalanmoqdamiz.

Endi butun dastur ishlashi kerak! Keling, buni sinab ko'raylik, avval Olma she'ridagi "karnay" ning aynan bir satrini qaytarishi kerak bo'lgan so'z bilan:

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Ajoyib! Keling, bir nechta qatorga mos keladigan so'zni sinab ko'raylik, masalan, "olma":

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

Va nihoyat, she’rning hech bir joyida bo‘lmagan so‘zni izlaganimizda, masalan, “monomorfizatsiya” kabi satrlar chiqmasligiga ishonch hosil qilaylik:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Ajoyib! Biz klassik dasturning o'z mini versiyasini yaratdik va ilovalarni qanday tuzish haqida ko'p narsalarni o'rgandik. Shuningdek, biz faylni kiritish(input) va chiqarish(output), lifetime, test va buyruq satrini tahlil qilish haqida bir oz o'rgandik.

Ushbu loyihani yakunlash uchun biz atrof-muhit(environment) o'zgaruvchilari bilan qanday ishlashni va standart xatoga qanday chop etishni qisqacha ko'rsatamiz, bu ikkalasi ham buyruq qatori dasturlarini yozishda foydalidir..

[validating-references-with-lifetimes]:ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
