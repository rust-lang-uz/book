## Modullarni turli fayllarga ajratish

Hozirgacha ushbu bobdagi barcha misollar bitta faylda bir nechta modullarni aniqladi.
Modullar kattalashganda, kodni boshqarishni osonlashtirish uchun ularning definitionlarini alohida faylga ko'chirishingiz mumkin.

Masalan, 7-17 ro'yxatdagi bir nechta restoran moduliga ega bo'lgan koddan boshlaylik. Biz cratening ildiz modulidagi barcha modullarni aniqlash o'rniga modullarni fayllarga ajratamiz. Bunday holda, cratening ildiz fayli *src/lib.rs* bo'ladi, lekin bu protsedura crate ildiz fayli *src/main.rs* bo'lgan binary cratelar bilan ham ishlaydi.

Birinchidan, biz `uyning_oldi` modulini o'z fayliga chiqaramiz. `uyning_oldi` moduli uchun jingalak qavslar ichidagi kodni olib tashlang va faqat `mod uyning_oldi` deklaratsiyasini qoldiring, shunda *src/lib.rs* ro'yxat 7-21da ko'rsatilgan kodni o`z ichiga oladi. E'tibor bering, biz 7-22 ro'yxatda *src/uyning_oldi.rs* faylini yaratmagunimizcha, bu kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-21. Tarkibi `src/uyning_oldi.rs` da joylashgan `uyning_oldi` modulini e'lon qilish</span>

Keyin, jingalak qavslardagi kodni yangi faylga joylashtiring
7-22 ro'yxatda ko'rsatilganidek *src/uyning_oldi.rs* deb nomlangan. Kompilyator bu faylda nimani izlash kerakligini biladi, chunki u `uyning_oldi` deb nomlangan cratening ildiz modulida modul deklaratsiyasiga duch keldi.

<span class="filename">Fayl nomi: src/uyning_oldi.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Ro'yxat 7-22. *src/uyning_oldi.rs* faylida `uyning_oldi` modulining mazmunini aniqlash</span>

Esda tutingki, modul daraxtida *bir marta* `mod` deklaratsiyasidan foydalanib faylni yuklashingiz kerak. Kompilyator fayl loyihaning bir qismi ekanligini bilgandan so'ng (va `mod` statementi qo'ygan joyingiz tufayli kod modul daraxtining qayerida joylashganligini biladi), loyihangizdagi boshqa fayllar yuklangan fayl kodiga u e'lon qilingan joyga yo'l orqali murojaat qilishi kerak, bu ["Modul daraxtidagi elementga murojaat qilish yo'llari"][paths]<!-- ignore --> bo'limida yoritilgan. Boshqacha qilib aytganda, `mod` boshqa dasturlash tillarida ko'rishingiz mumkin bo'lgan “include” operatsiyasi emas.

Keyinchalik, biz `xizmat` modulini o'z fayliga chiqaramiz. Jarayon biroz boshqacha, chunki `xizmat` ildiz modulining emas, balki `uyning_oldi` ichki modulidir.Biz `xizmat` faylini modul daraxtidagi ajdodlari nomi bilan ataladigan yangi jildga joylashtiramiz, bu holda *src/uyning_oldi/*.

`xizmat`ni ko‘chirishni boshlash uchun biz *src/uyning_oldi.rs* ni faqat `xizmat` moduli deklaratsiyasini o‘z ichiga olgan holda o‘zgartiramiz:

<span class="filename">Fayl nomi: src/uyning_oldi.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

Keyin biz *src/uyning_oldi* jildini va `xizmat` modulida berilgan definitionlarni o'z ichiga olgan *xizmat.rs* faylini yaratamiz:

<span class="filename">Fayl nomi: src/uyning_oldi/xizmat.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

Agar biz *src* jildiga *xizmat.rs* ni qo'ysak, kompilyator *xizmat.rs* kodi crate ildizida e'lon qilingan va `uyning_oldi` modulining yordamchisi sifatida e'lon qilinmagan `xizmat` modulida bo'lishini kutadi. Kompilyator qoidalari qaysi modullarning kodini o'z ichiga olgan fayllarni tekshirish uchun jildlar va fayllar modul daraxtiga to'liq mos kelishini taxmin qiladi.

> ### Muqobil fayl yo'llari
>
> Hozirgacha biz Rust kompilyatori foydalanadigan eng idiomatik fayl yo'llarini ko'rib chiqdik,
> lekin Rust fayl yo'lining eski uslubini ham qo'llab-quvvatlaydi. Crate ildizida e'lon qilingan
> `uyning_oldi` nomli modul uchun kompilyator modul kodini quyidagilardan qidiradi:
> module’s code in:
>
> * *src/uyning_oldi.rs* (biz nimani qamrab oldik)
> * *src/uyning_oldi/mod.rs* (eski uslub, hali ham qo'llab-quvvatlanadigan yo'l)
>
> `uyning_oldi` submodul bo'lgan `xizmat` nomli modul uchun kompilyator modul kodini qidiradi:
>
> * *src/uyning_oldi/xizmat.rs* (biz nimani qamrab oldik)
> * *src/uyning_oldi/xizmat/mod.rs* (eski uslub, hali ham qo'llab-quvvatlanadigan yo'l)
>
> Agar bir xil modul uchun ikkala uslubdan foydalansangiz, kompilyator xatosi paydo bo'ladi. Bitta
> loyihada turli modullar uchun ikkala uslubning aralashmasidan foydalanishga ruxsat
> beriladi, lekin loyihangizni boshqarayotgan odamlar uchun chalkash bo'lishi mumkin.
>
> *mod.rs* nomli fayllardan foydalanadigan uslubning asosiy kamchiligi
> shundaki, sizning loyihangiz *mod.rs* nomli ko‘plab fayllar bilan
> tugashi mumkin, ular bir vaqtning o‘zida muharriringizda ochilganda
> chalkash bo‘lishi mumkin.

Biz har bir modul kodini alohida faylga ko'chirdik va modul daraxti o'zgarishsiz qoldi. `restoranda_ovqatlanish` funksiyasi chaqiruvlari, definitionlar turli fayllarda bo'lsa ham, hech qanday o'zgartirishlarsiz ishlaydi. Ushbu texnika modullarni hajmi oshgani sayin yangi fayllarga ko'chirish imkonini beradi.

Esda tutingki, *src/lib.rs* dagi `pub use crate::uyning_oldi::xizmat` statementi ham o'zgarmagan va `use` qaysi fayllar cratening bir qismi sifatida tuzilganiga ta'sir qilmaydi. `mod` kalit so'zi modullarni e'lon qiladi va Rust ushbu modulga kiradigan kod moduli bilan bir xil nomdagi faylga qaraydi.

## Xulosa

Rust sizga paketni bir nechta cratelarga va crateni modullarga bo'lish imkonini beradi, shunda siz bir modulda belgilangan elementlarga boshqa moduldan murojaat qilishingiz mumkin. Buni mutlaq yoki nisbiy yo'llarni belgilash orqali amalga oshirishingiz mumkin. Ushbu yo'llar `use` statementi bilan qamrab olinishi mumkin, shuning uchun siz ushbu doiradagi elementdan bir nechta foydalanish uchun qisqaroq yo'ldan foydalanishingiz mumkin. Modul kodi standart boʻyicha maxfiydir, lekin `pub` kalit soʻzini qoʻshish orqali definitionlarni hammaga ommaviy qilishingiz mumkin.

Keyingi bobda biz standart kutubxonadagi ma'lumotlar tuzilmalarining ba'zi to'plamlarini ko'rib chiqamiz, ulardan siz o'zingizning aniq tartiblangan kodingizda foydalanishingiz mumkin.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
