## Vectorlar bilan qiymatlar ro'yxatini saqlash

Biz ko'rib chiqadigan birinchi to'plam turi `Vec<T>` bo'lib, u *vector* sifatida ham tanilgan.
Vectorlar xotirada barcha qiymatlarni yonma-yon joylashtirgan yagona ma'lumotlar strukturasida bir nechta qiymatlarni saqlash imkonini beradi. Vectorlar faqat bir xil turdagi qiymatlarni saqlashi mumkin. Ular sizda fayldagi matn satrlari yoki xarid qilish savatidagi narsalarning narxlari kabi elementlar ro'yxatiga ega bo'lsangiz foydali bo'ladi.

### Yangi vector yaratish

Yangi bo'sh vector yaratish uchun biz 8-1 ro'yxatda ko'rsatilganidek, `Vec::new` funksiyasini chaqiramiz.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-1: `i32` turidagi qiymatlarni saqlash uchun yangi, boʻsh vector yaratish</span>

E'tibor bering, biz bu erda annation tur qo'shdik. Biz ushbu vectorga hech qanday qiymat kiritmayotganimiz sababli, Rust biz qanday elementlarni saqlashni xohlayotganimizni bilmaydi. Bu muhim nuqta. Vectorlar generiklar yordamida amalga oshiriladi; Biz 10-bobda o'zingizning turlaringiz bilan generiklardan qanday foydalanishni ko'rib chiqamiz. Hozircha shuni bilingki, standart kutubxona tomonidan taqdim etilgan `Vec<T>` turi har qanday turni sig'dira oladi.
Muayyan turni ushlab turish uchun vector yaratganimizda, burchakli qavslar([]) ichida turni belgilashimiz mumkin. 8-1 roʻyxatida biz Rustga `v`dagi `Vec<T>` `i32` turidagi elementlarni saqlashini aytdik.

Ko'pincha siz boshlang'ich qiymatlari bilan `Vec<T>` ni yaratasiz va Rust siz saqlamoqchi bo'lgan qiymat turini aniqlaydi, shuning uchun kamdan-kam hollarda bu turdagi annotionni bajarishingiz kerak bo'ladi. Rust qulay tarzda `vec!` makrosini taqdim etadi, bu esa siz bergan qiymatlarni saqlaydigan yangi vectorni yaratadi. 8-2 roʻyxati `1`, `2` va `3` qiymatlariga ega boʻlgan yangi `Vec<i32>`ni yaratadi. Butun son turi `i32` dir, chunki bu standart butun son turi, biz 3-bobning ["Ma'lumotlar turlari"][data-types]<!-- ignore --> bo'limida muhokama qilganimizdek.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-2: qiymatlarni o'z ichiga olgan yangi vector yaratish</span>

Biz boshlang‘ich `i32` qiymatlarini berganimiz sababli, Rust `v` turi `Vec<i32>` ekanligini va tur izohi shart emas degan xulosaga kelishi mumkin. Keyinchalik vectorni qanday o'zgartirishni ko'rib chiqamiz.

### Vectorni yangilash

Vector yaratish va unga elementlar qo'shish uchun biz 8-3 ro'yxatda ko'rsatilganidek, `push` metodidan foydalanishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-3: vectorga qiymatlar qo'shish uchun `push` metodidan foydalanish</span>

Har qanday o'zgaruvchida bo'lgani kabi, agar biz uning qiymatini o'zgartirish imkoniyatiga ega bo'lishni istasak, 3-bobda muhokama qilinganimizdek, `mut` kalit so'zidan foydalanib, uni o'zgaruvchan qilishimiz kerak. Biz joylashtirgan raqamlarning barchasi `i32` turiga kiradi va Rust buni maʼlumotlardan chiqaradi, shuning uchun bizga `Vec<i32>` annotationi kerak emas.

### Vector elementlarini o'qish

Vectorda saqlangan qiymatga murojaat qilishning ikki yo'li mavjud: indekslash yoki `get` metodi yordamida. Quyidagi misollarda biz qo'shimcha aniqlik uchun ushbu funksiyalardan qaytariladigan qiymatlar turlarini izohladik.

8-4 ro'yxatda indekslash sintaksisi va `get` metodi bilan vectordagi qiymatga kirishning ikkala usuli ko'rsatilgan.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-4: Vectordagi elementga kirish uchun indekslash sintaksisi yoki `get` metodidan foydalanish</span>

Bu erda bir nechta detallarga e'tibor bering. Uchinchi elementni olish uchun `2` indeks qiymatidan foydalanamiz, chunki vectorlar noldan boshlab raqamlar boʻyicha indekslanadi. `&` va `[]` dan foydalanish bizga indeks qiymatidagi elementga reference beradi. Argument sifatida berilgan indeks bilan `get` metodidan  foydalansak, biz `match` bilan foydalanishimiz mumkin bo'lgan `Option<&T>`ni olamiz.

Rust elementga reference qilishning ushbu ikki usulini taqdim etishining sababi shundaki, siz mavjud elementlar doirasidan tashqarida indeks qiymatidan foydalanmoqchi bo'lganingizda dastur qanday harakat qilishini tanlashingiz mumkin. Misol sifatida, keling, besh elementli vectorga ega bo'lganimizda nima sodir bo'lishini ko'rib chiqamiz va keyin 8-5 ro'yxatda ko'rsatilganidek, har bir texnikada 100 indeksidagi elementga kirishga harakat qilamiz.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-5: besh elementni o'z ichiga olgan vectorda 100 indeksidagi elementga kirishga urinish</span>

Ushbu kodni ishga tushirganimizda, birinchi `[]` metodi dasturda panic chiqaradi, chunki u mavjud bo'lmagan elementga murojaat qiladi. Ushbu usul vector oxiridan o'tgan elementga kirishga urinish bo'lsa, dasturingiz ishdan chiqishini xohlasangiz yaxshi qo'llaniladi.

`get` metodi vectordan tashqaridagi indeksdan o'tganda, panic qo'ymasdan  `None`ni qaytaradi. Vector doirasidan tashqaridagi elementga kirish vaqti-vaqti bilan oddiy sharoitlarda sodir bo'lishi mumkin bo'lsa, siz ushbu usuldan foydalanasiz. Keyin sizning kodingiz 6-bobda muhokama qilinganidek, `Some(&element)`  yoki `None`ga ega bo'lish mantiqiga ega bo'ladi.Misol uchun, indeks raqamni kiritgan odamdan kelib chiqishi mumkin. Agar ular tasodifan juda katta raqamni kiritsa va dastur  `None` qiymatiga ega bo'lsa, siz foydalanuvchiga joriy vectorda nechta element borligini aytishingiz va ularga to'g'ri qiymat kiritish uchun yana bir imkoniyat berishingiz mumkin.Bu imlo xatosi tufayli dasturni buzishdan ko'ra foydalanuvchilar uchun qulayroq bo'lar edi!

Dasturda tegishli reference mavjud bo'lsa, borrow tekshiruvi ushbu reference va vector mazmuniga boshqa har qanday referencelar haqiqiyligini ta'minlash uchun ownership va borrowing qoidalarini (4-bobda ko'rsatilgan) amalga oshiradi. Bir xil doirada o'zgaruvchan va o'zgarmas referencelarga ega bo'lolmaysiz degan qoidani eslang. Ushbu qoida 8-6 ro'yxatda qo'llaniladi, bu yerda biz vectordagi birinchi elementga o'zgarmas referenceni ushlab turamiz va elementni oxiriga qo'shishga harakat qilamiz. Agar biz ushbu elementga keyinroq funksiyada murojaat qilsak, bu dastur ishlamaydi:


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-6. Vector elementiga reference mavjud bo'lganda vectorga biron bir element qo'shishga urinish</span>

Ushbu kodni kompilyatsiya qilish ushbu xatoga olib keladi:


```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

8-6 ro'yxatdagi kod ishlashi kerakdek ko'rinishi mumkin: nima uchun birinchi elementga reference vector oxiridagi o'zgarishlar haqida qayg'urishi kerak? Bu xato vectorlarning ishlash usuli bilan bog'liq: vectorlar qiymatlarni xotirada bir-birining yoniga qo'yganligi sababli vector oxiriga yangi element qo'shish yangi xotira ajratishni va eski elementlarni yangi bo'sh joyga ko'chirishni talab qilishi mumkin. Hozirda vector saqlanadigan barcha elementlarni bir-birining yoniga qo'yish uchun joy etarli emas. Bunday holda, birinchi elementga reference ajratilgan xotiraga ishora qiladi. Borrowing qoidalari dasturlarning bunday vaziyatga tushishiga yo'l qo'ymaydi.

> Eslatma: `Vec<T>` turini implement qilish haqida ko'proq ma'lumot olish uchun
> ["Rustonomikon"][nomicon] ga qarang.

### Vectordagi qiymatlarni takrorlash

Vectordagi har bir elementga navbatma-navbat kirish uchun biz indekslarni birma-bir kirish uchun ishlatmasdan, barcha elementlarni takrorlaymiz. 8-7 ro'yxatda `i32` qiymatlari vectoridagi har bir elementga o'zgarmas referencelarni olish va ularni chop etish uchun `for` siklidan qanday foydalanish ko'rsatilgan.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-7: `for` sikli yordamida elementlarni takrorlash orqali vectordagi har bir elementni chop etish</span>

Shuningdek, biz barcha elementlarga o'zgartirish kiritish uchun o'zgaruvchan vectordagi har bir elementga o'zgaruvchan referencelarni takrorlashimiz mumkin. 8-8 ro'yxatdagi `for` sikli har bir elementga `50` qo'shadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-8: Vectordagi elementlarga o'zgaruvchan referencelarni takrorlash</span>

O'zgaruvchan  reference nazarda tutilgan qiymatni o'zgartirish uchun biz `+=` operatoridan foydalanishimizdan oldin `i` qiymatiga o'tish uchun `*` dereference operatoridan foydalanishimiz kerak. Biz 15-bobning ["Dereference operatori bilan ko'rsatgichni qiymatga kuzatib borish"][deref]<!-- ignore --> bo'limida dereference operatori haqida ko'proq gaplashamiz.

O'zgarmas yoki o'zgaruvchan bo'lsin, vector bo'yicha takrorlash, borrow tekshiruvi qoidalari tufayli xavfsizdir. Agar biz 8-7 va 8-8 ro'yxatlardagi `for` siklining tanasiga elementlarni qo'shishga yoki olib tashlashga harakat qilsak, biz 8-6 ro'yxatdagi kodga o'xshash kompilyator xatosiga duch kelamiz. `for` siklidagi vectorga murojaat qilish butun vectorni bir vaqtning o'zida o'zgartirishni oldini oladi.

### Bir nechta turlarni saqlash uchun enumdan foydalanish

Vectorlar faqat bir xil turdagi qiymatlarni saqlashi mumkin. Bu noqulay bo'lishi mumkin; Har xil turdagi elementlar ro'yxatini saqlash zarurati uchun, albatta, foydalanish holatlari mavjud. Yaxshiyamki, enumlashning variantlari bir xil enum turi ostida aniqlanadi, shuning uchun bizga har xil turdagi elementlarni ko'rsatish uchun bitta tur kerak bo'lganda, enumni aniqlashimiz va ishlatishimiz mumkin!

Misol uchun, biz elektron jadvalning bir qator ustunlarida integer sonlar, ba'zi float raqamlar va ba'zi stringlar mavjud bo'lgan satrdan qiymatlarni olishni xohlaymiz. Variantlari turli qiymat turlariga ega bo'lgan enumni aniqlashimiz mumkin va barcha enum variantlari bir xil turdagi hisoblanadi: enum. Keyin biz ushbu enumni ushlab turish uchun vectorni yaratishimiz mumkin va natijada har xil turlarni ushlab turadi. Biz buni 8-9 ro'yxatda ko'rsatdik.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-9: Har xil turdagi qiymatlarni bitta vectorda saqlash uchun `enum`ni aniqlash</span>

Rust kompilyatsiya vaqtida vectorda qanday turlar bo'lishini bilishi kerak, shuning uchun u har bir elementni saqlash uchun heapda qancha xotira kerakligini aniq biladi. Shuningdek, ushbu vectorda qanday turlarga ruxsat berilganligini aniq bilishimiz kerak. Agar Rust vectorga har qanday turni ushlab turishga ruxsat bergan bo'lsa, bir yoki bir nechta tur vector elementlari ustida bajarilgan operatsiyalarda xatoliklarni keltirib chiqarishi mumkin edi. Enum va `match` ifodasidan foydalanish Rust kompilyatsiya vaqtida 6-bobda muhokama qilinganidek, barcha mumkin bo'lgan holatlar ko'rib chiqilishini ta'minlaydi.

Agar siz vectorda saqlash uchun dastur runtimeda oladigan turlarning to'liq to'plamini bilmasangiz, enum texnikasi ishlamaydi. Buning o'rniga, biz 17-bobda ko'rib chiqiladigan trait obyektidan foydalanishingiz mumkin.


Endi biz vectorlardan foydalanishning eng keng tarqalgan usullarini ko'rib chiqdik, standart kutubxona tomonidan `Vec<T>` da belgilangan barcha foydali usullar uchun [API texnik hujjatlarini][vec-api]<!-- ignore --> ko'rib chiqishni unutmang. Masalan, `push` dan tashqari, `pop` usuli oxirgi elementni olib tashlaydi va qaytaradi.

### Vectordan elementlarni olib tashlash

`struct`lar singari, vector ham 8-10 ro'yxatda ko'rsatilganidek, amal qilish doirasidan tashqariga chiqqanda xotirasini bo'shatadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-10. Vector va uning elementlarini qanday o'chirishni ko'rsatadi</span>

Vector o'chirilganda, uning barcha tarkibi ham o'chiriladi: vectorni o'chirish uning tarkibidagi qiymatlarni o'chirishni anglatadi. Borrow tekshiruvi vector mazmuniga har qanday referencelar faqat vectorning o'zi haqiqiy bo'lganda ishlatilishini ta'minlaydi.

Keling, keyingi to'plam turiga o'tamiz: `String`!

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
