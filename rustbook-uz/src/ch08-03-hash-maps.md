## Hash Maplarda bog'langan qiymatlarga ega kalitlarni saqlash

Umumiy to'plamlarimizning oxirgisi *hash map*. `HashMap<K, V>` turi `K` turidagi kalitlarning `V` turidagi qiymatlarga xaritasini *xeshlash funksiyasi* yordamida saqlaydi, bu kalit va qiymatlarni xotiraga qanday joylashtirishini belgilaydi. Ko'pgina dasturlash tillari bunday ma'lumotlar strukturasini qo'llab-quvvatlaydi, lekin ular ko'pincha bir nechtasini nomlash uchun hash, map, object, hash table, dictionary, yoki associative array kabi boshqa nomlardan foydalanadilar.

Hash maplar ma'lumotlarni vectorlar bilan bo'lgani kabi indeks yordamida emas, balki har qanday turdagi kalit yordamida qidirmoqchi bo'lsangiz foydali bo'ladi. Misol uchun, o'yinda siz har bir jamoaning balini hesh-mapda kuzatib borishingiz mumkin, unda har bir kalit jamoaning nomi va qiymatlar har bir jamoaning ballidir. Jamoa nomi berilgan bo'lsa, siz uning ballini olishingiz mumkin.

Ushbu bo'limda biz hesh-mapllarining asosiy API-ni ko'rib chiqamiz, ammo yana ko'plab foydali funksiyalar standart kutubxona tomonidan `HashMap<K, V>` da belgilangan funksiyalarda yashiringan.
Har doimgidek, qo'shimcha ma'lumot olish uchun standart kutubxona texnik hujjatlarini tekshiring.

### Yangi Hash Map yaratish

Bo'sh hesh mapni yaratishning bir usuli - `new` dan foydalanish va `insert` bilan elementlarni qo'shish. 8-20 ro'yxatda biz nomlari *Yashil* va *Sariq* bo'lgan ikkita jamoaning ballarini kuzatib boramiz. Yashil jamoa 10 ball bilan, Sariq jamoa esa 50 ball bilan boshlanadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-20: Yangi hesh mapni yaratish va ba'zi kalitlar va qiymatlarni kiritish</span>

E'tibor bering, biz birinchi navbatda standart kutubxonaning to'plamlar qismidagi `HashMap` dan foydalanishimiz kerak. Bu quyidagicha bo'ladi `use std::collections::HashMap;`. Bizning uchta keng tarqalgan to'plamlarimiz orasida bu eng kam qo'llaniladi, shuning uchun u muqaddimada avtomatik ravishda kiritilgan funtsiyalarga kiritilmagan. Hash Maplar standart kutubxonadan ham kamroq qo'llab-quvvatlanadi; masalan, ularni yaratish uchun o'rnatilgan makros mavjud emas.

Vectorlar singari, hash maplar ham o'z ma'lumotlarini heapda saqlaydi. Ushbu `HashMap`da `String` turidagi kalitlar va `i32` turidagi qiymatlar mavjud. Vectorlar singari, hash maplar ham bir xildir: barcha kalitlar bir-biri bilan bir xil turdagi va barcha qiymatlar bir xil turga ega bo'lishi kerak.

### HashMap-dagi ma'lumotlarga kirish

Biz 8-21 ro'yxatda ko'rsatilganidek, `get` metodiga kalitni taqdim etish orqali hash mapdan qiymat olishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-21: Hesh-Mapda saqlangan Yashil jamoa hisobiga kirish</span>

Bu yerda `ball` Yashil jamoa bilan bog'liq qiymatga ega bo'ladi va natija `10` bo'ladi. `get` metodi `Option<&V>`ni qaytaradi; agar hesh-mapda ushbu kalit uchun qiymat bo'lmasa, `get` `None` ni qaytaradi. Bu dastur `Option<&i32>` emas, `Option<i32>` olish uchun `copied` ga murojaat qilib `Option`ni boshqaradi, so'ngra `unwrap_or` `ballar` da ushbu kalit uchun ma'lumotlar bo'lmasa, ballni nolga o'rnatish uchun.

Biz hesh-mapdagi har bir kalit/qiymat juftligini vectorlar bilan bo'lgani kabi, `for` siklidan foydalangan holda takrorlashimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Ushbu kod har bir juftlikni tasodifiy tartibda chop etadi:

```text
Sariq: 50
Yashil: 10
```

### Hash Maplar va Ownership(Egalik)

`Copy` traitini amalga oshiradigan turlar uchun, masalan, `i32`, qiymatlar hesh-mapiga ko'chiriladi. `String` kabi tegishli qiymatlar uchun qiymatlar boshqa joyga koʻchiriladi va 8-22 roʻyxatda koʻrsatilganidek, hesh-map ushbu qiymatlarning egasi boʻladi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-22: kalitlar va qiymatlar kiritilgandan so'ng ular hesh-mapda tegishli ekanligini ko'rsatish</span>

`maydon_nomi` va `maydon_qiymati` o'zgaruvchilari qiymatlari `insert` metodini chaqirish orqali HashMap-ga ko'chirilgandan keyin foydalana olmaymiz.

Agar biz HashMap-ga qiymatlarga referencelar kiritsak, ular HashMap-ga ko'chirilmaydi. Murojaatlar ko'rsatadigan qiymatlar hech bo'lmaganda hesh-mapda amal qiladigan vaqt davomida amal qilishi kerak. Biz ushbu muammolar haqida 10-bobning ["Lifetime bilan referencelarni tekshirish"][validating-references-with-lifetimes]<!-- ignore --> bo'limida ko'proq gaplashamiz.

### Hash Mapni yangilash

Kalit va qiymat juftlarining soni o'sishi mumkin bo'lsa-da, har bir noyob kalit bir vaqtning o'zida u bilan bog'langan faqat bitta qiymatga ega bo'lishi mumkin (lekin aksincha emas: masalan, Yashil jamoa ham, Sariq jamoa ham `ballar` hash-mapida saqlangan 10 qiymatiga ega bo'lishi mumkin).

Hash-mapdagi ma'lumotlarni o'zgartirmoqchi bo'lganingizda, kalit allaqachon tayinlangan qiymatga ega bo'lgan holatda qanday ishlashni hal qilishingiz kerak. Eski qiymatni butunlay e'tiborsiz qoldirib, eski qiymatni yangi qiymat bilan almashtirishingiz mumkin. Eski qiymatni saqlab qolishingiz va yangi qiymatni e'tiborsiz qoldirishingiz mumkin, faqat kalitda *qiymat bo'lmasa*, yangi qiymat qo'shishingiz mumkin. Yoki eski qiymat va yangi qiymatni birlashtira olasiz. Keling, bularning har birini qanday qilishni ko'rib chiqaylik!

#### Qiymatni qayta yozish

Agar biz kalit va qiymatni hash-mapga kiritsak va keyin boshqa qiymat bilan bir xil kalitni kiritsak, bu kalit bilan bog'langan qiymat almashtiriladi. Ro'yxat 8-23dagi kod ikki marta `insert` ni chaqirsa ham, hash-mapda faqat bitta kalit/qiymat juftligi bo'ladi, chunki biz har ikki marta Yashil jamoa kaliti qiymatini kiritamiz.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-23: Saqlangan qiymatni ma'lum bir kalit bilan almashtirish</span>

Bu kod `{"Yashil": 25}`ni chop etadi. `10`ning asl qiymati ustiga yozildi.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Kalit va qiymatni faqat kalit mavjud bo'lmaganda qo'shish

Hash Mapda ma'lum bir kalit allaqachon qiymatga ega yoki yo'qligini tekshirish odatiy holdir, keyin quyidagi amallarni bajaring: agar kalit hash-mapda mavjud bo'lsa, mavjud qiymat avvalgidek qolishi kerak. Agar kalit mavjud bo'lmasa, insert va uning qiymatini kiriting.

Hash Mapda buning uchun `entry` deb nomlangan maxsus API mavjud bo'lib, siz tekshirmoqchi bo'lgan kalitni parametr sifatida qabul qiladi. `entry` metodining qaytish qiymati `Entry` nomli enum bo‘lib, u mavjud yoki bo‘lmasligi mumkin bo‘lgan qiymatni ifodalaydi. Aytaylik, biz Sariq jamoa uchun kalitning u bilan bog'liq qiymati bor-yo'qligini tekshirmoqchimiz. Agar shunday bo'lmasa, biz 50 qiymatini qo'shishni xohlaymiz va Yashil jamoa uchun ham xuddi shunday. `Entry` API-dan foydalanib, kod Ro'yxat 8-24 kabi ko'rinadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-24: `entry` metodidan faqat kalitda qiymat mavjud bo'lmasa, kiritish uchun foydalanish</span>

`Entry` da `or_insert` metodi mos keladigan `Entry` kaliti qiymatiga o'zgaruvchan referenceni qaytarish uchun belgilangan, agar bu kalit mavjud bo'lsa,  parametrni ushbu kalit uchun yangi qiymat sifatida kiritadi va yangi qiymatga o'zgaruvchan referenceni qaytaradi. Ushbu metod mantiqni o'zingiz yozishdan ko'ra ancha toza va u xavfsizroq va borrowing qoidalariga mos keladi.

8-24-raqamdagi kodni ishga tushirish `{"Sariq": 50, "Yashil": 10}` chop etiladi. `entry` ga birinchi chaqiruv 50 qiymatiga ega Sariq jamoa uchun kalitni kiritadi, chunki sariq jamoada allaqachon qiymat yo'q.  `entry` ga ikkinchi chaqiruv hash-mapni o'zgartirmaydi, chunki Yashil jamoa allaqachon 10 qiymatiga ega.

#### Eski qiymat asosida yangi qiymatni yangilash

Hash-Maplar uchun yana bir keng tarqalgan foydalanish holati kalit qiymatini izlash va keyin uni eski qiymat asosida yangilashdir. Misol uchun, 8-25 ro'yxatda har bir so'z ba'zi matnda necha marta takrorlanganini hisoblaydigan kodni ko'rsatadi. Biz kalit sifatida so'zlar bilan hash-mapdan foydalanamiz va bu so'zni necha marta ko'rganimizni kuzatib borish uchun qiymatni oshiramiz. Agar so'zni birinchi marta ko'rayotgan bo'lsak, avval 0 qiymatini kiritamiz.

```rustword
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-25: So'zlar va hisoblarni saqlaydigan hash-mapi yordamida so'zlarning necha marta takrorlanganini hisoblash</span>

Bu kod `{"qiziqarli": 1, "salom": 1, "rust": 2}`ni chop etadi. Siz boshqa tartibda chop etilgan bir xil kalit/qiymat juftliklarini ko'rishingiz mumkin: [“Hash-Mapdagi qiymatlarga kirish”][access]<!-- ignore --> bo'limidan hash-mapni takrorlash ixtiyoriy tartibda sodir bo'lishini eslang.

`split_whitespace` metodi `matn` qiymatining bo'sh joy bilan ajratilgan pastki bo'limlari ustidan iteratorni qaytaradi.`or_insert` metodi belgilangan kalit qiymatiga o'zgaruvchan havolani (`&mut V`) qaytaradi. Bu yerda biz o'zgaruvchan referenceni `hisoblash` o'zgaruvchisida saqlaymiz, shuning uchun bu qiymatni belgilash uchun avval yulduzcha (`*`) yordamida `hisoblash` ga murojaat qilishimiz kerak. O'zgaruvchan reference `for` siklining oxirida ko'lamdan chiqib ketadi, shuning uchun bu o'zgarishlarning barchasi xavfsiz va borrowing qoidalari bilan ruxsat etiladi.

### Hashing funksiyalari

Odatda, `HashMap` *SipHash* nomli hashlash funksiyasidan foydalanadi, u [^siphash]<!-- ignore --> hash-jadvallari ishtirokidagi Xizmatni rad etish (DoS) hujumlariga qarshilik ko'rsatishi mumkin. Bu mavjud bo'lgan eng tezkor hashlash algoritmi emas, lekin ishlashning pasayishi bilan birga keladigan yaxshi xavfsizlik uchun kelishuv bunga arziydi. Agar siz kodingizni profilga kiritsangiz va standart hash funksiyasi sizning maqsadlaringiz uchun juda sekin ekanligini aniqlasangiz, boshqa hasherni belgilash orqali boshqa funksiyaga o'tishingiz mumkin. *hasher* bu `BuildHasher` traitini amalga oshiradigan tur. Traitlar va ularni qanday implement qilish haqida 10-bobda gaplashamiz. Siz o'zingizning hasheringizni noldan implement qilishingiz shart emas; [crates.io](https://crates.io/)<!-- ignore -->-da boshqa Rust foydalanuvchilari tomonidan baham ko'rilgan kutubxonalar mavjud bo'lib, ular ko'plab umumiy hashlash algoritmlarini implement qiladigan hasherlarni ta'minlaydi.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Xulosa

Vectorlar, stringlar va hash-maplar ma'lumotlarni saqlash, kirish va o'zgartirish kerak bo'lganda dasturlarda zarur bo'lgan katta hajmdagi funksionallikni ta'minlaydi. Mana endi hal qilish uchun tayyorlanishingiz kerak bo'lgan ba'zi mashqlar:

* Butun sonlar roʻyxati berilgan boʻlsa, vectordan foydalaning va roʻyxatning medianasini (tartiblanganda, oʻrtadagi qiymat) va  rejimni (koʻpincha sodir boʻladigan qiymat; bu yerda hash-map foydali boʻladi) qaytaring.

* Satrlarni pig lotin tiliga aylantiring. Har bir so'zning birinchi undoshi so'z oxiriga ko'chiriladi va "ay" qo'shiladi, shuning uchun "birinchi" "birinchi-ay" bo'ladi. Unli tovush bilan boshlangan so‘zlarning oxiriga “hay” qo‘shiladi (“olma” “olma-hay”ga aylanadi). UTF-8 kodlash haqidagi tafsilotlarni yodda tuting!
* Hash Map va vectorlardan foydalanib, foydalanuvchiga kompaniyadagi bo'limga xodimlarning ismlarini qo'shishga ruxsat berish uchun matn interfeysini yarating. Masalan, "Asilbekni muhandislikka qo'shish" yoki "Sardorni savdoga qo'shish". Keyin foydalanuvchi bo'limdagi barcha odamlar yoki kompaniyadagi barcha odamlar ro'yxatini bo'limlar bo'yicha, alifbo tartibida tartiblangan holda olishiga ruxsat bering.

Standart kutubxona API texnik hujjatlari vectorlar, stringlar va hash-maplarda ushbu mashqlar uchun foydali bo'lgan usullarni tavsiflaydi!

Biz operatsiyalar muvaffaqiyatsiz bo'lishi mumkin bo'lgan yanada murakkab dasturlarga kirishmoqdamiz, shuning uchun xatolarni hal qilishni muhokama qilish uchun ajoyib vaqt. Qani kettik.

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
