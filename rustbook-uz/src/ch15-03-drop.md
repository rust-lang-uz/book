## `Drop` Trait bilan tozalash uchun kodni yuritish

Agar qiymat o‘z doirasidan chiqqanda uni o‘zgartirish imkonini beradigan ikkinchi muhim sanalgan smart pointer namunasidan biri bu `Drop`dir. Siz `Drop` traitini implementatsiya qilish uchun xohlagan turdan foydalanishingiz mumkin, va kodni fayl yoki tarmoqlarni ulash resurslarini yaratish uchun ham ishlatilishi mumkin 

`Drop`ni smart pointerlar kontekstida ishlatishimizning sababi `Drop` traiti smart pointerni implementatsiyasida deyarli har doim ishlatiladi. Masalan, qachonki `Box<T>` tashlab yuborilganda u quti ko‘rsatayotgan heapdan joy ajratadi.

Ayrim dasturlash tillarida ayrim turlar uchun dasturchi xotirani bo‘shatish uchun yoki har safar  resurslar o‘sha tur instancedan ishlatib bo‘lmagungacha kodni chaqirishi kerak. Fayl handlelari, soketlar va locklar bunga misol bo‘la oladi. Agar ular kodni chaqirishni unitsalar, tizimda haddan tashqari yuklanish yuzaga keladi va tizim ishdan chiqadi. Rustda agar qiymat o‘z doirasidan chiqqanda siz kodning ma’lum bir qismi ishga tushirishni belgilashingiz mumkin, kompilyator avtomatik ravishda kodni kiritadi. Natijada, ma’lum bir turdagi instance tugagan dasturning hamma joyiga tozalovchi kodni joylashtirishdan xavotir olmasangiz ham bo‘ladi va siz resurslarni sizib ketishini oldini olgan bo‘lasiz!

Siz `Drop` traiti implementatsiyasi yordamida agar qiymat doirasidan chiqqan holda kodni run qilish uchun belgilashingiz mumkin. `Drop` traiti sizdan `self`dan referens oluvchi `drop` nomli metodni implementatsiya qilishni talab qiladi. Rustda `drop` qachon chaqirilishini ko‘rish uchun, `drop`ni `println!` yordamida implementatsiya qilib ko‘raylik.

15-14 ni ko‘rib chiqadigan bo‘lsak, Rustda qachon `drop` funksiyasi ishlashini ko‘rish uchun faqat o‘ziga tegishli bo‘lgan`CustomSmartPointer` structi faqat agar instance o‘z doirasidan chiqqanda `Dropping CustomSmartPointer!` ni print qiladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<span class="caption">15-14ni ko'rib chiqish: `CustomSmartPointer` structi biz tozalash uchun qo’ygan kodda `Drop` traitining implementatsiyasi</span>

`Drop` traiti o‘z ichiga preludeni oladi, shuning uchun biz uni scopeni ichiga olishimiz shart emas. Biz `CustomSmartPointer`da `Drop`ni implementatsiya qilamiz va `drop` metodi implementatisyasi uchun `println!`ni chaqiramiz. `drop` funksiyasining tana (body) qismi bu sizning turdagi instance o‘z doirasidan (scope) chiqib ketgandagi ayrim bir logikaga ega koddir. Rustda qachon `drop` chaqirilishini ko‘rish uchun biz ozgina tekstni print qilamiz.

`main`da biz 2ta `CustomSmartPointer` instancelarini yaratamiz va keyin `CustomSmartPointers yaratildi`ni print qilamiz. `main`ning oxirida `CustomSmartPointer` doiradan (scope) chiqib ketadi va Rust yakuniy xabarni print qilib, biz kodga qo‘ygan `drop` metodini chaqiradi. E’tibor bering biz `drop` metodini to‘g‘ridan-to‘g‘ri chaqririshimiz shart emas.

Agar biz dasturni run qilsak, quyidagi outputni ko‘ramiz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust avtomatik ravishda bizning o‘rnimizga biz ko‘rsatgan kodni instance doiradan (scope) chiqqanda `drop`ni chaqirdi. O‘zgaruvchilar yaratilish paytida teskari tartibda tushib qoldiriladi (drop qilinadi), shuning uchun `d` `c`dan oldin tushib qoldirildi (drop qilindi). Ushbu misolning maqsadi sizga `drop` metodining qanday ishlashining vizual ko‘rinishini berishdir; odatda xabarni print qilishning o‘rniga siz sizning turingizni ishga tushirish (run qilish) uchun tozalash kodini ko‘rsatasiz. 

### `std::mem::drop` yordamida Qiymatni Erta Drop qilish

Afsuski, avtomatik `drop` funksiyasini o‘chirish oson emas.  Odatda `drop`ni o‘chirish zarur emas; `Drop`ning asosiy mohiyati uning avtomatik ravishda hal qilishidir. Ba’zi paytlarda siz qiymatni erta tozalashga duch kelishingiz mumkin. Lockalarni boshqaruvchi smart pointerlarni ishlatishga bir misol bo‘la oladi:  bir doirada (scope)da boshqa kodni olish uchun siz lockni chaqiradigan `drop` metodini majburiy ravishda ishlatishingiz mumkin. Rust sizga `Drop` traitidagi `drop` metodini qo‘lda tushurishga qo‘ymaydi; agar siz qiymatni o‘z doirani (scope) tugashidan oldin majburiy drop bo‘lishini xohlasangiz. uning uchun siz standart kutubxona tomonidan taqdim etilgan `std::mem::drop`ni ishlatishingiz mumkin.

Agar biz 15-14dagi ilovaga qo‘lda `Drop` traitining `drop` metodi yordamida `main`ga o‘zgaritirish kiratigan bo‘lsak, 15-15 ilovada ko‘rsatilgan kompilyator xatosini ko‘ramiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<span class="caption"> 15-15 ro'yxat: `Drop` traitidagi `drop` metodi orqali qo'lda erta tozalashga harakat qilish</span>

Ushbu kodni komplilyatsiya qilganimizda quyidagi xatolikni ko‘ramiz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Ushbu xatolikdagi xabarda `drop`ni to‘g‘ridan-to‘g‘ri chaqira olmasligizni ko‘rsatadi. Xatolikdagi xabar instanceni tozalovchi umumiy dasturlash atamasi bo‘lgan funksiya, ya’ni `destructor`ni ishlatadi. `destructor` `constructor`ga o‘xshash bo‘lib, instancelarni yaratadi. Rustda `drop` funksiyasi alohida bir destructordir. 

Rust bizga `drop`ni to‘g‘ridan-to‘g‘ri chaqrishga qo‘ymaydi chunki Rust qiymatni avtomatik ravishda baribir `main`ni oxirida `drop`ni chaqiradi. Ushbu holat *double free* xatoligini keltirib chiqarishi mumkin chunki Rust bitta qiymatni ikki marta tozalashga xarakat qiladi.

Agar qiymat o‘z doirasidan (scope) chiqqanda biz `drop`ni avtomatik kiritishini o‘chirib qo‘ya olmaymiz va `drop` metodini to‘g‘ridan-to‘g‘ri chaqira olmaymiz. Shuning uchun agar bizga majburiy ravishda qiymat tozalanishini xoxlasak, biz `std::mem::drop`funksiyasini ishlatamiz

`std::mem::drop` funksiyasi `Drop` traitidagi `drop` metodidan farq qiladi. Biz buni majburan tozalash (drop) qilishni xohlagan qiymatni argument sifatida berish deb ataymiz. Funksiya preludeda, va 15-16 ro'yxatda ko‘rsatilgandek biz 15-15 ro'yxatdagi `main`da `drop` funkisyasini chaqirish uchun o‘zgartirish kiritishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<span class="caption"> 15-16 ro'yxat: qiymat o'z doirasidan (scope) chiqqanda to'g'ridan-to'g'ri `std::mem::drop`ni chaqirish </span>

Run qilingan kod quyidagini print qiladi:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

`c` nuqtasida tozalash (drop qilish) uchun `drop` metodi kodini chaqirishini ko‘rsatish uchun`CustomSmartPointerdagi ma’lumot bilan tozalash (drop qilish)` `CustomSmartPointer yaratildi` va `CustomSmartPointer main tugashidan oldin tozalandi (drop qilindi)` matnlari orasida print qilindi.

Siz `Drop` traiti implementatsiyasida ko‘rsatilgan koddan har xil turda tozalashni qulay va xavfsiz qilishingiz mumkin: masalan, siz uni o‘zingizning xotira taqsimlagichni yaratish uchun ishlatsangiz bo‘ladi. `Drop` traiti va Rustning ownership tizimi bilan tozalash uchun bosh qotirmasangiz ham bo‘ladi chunki Rust buni avtomatik ravishda qiladi.

Siz ishlatilib turgan qiymatlarni bexosdan tozalanib ketish muammolaridan xavotir olmasangiz bo‘ladi: ownership tizimi referencelarni doim to‘g‘riligiga hamda `drop` qiymat bir marta chaqrilib boshqa ishlatilmasligini ta’minlaydi.

Hozirda biz `Box<T>`ni va smart pointerlarni ba’zi bir xususiyatlarini tekshirib oldik, keling standart kutubxonada keltirilgan boshqa smart pointerlarni ham ko‘rib chiqaylik.