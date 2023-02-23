## Modul daraxtidagi elementga murojaat qilish yo'llari

Rust-ga modul daraxtidagi elementni qayerdan topish mumkinligini ko'rsatish uchun biz fayl tizimida harakat qilishda qanday yo'l(path) ishlatgan bo'lsak, xuddi shunday yo'ldan foydalanamiz. Funksiyani chaqirish uchun biz uning yo'lini bilishimiz kerak.

Yo'l ikki shaklda bo'lishi mumkin:

* *Absolyut yo'l* - bu crate ildizidan boshlanadigan to'liq yo'l; tashqi cretedagi kod uchun mutlaq yo'l crate nomidan boshlanadi va joriy cratedagi kod uchun esa `crate` bilan boshlanadi..
* *N   isbiy yo‘l* joriy moduldan boshlanadi va joriy modulda `self`, `super` yoki identifikatordan foydalanadi.

Mutlaq va nisbiy yo‘llardan keyin ikki nuqta (`::`) bilan ajratilgan bir yoki bir nechta identifikatorlar keladi.

7-1 ro'yxatiga qaytsak, biz `navbat_listiga_qoshish` funksiyasini chaqirmoqchimiz deylik.
Bu so'rash bilan bir xil: `navbat_listiga_qoshish` funksiyasining yo'li nima?
7-3 ro'yxatda 7-1 ro'yxati mavjud bo'lib, ba'zi modullar va funksiyalar olib tashlangan.

Biz crate ildizida belgilangan yangi `restoranda_ovqatlanish` funksiyasidan `navbat_listiga_qoshish` funksiyasini chaqirishning ikkita usulini ko‘rsatamiz. Bu yoʻllar toʻgʻri, ammo bu misolni avvalgidek tuzishga toʻsqinlik qiladigan yana bir muammo bor. Sababini birozdan keyin tushuntiramiz.

`restoranda_ovqatlanish` funksiyasi kutubxonamizning ommaviy API-ning bir qismidir, shuning uchun biz uni `pub` kalit so'zi bilan belgilaymiz. [“`pub` kalit so'zi bilan yo'llarni ochish”][pub]<!-- ignore --> bo‘limida biz `pub` haqida batafsilroq to‘xtalib o'tamiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-3: `navbat_listiga_qoshish` funksiyasini mutlaq va nisbiy yo'llar yordamida chaqirish</span>

Biz birinchi marta `restoranda_ovqatlanish` ichida `navbat_listiga_qoshish` funksiyasini chaqirganimizda mutlaq yo'ldan foydalanamiz. `navbat_listiga_qoshish` funksiyasi `restoranda_ovqatlanish` bilan bir xil crateda belgilangan, ya'ni mutlaq yoʻlni boshlash uchun `crate` kalit soʻzidan foydalanishimiz mumkin. Keyin biz `navbat_listiga_qoshish` ga o'tgunimizcha ketma-ket modullarning har birini o'z ichiga olamiz. Siz bir xil strukturaga ega fayl tizimini tasavvur qilishingiz mumkin: biz `navbat_listiga_qoshish` dasturini ishga tushirish uchun `/uyning_oldi/xizmat/navbat_listiga_qoshish` yo'lini belgilaymiz; crate ildizidan boshlash uchun `crate` nomidan foydalanish shelldagi fayl tizimi ildizidan boshlash uchun `/` dan foydalanishga o'xshaydi.

Biz `restoranda_ovqatlanish` ichida `navbat_listiga_qoshish` ni ikkinchi marta chaqirganimizda nisbiy yo'ldan foydalanamiz. Yo'l `uyning_oldi` bilan boshlanadi, modul nomi `restoranda_ovqatlanish` bilan bir xil modul daraxti darajasida belgilangan. Bu yerda fayl tizimi ekvivalenti `uyning_oldi/xizmat/navbat_listiga_qoshish` yo'lidan foydalaniladi. Modul nomi bilan boshlash yo'l nisbiy ekanligini bildiradi.

Nisbiy yoki mutlaq yo‘ldan foydalanishni tanlash loyihangiz asosida qabul qilinadigan qaror bo‘lib, element definitioni kodini elementdan foydalanadigan koddan alohida yoki birga ko‘chirish ehtimoli ko‘proq ekanligiga bog‘liq.
Masalan, `uyning_oldi` moduli va `restoranda_ovqatlanish` funksiyasini `mijoz_tajribasi` nomli modulga o‘tkazsak, mutlaq yo‘lni `navbat_listiga_qoshish`ga yangilashimiz kerak bo‘ladi, lekin nisbiy yo‘l baribir amal qiladi.
Biroq, agar biz `restoranda_ovqatlanish` funksiyasini `ovqatlanish` nomli modulga alohida ko'chirsak, `restoranda_ovqatlanish` chaqiruvining mutlaq yo'li bir xil bo'lib qoladi, lekin nisbiy yo'l yangilanishi kerak bo'ladi. Umuman olganda, bizning afzal ko'rganimiz mutlaq yo'llarni belgilashdir, chunki biz kod definitionlari va element chaqiruvlarini bir-biridan mustaqil ravishda ko'chirishni xohlaymiz.

Keling, 7-3 ro'yxatini kompilatsiya qilishga harakat qilaylik va nima uchun u hali kompilatsiya bo'lmaganligini bilib olaylik! Biz olgan xato 7-4 ro'yxatda ko'rsatilgan.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Ro'yxat 7-4: 7-3 ro'yxatdagi kodni kompilyatsiya qilishda kompilyator xatolari</span>

Xato xabarlari `xizmat` moduli shaxsiy ekanligini aytadi. Boshqacha qilib aytadigan bo'lsak, bizda `xizmat` moduli va `navbat_listiga_qoshish` funksiyasi uchun to'g'ri yo'llar mavjud, ammo Rust ulardan foydalanishimizga ruxsat bermaydi, chunki u shaxsiy bo'limlarga kirish imkoniga ega emas. Rust-da barcha elementlar (funktsiyalar, metodlar, structlar, enumlar, modullar va konstantalar) standart bo'yicha ota-modullar uchun shaxsiydir. Agar siz funksiya yoki struktura kabi elementni yaratmoqchi bo'lsangiz, uni modulga joylashtirasiz.

Ota-moduldagi elementlar ichki modullar ichidagi shaxsiy elementlardan foydalana olmaydi, lekin bolalar modullaridagi elementlar o'zlarining ota-modullaridagi elementlardan foydalanishi mumkin. Buning sababi shundaki, bolalar modullari o'zlarining amalga oshirish tafsilotlarini o'rab oladi va yashiradi, lekin bolalar modullari ular aniqlangan kontekstni ko'rishlari mumkin. Bizning metaforamizni davom ettirish uchun, maxfiylik qoidalarini restoranning orqa ofisi kabi tasavvur qiling: u erda nima sodir bo'layotgani restoran mijozlari uchun shaxsiy, ammo ofis menejerlari o'zlari ishlayotgan restoranda hamma narsani ko'rishlari va qilishlari mumkin.

Rust modul tizimining shu tarzda ishlashini tanladi, shuning uchun ichki dastur tafsilotlarini yashirish standart bo'yichadir. Shunday qilib, siz ichki kodning qaysi qismlarini tashqi kodni buzmasdan o'zgartirishingiz mumkinligini bilasiz. Biroq, Rust sizga obyektni hammaga ochiq qilish uchun `pub` kalit so'zidan foydalanib, tashqi ajdod modullariga ichki modullar kodining ichki qismlarini ochish imkoniyatini beradi.

### `pub` kalit so'zi bilan yo'llarni ochish

Keling, 7-4 ro'yxatdagi xatoga qaytaylik, bu bizga `xizmat` moduli shaxsiy ekanligini aytdi. Biz ota-moduldagi `restoranda_ovqatlanish` funksiyasi bolalar modulidagi `navbat_listiga_qoshish` funksiyasiga kirishini xohlaymiz, shuning uchun biz `xizmat` modulini `pub` kalit so'zi bilan belgilaymiz, ro'yxat 7-5da ko`rsatilganidek.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-5: `xizmat` modulini `restoranda_ovqatlanish` dan foydalanish uchun `pub` deb e'lon qilish</span>

Afsuski, 7-5 ro'yxatdagi kod hali ham 7-6 ro'yxatda ko'rsatilganidek xatolikka olib keladi.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Ro'yxat 7-6: 7-5 ro'yxatdagi kodni build qilishda kompilyator xatolari</span>

Nima bo'ldi? `mod xizmat` oldiga `pub` kalit so‘zini qo‘shish modulni hammaga ochiq qiladi. Ushbu o'zgarish bilan, agar biz `uyning_oldi` ga kira olsak, biz `xizmat` ga kira olamiz. Lekin `xizmat` ning *tarkibi* hamon shaxsiy; modulni ommaviy qilish uning mazmunini ochiq qilmaydi. Moduldagi `pub` kalit so‘zi faqat uning ota-modullaridagi kodni unga murojaat qilish imkonini beradi, uning ichki kodiga kirishga ruxsat bermaydi.
Modullar konteyner bo'lgani uchun modulni faqat ommaviy qilish orqali biz ko'p narsa qila olmaymiz; biz oldinga borishimiz va modul ichidagi bir yoki bir nechta narsalarni ham hammaga ochiq qilishni tanlashimiz kerak.

7-6 roʻyxatdagi xatolar `navbat_listiga_qoshish` funksiyasi shaxsiy ekanligini bildiradi.
Maxfiylik qoidalari structlar, enumlar, funksiyalar va metodlar hamda modullarga nisbatan qo'llaniladi.

7-7 ro'yxatda ko'rsatilganidek, definitiondan oldin `pub` kalit so'zini qo'shish orqali `navbat_listiga_qoshish` funksiyasini ham hammaga ochiq qilaylik.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-7: `mod xizmat` va `fn navbat_listiga_qoshish` ga `pub` kalit so'zini qo'shish bizga `restoranda_ovqatlanish` funksiyasini chaqirish imkonini beradi.</span>

Endi kod kompilyatsiya qilinadi! Nima uchun`pub` kalit soʻzini qoʻshish ushbu yoʻllardan `navbat_listiga_qoshish` da maxfiylik qoidalariga nisbatan foydalanish imkonini berishini bilish uchun mutlaq va nisbiy yoʻllarni koʻrib chiqamiz.

Mutlaq yo'lda biz crate modul daraxtining ildizi bo'lgan `crate` dan boshlaymiz. `uyning_oldi` moduli crate ildizida belgilangan. `uyning_oldi` ochiq boʻlmasa-da, `restoranda_ovqatlanish` funksiyasi `uyning_oldi` bilan bir xil modulda aniqlanganligi sababli (yaʼni, `restoranda_ovqatlanish` va `uyning_oldi` siblingdir ya'ni aka-uka), biz `restoranda_ovqatlanish` dan `uyning_oldi`ga murojaat qilishimiz mumkin. Keyingi o'rinda `pub` bilan belgilangan `xizmat` moduli. Biz `xizmat` ning ota-moduliga kira olamiz, shuning uchun biz `xizmat` ga kira olamiz. Nihoyat, `navbat_listiga_qoshish` funksiyasi `pub` bilan belgilangan va biz uning asosiy moduliga kira olamiz, shuning uchun bu funksiya chaqiruvi ishlaydi!

Nisbiy yo'lda mantiq birinchi qadamdan tashqari mutlaq yo'l bilan bir xil bo'ladi: yo'l crate ildizidan emas, `uyning_oldi`dan boshlanadi. `uyning_oldi` moduli `restoranda_ovqatlanish` bilan bir xil modul ichida aniqlanadi, shuning uchun `restoranda_ovqatlanish` belgilangan moduldan boshlanadigan nisbiy yo‘l ishlaydi. Keyin, `xizmat` va `navbat_listiga_qoshish` `pub` bilan belgilanganligi sababli, qolgan yo‘l ishlaydi va bu funksiya chaqiruvi amal qiladi!

Agar siz kutubxona crateyingizni boshqa loyihalar sizning kodingizdan foydalanishi uchun baham ko'rishni rejalashtirmoqchi bo'lsangiz, ommaviy API sizning crateyingiz foydalanuvchilari bilan tuzilgan shartnoma bo'lib, ular sizning kodingiz bilan qanday aloqada bo'lishini belgilaydi. Odamlar sizning crateyingizga bog'liq bo'lishini osonlashtirish uchun ommaviy API-ga o'zgartirishlarni boshqarish bo'yicha ko'plab fikrlar mavjud. Bu mulohazalar ushbu kitob doirasidan tashqarida; agar sizni ushbu mavzu qiziqtirsa, [Rust API ko'rsatmalari][api-guidelines]ga qarang.

> #### Binary va kutubxonaga ega paketlar uchun eng yaxshi amaliyotlar
>
> Paketda *src/main.rs* binary crate ildizi ham, *src/lib.rs* kutubxona cratesi ildizi
> ham bo‘lishi mumkinligini aytib o'tdik va ikkala crate ham standart bo‘yicha
> paket nomiga ega bo‘ladi. Odatda, kutubxona va binary crateni o'z ichiga olgan
> ushbu patternli paketlar kutubxona cratesi bilan kod chaqiradigan bajariladigan
> faylni ishga tushirish uchun binary crateda yetarli kodga ega bo'ladi. Bu boshqa
> loyihalarga paket taqdim etadigan eng ko'p funksiyalardan foydalanish imkonini
> beradi, chunki kutubxona cratesi kodi ommaviy bo'lishi mumkin.
>
> Modul daraxti *src/lib.rs* da aniqlanishi kerak. Keyin har qanday ommaviy obyektlar
> binary crateda paket nomi bilan yo'llarni boshlash orqali ishlatilishi mumkin.
> Binary crate kutubxona cratesidan foydalanuvchiga aylanadi, xuddi butunlay tashqi
> crate kutubxona cratesidan foydalanadi: u faqat ommaviy APIdan foydalanishi mumkin.
> Bu sizga yaxshi API yaratishga yordam beradi; Siz nafaqat muallif, balki
> mijoz hamsiz!
>
> [12-bobda][ch12]<!-- ignore --> biz ushbu tashkiliy amaliyotni binary crate va
> kutubxona cratesini o'z ichiga olgan buyruq qatori dasturi bilan ko'rsatamiz.

### Nisbiy yo'llarni `super` bilan boshlash

Yo'l boshida `super` dan foydalanib, joriy modul yoki crate ildizi emas, balki ota-modulda boshlanadigan nisbiy yo'llarni qurishimiz mumkin. Bu fayl tizimi yoʻlini `..` sintaksisi bilan boshlashga oʻxshaydi. `super` dan foydalanish bizga ota-modulda ekanligini biladigan elementga murojaat qilish imkonini beradi, bu modul ota-ona bilan chambarchas bog'liq bo'lsa, modul daraxtini qayta tartibga solishni osonlashtiradi, lekin ota-ona bir kun kelib modul daraxtining boshqa joyiga ko'chirilishi mumkin.

7-8 ro'yxatdagi kodni ko'rib chiqing, unda oshpaz noto'g'ri buyurtmani tuzatgan va uni mijozga shaxsan yetkazgan vaziyatni modellashtiradi. `uyning_orqasi` modulida aniqlangan `buyurtmani_tuzatish` funksiyasi `super` bilan boshlanadigan `yetkazib_berish` yo‘lini belgilash orqali asosiy modulda belgilangan `yetkazib_berish` funksiyasini chaqiradi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-8: `super` bilan boshlanadigan nisbiy yo'l yordamida funksiyani chaqirish</span>

`buyurtmani_tuzatish` funksiyasi `uyning_orqasi` modulida joylashgan, shuning uchun biz `super` dan `uyning_orqasi` ota-moduliga o'tishimiz mumkin. U yerdan `yetkazib_berish` ni qidiramiz va uni topamiz.
Muvaffaqiyat! Bizning fikrimizcha, `uyning_orqasi` moduli va `yetkazib_berish` funksiyasi bir-biri bilan bir xil munosabatda bo'lib qoladi va agar biz cratening modul daraxtini qayta tashkil etishga qaror qilsak, birgalikda harakatlanadi. Shu sababli, biz `super` dan foydalandik, shuning uchun kelajakda bu kod boshqa modulga ko‘chirilsa, kodni yangilash uchun kamroq joylarga ega bo‘lamiz.

### Structlar va Enumlarni ommaviy qilish(Public)

Shuningdek, structlar va enumlarni ommaviy sifatida belgilash uchun `pub` dan foydalanishimiz mumkin, ammo `pub` dan structlar va enumlar bilan foydalanish uchun qo'shimcha tafsilotlar mavjud. Agar struct definitiondan oldin `pub` dan foydalansak, biz structni hammaga ommaviy qilamiz, lekin structning maydonlari hali ham shaxsiy bo'lib qoladi. Biz har bir sohani alohida-alohida ommaviy qilishimiz yoki qilmasligimiz mumkin. 7-9 roʻyxatda biz ommaviy `qizdirilgan_non` maydoni, lekin shaxsiy `mavsumiy_meva` maydoni bilan ommaviy `uyning_orqasi:: nonushta` structini belgilab oldik. Bu restoranda mijoz ovqat bilan birga keladigan non turini tanlashi mumkin bo'lgan holatni modellashtiradi, ammo oshpaz qaysi meva mavsumda va omborda borligiga qarab ovqatga hamroh bo'lishini hal qiladi. Mavjud mevalar tezda o'zgaradi, shuning uchun mijozlar mevani tanlay olmaydi yoki hatto qaysi mevani olishini ko'ra olmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-9: Ba'zi ommaviy maydonlari va ba'zilari bo'lgan struct
xususiy maydonlar</span>

`uyning_orqasi::Nonushta` structdagi `yopilgan_non` maydoni ommaviy bo'lgani uchun `restoranda_ovqatlanish` da biz `yopilgan_non` maydoniga nuqta belgisi yordamida yozishimiz va o'qishimiz mumkin. Esda tutingki, biz `mavsumiy_meva` maydonidan `restoranda_ovqatlanish`da foydalana olmaymiz, chunki `mavsumiy_meva` shaxsiydir. Qaysi xatoga yo'l qo'yganingizni bilish uchun `mavsumiy_meva` maydoni qiymatini o'zgartiruvchi qatorni izohdan chiqarib ko'ring!

Shuni ham yodda tutingki, `uyning_orqasi::Nonushta` shaxsiy maydonga ega bo'lgani uchun struct `Nonushta` misolini yaratuvchi ommaviy bog'langan funksiyani ta'minlashi kerak (biz uni bu yerda `yoz` deb nomladik).Agar `Nonushta` bunday funksiyaga ega boʻlmagan boʻlsa, biz `restoranda_ovqatlanish`da `Nonushta` misolini yarata olmadik, chunki biz `restoranda_ovqatlanish`da shaxsiy `mavsumiy_meva` maydonining qiymatini oʻrnata olmadik.

Aksincha, agar biz enumni ommaviy qilsak, uning barcha variantlari ommaviy bo'ladi. 7 10 roʻyxatda koʻrsatilganidek, bizga faqat `enum` kalit soʻzidan oldin `pub` kerak boʻladi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-10: Enumni ommaviy deb belgilash uning barcha variantlarini hammaga ochiq qiladi</span>

Biz `Taom` ro‘yxatini hammaga ommaviy qilganimiz uchun `restoranda_ovqatlanish`da `Palov` va `Salat` variantlaridan foydalanishimiz mumkin.

Enumlar, agar ularning variantlari ommaviy bo'lmasa, unchalik foydali emas; Har bir holatda `pub` bilan barcha enum variantlariga izoh qo'yish zerikarli bo'lar edi, shuning uchun enum variantlari uchun standart umumiy bo'lishi kerak. Structlar ko'pincha maydonlari ommaviy bo'lmasdan foydali bo'ladi, shuning uchun struct maydonlari, agar `pub` bilan izohlanmagan bo'lsa, standart bo'yicha hamma narsa shaxsiy bo'lishining umumiy qoidasiga amal qiladi.

`pub` bilan bog'liq yana bir holat bor, biz uni ko'rib chiqmaganmiz va bu bizning modul tizimining oxirgi xususiyati: `use` kalit so'zi. Biz avval `use` ni o'z ichiga olamiz, so'ngra `pub` va `use` ni qanday birlashtirishni ko'rsatamiz.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
