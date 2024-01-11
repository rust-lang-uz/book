## Threadlar orasidagi ma'lumotlarni uzatish uchun Message Passing(xabar uzatish)dan foydalanish

Xavfsiz concurrencyni ta'minlashning tobora ommalashib borayotgan yondashuvlaridan biri bu *message passing* bo'lib, bu yerda threadlar yoki actorlar bir-biriga ma'lumotlarni o'z ichiga olgan xabarlarni yuborish orqali muloqot qilishadi. [Go tili texnik hujjatlaridagi](https://golang.org/doc/effective_go.html#concurrency) shiordagi g‘oya: “Xotirani almashish(share) orqali muloqot qilmang; Buning o'rniga, muloqot(communication) orqali xotirani share qiling."

Message-sending(xabar yuborish) concurrencyni amalga oshirish uchun Rustning standart kutubxonasi *channels* amalga oshirishni ta'minlaydi. Channel(kanal) - bu umumiy dasturlash tushunchasi bo'lib, uning yordamida ma'lumotlar bir threaddan ikkinchisiga yuboriladi.

Dasturlashdagi kanalni(channel) oqim yoki daryo kabi suvning yo'naltirilgan kanali kabi tasavvur qilishingiz mumkin. Agar siz daryoga rezina o'rdak kabi narsalarni qo'ysangiz, u suv yo'lining oxirigacha pastga tushadi.

Kanalning ikkita yarmi bor: uzatuvchi(transmitte) va qabul qiluvchi(receiver). Transmitterning yarmi daryoga rezina o'rdak qo'yadigan yuqori oqim joyidir va qabul qiluvchining yarmi rezina o'rdak quyi oqimga tushadi. Kodingizning bir qismi siz yubormoqchi bo'lgan ma'lumotlarga ega bo'lgan transmitterdagi metodlarni chaqiradi, boshqa qismi esa kelgan xabarlarni qabul qiluvchi tomonni tekshiradi. Agar transmitter yoki receiverning yarmi tushib qolsa, kanal *closed(yopiq)* deyiladi.

Bu yerda biz qiymatlarni yaratish va ularni kanalga yuborish uchun bitta threadga ega bo'lgan dasturni va qiymatlarni qabul qiladigan va ularni chop etadigan boshqa threadni ishlab chiqamiz. Featureni tasvirlash uchun kanal yordamida  threadlar orasidagi oddiy qiymatlarni yuboramiz. Texnika bilan tanishganingizdan so'ng, siz bir-biringiz bilan aloqa o'rnatishingiz kerak bo'lgan har qanday threadlar uchun kanallardan foydalanishingiz mumkin, masalan, chat tizimi yoki ko'p threadlar hisoblash qismlarini bajaradigan va qismlarni natijalarni jamlaydigan bitta threadga yuboradigan tizim.

Birinchidan, 16-6 ro'yxatda biz channel(kanal) yaratamiz, lekin u bilan hech narsa qilmaymiz.
E'tibor bering, bu hali kompilyatsiya qilinmaydi, chunki Rust kanal orqali qanday turdagi qiymatlarni yuborishimizni ayta olmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">Ro'yxat 16-6: Kanal yaratish va ikkita yarmini `tx` va `rx` ga belgilash</span>

Biz `mpsc::channel` funksiyasidan foydalanib yangi kanal yaratamiz; `mpsc` *multiple producer, single consumer* degan maʼnoni anglatadi. Qisqa qilib aytganda, Rustning standart kutubxonasi kanallarni implement qilish usuli kanalda qiymatlarni ishlab chiqaradigan bir nechta *sending(jo'natish)* uchlari bo'lishi mumkin, ammo bu qiymatlarni qabul qiladigan consumer faqat bitta *receiving(qabul qiluvchi)* end bo'lishi mumkinligini anglatadi. Tasavvur qiling-a, bir nechta daryolar birlashib, bitta katta daryoga quyiladi: har qanday oqim oxirida bitta daryoga tushadi. Hozircha biz bitta ishlab chiqaruvchidan(single producer) boshlaymiz, lekin biz ushbu misol ishlaganda bir nechta producerlarni(multiple producer) qo'shamiz.

`mpsc::channel` funksiyasi tupleni qaytaradi, uning birinchi elementi jo'natuvchi end - transmitter va ikkinchi element - receiver end - qabul qiluvchidir. `tx` va `rx` qisqartmalari an'anaviy ravishda ko'plab sohalarda mos ravishda *transmitter* va *receiver* uchun ishlatiladi, shuning uchun biz har bir endni ko'rsatish uchun o'zgaruvchilarimizni shunday nomlaymiz. Biz tuplelarni destruksiya pattern `let` statementdan foydalanmoqdamiz; Biz 18-bobda `let` statementlarida patternlardan foydalanish va destruksiyani muhokama qilamiz. Hozircha shuni bilingki, `let` statementdan shu tarzda foydalanish `mpsc::channel` tomonidan qaytarilgan tuple qismlarini ajratib olishning qulay usuli hisoblanadi.

16-7 ro'yxatda ko'rsatilganidek, transmitter uchini ochilgan threadga o'tkazamiz va u bitta threadni yuborsin, shunda hosil qilingan thread main thread bilan bog'lanadi. Bu daryoning yuqori oqimiga rezina o'rdak qo'yish yoki bir threaddan ikkinchisiga chat xabarini yuborishga o'xshaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<span class="caption">Roʻyxat 16-7: `tx` ni ochilgan threadga koʻchirish va `salom` yuborish</span>

Shunga qaramay, biz yangi thread yaratish uchun `thread::spawn` dan foydalanamiz va keyin `move` yordamida `tx` ni yopishga(close) o'tkazamiz, shunda hosil qilingan thread `tx`ga ega bo'ladi. Kanal orqali xabarlarni jo'natish uchun ochilgan thread transmitterga ega bo'lishi kerak. Transmitterda biz jo'natmoqchi bo'lgan qiymatni qabul qiluvchi `send` metodi mavjud. `send` metodi `Result<T, E>` typeni qaytaradi, shuning uchun agar qabul qiluvchi(receiver) allaqachon drop qilingan bo'lsa va qiymatni yuborish uchun joy bo'lmasa, yuborish operatsiyasi xatolikni qaytaradi. Ushbu misolda biz xatolik yuz berganda panic qo'yish uchun `unwrap` ni chaqiramiz. Ammo haqiqiy dasturda biz uni to'g'ri hal qilamiz: xatolarni to'g'ri hal qilish strategiyalarini ko'rib chiqish uchun 9-bobga qayting.

16-8 ro'yxatda biz main threaddagi qabul qiluvchidan(receive) qiymatni olamiz. Bu daryoning oxiridagi suvdan rezina o'rdakni olish yoki suhbat xabarini olish kabi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<span class="caption">Ro'yxat 16-8: main threadda `salom` qiymatini olish va uni chop etish</span>

Receiverda ikkita foydali metod mavjud: `recv` va `try_recv`. Biz `recv` dan foydalanmoqdamiz, bu *receive(qabul qilish)* ning qisqartmasi bo'lib, u main threadning bajarilishini bloklaydi va kanalga qiymat yuborilguncha kutadi. Qiymat yuborilgach, `recv` uni `Result<T, E>` shaklida qaytaradi. Transmitter yopilganda, `recv` boshqa qiymatlar kelmasligini bildirish uchun xatolikni qaytaradi.

`try_recv` metodi bloklanmaydi, aksincha darhol `Result<T, E>`ni qaytaradi: `Ok` qiymati, agar mavjud bo‘lsa, xabarni ushlab turadi va bu safar hech qanday xabar bo‘lmasa, `Err` qiymati. `try_recv` dan foydalanish, agar ushbu thread xabarlarni kutayotganda boshqa ishi boʻlsa foydali boʻladi: biz tez-tez `try_recv` ni chaqiradigan, agar mavjud bo'lsa, xabarni ko'rib chiqadigan va boshqasi qayta tekshirilgunga qadar biroz vaqt ishlaydigan sikl yozishimiz mumkin.

Biz ushbu misolda soddalik uchun `recv` dan foydalandik; bizda main thread uchun xabarlarni kutishdan boshqa ishimiz yo'q, shuning uchun main threadi bloklash maqsadga muvofiqdir.

Kodni 16-8 ro'yxatda ishga tushirganimizda, biz main threaddan chop etilgan qiymatni ko'ramiz:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Tushundim: salom
```

Mukammal! Perfect!

### Kanallar va ownershiplik(egalkik) huquqini o'tkazish

Ownershiplik qoidalari xabarlarni jo'natishda muhim rol o'ynaydi, chunki ular xavfsiz, bir vaqtda kod yozishga yordam beradi. Bir vaqtning o'zida dasturlashda(concurrent programming) xatolarning oldini olish Rust dasturlarida ownershiplik haqida o'ylashning afzalligi hisoblanadi. Muammolarning oldini olish uchun kanallar va ownershiplik qanday ishlashini ko‘rsatish uchun tajriba o‘tkazamiz: biz kanalga yuborganimizdan so‘ng `val` qiymatidan foydalanishga harakat qilamiz. Nima uchun bu kodga ruxsat berilmaganligini bilish uchun 16-9-raqamdagi kodni kompilyatsiya qilib ko'ring:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<span class="caption">Roʻyxat 16-9: `val`ni kanalga yuborganimizdan keyin foydalanishga urinish</span>

Bu yerda biz `tx.send` orqali kanalga yuborganimizdan so‘ng `val`ni chop etishga harakat qilamiz.
Bunga ruxsat berish noto'g'ri fikr bo'ladi: qiymat boshqa threadga yuborilgandan so'ng, biz qiymatni qayta ishlatishdan oldin uni o'zgartirishi yoki tashlab yuborishi(drop) mumkin. Potensial ravishda, boshqa threadning o'zgartirishlari nomuvofiq yoki mavjud bo'lmagan ma'lumotlar tufayli xatolar yoki kutilmagan natijalarga olib kelishi mumkin. Biroq, agar biz 16-9 ro'yxatdagi kodni kompilyatsiya qilmoqchi bo'lsak, Rust bizga xato qiladi:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Bizning concurrency xatomiz kompilyatsiya vaqtida xatolikka olib keldi. `send` funksiyasi oʻz parametriga ownershiplik qiladi va qiymat koʻchirilganda qabul qiluvchi(receiver) unga ownershiplik qiladi. Bu bizni qiymatni yuborgandan keyin tasodifan qayta ishlatishdan to'xtatadi; ownershiplik tizimi hamma narsa yaxshi ekanligini tekshiradi.

### Bir nechta qiymatlarni yuborish va qabul qiluvchining(receiver) kutayotganini ko'rish

16-8 ro'yxatdagi kod kompilatsiya bo'ldi va ishga tushirildi, lekin u bizga ikkita alohida thread kanal orqali bir-biri bilan gaplashayotganini aniq ko'rsatmadi. 16-10-ro'yxatda biz 16-8-ro'yxatdagi kod bir vaqtda ishlayotganini tasdiqlovchi ba'zi o'zgartirishlar kiritdik: ochilgan thread endi bir nechta xabarlarni yuboradi va har bir xabar o'rtasida bir soniya pauza qiladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<span class="caption">Ro'yxat 16-10: Bir nechta xabarlarni yuborish va har biri o'rtasida pauza qilish</span>

Bu safar, ochilgan threadda biz main threadga yubormoqchi bo'lgan satrlar vektori mavjud. Biz ularni takrorlaymiz, har birini alohida yuboramiz va 1 soniyalik `Duration` qiymati bilan `thread::sleep` funksiyasini chaqirish orqali har biri o‘rtasida pauza qilamiz.

Main threadda biz endi `recv` funksiyasini aniq chaqirmayapmiz: buning o'rniga biz `rx` ni iterator sifatida ko'rib chiqamiz. Qabul qilingan har bir qiymat uchun biz uni chop etamiz. Kanal yopilganda(close), iteratsiya tugaydi.

16-10 ro'yxatdagi kodni ishga tushirganda, har bir satr orasida 1 soniyalik pauza bilan quyidagi chiqishni ko'rishingiz kerak:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Tushundim: qandaysan
Tushundim: otabek
Tushundim: vodiyga
Tushundim: ketti
```

Bizda main threaddagi `for` siklida pauza yoki kechikishlar keltirib chiqaradigan kod yo‘qligi sababli, biz main thread hosil qilingan threaddan qiymatlarni olishni kutayotganini aytishimiz mumkin.

### Transmitterni klonlash orqali bir nechta producerlarni yaratish

Avvalroq `mpsc` *multiple producer, single consumer* degan qisqartma ekanligini eslatib o'tgan edik. Keling, 16-10 ro'yxatdagi kodni ishlatish va kengaytirish uchun `mpsc` ni qo'yaylik va barchasi bir xil qabul qiluvchiga(receiver) qiymatlarni yuboradigan bir nechta threadlarni yaratamiz. Biz buni 16-11 ro'yxatda ko'rsatilganidek, transmitterni klonlash orqali amalga oshirishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 16-11: Bir nechta producerlardan(multiple producer) bir nechta xabarlarni(multiple message) yuborish</span>

Bu safar, birinchi ochilgan threadni yaratishdan oldin, biz transmitterda `clone` deb nomlaymiz. Bu bizga yangi transmitterni beradi, biz birinchi ochilgan threadga o'tishimiz mumkin. Biz asl transmitterni ikkinchi ochilgan threadga o'tkazamiz.
Bu bizga ikkita thread beradi, ularning har biri bitta qabul qiluvchiga(receiver) turli xabarlar yuboradi.

Kodni ishga tushirganingizda, chiqishingiz quyidagicha ko'rinishi kerak:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Tushundim: salom
Tushundim: threaddan
Tushundim: siz
Tushundim: uchun
Tushundim: ko'plab
Tushundim: habarlar
Tushundim: hammasi
Tushundim: ishlayapti
```

Tizimingizga qarab qiymatlarni boshqa tartibda ko'rishingiz mumkin. Bu concurrencyni qiziqarli va qiyin qiladi. Agar siz `thread::sleep` bilan tajriba o'tkazsangiz, unga turli threadlarda turli qiymatlar bersangiz, har bir ishga tushirish aniqroq bo'lmaydi va har safar har xil chiqish hosil qiladi.

Endi biz kanallar qanday ishlashini ko'rib chiqdik, keling, boshqa concurrency usulini ko'rib chiqaylik.
