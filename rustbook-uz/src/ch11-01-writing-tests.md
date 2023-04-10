## Testlarni qanday yozish kerak

Testlar - bu sinovdan tashqari kod kutilgan tarzda ishlayotganligini tasdiqlovchi Rust funksiyalari. Test funksiyalari organlari odatda ushbu uchta harakatni bajaradi:

1. Har qanday kerakli ma'lumotlarni yoki holatni o'rnating.
2. Test qilmoqchi bo'lgan kodni ishga tushiring.
3. Natijalar siz kutgan narsa ekanligini tasdiqlang.

Keling, Rust ushbu amallarni bajaradigan testlarni yozish uchun taqdim etgan xususiyatlarni ko'rib chiqaylik, ular orasida `test` atributi, bir nechta makroslar va `should_panic` atributi mavjud.

### Test funksiyasining anatomiyasi

Eng sodda qilib aytganda, Rust-dagi test `test` atributi bilan izohlangan funksiyadir. Atributlar Rust kodining bo'laklari haqidagi metama'lumotlardir; bir misol, biz 5-bobda structlar bilan ishlatgan `derive` atributidir. Funksiyani test funksiyasiga oʻzgartirish uchun `fn` oldidan qatorga `#[test]` qoʻshing. `cargo test` buyrug'i bilan testlarni o'tkazganingizda, Rust izohli funksiyalarni ishga tushiradigan test dasturining binaryrini yaratadi va har bir test funksiyasidan o'tgan yoki muvaffaqiyatsizligi haqida hisobot beradi.

Har safar biz Cargo bilan yangi kutubxona loyihasini yaratganimizda, biz uchun test funksiyasi bo'lgan test moduli avtomatik ravishda yaratiladi. Ushbu modul sizga testlarni yozish uchun shablonni taqdim etadi, shuning uchun har safar yangi loyihani boshlaganingizda aniq struktura va sintaksisni izlashga hojat qolmaydi. Siz xohlagancha qo'shimcha test funksiyalari va test modullarini qo'shishingiz mumkin!

Har qanday kodni sinab ko'rishdan oldin shablon testi bilan tajriba o'tkazish orqali testlar qanday ishlashining ba'zi jihatlarini o'rganamiz. Keyin biz yozgan ba'zi kodlarni chaqiradigan va uning xatti-harakati to'g'riligini tasdiqlaydigan haqiqiy dunyo testlarini yozamiz.

Keling, ikkita raqamni qo'shadigan `qoshuvchi` nomli yangi kutubxona loyihasini yarataylik:

```console
$ cargo new qoshuvchi --lib
     Created library `qoshuvchi` project
$ cd qoshuvchi
```

`qoshuvchi` kutubxonangizdagi *src/lib.rs* faylining mazmuni 11-1 roʻyxatdagi kabi koʻrinishi kerak.

<span class="filename">Fayl nomi: src/lib.rs</span>

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
cargo test
git co output.txt
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">Ro'yxat 11-1: Test moduli va funksiyasi avtomatik ravishda `cargo new` tomonidan yaratilgan</span>

Hozircha, keling, yuqoridagi ikkita qatorga e'tibor bermaylik va funksiyaga e'tibor qarataylik. `#[test]` izohiga e'tibor bering: bu atribut bu test funksiyasi ekanligini bildiradi, shuning uchun test ishtirokchisi bu funksiyani test sifatida ko'rishni biladi. Umumiy stsenariylarni oʻrnatish yoki umumiy operatsiyalarni bajarishda yordam beradigan `tests` modulida testdan tashqari funksiyalar ham boʻlishi mumkin, shuning uchun biz har doim qaysi funksiyalar test ekanligini koʻrsatishimiz kerak.

Misol funksiya tanasi 2 va 2 qo‘shilishi natijasini o‘z ichiga olgan `natija` 4 ga teng ekanligini tasdiqlash uchun `assert_eq!` makrosidan foydalanadi. Ushbu tasdiq odatiy test formatiga misol bo'lib xizmat qiladi. Ushbu sinovdan o'tishini ko'rish uchun uni ishga tushiramiz.

`cargo test` buyrug'i 11-2 ro'yxatda ko'rsatilganidek, loyihamizdagi barcha testlarni amalga oshiradi.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">Ro'yxat 11-2: Avtomatik ishlab chiqarilgan testni bajarishdan olingan natija</span>

Cargo kompilyatsiya qilindi va sinovdan o'tdi. Biz `running 1 test` qatorini ko'ramiz. Keyingi qatorda `ishlaydi` deb nomlangan yaratilgan test funksiyasining nomi va bu testni bajarish natijasi `ok` ekanligini ko'rsatadi. Umumiy xulosa test natijasi `test result: ok.` barcha testlardan muvaffaqiyatli oʻtganligini va `1 passed;` deb yozilgan qismi muvaffaqiyatli oʻtganligini bildiradi; `0 failed` muvaffaqiyatsiz boʻlgan testlar sonini ifodalaydi.

Muayyan misolda ishlamasligi uchun testni e'tiborsiz(ignor) deb belgilash mumkin; Biz buni ushbu bobning keyingi qismida ["Agar aniq talab qilinmasa, ba'zi testlarni e'tiborsiz qoldirish"][ignoring]<!-- ignore --> bo'limida ko'rib chiqamiz. Bu yerda biz buni qilmaganimiz sababli, xulosada  `0 ignored` 0-ta eʼtibor berilmagan koʻrsatiladi. Shuningdek, biz argumentni faqat nomi satrga mos keladigan testlarni o'tkazish uchun `cargo test` buyrug'iga o'tkazishimiz mumkin; bu *filtrlash* deb ataladi va biz buni ["Testlar to'plamini nomi bo'yicha ishga tushirish"][subset]<!-- ignore --> bo'limida ko'rib chiqamiz. Shuningdek, biz bajarilayotgan testlarni filtrlamadik, shuning uchun xulosa oxirida `0 filtered out` 0-ta filtrlangan deb ko‘rsatiladi.

`0 measured` statistikasi samaradorlikni o'lchaydigan benchmark testlari uchundir.
Benchmark testlari, ushbu yozuvdan boshlab, faqat nightly Rust-da mavjud. Batafsil ma'lumot olish uchun [benchmark testlari haqidagi hujjatlarga][bench] qarang.

`Doc-tests adder`(Hujjat testlari qoʻshuvchisi) dan boshlanadigan test natijasining keyingi qismi har qanday hujjat sinovlari natijalariga moʻljallangan. Bizda hali hech qanday hujjat sinovlari yo'q, lekin Rust API hujjatlarida ko'rinadigan har qanday kod misollarini to'plashi mumkin.
Bu xususiyat hujjatlaringiz va kodingizni sinxronlashtirishga yordam beradi! Hujjat testlarini qanday yozishni 14-bobning [“Hujjatlarga sharhlar test sifatida”][doc-comments]<!-- ignore --> bo‘limida muhokama qilamiz. Hozircha biz `Doc-tests` chiqishini e'tiborsiz qoldiramiz.

Keling, testni o'z ehtiyojlarimizga moslashtirishni boshlaylik. Avval `ishlaydi` funksiyasining nomini `tadqiqot` kabi boshqa nomga o'zgartiring, masalan:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Keyin yana `cargo test` bajaring. Chiqish(output) endi `ishlaydi` o‘rniga `tadqiqot`ni ko‘rsatadi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Endi biz yana bir test qo'shamiz, lekin bu safar muvaffaqiyatsiz bo'lgan testni qilamiz! Test funktsiyasidagi biror narsa panic qo'zg'atganda, testlar muvaffaqiyatsiz tugaydi. Har bir test yangi threadda o'tkaziladi va asosiy(main) thread sinov chizig'i o'lganini ko'rsa, test muvaffaqiyatsiz deb belgilanadi. 9-bobda biz panic qo'zg'ashning eng oddiy yo'li `panic!` makrosini chaqirish haqida gapirdik. Yangi testni `boshqa` funksiya sifatida kiriting, shunda *src/lib.rs* faylingiz 11-3 roʻyxatiga oʻxshaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-3: Muvaffaqiyatsiz bo'ladigan ikkinchi testni qo'shish, chunki biz `panic!` makrosini chaqiramiz.</span>

`cargo test` yordamida testlarni qaytadan test qiling. Chiqish 11-4 ro'yxatga o'xshash bo'lishi kerak, bu bizning `tadqiqot` sinovimizdan o'tganligini va `boshqa` muvaffaqiyatsiz ekanligini ko'rsatadi.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">Ro'yxat 11-4: Bitta test sinovdan o'tgan va bitta test muvaffaqiyatsizlikka uchragan sinov natijalari</span>

`OK` o'rniga `test tests::boshqa` qatori `FAILED`ni koʻrsatadi. Shaxsiy natijalar va xulosa o'rtasida ikkita yangi bo'lim paydo bo'ladi: birinchisida har bir sinov muvaffaqiyatsizligining batafsil sababi ko'rsatiladi. Bunday holda, biz *src/lib.rs* faylidagi 10-qatordagi `panicked at 'Make this test fail'` da panic qo'ygani uchun `boshqa` muvaffaqiyatsizlikka uchraganligi haqidagi tafsilotlarni olamiz. Keyingi bo'limda barcha muvaffaqiyatsiz testlarning nomlari keltirilgan, bu juda ko'p sinovlar va ko'plab batafsil muvaffaqiyatsiz sinov natijalari mavjud bo'lganda foydalidir. Muvaffaqiyatsiz test nomidan uni osonroq debug qilish uchun ishlatishimiz mumkin; testlarni o'tkazish usullari haqida ko'proq ["Testlar qanday o'tkazilishini nazorat qilish"][controlling-how-tests-are-run]<!-- ignore
--> section bo'limida gaplashamiz.

Xulosa qatori oxirida ko'rsatiladi: umuman olganda, bizning test natijasimiz `FAILED` muvaffaqiyatsiz. Bizda bitta test sinovi bor edi va bitta sinov muvaffaqiyatsiz tugadi.

Sinov natijalari turli stsenariylarda qanday ko‘rinishini ko‘rganingizdan so‘ng, keling, testlarda foydali bo‘lgan  `panic!`dan tashqari ba’zi makrolarni ko‘rib chiqaylik.

### Natijalarni `assert!` makrosi bilan tekshirish!

Standart kutubxona tomonidan taqdim etilgan `assert!` makrosi testdagi baʼzi shartlar `true`(toʻgʻri) boʻlishini taʼminlashni istasangiz foydali boʻladi. Biz `assert!` makrosiga mantiqiy(boolean) qiymatga baholovchi argument beramiz. Qiymat `true` bo'lsa, hech narsa sodir bo'lmaydi va sinovdan o'tadi. Agar qiymat `false` bo‘lsa, `assert!` makros testning muvaffaqiyatsiz bo‘lishiga olib kelishi uchun `panic!` chaqiradi. `assert!` makrosidan foydalanish bizning kodimiz biz rejalashtirgan tarzda ishlayotganligini tekshirishga yordam beradi.

5-bob, 5-15-ro'yxarda biz 11-5-ro'yxardada takrorlangan `Kvadrat` strukturasi va `ushlab_tur` metodidan foydalandik. Keling, ushbu kodni *src/lib.rs* fayliga joylashtiramiz, so'ngra `assert!` makrosidan foydalanib, u uchun testlarni yozamiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-5: 5-bobdagi `Kvadrat` strukturasi va uning `ushlab_tur` metodidan foydalanish</span>

`ushlab_tur` metodi mantiqiy(boolean) qiymatini qaytaradi, ya'ni bu `assert!` makrosi uchun mukammal foydalanish holati. 11-6 ro'yxatda biz kengligi 8 va balandligi 7 bo'lgan `Kvadrat` misolini yaratish va uning kengligi 5 va balandligi 1 bo'lgan boshqa `Kvadrat` misolini ushlab turishi mumkinligini tekshirish orqali `ushlab_tur` metodini bajaradigan testni yozamiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-6: Kattaroq kvadrat haqiqatan ham kichikroq kvadratni sig'dira oladimi yoki yo'qligini tekshiradigan `ushlab_tur` testi</span>

E'tibor bering, biz `tests` moduliga yangi qator qo'shdik: `use super::*;`. `tests` moduli odatiy modul bo'lib, biz 7-bobda ["Modul daraxtidagi elementga murojaat qilish yo'llari"][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore --> bo'limida ko'rib chiqqan odatiy ko'rinish qoidalariga amal qiladi. `tests` moduli ichki modul bo'lgani uchun biz tashqi moduldagi sinovdan o'tayotgan kodni ichki modul doirasiga kiritishimiz kerak. Biz bu yerda globdan foydalanamiz, shuning uchun tashqi modulda biz aniqlagan har qanday narsa ushbu `tests` modulida mavjud bo'ladi.

Biz sinovimizga `katta_kichikni_ushlab_turadi` deb nom berdik va o‘zimizga kerak bo‘lgan ikkita `Kvadrat` misolini yaratdik.
Keyin biz `assert!` makrosini chaqirdik va uni `kattaroq.ushlab_tur(&kichikroq)` deb chaqirish natijasini berdik. Bu ifoda `true` ni qaytarishi kerak, shuning uchun testimiz muvaffaqiyatli o'tishi kerak. Keling, bilib olaylik!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

Test muvaffaqiyatli o'tadi! Keling, yana bir sinovni qo'shamiz, bu safar kichikroq kvadrat kattaroq kvadratni ushlab turolmaydi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Chunki bu holda `ushlab_tur` funksiyasining to'g'ri natijasi `false` bo'lsa, biz uni `assert!` makrosiga o'tkazishdan oldin bu natijani inkor etishimiz kerak. Natijada, agar `ushlab_tur` `false` qiymatini qaytarsa, testimiz o'tadi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Ikkita sinovdan o'tadi! Keling, kodimizga xatolik kiritganimizda test natijalarimiz bilan nima sodir bo'lishini ko'rib chiqaylik. Kengliklarni solishtirganda katta belgisini kichikroq belgisi bilan almashtirish orqali `ushlab_tur` metodini amalga oshirishni o‘zgartiramiz:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Sinovlarni o'tkazish endi quyidagilarga olib keladi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Sinovlarimiz xatoni aniqladi! `kattaroq.kenglik` 8 va `kichikroq.kenglik` 5 bo'lganligi sababli, `ushlab_tur`da kengliklarni taqqoslash endi `false`ni qaytaradi: 8 5-dan kichik emas.

### Tenglikni `assert_eq!` va `assert_ne!` makroslari bilan tekshirish

Funksionallikni tekshirishning keng tarqalgan usuli - bu testdan o'tayotgan kod natijasi va kod qaytarilishini kutayotgan qiymat o'rtasidagi tenglikni tekshirish. Buni `assert!` makrosidan foydalanib, unga `==` operatori yordamida ifoda o'tkazishingiz mumkin. Biroq, bu shunday keng tarqalgan testki, standart kutubxona ushbu testni yanada qulayroq bajarish uchun bir juft makros-`assert_eq!` va `assert_ne!`-ni taqdim etadi. Ushbu makrolar mos ravishda tenglik yoki tengsizlik uchun ikkita argumentni solishtiradi. Agar tasdiqlash muvaffaqiyatsiz bo'lsa, ular ikkita qiymatni chop etadilar, bu esa *nima uchun* sinov muvaffaqiyatsiz tugaganini ko'rishni osonlashtiradi; aksincha, `assert!` makros `false` qiymatiga olib kelgan qiymatlarni chop etmasdan, `==` ifodasi uchun `false` qiymatini olganligini bildiradi.
11-7 ro'yxatda biz o'z parametriga `2` qo'shadigan `ikkita_qoshish` nomli funksiyani yozamiz, so'ngra bu funksiyani `assert_eq!` makrosidan foydalanib tekshiramiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">Roʻyxat 11-7: `assert_eq!` makrosidan foydalanib `ikkita_qoshish` funksiyasini sinab koʻrish</span>

Keling test o'tganligini tekshirib ko'raylik!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

Argument sifatida `4` ni `assert_eq!`ga o'tkazamiz, bu esa `ikkita_qoshish(2)` ni chaqirish natijasiga teng. Ushbu test qatori  `test tests::it_adds_two ... ok` va `ok` matni testimiz muvaffaqiyatli o'tganligini bildiradi!

`assert_eq!` muvaffaqiyatsiz bo'lganda qanday ko'rinishini ko'rish uchun kodimizga xato kiritamiz. `ikkita_qoshish` funksiyasining bajarilishini o'rniga `3` qo'shish uchun o`zgartiramiz:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Testlarni qayta ishga tushiring:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Bizning sinovimiz xatoni aniqladi! `ikkita_qosh` testi muvaffaqiyatsiz tugadi va xabarda muvaffaqiyatsizlikka uchragan tasdiqlash `` assertion failed: `(left == right)` `` va `left` va `right` qiymatlari nima. Bu xabar nosozliklarni(debugging) tuzatishni boshlashimizga yordam beradi: ``left``(chap) argumenti `4` edi, lekin `ikkita_qoshish(2)` bo'lgan `right`(o'ng) argumenti `5` edi. Tasavvur qilishingiz mumkinki, bu bizda juda ko'p sinovlar o'tkazilayotganda ayniqsa foydali bo'ladi.

E'tibor bering, ba'zi dasturlash tillarda va test tizimlarida(framework) tenglikni tasdiqlash funksiyalari parametrlari `expected` va `actual` deb nomlanadi va biz argumentlarni ko'rsatish tartibi muhim ahamiyatga ega. Biroq, Rustda ular `left` va `right` deb nomlanadi va biz kutgan qiymat va kod ishlab chiqaradigan qiymatni belgilash tartibi muhim emas. Biz ushbu testdagi tasdiqni `assert_eq!(ikkita_qoshish(2), 4)` deb yozishimiz mumkin, natijada `` assertion failed: `(left == right)` `` ko'rsatiladigan bir xil xato xabari paydo bo'ladi.

`assert_ne!` makros biz bergan ikkita qiymat teng bo'lmasa o'tadi va teng bo'lsa muvaffaqiyatsiz bo'ladi. Ushbu makro biz qiymat nima bo'lishini amin bo'lmagan holatlar uchun juda foydali bo'ladi, lekin biz qiymat nima bo'lmasligi kerakligini bilamiz.
Misol uchun, agar biz biron-bir tarzda uning kiritilishini o'zgartirishi kafolatlangan funksiyani sinab ko'rayotgan bo'lsak, lekin kirishni o'zgartirish metodi testlarimizni o'tkazadigan hafta kuniga bog'liq bo'lsa, tasdiqlash uchun eng yaxshi narsa, funksiyaning chiqishi kirishga teng emasligi bo'lishi mumkin.

Sirt ostida `assert_eq!` va `assert_ne!` makroslari mos ravishda `==` va `!=` operatorlaridan foydalanadi. Tasdiqlar bajarilmasa, bu makroslar debug formati yordamida o‘z argumentlarini chop etadi, ya’ni solishtirilayotgan qiymatlar `PartialEq` va `Debug` traitlarini bajarishi kerak. Barcha primitiv turlar va standart kutubxona turlarining aksariyati bu traittlarni amalga oshiradi. O'zingiz belgilagan structlar va enumlar uchun ushbu turlarning tengligini tasdiqlash uchun `PartialEq` ni qo'llashingiz kerak bo'ladi. Tasdiqlash muvaffaqiyatsizlikka uchraganida qiymatlarni chop etish uchun `Debug` ni ham qo'llashingiz kerak bo'ladi. 5-bobdagi 5-12 roʻyxatda aytib oʻtilganidek, ikkala trait ham derivable traitli boʻlganligi sababli, bu odatda struct yoki enum taʼrifiga `#[derive(PartialEq, Debug)]` izohini qoʻshishdek oddiy. Ushbu va boshqa ["Derivable Trait"][derivable-traits]<!-- ignore -->lari haqida batafsil ma'lumot olish uchun C ilovasiga qarang.

### Maxsus nosozlik xabarlarini qo'shish

Shuningdek, `assert!`, `assert_eq!` va `assert_ne!` makroslariga ixtiyoriy argumentlar sifatida xato xabari bilan chop etiladigan maxsus xabarni qo'shishingiz mumkin. Kerakli argumentlardan so‘ng ko‘rsatilgan har qanday argumentlar `format!` makrosiga uzatiladi (8-bobda ["`+` operatori yoki `format!` makrosi bilan birlashtirish"][concatenation-with-the--operator-or-the-format-macro]<!-- ignore --> bo‘limida muhokama qilingan), shuning uchun siz `{}` to'ldirgichlar va qiymatlarni o'z ichiga olgan format qatorini o'tkazishingiz mumkin. Maxsus xabarlar tasdiqlash nimani anglatishini hujjatlashtirish uchun foydalidir; test muvaffaqiyatsiz tugagach, kod bilan bog'liq muammo nimada ekanligini yaxshiroq tushunasiz.

Masalan, bizda odamlarni ism bilan kutib oladigan funksiya bor va biz funksiyaga kiritgan ism chiqishda(output) paydo bo‘lishini sinab ko‘rmoqchimiz:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Ushbu dasturga qoʻyiladigan talablar hali kelishib olinmagan va salomlashish boshidagi `Salom` matni oʻzgarishiga ishonchimiz komil. Talablar o'zgarganda testni yangilashni xohlamasligimizga qaror qildik, shuning uchun `salomlashish` funksiyasidan qaytarilgan qiymatga aniq tenglikni tekshirish o‘rniga, biz faqat chiqishda kirish parametrining matni borligini tasdiqlaymiz.

Endi standart sinov xatosi qanday koʻrinishini koʻrish uchun `name`ni chiqarib tashlash uchun `salomlashish` ni oʻzgartirish orqali ushbu kodga xatolik kiritamiz:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Ushbu testni bajarish quyidagi natijalarni beradi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Bu natija faqat tasdiqlash(assertion) muvaffaqiyatsizligini va tasdiqlash qaysi qatorda ekanligini ko'rsatadi. Foydaliroq xato xabari `salomlashish` funksiyasidan qiymatni chop etadi. Keling, `salomlashish` funksiyasidan olingan haqiqiy qiymat bilan toʻldirilgan maxsus xabar to'ldiruvchisi(plaseholder) bilan format qatoridan iborat maxsus xato xabarini qoʻshamiz:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Endi sinovni o'tkazganimizda, biz ko'proq ma'lumot beruvchi xato xabarini olamiz:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Sinov natijasida biz haqiqatda olgan qiymatni ko'rishimiz mumkin, bu biz kutgan narsaning o'rniga nima sodir bo'lganligini aniqlashga yordam beradi.

### `should_panic` yordamida panic tekshirish

Qaytish(return) qiymatlarini tekshirishdan tashqari, bizning kodimiz xato holatlarini biz kutganidek hal qilishini tekshirish muhimdir. Misol uchun, biz 9-bob, 9-13 ro'yxatda yaratgan `Taxmin` turini ko'rib chiqaylik. `Taxmin` dan foydalanadigan boshqa kod `Taxmin` misollarida faqat 1 dan 100 gacha bo'lgan qiymatlarni o'z ichiga olishi kafolatiga bog'liq. Ushbu diapazondan(chegaradan) tashqaridagi qiymatga ega `Taxmin` misolini yaratishga urinish panic qo'yishini ta'minlaydigan test yozishimiz mumkin.

Buni test funksiyamizga `should_panic` atributini qo‘shish orqali qilamiz. Funktsiya ichidagi kod panic qo'zg'atsa, test o'tadi;funksiya ichidagi kod panic qo'ymasa, test muvaffaqiyatsiz tugaydi.

11-8 ro'yxatda `Taxmin::new` xatolik holatlari biz kutgan vaqtda sodir bo'lishini tekshiradigan test ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">Ro'yxat 11-8: Test `panic!` keltirib chiqarishini tekshirish</span>

Biz `#[should_panic]` atributini `#[test]` atributidan keyin va u amal qiladigan test funksiyasidan oldin joylashtiramiz. Keling, ushbu testdan o'tgan natijani ko'rib chiqaylik:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Yaxshi ko'rinadi! Endi shartni olib tashlash orqali kodimizga xatolik kiritamiz,
agar qiymat 100 dan katta bo'lsa, `new` funksiya panic qo'zg'atadi:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Sinovni 11-8 ro'yxatda o'tkazganimizda, u muvaffaqiyatsiz bo'ladi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

Biz bu holatda unchalik foydali xabar olmaymiz, lekin test funksiyasini ko‘rib chiqsak, u `#[should_panic]` bilan izohlanganini ko‘ramiz. Biz erishgan muvaffaqiyatsizlik test funksiyasidagi kod panic qo'zg'atmaganligini anglatadi.

`should_panic` ishlatadigan testlar noaniq bo'lishi mumkin. Agar test biz kutgandan boshqa sababga ko'ra panic qo'zg'atsa ham, `should_panic` testi o'tadi. `should_panic` testlarini aniqroq qilish uchun biz `should_panic` atributiga ixtiyoriy `expected`  parametrini qo'shishimiz mumkin. Test dasturi xato xabarida taqdim etilgan matn mavjudligiga ishonch hosil qiladi. Masalan, 11-9 ro'yxatdagi `Taxmin` uchun o'zgartirilgan kodni ko'rib chiqing, bu erda `new` funksiya qiymat juda kichik yoki juda kattaligiga qarab turli xabarlar bilan panicga tushadi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 11-9: Belgilangan substringni oʻz ichiga olgan panic xabari bilan `panic!` sinovi</span>

Bu testdan o‘tadi, chunki biz `should_panic` atributining `expected` parametriga qo‘ygan qiymat `Taxmin::new` funksiyasi panicga tushadigan xabarning substringi hisoblanadi. Biz kutgan vahima haqidagi xabarni toʻliq koʻrsatishimiz mumkin edi, bu holda `Taxmin qilingan qiymat 1 dan 100 gacha bo'lishi kerak, 200 qabul qilinmaydi.`. Siz belgilashni tanlagan narsa panic xabarining qanchalik noyob yoki dinamik ekanligiga va testingiz qanchalik aniq bo'lishini xohlayotganingizga bog'liq. Bunday holda, test funksiyasidagi kod `else if qiymat > 100` holatini bajarishini ta`minlash uchun panic xabarining substringi kifoya qiladi.

`expected`  xabari bilan `should_panic` testi muvaffaqiyatsiz tugashi bilan nima sodir bo'lishini ko'rish uchun `if qiymat < 1` va `else if qiymat > 100` bloklarini almashtirish orqali kodimizga yana xato kiritamiz:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Bu safar biz `should_panic` testini o'tkazsak, u muvaffaqiyatsiz bo'ladi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

Muvaffaqiyatsizlik xabari shuni ko'rsatadiki, bu test biz kutgandek panic qo'zg'atdi, lekin panic xabarida kutilgan `Taxmin qilingan qiymat 100 dan kichik yoki unga teng bo'lishi kerak` qatori yo'q edi. Bu holatda biz olgan vahima xabari: `Taxmin qilingan qiymat 1 dan katta yoki teng bo'lishi kerak, 200 qabul qilinmaydi.`. Endi biz xatomiz qayerda ekanligini aniqlashni boshlashimiz mumkin!

### Testlarda `Result<T, E>` dan foydalanish

Bizning testlarimiz muvaffaqiyatsiz bo'lganda panic qo'zg'atadi. Biz `Result<T, E>` dan foydalanadigan testlarni ham yozishimiz mumkin! 11-1 roʻyxatidagi test `Result<T, E>` dan foydalanish va panic oʻrniga `Err`ni qaytarish uchun qayta yozilgan:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

`ishlaydi` funksiyasi endi `Result<(), String>` qaytish(return) turiga ega. Funksiya tanasida `assert_eq!` makrosini chaqirishdan ko'ra, testdan o'tganda `Ok(())` va test muvaffaqiyatsiz bo'lganda ichida `String` bilan `Err`ni qaytaramiz.

Testlarni `Result<T, E>` qaytaradigan qilib yozish testlar matnida savol belgisi operatoridan foydalanish imkonini beradi, bu testlarni yozishning qulay usuli bo'lishi mumkin, agar ulardagi har qanday operatsiya `Err` variantini qaytarsa, muvaffaqiyatsiz bo'lishi mumkin.

`Result<T, E>` ishlatadigan testlarda `#[should_panic]` izohidan(annotation) foydalana olmaysiz. Amaliyot `Err` variantini qaytarishini tasdiqlash uchun `Result<T, E>` qiymatida savol belgisi operatoridan foydalanmang. Buning oʻrniga `assert!(value.is_err())` dan foydalaning.

Endi siz testlarni yozishning bir necha usullarini bilganingizdan so'ng, keling, testlarimizni o'tkazganimizda nima sodir bo'layotganini ko'rib chiqamiz va `cargo test` bilan foydalanishimiz mumkin bo'lgan turli xil variantlarni ko'rib chiqamiz.

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
