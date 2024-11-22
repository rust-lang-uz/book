## Concurrencyda Shared-State

Message passing(Xabarni uzatish) - bu concurrencyni boshqarishning yaxshi usuli, ammo bu yagona emas. Yana bir usul bir nechta(multiple) threadlar bir xil umumiy ma'lumotlarga(shared data) kirishlari mumkin. Go tilidagi texnik hujjatlardagi shiorning ushbu qismini yana bir bor ko'rib chiqing: "xotirani almashish(sharing memory) orqali muloqot(comminicate) qilmang."

Xotirani almashish(sharing memory) orqali muloqot(comminication) qanday ko'rinishga ega bo'lar edi? Bundan tashqari, nima uchun message-passing enthusiastlar memory sharingdan foydalanmaslik haqida ogohlantiradilar?

Qaysidir ma'noda, har qanday dasturlash tilidagi kanallar bitta ownershiplik huquqiga o'xshaydi, chunki qiymatni kanalga o'tkazganingizdan so'ng, siz boshqa qiymatdan foydalanmasligingiz kerak. Shared memory concurrencyda bir nechta ownershiplik huquqiga o'xshaydi: concurrencyda bir nechta threadlar bir xil xotira joyiga(memory location) kirishi mumkin. 15-bobda ko'rganingizdek, smart pointerlar bir nechta ownershiplik qilish imkoniyatini yaratdi, bir nechta(multiple) ownershiplik murakkablikni oshirishi mumkin, chunki bu turli ownerlarni boshqarish kerak. Rust type tizimi va ownershiplik qoidalari ushbu boshqaruvni to'g'ri bajarishga katta yordam beradi. Misol uchun, shared memory uchun eng keng tarqalgan concurrency primitivlaridan biri bo'lgan mutexlarni ko'rib chiqaylik.

### Bir vaqtning o'zida bitta threaddan ma'lumotlarga kirishga ruxsat berish uchun mutexlardan foydalanish

*Mutex* bu *mutual exclusion* ning qisqartmasi boʻlib, mutex istalgan vaqtda baʼzi maʼlumotlarga faqat bitta threadga kirish imkonini beradi. Mutexdagi ma'lumotlarga kirish uchun thread birinchi navbatda mutexning *lock(qulf)*ni olishni so'rab kirishni xohlashini bildirishi kerak. Lock(qulf) - bu mutexning bir qismi bo'lgan ma'lumotlar tuzilmasi bo'lib, u hozirda ma'lumotlarga kimning eksklyuziv kirish huquqiga ega ekanligini kuzatib boradi. Shuning uchun, mutex qulflash tizimi(locking system) orqali o'zida mavjud bo'lgan ma'lumotlarni *himoya qilish(guarding)* sifatida tavsiflanadi.

Mutexlardan foydalanish qiyinligi bilan mashhur, chunki siz ikkita qoidani eslab qolishingiz kerak:

* Ma'lumotlardan foydalanishdan oldin siz qulfni olishga harakat qilishingiz kerak.
* Mutex himoya qiladigan ma'lumotlar bilan ishlashni tugatgandan so'ng, boshqa threadlar qulfni(lock) olishi uchun ma'lumotlarni qulfdan chiqarishingiz(unlock) kerak.

Mutexni tushunish uchun bitta mikrofon bilan konferensiyada guruh muhokamasining haqiqiy hayotiy misolini tasavvur qiling. Panel ishtirokchisi gapirishdan oldin mikrofondan foydalanishni xohlashini so'rashi yoki signal berishi kerak. Mikrofonni olishganda, ular xohlagancha gaplashishi mumkin va keyin mikrofonni gapirishni so'ragan keyingi ishtirokchiga beradi. Agar panel ishtirokchisi mikrofon bilan ishlashni tugatgandan so'ng uni o'chirishni unutib qo'ysa, boshqa hech kim gapira olmaydi. Agar umumiy mikrofonni boshqarish noto'g'ri bo'lsa, panel rejalashtirilganidek ishlamaydi!

Mutexlarni boshqarish juda qiyin bo'lishi mumkin, shuning uchun ko'p odamlar kanallarga(channel) ishtiyoq bilan qarashadi. Biroq, Rust type tizimi va ownershiplik qoidalari tufayli siz qulflash(locking) va qulfni noto'g'ri ochishingiz(unlocking) mumkin emas.

#### `Mutex<T>` API

Mutexdan qanday foydalanishga misol sifatida, keling, 16-12 ro'yxatda ko'rsatilganidek, bitta threadli kontekstda mutexdan foydalanishdan boshlaylik:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

<span class="caption">Ro'yxat 16-12: `Mutex<T>` API-ni soddaligi uchun single-threadli kontekstda oʻrganish</span>

Ko'pgina turlarda(type) bo'lgani kabi, biz bog'langan `new` funksiyasidan foydalangan holda `Mutex<T>` ni yaratamiz.
Mutex ichidagi ma'lumotlarga kirish uchun biz qulfni olish uchun `lock` metodidan foydalanamiz. Bu chaqiruv joriy threadni bloklaydi, shuning uchun u bizni qulflash navbati kelmaguncha hech qanday ishni bajara olmaydi.

Qulfni ushlab turgan boshqa thread panic qo'zg'atsa, `lock` chaqiruvi muvaffaqiyatsiz bo'ladi. Bunday holda, hech kim qulfni qo'lga kirita olmaydi, shuning uchun biz  `unwrap`ni tanladik va agar shunday vaziyatda bo'lsak, bu threadni panic qo'yishni tanladik.

Qulfni qo'lga kiritganimizdan so'ng, biz bu holatda `num` deb nomlangan return qiymatini ichidagi ma'lumotlarga o'zgaruvchan reference sifatida ko'rib chiqishimiz mumkin. Tur(type) tizimi `m` dagi qiymatni ishlatishdan oldin qulfni olishimizni ta'minlaydi. `m` turi `i32` emas, `Mutex<i32>`, shuning uchun biz `i32` qiymatidan foydalanish uchun `lock`ni chaqirishimiz kerak. Biz unuta olmaymiz; aks holda turdagi tizim bizga ichki `i32` ga kirishga ruxsat bermaydi.

Taxmin qilgan bolishingiz mumkinki Mutex<T> aqlli ko'rsatgich. Aniqroq qilib aytadigan bo'lsak, `lock` qo'ng'irog'i MutexGuard deb nomlangan  ochish
qo'ng'irog'i bilan oralgan LockResult-ga o'ralgan aqlli ko'rsatgichni qaytaradi . `MutexGuard` ko'rsatkichi esa bizning ichki ma'lumotlarimizga ishora
qilish uchun `Deref`ni amalga oshiradi( Derefdan foydalanadi). Aqlli ko'rsatgichda `Drop` ilovasi ham mavjud bo'lib, MutexGuard qo'llanilish doirasidan
tashqariga chiqqanda avtomatik ravishda  qulfni chiqaradi va bu esa ichki doiraning oxirida sodir bo'ladi. Natijada, biz qulfni(lock) bo'shatishni unutib
qo'ymaymiz va asosiysi mutexni boshqa threadlar tomonidan ishlatilishini bloklaymiz, chunki qulfni(lock) chiqarish avtomatik ravishda sodir bo'ladi.

Qulfni tashlaganimizdan so'ng, biz mutex qiymatini print qilishimiz(chop etishimiz ) va ichki `i32` ni 6 ga o'zgartira olganimizni ko'rishimiz mumkin.

#### Bitta `Muteks<T>`ni  Bir nechta mavzular o'rtasida ulashish(almashtirish):

Keling, `Mutex<T>`-dan foydalanib, bir nechta oqimlar o'rtasida qiymatni  share qilishga(qiymatni almashtirishga) harakat qilaylik. Biz 10 ta threadni
aylantiramiz va ularning har biri hisoblagich qiymatini 1 ga oshiradi, shuning uchun hisoblagich 0 dan 10 gacha boradi. 16-13 ro'yxatdagi keyingi misolda
kompilyator xatosi (compiler error)bo'ladi va biz bu xatoni o'rganish uchun ishlatamiz. `Mutex<T>`-dan foydalanish va Rust uni to'g'ri ishlatishimizga
qanday yordam berishi haqida ko'proq o'rganamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

<span class="caption">Ro'yxat 16-13: `Mutex<T>` tomonidan qo'riqlanadigan hisoblagichni har biri o'nta threaddan amalga oshirishi.</span>

`Mutex<T>`ni ichida `i32` ni ushlab turish uchun hisoblagich o'zgaruvchisini yaratamiz, xuddi 16-12 ro'yxatdagi kabi(listing 16-12). Keyingi amal esa,
biz raqamlar oralig'ida takrorlash orqali 10 ta thread yaratamiz. Biz `thread::spawn` dan foydalanamiz va barcha threadlarga bir xil yopilishni beramiz:
hisoblagichni threadga o'tkazish uchun ishlatiladigan  qulflash usulini amalga oshirish orqali(chaqirish orqali) orqali `Mutex<T>` da blokirovkaga ega
bo'ladi va keyin mutexdagi qiymatga 1 qo'shiladi. Thread o'zining yopilishini tugatgandan so'ng, `num` doirasi tashqariga chiqadi va boshqa thread uni
olishi uchun qulfni bo'shatadi.

Asosiy threadda biz barcha birlashma tutqichlarini yig'amiz. Keyin, 16-2 ro'yxatdagidek,barcha threadlar tugashiga ishonch hosil qilish uchun har bir
tutqichga `join` chaqiramiz. O'sha paytda asosiy thread qulfni oladi va ushbu dasturning natijasini print(chop etadi).

Bu misol tuzilmasligiga ishora qilingan. Endi nima uchunligini o'ylab ko'raylik!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

Xato xabari `counter`(hisoblagich) qiymati tsiklning oldingi iteratsiyasida ko'chirilganligini bildiradi. Rust bizga qulflash `counter`(hisoblagichining)
egaligini bir nechta mavzularga o'tkaza olmasligimizni aytadi. Keling, 15-bobda muhokama qilgan bir nechta egalik usuli bilan kompilyator xatosini
tuzataylik.

#### Bir nechta mavzular bilan bir nechta egalik

15-bobda mos yozuvlar hisoblangan qiymatni yaratish uchun aqlli ko'rsatkich Rc<T> yordamida bir nechta egalarga qiymat berdik. Bu yerda ham xuddi shunday
qilaylik va nima bo'lishini ko'ramiz.  `Mutex<T>`-ni  `Rc <T>`-ga 16-14-listingda o'rab olamiz va egalikni threadga ko'chirishdan oldin  `Rc<T>`-ni
klonlaymiz(nusxasini yaratmoq, cloning).

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

<span class="caption">Listing 16-14: `Rc<T>` ni ishlatib, bir nechta iplar (threads) `Mutex<T>` ga egalik qilishiga imkon berishga urinish.</span>

Yana bir bor, biz kompilyatsiya qilamiz va... turli xatolarni olamiz! Kompilyator bizga ko'p narsani o'rgatmoqda.

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Afsus, bu xato xabari juda uzun va yoqimsizroq(rasmiyligi uchun) ekan! Bu yerda diqqat qilish kerak bo'lgan muhim qism:
`` Rc<Mutex<i32>> ` threadlar o'rtasida xavfsiz yuborilishi mumkin emas ``. Kompilyator bizga buning sababini ham aytib beradi: `Send ` xususiyati
`Rc<Mutex<i32>> ` uchun amalga oshirilmagan. Keyingi bo'limda` Send`(Yuborish) haqida gaplashamiz: bu biz threadlar bilan ishlatadigan turlarni bir
vaqtda vaziyatlarda foydalanishga mo'ljallanganligini ta'minlaydigan xususiyatlardan biridir.

Afsuski, `Rc<T>` ni threadlar bo'ylab almashish xavfsiz emas(yuqorida ham aytilishicha mumkin ham emas). `Rc<T>` mos yozuvlar sonini boshqarganda, u clone
(klonlash) uchun har bir qo'ng'iroq uchun hisobni qo'shadi va har bir clone(klon) tushirilganda hisobdan ayiradi. Ammo hisobdagi o'zgarishlarni boshqa
oqim bilan to'xtatib qo'ymasligiga ishonch hosil qilish uchun u parallellik ibtidoiylaridan(parallallik ibtidoiysi bu concurrency yani raqobatga tegishli
mavzu) foydalanmaydi. Bu noto'g'ri hisob-kitoblarga olib kelishi mumkin - nozik xatolar, o'z navbatida, xotiraning oqishi yoki biz bilan ishlash
tugashidan oldin qiymatning tushib ketishiga olib kelishi mumkin. Bizga aynan `Rc<T>`ga o'xshash tur kerak bo'ladi, ammo u mos yozuvlar soniga
o'zgartirish kiritadi.

####  `Arc<T>` bilan atomik havolalarni hisoblash

Yaxshiyamki, `Arc<T>` `Rc<T>` kabi bir xil vaziyatlarda foydalanish uchun xavfsiz tur. A atomik degan ma'noni anglatadi, ya'ni bu atomik havola orqali
hisoblangan tur. Atomlar parallellik ibtidoiyning(concurrency:konkurentlik) qo'shimcha turi bo'lib,bu yerda batafsik ko'rib chiqolmaymiz: batafsil
ma'lumot uchun `std::sync::atomic` uchun standart kutubxona hujjatlariga(dokumentatsiyasiga) qarang. Shu nuqtada, atomlar ibtidoiy turlar kabi ishlashini
bilishingiz kerak, lekin ularni threadlar bo'ylab almashish xavfsizdir.

Keyin nima uchun barcha ibtidoiy turlar atom emasligi va nega standart kutubxona turlari sukut bo'yicha `Arc<T>` dan foydalanish uchun amalga
oshirilmaganligi haqida hayron bo'lishingiz mumkin. Buning sababi shundaki, thread xavfsizligi faqat sizga kerak bo'lganda to'lamoqchi bo'lgan ishlash
jazosi bilan birga keladi(PERFORMANCE PENALTY-IJRO,BAJARISH UCHUN JAZO) .Agar siz faqat bitta oqim ichidagi qiymatlar ustida amllarni bajarayotgan
bo'lsangiz yani atomik kafolatlarni bajarish shart bo'lmasa, kodingiz tezroq ishlashi mumkin.

Keling, misolimizga qaytaylik: `Arc<T>` va `Rc<T>` bir xil APIga ega, shuning uchun biz dasturimizni `use`(foydalanish) qatorini, `new`(yangi) chaqiruvni
va `clone`(klonlash) uchun qo'ng'iroqni o'zgartirish orqali tuzatamiz. 16-15 ro'yxatdagi kod nihoyat togri boladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

<span class="caption">Ro'yxat 16-15: `Mutex<T>`-ni o'rash uchun `Arc<T>` dan foydalanish, bir nechta mavzular bo'ylab egalik huquqini baham ko'rish
uchun</span>

Ushbu kod quyidagilarni print qiladi:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

Biz uddaladik! Biz 0 dan 10 gacha hisobladik, bu katta ishdek korinmasligi mumkin, ammo bu bizga `Mutex<T>` va thread xavfsizligi haqida ko‘p narsalarni 
o‘rgatadi. Hisoblagich faqat ko'paytirishdan koproq ish qila olishini orgatdi. Ushbu strategiyadan foydalanib, siz hisobni mustaqil qismlarga
bo'lishingiz, bu qismlarni threadlar bo'ylab ajratishingiz va keyin `Mutex<T>` dan foydalanib, har bir thread yakuniy natijani o'z qismi bilan yangilashi
mumkin.

E'tibor bering, agar siz oddiy raqamli amallarni bajarayotgan bo'lsangiz, standart kutubxonaning `std::sync::atomic` modulida taqdim etilgan `Mutex<T>`
turlaridan oddiyroq turlar mavjud. Ushbu turlar ibtidoiy turlarga xavfsiz, parallel, atomik kirishni ta'minlaydi va  ushbu misol uchun `Mutex<T>`ning
ibtidoiy turi bilan foydalanishni tanladik, shuning uchun `Mutex<T>` qanday ishlashiga e'tibor qaratishimiz mumkin.

### `RefCell<T>`/`Rc<T>` va `Mutex<T>`/`Arc<T>` o'rtasidagi o'xshashliklar 

Hisoblagich(counter) o'zgarmasligini payqagan bo'lishingiz mumkin, lekin biz uning ichidagi qiymatga o'zgaruvchan havolani olishimiz mumkin; bu `Mutex<T>
Cell` oilasi kabi ichki o'zgaruvchanlikni qollab quvvatlaydi. Xuddi shu tarzda biz `Rc<T>` ichidagi tarkibni o'zgartirishga ruxsat berish uchun 15-bobda
`RefCell<T>` dan foydalanganmiz, `Arc<T>` ichidagi tarkibni mutatsiya qilish uchun `Mutex<T>` dan foydalanamiz.

Yana bir muhim ma' lumot, `Mutex<T>` dan foydalanganda Rust sizni barcha turdagi mantiqiy xatolardan himoya qila olmaydi.  15-bobda `Rc<T>` dan
foydalanish oziga xos yozuvlar sikllarini yaratish xavfi bilan kelganligini eslang, bu erda ikkita `Rc<T>` qiymati bir-biriga tegishli bo'lib, xotira
susayishi, tanqisligiga olib keladi. Xuddi shunday, `Mutex<T>` ham boshi berk deadlocks(ko'chalarni) yaratish xavfi bilan birga keladi. Bular amal ikkita
resursni bloklashi kerak bo'lganda sodir bo'ladi va ikkita thread har biri locks(qulflardan) birini qo'lga kiritib. Agar siz ziddiyatlarga qiziqsangiz,
tanqislik deadlocks(ko'chasiga) ega Rust dasturini yaratishga harakat qiling; keyin har qanday tilda mutekslar uchun ziddiyatni yengilashtirish, yechim
topish strategiyalarini o'rganing va Rustda ularni amalga oshirishga kirishing. `Mutex<T>` va `MutexGuard` uchun standart kutubxona API hujjatlari
foydali ma'lumotlarni taqdim etadi.

Biz ushbu bobni `Send`(Yuborish) va `Sync`(Sinxronlashtirish) xususiyatlari va ularni maxsus turlar bilan qanday ishlatishimiz haqida gapirib,
yakunlaymiz. 

[atomic]: ../std/sync/atomic/index.html
