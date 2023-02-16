<!-- Old heading. Do not remove or links may break. -->
<a id="the-match-control-flow-operator"></a>
## `match` Control Flow konstruksiyasi

Rust `match` deb nomlangan juda kuchli control flow konstruksiyasiga ega, bu sizga qiymatni bir qator patternlar bilan solishtirish va keyin qaysi pattern mos kelishiga qarab kodni bajarish imkonini beradi. Patternlar literal qiymatlar, o'zgaruvchilar nomlari, wildcardlar va boshqa ko'plab narsalardan iborat bo'lishi mumkin; [18-bobda][ch18-00-patterns]<!-- ignore --> har xil turdagi patternlar va ular bajaradigan ishlar yoritilgan. `match`ning kuchi patternlarning ifodaliligidan va kompilyator barcha mumkin bo'lgan holatlar ko'rib chiqilishini tasdiqlashidan kelib chiqadi.

`match` iborasini tanga saralash mashinasiga o'xshatib tasavvur qiling: tangalar bo'ylab turli o'lchamdagi teshiklari bo'lgan yo'ldan pastga siljiydi va har bir tanga o'zi mos keladigan birinchi teshikdan tushadi. Xuddi shu tarzda, qiymatlar `match` dagi har bir patterndan o'tadi va birinchi patternda qiymat “fits,”, qiymat bajarish paytida ishlatiladigan tegishli kod blokiga tushadi.

Tangalar haqida gap ketganda, keling, ularni `match` yordamida misol qilib olaylik! Biz noma'lum AQSH tangasini oladigan funksiyani yozishimiz mumkin va xuddi sanash mashinasiga o'xshab uning qaysi tanga ekanligini aniqlaydi va 6-3 ro'yxatda ko'rsatilganidek, uning qiymatini sentlarda qaytaradi.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 6-3: Enum va `match` ifodasi, uning namunalari sifatida enumning variantlari mavjud</span>

Keling, `sentdagi_qiymat` funksiyasidagi `match` ni ajratamiz. Avval biz `match` kalit so'zidan keyin ifodani keltiramiz, bu holda bu qiymat `tanga` bo'ladi. Bu `if` bilan ishlatiladigan shartli ifodaga juda o'xshaydi, lekin
katta farq bor: `if` bilan shart mantiqiy qiymatga baholanishi kerak, ammo bu yerda u har qanday turdagi bo'lishi mumkin. Ushbu misoldagi `tanga` turi biz birinchi qatorda belgilagan `Tanga` enumidir.

Keyingi `match` armlari. Arm ikki qismdan iborat: pattern va ba'zi kod. Bu yerdagi birinchi arm `Tanga::Penny` qiymati boʻlgan patternga ega, soʻngra ishlash uchun pattern va kodni ajratuvchi `=>` operatori. Bu holatda kod faqat `1` qiymatidan iborat. Har bir arm keyingisidan vergul bilan ajratiladi.

`match` ifodasi bajarilganda, natijaviy qiymatni har bir armning patterniga solishtiradi. Agar pattern qiymatga mos kelsa, ushbu pattern bilan bog'langan kod bajariladi. Agar bu pattern qiymatga mos kelmasa, ijro tanga saralash mashinasida bo'lgani kabi keyingi armda davom etadi.
Bizda qancha arm kerak bo'lsa, shuncha arm bo'lishi mumkin: 6-3 ro'yxatda bizning `match`imizda to'rtta arm bor.

Har bir arm bilan bog'langan kod ifodadir va mos keladigan qismdagi ifodaning natijaviy qiymati butun `match` ifodasi uchun qaytariladigan qiymatdir.

Agar mos keladigan arm kodi qisqa bo'lsa, biz odatda jingalak qavslardan foydalanmaymiz, chunki bu ro'yxat 6-3da bo'lgani kabi, har bir arm shunchaki qiymat qaytaradi. Agar siz mos keladigan chiziqda bir nechta kod qatorlarini ishlatmoqchi bo'lsangiz, jingalak qavslardan foydalaning va armdan keyingi vergul ixtiyoriy bo'ladi. Masalan, quyidagi kodda `Omadli tanga!` metod har safar `Tanga::Penny` bilan chaqirilganda, lekin baribir blokning oxirgi qiymatini qaytaradi, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Qiymatlarni bog'laydigan patternlar

match armlarining yana bir foydali xususiyati shundaki, ular patternga mos keladigan qiymatlarning qismlarini bog'lashlari mumkin. Enum variantlaridan qiymatlarni shunday chiqarishimiz mumkin.

Misol tariqasida, uning ichida ma'lumotlarni saqlash uchun enum variantlarimizdan birini o'zgartiraylik.
1999 yildan 2008 yilgacha Qo'shma Shtatlar bir tomondan 50 shtatning har biri uchun turli dizayndagi tangalarni bosib chiqardi. Boshqa hech qanday tangalar davlat dizayniga ega emas, shuning uchun faqat quarterlarda bunday qo'shimcha qiymat mavjud. Biz ushbu maʼlumotni `Quarter` variantini uning ichida saqlangan `UsState` qiymatini kiritish uchun oʻzgartirish orqali `enum`ga qoʻshishimiz mumkin, biz buni 6-4 roʻyxatda qilganmiz.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Roʻyxat 6-4: `Quarter` varianti ham `UsState` qiymatiga ega boʻlgan `Tanga` enumi</span>

Tasavvur qiling-a, sizning do'stingiz barcha 50 shtatdan quarter yig'ishga harakat qilmoqda. Biz tangalar turi bo'yicha saralashimiz bilan birga, agar do'stimizda yo'q bo'lsa, ular uni o'z kollektsiyasiga qo'shishlari uchun har quarter bilan bog'liq shtat nomini ham chaqiramiz.

Ushbu kod uchun match ifodasida biz `Tanga::Quarter` varianti qiymatlariga mos keladigan patternga `shtat` deb nomlangan o'zgaruvchini qo‘shamiz. `Tanga::Quarter` mos kelganda, `shtat` o'zgaruvchisi o'sha quarter holati qiymatiga bog'lanadi. Keyin biz ushbu arm uchun kodda `shtat` dan foydalanishimiz mumkin, masalan:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Agar biz `sentdagi_qiymat(Tanga::Quarter(UsState::Alaska))` deb ataydigan bo'lsak, `tanga` `Tanga::Quarter(UsState::Alaska)` bo'ladi. Ushbu qiymatni har bir match armi bilan solishtirganda, biz `Tanga::Quarter(shtat)` ga yetguncha ularning hech biri mos kelmaydi. O'sha paytda `shtat` uchun majburiy `UsState::Alaska` qiymati bo'ladi. Keyin biz bu bog'lanishni `println!` ifodasida qo'llashimiz mumkin, shu bilan `Quarter` uchun `Tanga` enum variantidan ichki holat qiymatini olamiz.

### `Option<T>` uchun Match

Oldingi bo'limda biz `Option<T>` dan foydalanilganda `Some` holatidan ichki `T` qiymatini olishni xohladik; Biz, shuningdek, `Tanga` enum bilan qilganimizdek, `match` yordamida `Option<T>`ni boshqarishimiz mumkin! Tangalarni solishtirish o'rniga, biz `Option<T>` variantlarini solishtiramiz, lekin `match` ifodasining ishlash usuli bir xil bo'lib qoladi.

Aytaylik, biz `Option<i32>` ni oladigan funksiya yozmoqchimiz va agar ichida qiymat bo'lsa, bu qiymatga 1 qo'shiladi. Agar ichida qiymat bo'lmasa, funktsiya `None` qiymatini qaytarishi va hech qanday operatsiyani bajarishga urinmasligi kerak.

Ushbu funktsiyani yozish juda oson,  `match` tufayli va 6-5-Ro'yxatga o'xshaydi.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Roʻyxat 6-5: `Option`da `match` ifodasidan foydalanadigan funksiya<i32>`</span>

Keling, `bir_qoshish` ning birinchi bajarilishini batafsilroq ko'rib chiqamiz. Biz `bir_qoshish(besh)` ni chaqirganimizda, `bir_qoshish` tanasidagi `x` o'zgaruvchisi `Some(5)` qiymatiga ega bo'ladi. Keyin biz buni har bir matchning armi bilan taqqoslaymiz:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

`Some(5)` qiymati `None` patterniga mos kelmaydi, shuning uchun keyingi armga o'tamiz:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` ga `Some(i)` pattern mos keladimi? Ha bu shunday! Bizda ham xuddi shunday variant bor. Keyin `i` o'zgaruvchisi `Some` ichidagi qiymatga bog'lanadi, shuning uchun `i` `5` qiymatini oladi. Shundan so'ng match armidagi kod bajariladi, shuning uchun biz `i` qiymatiga 1 qo'shamiz va ichida jami `6` bo'lgan yangi `Some` qiymatini yaratamiz.

Keling, 6-5-Ro'yxatdagi `bir_qoshish` ning ikkinchi chaqiruvini ko'rib chiqaylik, bunda `x` `None`. Biz `match` ga kiramiz va birinchi arm bilan taqqoslaymiz:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Bu mos keladi! Qo'shiladigan qiymat yo'q, shuning uchun dastur to'xtaydi va `=>` o'ng tomonidagi `None` qiymatini qaytaradi. Birinchi arm mos kelganligi sababli, boshqa armlar taqqoslanmaydi.

`match` va enumlarni birlashtirish ko'p holatlarda foydalidir. Rust kodida siz ushbu patterni juda ko'p ko'rasiz: enum bilan `match`, o'zgaruvchini ichidagi ma'lumotlarga bog'lang va keyin unga asoslangan kodni bajaring. Avvaliga bu biroz qiyin, lekin ko'nikkaningizdan so'ng uni barcha tillarda bo'lishini xohlaysiz. Bu har doim foydalanuvchilarning sevimli texnikasi.

### Match barcha qiymat variantlarini qamrab oladi

Biz muhokama qilishimiz kerak bo'lgan `match` ning yana bir jihati bor: arm patterlari barcha imkoniyatlarni qamrab olishi kerak. Xatoga ega va kompilyatsiya qilinmaydigan `bir_qoshish` funksiyamizning ushbu versiyasini ko'rib chiqing:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Biz `None` holatini ko‘rib chiqmadik, shuning uchun bu kod xatolikka olib keladi. Yaxshiyamki, bu xato Rust qanday tutishni biladi. Agar biz ushbu kodni kompilyatsiya qilishga harakat qilsak, biz ushbu xatoni olamiz:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust biz barcha mumkin bo'lgan holatlarni qamrab olmaganimizni biladi va hatto qaysi patterni unutganimizni biladi! Rust-da matchlar to'liq: kod to'g'ri bo'lishi uchun biz barcha mumkin bo'lgan holatlarni qamrab olishimiz kerak. Ayniqsa, `Option<T>` holatida, Rust bizni `None` holatini aniq ko'rib chiqishni unutib qo'yishimizga to'sqinlik qilsa, bizni null bo'lishi mumkin bo'lgan qiymatga ega bo`lishimizdan himoya qiladi, shunday qilib, ilgari muhokama qilingan milliard dollarlik xatoni imkonsiz qiladi.

### Hammasini ushlash patternlari va `_` placeholder

Enumlardan foydalanib, biz bir nechta ma'lum qiymatlar uchun maxsus harakatlarni amalga oshirishimiz mumkin, ammo boshqa barcha qiymatlar uchun bitta standart amalni bajaramiz. Tasavvur qiling-a, biz o'yinni amalga oshirmoqdamiz, unda 3 ta o'yinda o'yinchi qimirlamaydi, aksincha, chiroyli yangi shlyapa oladi. Agar siz 7 ni aylantirsangiz, o'yinchingiz chiroyli shlyapasini yo'qotadi. Boshqa barcha qiymatlar uchun o'yinchi o'yin taxtasida shuncha bo'sh joyni siljitadi. Mana, bu mantiqni amalga oshiradigan `match`, bu erda narda toshlarni o'rash natijasi tasodifiy qiymat emas, balki qattiq kodlangan va mantiqning qolgan qismi jismlarsiz funktsiyalar bilan ifodalanadi, chunki ularni amalga oshirish ushbu doiradan tashqarida. misol:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Dastlabki ikkita arm uchun patternlar `3` va `7` harfli qiymatlardir. Boshqa barcha mumkin bo'lgan qiymatlarni qamrab oladigan oxirgi arm uchun pattern biz `boshqa` deb nomlash uchun tanlagan o'zgaruvchidir. `boshqa` arm uchun ishlaydigan kod o'zgaruvchini `player_harakati` funksiyasiga o'tkazish orqali ishlatadi.

Ushbu kod kompilatsiya qilinadi, garchi biz `u8` ga ega bo'lishi mumkin bo'lgan barcha qiymatlarni sanab o'tmagan bo'lsak ham, chunki oxirgi pattern maxsus sanab o'tilmagan barcha qiymatlarga mos keladi. Bu `match` toʻliq boʻlishi kerakligi haqidagi talabga javob beradi. E'tibor bering, biz armni eng oxirgi qo'yishimiz kerak, chunki patternlar tartibda baholanadi. Agar biz ushlovchi armni oldinroq qo'ysak, boshqa armlar hech qachon run bo'lmaydi, shuning uchun biz hammamizni tutgandan keyin arm qo'shsak, Rust bizni ogohlantiradi!

Rustda umumiy patterda qiymatdan foydalanishni istamaganimizda foydalanish mumkin bo'lgan pattern ham mavjud: `_` - har qanday qiymatga mos keladigan va bu qiymatga bog'lanmaydigan maxsus pattern. Bu Rustga biz qiymatdan foydalanmasligimizni bildiradi, shuning uchun Rust bizni foydalanilmagan o'zgaruvchi haqida ogohlantirmaydi.

Keling, o'yin qoidalarini shunday o'zgartiraylik: agar 3 yoki 7 dan boshqa narda toshi paydo bo'lsa, siz yana boshqatdan aylantirib tashlashingiz  kerak. Biz endi catch-all qiymatidan foydalanishimiz shart emas, shuning uchun biz kodimizni `boshqa` deb nomlangan o‘zgaruvchi o‘rniga `_` ishlatish uchun o‘zgartirishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Bu misol, shuningdek, to'liqlik talabiga javob beradi, chunki biz oxirgi qismdagi barcha boshqa qiymatlarni e'tiborsiz qoldiramiz; biz hech narsani unutmadik.

Nihoyat, biz o'yin qoidalarini yana bir bor o'zgartiramiz, shunda siz 3 yoki 7 ni o'tkazmaguningizcha sizning navbatingizda hech narsa sodir bo'lmaydi. Biz buni birlik qiymatidan (biz ["Tuple turi"][tuples]<!-- ignore --> section da aytib o'tgan bo'sh tuple turi) `_` armi bilan birga keladigan kod sifatida ifodalashimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Bu yerda biz Rustga aniq aytamizki, biz avvalgi armdagi patternga mos kelmaydigan boshqa qiymatdan foydalanmaymiz va bu holda hech qanday kodni ishga tushirishni xohlamaymiz.

[18-bobda][ch18-00-patterns]<!-- ignore --> biz ko'rib chiqadigan patternlar va match haqida ko'proq ma'lumot bor.
Hozircha biz `if let` sintaksisiga o‘tamiz, bu `match` ifodasi juda batafsil bo'lgan holatlarda foydali bo'lishi mumkin.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html
