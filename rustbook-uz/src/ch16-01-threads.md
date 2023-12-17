## Kodni bir vaqtning o'zida ishga tushirish uchun threadlardan foydalanish

Kodni bir vaqtning o'zida ishga tushirish(simultaneously) uchun threadlardan foydalanish hozirgi operatsion tizimlarning ko'pchiligida bajarilgan(execute) dastur kodi *process* ishga tushiriladi va operatsion tizim bir vaqtning o'zida bir nechta processlarni boshqaradi.Dastur doirasida siz bir vaqtning o'zida ishlaydigan(simultaneously) mustaqil qismlarga(independent part) ham ega bo'lishingiz mumkin. Ushbu mustaqil qismlarni boshqaradigan xususiyatlar *threadlar* deb ataladi. Masalan, veb-server bir vaqtning o'zida bir nechta so'rovlarga(requestlar) javob berishi uchun bir nechta(multiple) threadlarga ega bo'lishi mumkin.

Bir vaqtning o'zida bir nechta vazifalarni(multiple task) bajarish uchun dasturingizdagi hisoblashni bir nechta(multiple) threadlarga bo'lish unumdorlikni oshirishi mumkin, ammo bu murakkablikni ham oshiradi.
Theredlar bir vaqtning o'zida(simultaneously) ishlashi mumkinligi sababli, kodingizning turli xil ish threadlaridagi qismlari qaysi tartibda ishlashi haqida hech qanday kafolat yo'q. Bu muammolarga olib kelishi mumkin, masalan:

* Race conditionlari, bu yerda threadlar ma'lumotlar yoki resurslarga mos kelmaydigan tartibda kirishadi
* Deadlock, bu yerda ikkita thread bir-birini kutib, ikkala threadning davom etishiga to'sqinlik qiladi
* Faqat ma'lum holatlarda yuzaga keladigan va qayta ishlab chiqarish va ishonchli tarzda tuzatish qiyin bo'lgan xatolar

Rust threadlardan foydalanishning salbiy ta'sirini yumshatishga harakat qiladi, lekin multithreadli kontekstda dasturlash hali ham ehtiyotkorlik bilan o'ylashni talab qiladi va bitta threadda ishlaydigan dasturlardan farq qiladigan kod tuzilishini talab qiladi.

Dasturlash tillari threadlarni turli yo'llar bilan amalga oshiradi(impelement qiladi) va ko'pgina operatsion tizimlar yangi threadlarni yaratish uchun til chaqirishi mumkin bo'lgan API-ni taqdim etadi. Rust standart kutubxonasi(standard library) *1:1* threadni amalga oshirish modelidan foydalanadi, bunda dastur har bir til uchun bitta operatsion tizim threadidan foydalanadi. 1:1 modeliga turli xil o'zgarishlarni keltirib chiqaradigan boshqa theredlar modellarini amalga oshiradigan(implement qiladigan) cratelar mavjud.

### `spawn` yordamida yangi thread yaratish

Yangi thread yaratish uchun biz `thread::spawn` funksiyasini chaqiramiz va unga yangi threadda ishga tushirmoqchi bo'lgan kodni o'z ichiga olgan closureni (biz 13-bobda closurelar haqida gapirgan edik) o'tkazamiz. 16-1 ro'yxatdagi misol asosiy(main) threaddagi ba'zi matnni va yangi threaddagi boshqa matnni chop etadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

<span class="caption">Ro'yxat 16-1: Bir narsani chop etish uchun yangi thread yaratish, main thread esa boshqa narsalarni chop etish</span>

Esda tutingki, Rust dasturining asosiy ishi tugagach, barcha ochilgan threadlar ishlashni tugatgan yoki tugatmaganidan qat'i nazar, o'chiriladi. Ushbu dasturning chiqishi har safar bir oz boshqacha bo'lishi mumkin, ammo u quyidagilarga o'xshaydi:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
salom, main threaddan 1-raqam!
salom, ochilgan threaddan 1-raqam!
salom, main threaddan 2-raqam!
salom, ochilgan threaddan 2-raqam!
salom, main threaddan 3-raqam!
salom, ochilgan threaddan 3-raqam!
salom, main threaddan 4-raqam!
salom, ochilgan threaddan 4-raqam!
salom, ochilgan threaddan 5-raqam!
```

`thread::sleep`ga chaqiruvlar threadni qisqa muddatga uning bajarilishini to'xtatishga majbur qiladi, bu esa boshqa threadning ishlashiga imkon beradi. Ehtimol, threadlar navbatma-navbat bo'ladi, lekin bu kafolatlanmaydi: bu sizning operatsion tizimingiz threadlarni qanday rejalashtirishiga(schedule) bog'liq. Bu ishga tushirishda birinchi bo'lib main thread chop etiladi, garchi ishlab chiqarilgan threadning chop etish bayonoti kodda birinchi bo'lib paydo bo'lsa ham. Va biz paydo bo'lgan thredga `i` 9 bo'lguncha chop etishni aytgan bo'lsak ham, asosiy thread yopilishidan oldin u 5 ga yetdi.

Agar siz ushbu kodni ishga tushirsangiz va faqat main threaddan olingan ma'lumotlarni ko'rsangiz yoki hech qanday o'xshashlikni ko'rmasangiz, operatsion tizimning threadlar o'rtasida almashishi uchun ko'proq imkoniyatlar yaratish uchun diapazonlardagi raqamlarni oshirib ko'ring.

### `join` handlerlari yordamida barcha threadlar tugashini kutilmoqda

16-1 ro'yxatidagi kod ko'pincha main thread tugashi tufayli paydo bo'lgan threadni muddatidan oldin to'xtatibgina qolmay, balki threadlarning ishlash tartibiga kafolat yo'qligi sababli, biz ham yangi ochilgangan threadning umuman ishga tushishiga kafolat bera olmaymiz!

O'zgaruvchida `thread::spawn` ning qaytish(return) qiymatini saqlash orqali ochilgangan threadning ishlamasligi yoki muddatidan oldin tugashi muammosini hal qilishimiz mumkin. `thread::spawn` ning qaytish turi(return type) `JoinHandle` dir. `JoinHandle` - bu tegishli qiymat bo'lib, biz `join` metodini chaqirganimizda, uning threadi tugashini kutamiz. 16-2 ro'yxatda biz 16-1 ro'yxatda yaratgan `JoinHandle` dan qanday foydalanish va `join`ni chaqirish orqali yaratilgan thread `main` chiqishdan oldin tugashini ko'rsatadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

<span class="caption">Roʻyxat 16-2: `JoinHandle` ni `thread::spawn` dan saqlash, threadning oxirigacha ishga tushishini kafolatlash</span>

Handleda `join`ni chaqirish, handle bilan ifodalangan thread tugaguncha ishlayotgan threadni bloklaydi. Threadni *bloklash* uning ish bajarishi yoki chiqishining oldini olish degani. Biz chaqiruvni(call) main threadning `foor` loop siklidan keyin qo'yganimiz sababli, 16-2 ro'yxatini ishga tushirish shunga o'xshash natijani berishi kerak:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
salom, main threaddan 1-raqam!
salom, main threaddan 2-raqam!
salom, ochilgan threaddan 1-raqam!
salom, main threaddan 3-raqam!
salom, ochilgan threaddan 2-raqam!
salom, main threaddan 4-raqam!
salom, ochilgan threaddan 3-raqam!
salom, ochilgan threaddan 4-raqam!
salom, ochilgan threaddan 5-raqam!
salom, ochilgan threaddan 6-raqam!
salom, ochilgan threaddan 7-raqam!
salom, ochilgan threaddan 8-raqam!
salom, ochilgan threaddan 9-raqam!
```

Ikki thread almashishda davom etadi, lekin main thread `handle.join()` chaqiruvi tufayli kutadi va hosil qilingan thread tugamaguncha tugamaydi.

Ammo keling, `main` da `for` loop siklidan oldin `handle.join()` ni ko‘chirsak nima bo‘lishini ko‘rib chiqamiz, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

Main thread ochilgan thread tugashini kutadi va keyin `for` loop siklini ishga tushiradi, shuning uchun bu yerda ko'rsatilganidek, chiqish boshqa qo'shilmaydi:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
salom, ochilgan threaddan 1-raqam!
salom, ochilgan threaddan 2-raqam!
salom, ochilgan threaddan 3-raqam!
salom, ochilgan threaddan 4-raqam!
salom, ochilgan threaddan 5-raqam!
salom, ochilgan threaddan 6-raqam!
salom, ochilgan threaddan 7-raqam!
salom, ochilgan threaddan 8-raqam!
salom, ochilgan threaddan 9-raqam!
salom, main threaddan 1-raqam!
salom, main threaddan 2-raqam!
salom, main threaddan 3-raqam!
salom, main threaddan 4-raqam!
```

Kichik tafsilotlar(detail), masalan, `join` deb ataladigan joy, sizning threadlaringiz bir vaqtning o'zida ishlashi yoki ishlamasligiga ta'sir qilishi mumkin.

### Threadlar bilan `move` closuredan foydalanish

Biz tez-tez closurelar `thread::spawn` ga o'tiladigan `move` kalit so'zidan foydalanamiz, chunki closure keyinchalik environmentdan foydalanadigan qiymatlarga ownershiplik(egalik) qiladi va shu tariqa bu qiymatlarga ownershiplik huquqini bir threaddan ikkinchisiga o'tkazadi. 13-bobning “Ma’lumotnomalarni qo‘lga kiritish yoki ownershiplik huquqini ko‘chirish” bo‘limida biz closure kontekstida `move`ni muhokama qildik. Endi biz `move` va `thread::spawn` o'rtasidagi o'zaro ta'sirga ko'proq e'tibor qaratamiz.

Ro'yxat 16-1da e'tibor bering, biz `thread::spawn` ga o'tadigan closure hech qanday argument talab qilmaydi: biz ochilgan thread kodidagi main threaddan hech qanday ma'lumotdan foydalanmayapmiz. Tugallangan threaddagi main threaddan ma'lumotlarni ishlatish uchun ochilgan threadning yopilishi kerakli qiymatlarni olishi kerak. 16-3 ro'yxatda main threadda vector yaratish va uni ishlab ochilgan threadda ishlatishga urinish ko'rsatilgan. Biroq, bu hali ishlamaydi, buni birozdan keyin ko'rasiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

<span class="caption">Ro'yxat 16-3: main thread tomonidan yaratilgan vectorni boshqa threadda ishlatishga urinish</span>

Closure `v` dan foydalanadi, shuning uchun u `v` ni oladi va uni closure environmentining bir qismiga aylantiradi. Chunki `thread::spawn` bu closureni yangi threadda ishga tushiradi, biz ushbu yangi thread ichidagi `v` ga kirishimiz kerak. Ammo biz ushbu misolni kompilatsiya qilganimizda, biz quyidagi xatoni olamiz:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust `v` ni qanday capture qilishni *infers*(xulosa) qiladi va `println!` faqat `v` ga reference kerakligi sababli, closure `v` ni olishga harakat qiladi. Biroq, muammo bor: Rust ochilgan threrad qancha vaqt ishlashini ayta olmaydi, shuning uchun `v` ga reference har doim haqiqiy(valiq yaroqli) bo'lishini bilmaydi.

16-4 ro'yxatda `v` ga reference bo'lishi mumkin bo'lgan senariy ko'rsatilgan va u yaroqli(valiq) bo'lmaydi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

<span class="caption">Roʻyxat 16-4: `v` tushiradigan main threaddan `v` ga referenceni olishga harakat qiluvchi yopilgan thread.</span>

Agar Rust bizga ushbu kodni ishga tushirishga ruxsat bergan bo'lsa, ochilgan thread umuman ishlamasdan darhol fonga qo'yilishi mumkin. Ochilgan thread ichida `v` ga reference bor, lekin main thread darhol `v` ni tushiradi, biz 15-bobda muhokama qilgan `drop` funksiyasidan foydalangan holda. Keyin, ochilgan thread bajarila boshlaganda, `v` endi haqiqiy(valiq) emas, shuning uchun unga referense ham yaroqsiz. Oh yo'q!

16-3 ro'yxatdagi kompilyator xatosini tuzatish uchun xato xabari maslahatidan foydalanishimiz mumkin:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Closuredan oldin `move` kalit so‘zini qo‘shish orqali biz closureni Rustga qiymatlarni olishi kerak degan xulosaga(infer) kelishga ruxsat berishdan ko‘ra, u foydalanadigan qiymatlarga ownershiklik qilishga majburlaymiz. 16-5 ro'yxatda ko'rsatilgan 16-3 ro'yxatga kiritilgan o'zgartirish biz xohlagan tarzda kompilatsiya bo'ladi va ishlaydi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

<span class="caption">Roʻyxat 16-5: `move` kalit soʻzidan foydalanib, closureni oʻzi foydalanadigan qiymatlarga ownershiplik qilishga majburlash</span>

Kodni 16 4 ro'yxatida tuzatish uchun xuddi shu narsani sinab ko'rishimiz mumkin, bu yerda main thread `move` closuresi orqali `drop` deb ataladi. Biroq, bu tuzatish ishlamaydi, chunki 16-4-raqamli roʻyxat boshqa sabablarga koʻra amalga oshirilmaydi. Agar biz closurega `move` ni qo‘shsak, biz `v` ni closure environmentiga o'tkazamiz va biz main threadda endi `drop` ni chaqira olmaymiz. Buning o'rniga biz ushbu kompilyator xatosini olamiz:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rustning ownershiplik(egalik) qoidalari bizni yana qutqardi! 16-3 roʻyxatdagi koddan xatoga yoʻl qoʻydik, chunki Rust konservativ boʻlib, thread uchun faqat `v` harfini oldi, bu esa main thread nazariy jihatdan ochilgangan threadning referenceni bekor qilishi mumkinligini anglatadi. Rustga `v` ownershiplik huquqini ochilgan threadga o'tkazishni aytish orqali biz Rustga main thread endi `v` dan foydalanmasligiga kafolat beramiz. Agar biz 16-4 ro'yxatni xuddi shunday o'zgartirsak, main threadda `v` dan foydalanmoqchi bo'lganimizda ownershiplik qoidalarini buzgan bo'lamiz. `move` kalit so'zi Rustning borrowing olishning konservativ defaultini bekor qiladi; ownershiplik qoidalarini buzishimizga yo'l qo'ymaydi.

Threadar va thread API haqida asosiy tushunchaga ega bo'lgan holda, keling, threadlar bilan nima qilishimiz mumkinligini ko'rib chiqaylik.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
