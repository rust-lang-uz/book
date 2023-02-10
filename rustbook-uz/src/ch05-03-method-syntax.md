## Metod Sintaksisi

*Metodlar* funksiyalarga oʻxshaydi: biz ularni `fn` kalit soʻzi va nomi bilan eʼlon qilamiz, ular parametrlari va qaytish qiymatiga ega boʻlishi mumkin va ular boshqa joydan metod chaqirilganda ishga tushadigan kodni oʻz ichiga oladi. Funktsiyalardan farqli o'laroq, metodlar struct (yoki biz mos ravishda [6-bob][enums]<!-- ignore --> va [17-bobda][trait-objects]<!-- ignore --> ko'rib chiqiladigan enum yoki trait obyekti) kontekstida aniqlanadi va ularning birinchi parametri har doim `self` dir metod chaqirilayotgan structning namunasini ifodalaydi.

### Metodlarni aniqlash

Parametr sifatida `Kvadrat` misoliga ega bo‘lgan `area` funksiyasini o‘zgartiramiz va uning o‘rniga 5-13 ro‘yxatda ko'rsatilganidek, `Kvadrat` structida belgilangan `area` metodini yaratamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Ro'yxat 5-13: `Kvadrat` structida `area` metodini aniqlash</span>

`Kvadrat` kontekstida funksiyani aniqlash uchun `Kvadrat` uchun `impl` (implementation) blokini ishga tushiramiz. Ushbu `impl` blokidagi hamma narsa `Kvadrat` turi bilan bog'lanadi. Keyin biz  `area` funksiyasini `impl` jingalak qavslar ichida harakatlantiramiz va birinchi (va bu holda, faqat) parametrni signatureda va tananing hamma joyida `self` o‘zgartiramiz. `main` da, biz `area` funksiyasini chaqirib, argument sifatida `kvadrat1` ni topshirgan bo‘lsak, o‘rniga `Kvadrat` misolida `area` metodini chaqirish uchun *metod sintaksisi* dan foydalanishimiz mumkin. Metod sintaksisi misoldan keyin keladi: biz nuqta qo'shamiz, undan keyin metod nomi, qavslar va har qanday argumentlar qo'shiladi.

`area` uchun signatureda `kvadrat: &Kvadrat` o‘rniga `&self` dan foydalanamiz. `&self` aslida  `self: &Self` ning qisqartmasi. `impl` blokida `Self` turi `impl` bloki uchun bo'lgan turdagi taxallusdir. Metodlar birinchi parametri uchun `Self` turidagi `self` deb nomlangan parametrga ega bo'lishi kerak, shuning uchun Rust birinchi parametr joyida faqat `self` nomi bilan qisqartirish imkonini beradi.
Esda tutingki, biz hali ham `kvadrat: &Kvadrat` da qilganimizdek, bu metod `Self` misolini olishini koʻrsatish uchun `Self` stenografiyasi oldida `&` dan foydalanishimiz kerak. Boshqa har qanday parametr singari, metodlar `self` egallashi, o'zgarmas `self` borrow qilishi mumkin, xuddi biz bu yerda qilganimizdek yoki o'zgaruvchan `self`ni borrow qilishi mumkin.

Biz bu yerda funksiya versiyasida `&Kvadrat` dan foydalanganimiz uchun xuddi shu sababga ko‘ra `&self` tanladik: biz ownershiplik qilishni istamaymiz va faqat structdagi ma’lumotlarni o‘qishni istaymiz, unga yozishni emas. Agar biz ushbu metodning bir qismi sifatida chaqirgan misolni o'zgartirmoqchi bo'lsak, birinchi parametr sifatida `&mut self` dan foydalanamiz. Birinchi parametr sifatida faqat `self`ni ishlatib, misolga ownershiplik qiladigan metod kamdan-kam uchraydi; bu metod odatda `self`ni boshqa narsaga aylantirganda va siz murojat qiluvchiga transformatsiyadan keyin asl nusxadan foydalanishiga yo'l qo'ymaslikni istasangiz ishlatiladi.

Funktsiyalar o'rniga metodlardan foydalanishning asosiy sababi, har bir metod signaturesida `self`turini takrorlashning hojati bo'lmagan metod sintaksisidan tashqari, kodni tashkil qilishdir. Biz kelajakdagi kod foydalanuvchilarini biz taqdim etayotgan kutubxonaning turli joylarida `Kvadrat` imkoniyatlarini izlashga majburlashdan ko‘ra, biz tur namunasi bilan qila oladigan barcha narsalarni bitta `impl` blokiga joylashtirdik.

E'tibor bering, biz metodga structning maydonlaridan biri bilan bir xil nom berishni tanlashimiz mumkin. Misol uchun, biz `Kvadrat` da `kenglik` deb nomlangan metodni belgilashimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Bu yerda, agar misolning `kenglik` maydonidagi qiymat `0` dan katta bo‘lsa, `kenglik` metodi `true` qiymatini qaytaradi, agar qiymat `0` bo'lsa, `false` bo‘lishini tanlaymiz: biz bir xil nomdagi metod ichidagi maydonni istalgan maqsadda ishlatishimiz mumkin. `main` da, biz qavslar bilan `kvadrat1.kenglik` ga amal qilsak, Rust `kenglik` metodini nazarda tutayotganimizni biladi. Qavslardan foydalanmasak, Rust `kenglik` maydonini nazarda tutayotganimizni biladi.

Ko'pincha, lekin har doim emas, biz metodga maydon bilan bir xil nom berganimizda, biz u faqat maydondagi qiymatni qaytarishini va boshqa hech narsa qilmasligini xohlaymiz. Shunga o'xshash metodlar *getters* deb ataladi va Rust ularni boshqa tillarda bo'lgani kabi tizim maydonlari uchun avtomatik ravishda amalga oshirmaydi. Getterslar foydalidir, chunki siz maydonni shaxsiy, lekin metodni hammaga ochiq qilib qo'yishingiz mumkin va shu tariqa ushbu maydonga umumiy API ning bir qismi sifatida faqat o'qish uchun ruxsatni yoqishingiz mumkin. Biz [7-bobda][public]<!-- ignore --> public va private nima ekanligini va qanday qilib maydon yoki metodni public yoki private deb belgilashni muhokama qilamiz.

> ### `->` operatori qayerda ishlatiladi?
>
> C va C++ tillarida metodlarni chaqirish uchun ikki xil operator qo'llaniladi:
> obyektdagi metodni to'g'ridan-to'g'ri chaqirayotgan bo'lsangiz `.` va agar
> siz ko'rsatgichdagi metodni obyektga chaqirayotgan bo'lsangiz va avval
> ko'rsatgichni yo'qotishingiz kerak bo'lsa `->` dan foydalanasiz. Boshqacha qilib aytganda,
> agar `object` havola bo'lsa, u holda `object->something()` va `(*object).something()` metodi
> chaqiruvlari bir xil bo'ladi.
>
> Rust `->` operatoriga ekvivalentga ega emas;  Buning o'rniga Rustda
> *avtomatik reference va dereferencing* deb nomlangan xususiyat mavjud. Metodni chaqirish
> Rustda bunday xatti-harakatlarga ega bo'lgan kam sonli joylardan biridir.
>
> Bu shunday ishlaydi: `object.something()` bilan metodni chaqirganingizda,
> Rust avtomatik ravishda `&`, `&mut` yoki `*` ni qo'shadi, shuning uchun `object`
> metod signaturega mos keladi. Boshqacha qilib aytganda, quyidagilar bir xil:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn masofa(&self, other: &Point) -> f64 {
> #        let x_kvadrat = f64::powi(other.x - self.x, 2);
> #        let y_kvadrat = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_kvadrat + y_kvadrat)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.masofa(&p2);
> (&p1).masofa(&p2);
> ```
>
> Birinchisi ancha toza ko'rinadi. Ushbu avtomatik reference qilish harakati,
> metodlar aniq qabul qiluvchiga ega bo'lganligi sababli ishlaydi - `self` turi. Qabul qiluvchi
> va metod nomini hisobga olgan holda, Rust ma'lum bir holatda kod nima qilayotganini aniq aniqlashi mumkin:
> o'qish `(&self)`, o'zgartirish (`&mut self`) yoki iste'mol qilish  (`self`). Rust metodi
> qabul qiluvchilar uchun borrow qilishni yashirin qilib qo'yganligi amalda ownershipni
> ergonomik qilishning katta qismidir.

### Ko'proq parametrlarga ega metodlar

`Kvadrat` structida ikkinchi metodni implement qilish orqali metodlardan foydalanishni mashq qilaylik. Bu safar biz `Kvadrat` misoli `Kvadrat` ning boshqa nusxasini olishini va agar ikkinchi `Kvadrat` to'liq o'ziga (birinchi `Kvadrat`) sig'ishi mumkin bo'lsa, `true` qiymatini qaytarishini istaymiz; aks holda u `false`ni qaytarishi kerak.
Ya'ni, `ushlab_tur` metodini aniqlaganimizdan so'ng, biz 5-14 ro'yxatda ko'rsatilgan dasturni yozish imkoniyatiga ega bo'lishni xohlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Ro'yxat 5-14: Hali yozilmagan `ushlab_tur` dan foydalanish
metodi</span>

Kutilgan natija quyidagicha ko‘rinadi, chunki `kvadrat2` ning ikkala o‘lchami `kvadrat1` o‘lchamidan kichikroq, lekin `kvadrat3` `kvadrat1` dan kengroq:

```text
kvadrat1 kvadrat2ni ushlab turadimi? true
kvadrat1 kvadrat3ni ushlab turadimi? false
```

Biz metodni aniqlamoqchi ekanligimizni bilamiz, shuning uchun u `impl Kvadrat` blokida bo'ladi. Metod nomi `ushlab_tur` bo'ladi va u parametr sifatida boshqa `Kvadrat` ning o'zgarmas borrowini oladi. Parametrning turi qanday bo'lishini metodni chaqiruvchi kodga qarab aniqlashimiz mumkin: `kvadrat1.ushlab_tur(&kvadrat2)` `&kvadrat2` da o'tadi, bu `kvadrat2` ga o'zgarmas borrow, `Kvadrat` misoli. Bu mantiqqa to'g'ri keladi, chunki biz faqat `kvadrat2` ni o'qishimiz kerak (yozishdan ko'ra, bu bizga o'zgaruvchan borrow kerak degan ma'noni anglatadi), va biz `main` `kvadrat2` ownershipligini saqlab qolishini istaymiz, shuning uchun `ushlab_tur` metodini chaqirganimizdan keyin uni qayta ishlatishimiz mumkin. `ushlab_tur` ning return qiymati mantiqiy qiymat bo'ladi va implement `self` ning kengligi va balandligi mos ravishda boshqa `Kvadrat` ning kengligi va balandligidan katta ekanligini tekshiradi. Keling, 5-15 ro'yxatda ko'rsatilgan 5-13 ro'yxatdagi `impl` blokiga yangi `ushlab_tur` metodini qo'shamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-15: Parametr sifatida boshqa `Kvadrat` misolini oladigan `ushlab_tur` metodini `Kvadrat`da qo'llash</span>

Ushbu kodni 5-14 ro'yxatdagi `main` funksiya bilan ishga tushirganimizda, biz kerakli natijani olamiz. Metodlar biz signaturega `self` parametridan keyin qo'shadigan bir nechta parametrlarni olishi mumkin va bu parametrlar funksiyalardagi parametrlar kabi ishlaydi.

### Associate Funksiyalar

Associate Funksiyalar (Bog'langan Funktsiyalar).`impl` blokida aniqlangan barcha funksiyalar *associated funksiyalar* deb ataladi, chunki ular `impl` nomi bilan atalgan tur bilan bog‘langan. Biz birinchi parametr sifatida `self` ega bo'lmagan associated funksiyalarni belgilashimiz mumkin (va shuning uchun metodlar emas), chunki ular bilan ishlash uchun turdagi namuna kerak emas.
Biz allaqachon shunday funksiyadan foydalanganmiz: `String` turida aniqlangan `String::from` funksiyasi.

Metod bo'lmagan associated funktsiyalar ko'pincha structning yangi nusxasini qaytaradigan konstruktorlar uchun ishlatiladi. Ular ko'pincha `new` deb ataladi, ammo `new` maxsus nom emas va tilga kiritilmagan. Masalan, biz bir o‘lchamli parametrga ega bo‘lgan `kvadrat` nomli associated funksiyani taqdim etishimiz va undan kenglik va balandlik sifatida foydalanishimiz mumkin, bu esa bir xil qiymatni ikki marta belgilashdan ko‘ra `Kvadrat` kvadratini yaratishni osonlashtiradi. :

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Return turidagi va funksiya tanasidagi `Self` kalit so'zlari `impl` kalit so'zidan keyin paydo bo'ladigan turning taxalluslari bo'lib, bu holda `Kvadrat` bo'ladi.We’ll discuss modules in [Chapter 7][modules]<!-- ignore -->.

Ushbu associated funktsiyani chaqirish uchun biz struct nomi bilan `::` sintaksisidan foydalanamiz; `let kv = Kvadrat::kvadrat(3);` misol bo'la oladi. Bu funksiya struct tomonidan nom maydoniga ega: `::` sintaksisi ham associated funksiyalar, ham modullar tomonidan yaratilgan nomlar bo'shliqlari uchun ishlatiladi. Biz modullarni [7-bobda][modules]<!-- ignore --> muhokama qilamiz.

### Bir nechta `impl` bloklari

Har bir structga bir nechta `impl` bloklari ruxsat etiladi. Masalan, 5-15 ro'yxati 5-16 ro'yxatida ko'rsatilgan kodga ekvivalent bo'lib, har bir metod o'zining `impl` blokiga ega yani har bir metod o'z `impl` blokida.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Ro'yxat 5-16: Bir nechta `impl` bloklari yordamida 5-15 ro'yxatini qayta yozish</span>

Bu metodlarni bir nechta `impl` bloklariga ajratish uchun hech qanday sabab yo'q, lekin bu to'g'ri sintaksis. Biz 10-bobda bir nechta `impl` bloklari foydali bo'lgan holatni ko'rib chiqamiz, bu yerda biz umumiy turlar va taritlarni muhokama qilamiz.

## Xulosa

Structlar sizning domeningiz uchun mazmunli bo'lgan maxsus turlarni yaratishga imkon beradi. Structlardan foydalanib, siz bog'langan ma'lumotlar qismlarini bir-biriga bog'lab qo'yishingiz va kodingizni aniq qilish uchun har bir qismga nom berishingiz mumkin. `impl` bloklarida siz o'zingizning turingiz bilan bog'liq bo'lgan funksiyalarni belgilashingiz mumkin va metodlar - bu sizning structlaringiz misollarining xatti-harakatlarini belgilashga imkon beruvchi associated funksiyaning bir turi.

Ammo structlar maxsus turlarni yaratishning yagona usuli emas: toolboxga boshqa toolni qo'shish uchun Rust enum xususiyatiga murojaat qilaylik.

[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
