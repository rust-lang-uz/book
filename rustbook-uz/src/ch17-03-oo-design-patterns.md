## Ob'ektga Yo'naltirilgan Dizayn Shablonini Amalga Oshirishi

*Holat shabloni* — bu ob'ektga yo'naltirilgan dizayn shabloni. Ushbu shablonning asosiy maqsadi - ichki holda bir qiymatning olishi mumkin bo'lgan
holatlarning to'plamini aniqlashdir. Holatlar *holat ob'ektlari* to'plami bilan ifodalanadi va qiymatning xatti-harakatlari uning holatiga qarab
o'zgaradi. Biz blog postining strukturasini misol sifatida ko'rib chiqamiz, bu struktura "draft", "review" yoki "published" deb nomlangan holatlarni
saqlovchi maydonni o'z ichiga oladi.

Holat ob'ektlari funktsionallikni baham ko'radi: Rustda, albatta, biz ob'ektlar va meros olish o'rnini strukturalar va xususiyatlar bilan to'ldiramiz.
Har bir holat ob'ekti o'z xatti-harakatlari uchun mas'ul bo'lib, qachon boshqa holatga o'tishini belgilaydi. Holat ob'ektini saqlovchi qiymat holatlar
turli xatti-harakatlari yoki holatlarni o'tkazish qachonligini bilmaydi.

Holat shablonidan foydalanishning foydasi shundaki, agar dasturiy ta'minot talablariga o'zgarishlar kirsa, biz holatni ushlab turuvchi qiymat kodini yoki
qiymatni ishlatadigan kodni o'zgartirishimiz shart emas. Biz faqat bitta holat ob'ekti ichidagi qoidalarni yoki ehtimol yangi holat ob'ektlarini
qo'shishimiz kerak.
Avvalambor, biz holat shablonini an’anaviy ob’ektga yo‘naltirilgan uslubda amalga oshiramiz, so‘ngra Rustda biroz tabiiyroq bo‘lgan yondashuvdan
foydalanamiz. Keling, holat shablonini ishlatib, blog postining ish oqimini bosqichma-bosqich amalga oshirishga kirishamiz.

Oxirgi funksionallik quyidagi ko‘rinishga ega bo‘ladi:

1. Blog posti bo’sh loyiha sifatida boshlanadi.
2. Loyiha tugagach, postning ko‘rigi so‘raladi.
3. Post ma’qullangach, u e’lon qilinadi.
4. Faqat e’lon qilingan blog postlari chop etishga kontent qaytaradi, shuning uchun ma’qullanmagan postlar tasodifan e’lon qilinmaydi.

Postga boshqa har qanday o‘zgarishlar kiritganimizda, u ta’sir qilmasligi kerak. Misol uchun, agar biz ko‘rishni so‘ramasdan, loyiha blog postini
ma’qullamoqchi bo‘lsak, post e’lon qilinmagan loyiha sifatida qolishi kerak.

Ro‘yxat 17-11 bu ish oqimini kod ko‘rinishida qiladi: bu biz amalga oshirmoqchi bo‘lgan `blog` nomli kutubxona qutisi API ni ishlatishining misolidir. Bu
hali kompilyatsiya bo‘lmaydi, chunki biz `blog` qutisini amalga oshirmaganmiz.

<span class="filename">Fayl nomi: src/main.rs</span>

rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:all}}


<span class="caption">Ro‘yxat 17-11: Bizning `blog` qutimizda bo‘lishini xohlagan xatti-harakatlarni namoyish etuvchi kod</span>

Biz foydalanuvchiga `Post::new` yordamida yangi loyiha blog postini yaratishga ruxsat berishni istaymiz. Biz blog postiga matn qo‘shishga ruxsat
bermoqchimiz. Agar biz postning kontentini darhol, ma’qullashdan oldin olishga harakat qilsak, post hali loyiha bo‘lganligi sababli hech qanday matnni
olmaymiz. Demonstratsiya maqsadlari uchun kode `assert_eq!` qo‘shilgan. Ushbu misol uchun juda yaxshi bir birlik testi, loyiha blog posti `content`
metodidan bo‘sh qatorni qaytarishini tasdiqlash bo‘lardi, ammo biz bu misol uchun testlar yozmaymiz.

Keyin, biz postning ko‘rigini so‘rashni yoqmoqchimiz va `content` ko‘rishni kutayotgan davrda bo‘sh qatorni qaytarishi kerak. Post ma’qullangach, u e’lon
qilinishi kerak, ya’ni `content` chaqirilganda postning matni qaytarilishi kerak.
E'tibor bering, biz qutidan faqat `Post` turidagi ob'ekt bilan o'zaro aloqada bo'lmoqdamiz. Ushbu tur holat shablonidan foydalanadi va postning qanday
holatlarda bo'lishini ifodalaydigan uchta holat ob'ektidan birini saqlaydi — loyiha, ko'rish uchun kutish yoki e'lon qilingan. Bir holatdan boshqasiga 
‘tish `Post` turida ichki ravishda boshqariladi. Holatlar kutubxona foydalanuvchilari tomonidan `Post` eksemplarida chaqirilgan metodlarga javoban
o'zgaradi, lekin ular holat o'zgarishlarini to'g'ridan-to'g'ri boshqarishlari shart emas. Shuningdek, foydalanuvchilar holatlar bilan xato qila
olmaydilar, masalan, postni ko'rmasdan e'lon qilish.

### `Post` ni ta'riflash va Loyiha Holatidagi Yangi Instansiya Yaratish

Kutubxonaning amalga oshirilishiga kirishaylik! Bizga ba'zi kontentlarni saqlovchi jamoat `Post` strukturasini yaratish kerakligini bilamiz, shuning
uchun biz strukturaning ta'rifidan va `Post` instansiyasini yaratish uchun bog'liq jamoat `new` funksiyasini quyida keltirilgan Ro'yxat 17-12 dan
boshlaymiz. Shuningdek, biz `Post` uchun barcha holat ob'ektlarida bo'lishi kerak bo'lgan xatti-harakatlarni aniqlaydigan maxfiy `State` xususiyatini ham
yaratamiz.

Keyin `Post` ichida `state` deb nomlangan maxfiy maydonda holat ob'ektini saqlash uchun `Option<T>` ichida `Box<dyn State>` turidagi xususiyat ob'ektini 
tutadi. `Option<T>` ni nima uchun zarurligini bir ozdan keyin ko'rasiz.

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;

rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-12/src/lib.rs}}


&lt;span class=&quot;caption&quot;&gt;Ro'yxat 17-12: `Post` strukturasining ta'rifi va yangi `Post` instansiyasini yaratadigan `new` funksiyasi, `State` 
xususiyati va `Draft` strukturasiga oid&lt;/span&gt;

`State` xususiyati turli post holatlari tomonidan baham ko'riladigan xatti-harakatlarni belgilaydi. Holat ob'ektlari `Draft`, `PendingReview` va 
`Published` bo'lib, ularning barchasi `State` xususiyatini amalga oshiradi. Hozircha, xususiyatda hech qanday metod yo'q, va biz faqatgina post 
boshlanishi kerak bo'lgan `Draft` holatini belgilashdan boshlaymiz.


Yangi `Post` yaratayotganimizda, biz uning `state` maydonini `Box` tutuvchi `Some` qiymatiga o'rnatamiz. Ushbu `Box` yangi `Draft` strukturasi 
instansiyasiga yo'naltirilgan. Bu, har safar yangi `Post` instansiyasini yaratganimizda, u loyiha sifatida boshlanishini ta'minlaydi. `Post`ning `state` 
maydoni maxfiy bo'lgani uchun, `Post`ni boshqa holatda yaratishning iloji yo'q! `Post::new` funksiyasida, biz `content` maydonini yangi, bo'sh `String` 
ga o'rnatamiz. 

### Post Kontentining Matnini Saqlash

Ro'yxat 17-11 da biz `add_text` nomli metodni chaqirishni xohlayotganimizni va unga beriladigan `&str` ni blog postining matn kontenti sifatida 
qo'shishni xohlaymiz. Biz buni metod sifatida amalga oshiramiz, `content` maydonini `pub` sifatida ochmaslik uchun, shunda keyinchalik `content` 
maydonidagi ma'lumotlarni qanday o'qishni boshqaradigan metodni amalga oshira olishimiz mumkin. `add_text` metodi juda oddiy, shuning uchun uni Ro'yxat 
17-13 ga `impl Post` blokiga qo'shamiz:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;

rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-13/src/lib.rs:here}}


&lt;span class=&quot;caption&quot;&gt;Ro'yxat 17-13: Postning `content` qismini to'ldirish uchun `add_text` metodini amalga oshirish&lt;/span&gt;

`add_text` metodi `self` ga o'zgaruvchan havolani oladi, chunki biz `add_text` chaqirayotgan `Post` instansiyasini o'zgartirmoqdamiz. Keyin, biz 
`content` dagi `String` ustida `push_str` ni chaqiramiz va saqlangan `content` ga qo'shish uchun `text` argumentini uzatamiz. Bu xatti-harakat post qaysi 
holatda bo'lishidan bog'liq emas, shuning uchun bu holat shablonining bir qismi emas. `add_text` metodi `state` maydoni bilan umuman o'zaro aloqada emas, 
lekin bu biz qo'llab-quvvatlamoqchi bo'lgan xatti-harakatlar qismidir.

### Loyiha Postining Kontenti Bo'shligini Ta'minlash

Biz `add_text` ni chaqirib, postimizga kontent qo'shgandan keyin ham, `content` metodimiz bo'sh qator qaytarishini xohlaymiz, chunki post hali loyiha 
holatida, Ro'yxat 17-11 da 7-qatorda ko'rsatilganidek. Hozircha, biz ushbu talabni bajaradigan eng oddiy narsani amalga oshirish orqali `content` 
metodini belgilaymiz: doimo bo'sh qator qaytarish. Postning holatini o'zgartirish imkonini beradigan kodni amalga oshirgandan so'ng, bu yerda 
o'zgartiramiz. Hozirgi kunda postlar faqat loyiha holatida bo'lishi mumkin, shuning uchun post kontenti doimo bo'sh bo'lishi kerak. Ro'yxat 17-14 bu 
asqotuvchi amalga oshirishni ko'rsatadi:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;

rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-14/src/lib.rs:here}}


&lt;span class=&quot;caption&quot;&gt;Ro'yxat 17-14: `content` metodining bo'sh qator qaytaradigan asqotuvchi amalga oshirilishi&lt;/span&gt;

Ushbu qo'shilgan `content` metodi bilan, Ro'yxat 17-11 da 7-qatorgacha bo'lgan hamma narsa kutilganidek ishlaydi.

### Postni Ko'rish So'rash Holatini O'zgartiradi

Keyingi kerakli funksionallik postning ko'rinishi so'rash uchun bir imkoniyat qo'shishdir, bu esa uning holatini `Draft` dan `PendingReview` ga 
o'zgartirishi kerak. Ro'yxat 17-15 ushbu kodni ko'rsatadi:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;

rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-15/src/lib.rs:here}}


&lt;span class=&quot;caption&quot;&gt;Ro'yxat 17-15: `Post` da va `State` xususiyatida `request_review` metodlarini amalga oshirish&lt;/span&gt;

Biz `Post` ga `request_review` nomli jamoat metodini beramiz, bu `self` ga o'zgaruvchan havolani oladi. Keyin biz `Post`ning joriy holatida ichki 
`request_review` metodini chaqiramiz va bu ikkinchi `request_review` metodi joriy holatni iste'mol qiladi va yangi holatni qaytaradi.

Biz `request_review` metodini `State` xususiyatiga qo'shamiz; endi xususiyatni amalga oshiradigan barcha turlarga `request_review` metodini amalga 
oshirish zarur. E'tibor bering, metodning birinchi parametrida `self`, `&self` yoki `&mut self` o'rniga `self: Box<Self>` bor. Ushbu sintaksis, metod 
faqatgina turini saqlovchi `Box` da chaqirilganda haqiqiy bo'lishini anglatadi. Ushbu sintaksis `Box<Self>` ning mulkini o'zlashtiradi va eski holatni 
yo'q qiladi, shuning uchun `Post`ning holat qiymati yangi holatga aylanishi mumkin.

Eski holatni iste'mol qilish uchun `request_review` metodi holat qiymatining mulkini olishi kerak. Bu yerda `Post`ning `state` maydonidagi `Option` kela 
boshlaydi: biz `state` maydonidan `Some` qiymatini olib, o'rniga `None` qo'yish uchun `take` metodini chaqiramiz, chunki Rust bizga strukturalarda 
to'ldirilmagan maydoni bo'lishiga yo'l qo'ymaydi. Bu bizga `state` qiymatini `Post`dan olib yurishni ta'minlaydi, uni ijaraga olish o'rniga. Keyin biz 
postning `state` qiymatini ushbu amaliyot natijasiga o'rnatamiz.

Biz `state` ni bevosita `self.state = self.state.request_review();` kabi kod yozish orqali sozlashimiz kerak emas, o'z mulkimizni olish uchun 
vaqtinchalik `None` qilib qo'yamiz. Bu, `Post` yangilangan holatga o'girilib olgandan keyin eski `state` qiymatini ishlata olmaydiganligini ta'minlaydi.

`Draft` dan `request_review` metodi yangi, `PendingReview` strukturasining yangi, qutichalangan instansiyasini qaytaradi, bu post ko'rinishini kutayotgan 
holatni ifodalaydi. `PendingReview` strukturasiga ham `request_review` metodi amalga oshirilgan, lekin hech qanday o'zgarishlar qilmadi. Aksincha, u 
o'zini qaytaradi, chunki post allaqachon `PendingReview` holatida bo'lganda, biz ko'rinish so'raganimizda, u `PendingReview` holatida qolishi kerak.
Endi biz holat shablonining afzalliklarini ko‘ra boshlaymiz: `Post`’dagi `request_review` metodi uning `state` qiymatiga qarab bir xil bo‘ladi. Har bir 
holat o‘z qoidalari uchun mas’uldir.

Biz `Post`dagi `content` metodini o‘z holatida qoldiramiz, ya’ni bo‘sh qator qaytaradi. Endi bizda `PendingReview` holatidagi va `Draft` holatidagi 
`Post` bo‘lishi mumkin, lekin `PendingReview` holatida bir xil xatti-harakatlarni xohlaymiz. Ro‘yxat 17-11 endi 10-qatorgacha kutilganidek ishlaydi!

### `content` metodining xatti-harakatlarini o‘zgartirish uchun `approve` qo‘shish

`approve` metodi `request_review` metodiga o‘xshash bo‘ladi: bu holat ma’qullanganda joriy holat aytgan qiymatga `state` ni o‘rnatadi, Ro‘yxat 17-16 da ko
‘rsatilganidek:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;
rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-16/src/lib.rs:here}}


&lt;span class=&quot;caption&quot;&gt;Ro‘yxat 17-16: `Post` da va `State` xususiyatida `approve` metodini amalga oshirish&lt;/span&gt;

Biz `approve` metodini `State` xususiyatiga qo‘shamiz va `State` ni amalga oshiruvchi yangi struktura, ya’ni `Published` holatini qo‘shamiz.

`PendingReview` holatidagi `request_review` qanday ishlashini o‘z ichiga olgan holda, agar biz `Draft` ustida `approve` metodini chaqirsak, bu hech 
qanday ta’sir qilmaydi, chunki `approve` `self` ni qaytaradi. `PendingReview` ustida `approve` chaqirsak, u yangi, qutichalangan `Published` 
strukturasining instansiyasini qaytaradi. `Published` strukturasi `State` xususiyatini amalga oshiradi va `request_review` va `approve` metodlari bo
‘yicha u o‘zini qaytaradi, chunki post shu holatlarda `Published` holatida qolishi kerak.

Endi biz `Post`dagi `content` metodini yangilashimiz kerak. `content` dan qaytariladigan qiymat `Post`ning joriy holatiga bog‘liq bo‘lishini xohlaymiz, 
shuning uchun `Post` `state` da aniqlangan `content` metodiga delegatsiya qiladi, bu Ro‘yxat 17-17 da ko‘rsatilgan:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;
rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-17/src/lib.rs:here}}


&lt;span class=&quot;caption&quot;&gt;Ro‘yxat 17-17: `Post`dagi `content` metodini yangilash, `State` da joylashgan `content` metodiga delegatsiya 
qilish &lt;/span&gt;
Maqsadimiz, `State` ni amalga oshiruvchi strukturalarda barcha qoidalarni saqlab qolish bo‘lgani uchun, biz `state` qiymatidagi `content` metodini 
chaqiramiz va post instansiyasini (ya’ni, `self`) argument sifatida uzatamiz. So‘ngra, `state` qiymatidagi `content` metodidan qaytgan qiymatni 
qaytaramiz.&lt;br/&gt;&lt;br/&gt;Biz `Option` ustida `as_ref` metodini chaqiramiz, chunki biz qiymatning mulkini emas, balki `Option` ichidagi qiymatga 
murojaatni xohlaymiz. `state` `Option<Box<dyn State>>` bo‘lganligi sababli, `as_ref` chaqirganda `Option<&Box<dyn State>>` qaytadi. Agar biz `as_ref` ni 
chaqirmasak, xato xabari yuklanadi, chunki biz `state` ni funksiyaning parametrning ijaraga olingan `&self` dan harakatga o'tkaza olmaymiz.&lt;br/&gt;&lt;
br/&gt;Keyin biz `unwrap` metodini chaqiramiz, bu esa hech qachon xato qilmaydi, chunki `Post`dagi metodlar bizga `state` har doim `Some` qiymatini o‘z 
ichiga olishini ta’minlaydi. Bu biz 9-bobdagi [“Compiler’dan ko‘proq ma’lumotga ega bo‘lgan holatlar”][more-info-than-rustc] bo‘limida muhokama qilgan 
holatlardan biridir, bunda `None` qiymatini olish mumkin emas, garchi kompilyator bu ma’lumotni tushunmasa ham.&lt;br/&gt;&lt;br/&gt;Hozirgi paytda, biz 
`&Box<dyn State>` ustida `content` ni chaqirganda, `&` va `Box` ustida deref coercion amal qiladi, shuning uchun `content` metodi `State` xususiyatini 
amalga oshiradigan tur ustida chaqirilib qoladi. Bu bizdan `State` xususiyatining ta’rifiga `content` qo‘shishni talab qiladi, va bu yerda biz qaysi 
holatga bog‘liq holda qaysi kontentni qaytarish uchun mantiq qo‘yamiz, Ro‘yxat 17-18 ga ko‘rsatilgan:&lt;br/&gt;&lt;br/&gt;&amp;lt;span class=&amp;quot;
filename&amp;quot;&amp;gt;Fayl nomi: src/lib.rs&amp;lt;/span&amp;gt;&lt;br/&gt;&lt;pre&gt;&lt;code&gt;rust,noplayground&lt;br/&gt;{{#rustdoc_include ../
listings/ch17-oop/listing-17-18/src/lib.rs:here}}&lt;br/&gt;&lt;/code&gt;&lt;/pre&gt;&lt;br/&gt;&lt;br/&gt;&amp;lt;span class=&amp;quot;caption&amp;quot;&
amp;gt;Ro‘yxat 17-18: `State` xususiyatiga `content` metodini qo‘shish&amp;lt;/span&amp;gt;&lt;br/&gt;&lt;br/&gt;Biz `content` metodining bo‘sh qatorni 
qaytaradigan standart amalga oshirishini qo‘shamiz. Bu demak, biz `Draft` va `PendingReview` strukturalarida `content` ni amalga oshirish shart emas. 
`Published` struktura esa `content` metodini o‘zgartirib, `post.content` da saqlangan qiymatni qaytaradi.&lt;br/&gt;&lt;br/&gt;E’tibor bering, biz ushbu 
metodda umr davomiyligi annotatsiyalarini qo‘shishimiz kerak, bu haqda 10-bobda muhokama qilganmiz. Biz `post` ga havola olib, bu `post`ning bir qismiga 
havola qaytaryapmiz, shuning uchun qaytgan havolaning umr davomiyligi `post` argumentining umr davomiyligiga bog‘liq.&lt;br/&gt;&lt;br/&gt;Biz ishimizni 
tugatdik — Endi Ro‘yxat 17-11 ning hammasi ishlaydi! Biz blog postlari ish oqimi qoidalarini belgilash bilan holat shablonini amalga oshirdik. Qoidalarga 
bog‘liq bo‘lgan mantiq holat ob’ektlarida yashaydi, bu esa `Post` bo‘ylab tarqalmaydi.
> #### Nega Enum Emas?
>
> Siz biz niqob sifatida postning turli holatlari uchun `enum` ishlatmaganimizni o‘ylab qolgan bo‘lishingiz mumkin. Bu albatta mumkin bo‘lgan yechim, 
buni sinab ko‘rishingiz va natijalarini taqqoslab ko‘rishingiz mumkin! Enumdan foydalanishning bir kamchiligi shundaki, enum qiymatini tekshiradigan har 
bir joyda har qanday imkoniyati variantni boshqarish uchun `match` ifodasi yoki shunga o‘xshash narsa bo‘lishi kerak. Bu holat ob'ektli yechimga nisbatan 
ko‘proq takroriy bo‘lishi mumkin.

### Holat Shablonining O‘zaro Ta'siri

Biz Rustda post har bir holatda qanday xil xatti-harakatlarga ega bo‘lishini o‘z ichiga oladigan ob'ektga yo‘naltirilgan holat shablonini amalga oshirib 
bo‘ldik. `Post`dagi metodlar turli xatti-harakatlar haqida hech narsa bilmaydi. Biz kodni qanday tashkil qilganimiz tufayli, nashr etilgan postning turli 
xil xatti-harakatlarini bilish uchun faqat bitta joyni ko‘rishimiz kerak: `Published` strukturasi ustida `State` xususiyatini amalga oshirishi.

Agar biz holat shablonidan foydalanmaydigan muqobil amalga oshirishni yaratsak, biz `Post`dagi metodlarda yoki hatto postning holatini tekshiradigan va u 
erda xatti-harakatlarni o‘zgartiradigan `main` kodida `match` ifodalaridan foydalanishimiz mumkin edi. Bu nashr etilgan holatda postning barcha 
ta'sirlarini tushunish uchun bir necha joylarni ko‘rishimiz kerak bo‘lardi! Bu qanchalik ko‘p holat qo‘shsak, shuncha ko‘p joylarni ko‘rishimiz kerak bo
‘ladi, har bir `match` ifodasi yana bir quloq qo‘shishi kerak.

Holat shablonini qo‘llaganimizda, `Post` metodlari va `Post`ni ishlatayotgan joylar `match` ifodalariga muhtoj bo‘lmaydi, va yangi holat qo‘shish uchun 
faqat yangi strukturani qo‘shish va shu bir struktura ustida xususiyat metodlarini amalga oshirishimiz kerak bo‘ladi.

Holat shablonidan foydalanish orqali amalga oshirishni kengaytirish va ko‘proq funksionallik qo‘shish oson. Holat shablonini qo‘llaydigan kodni 
saqlashning oddiyligini ko‘rish uchun bir nechta takliflarni amalga oshirishga harakat qiling:

* `PendingReview` holatini `Draft` ga qaytaradigan `reject` metodini qo‘shing.
* Holat `Published` ga o‘zgartirilishi uchun `approve` ga ikki marotaba chaqirishni talab qiling.
* Foydalanuvchilarga faqat post `Draft` holatida bo‘lganda matn kontentini qo‘shishiga ruxsat bering. Ishoralar: holat ob'ekti kontentni qanday o
‘zgarishi bo‘yicha mas’ul bo‘ladi, lekin `Post`ni o‘zgartirish bo‘yicha mas’ul bo‘lmaydi.

Holat shablonining bir kamchiligi shundaki, holatlar o‘rtasidagi o‘tishni amalga oshirishi tufayli, ba'zi holatlar bir-biriga bog‘langan. Agar biz 
`PendingReview` va `Published` o‘rtasida `Scheduled` kabi yana bir holat qo‘shsak, `PendingReview` da `Scheduled` ga o‘tish uchun kodni o‘zgartirishimiz 
kerak bo‘ladi. Agar `PendingReview` yangi holat qo‘shilganda o‘zgarish qilmasa, bu kamroq ish bo‘lardi, lekin bu boshqa dizayn shabloniga o‘tishni 
anglatadi.

Yana bir kamchiligi shundaki, ba'zi mantiqlarni takrorladik. Takrorlashlarni yo‘qotish uchun, ehtimol, `State` xususiyatida `self` ni qaytadigan 
`request_review` va `approve` metodlari uchun standart amalga oshirishlarni yaratishga harakat qilishimiz mumkin; lekin bu ob'ekt xavfsizligini buzadi, 
chunki xususiyat aniq `self` nima bo‘lishini bilmaydi. Biz `State` ni ob'ekt sifatida ishlatishimiz kerak, shuning uchun uning metodlarini ob'ekt xavfsiz 
bo‘lishi kerak.

Boshqa takrorlashlarga `Post`dagi `request_review` va `approve` metodlarining o‘xshash amalga oshirilishi kiradi. Ikkala metod ham `Option` ning `state` 
maydonidagi qiymatdagi bir xil metodni amalga oshirishga va `state` maydonining yangi qiymatini natijaga o‘rnatishga delegatsiya qiladi. Agar `Post`da 
ushbu naqshga muvofiq ko‘p metodlar mavjud bo‘lsa, takrorlanishni yo‘qotish uchun makro ta'riflashni ko‘rib chiqamiz (19-bobdagi [“Makrolar”][macros] bo
‘limini ko‘ring).

Holat shablonini ob'ektga yo‘naltirilgan tillar uchun aniq belgilanganidek amalga oshirish orqali biz Rustning kuchlarini to‘liq foydalana olmaymiz. 
Keling, `blog` crate-ga o‘zgartirishlar kiritishni ko‘rib chiqaylik, bu esa to‘g‘ri holat va o‘tishlarni kompilyatsiya vaqtida xatolarga aylantiradi.
#### Holatlarni va Xatti-harakatlarni Turlarga Kodlash

Biz sizga holat shablonini qayta o‘ylab ko‘rishni ko‘rsatamiz, bunda boshqa turdagi o‘zaro ta’sirlar olamiz. Holatlar va o‘tishlarni to‘liq yashirish o
‘rniga, ularni turli turlarga kodlaymiz. Shu tariqa, Rustning type checking tizimi draft postlar faqat nashr etilgan postlar uchun ruxsat etilgan joyda 
ishlatilishini oldini olish uchun kompilyator xatosi chiqaradi.

Keling, Ro‘yxat 17-11 da `main` ning birinchi qismiga nazar solamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:here}}


Biz hali ham `Post::new` yordamida draft holatidagi yangi postlarni yaratish imkoniyatini beramiz va postning kontentiga matn qo‘shish imkoniyatini 
saqlab qolamiz. Lekin, draft postda bo‘sh qator qaytaradigan `content` metodi bo‘lishining o‘rniga, draft postlar `content` metodiga umuman ega emas. 
Shunday qilib, agar biz draft postning kontentini olishga harakat qilsak, bu metod mavjud emasligi haqida kompilyator xatosini olamiz. Natijada, draft 
post kontentini tasodifan ishlab chiqarishda ko‘rsatishimiz imkonsiz bo‘ladi, chunki bu kod hatto kompilyatsiyadan o‘tmaydi.

Ro‘yxat 17-19 da `Post` strukturasining va `DraftPost` strukturasining ta’riflari, shuningdek, har birida metodlar ko‘rsatilgani keltiriladi:

<span class="filename">Fayl nomi: src/lib.rs</span>

rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-19/src/lib.rs}}


<span class="caption">Ro‘yxat 17-19: `content` metodi bor `Post` va `content` metodi yo‘q `DraftPost`</span>

`Post` va `DraftPost` strukturalari blog post matnini saqlaydigan maxfiy `content` maydoniga ega. Strukturalar endi `state` maydoniga ega emas, chunki 
biz holatni kodlashni struktura turlariga o‘tkaryapmiz. `Post` strukturasini nashr qilinadigan postni ifodalash uchun ishlatamiz va u `content` metodini o
‘z ichiga oladi, bu esa `content` ni qaytaradi.

Bizda hali `Post::new` funksiyasi bor, lekin u `Post` instansiyasini qaytarish o‘rniga `DraftPost` instansiyasini qaytaradi. `content` maxfiy bo‘lgani va 
`Post` ni qaytaradigan funksiyalar yo‘q bo‘lgani uchun, hozirda `Post` instansiyasini yaratish imkoni yo‘q.

`DraftPost` strukturasida `add_text` metodi mavjud, shuning uchun biz oldiniga o‘xshab `content` ga matn qo‘shishimiz mumkin, ammo `DraftPost` da 
`content` metodi aniqlanmaganini unutmang! Endi dastur barcha postlar dastlab draft postlar sifatida boshlanishini ta’minlaydi va draft postlarning 
kontenti ko‘rsatish uchun mavjud emas. Ushbu cheklovlarni aylanib o‘tishga harakat qilinadigan har qanday urinish kompilyator xatosiga olib keladi.

#### O‘tishlarni Turli Turlarga Transformatsiya Qilish Olaroq Amalga Oshirishi

Qanday qilib biz nashr etilgan postni olamiz? Draft postni nashr qilishdan oldin ko‘rib chiqish va tasdiqlash qoidalarini o‘rnatishni xohlaymiz. 
Kutilayotgan ko‘rib chiqish holatidagi post hech qanday kontentni ko‘rsatmasligi kerak. Ushbu cheklovlarni amalga oshirish uchun yana bir struktura, 
`PendingReviewPost` qo‘shamiz, `DraftPost` da `request_review` metodini `PendingReviewPost` ni qaytarishi uchun aniqlaymiz va `PendingReviewPost` da 
`Post` ga aylantiruvchi `approve` metodini aniqlaymiz, bu Ro‘yxat 17-20 da ko‘rsatiladi:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/lib.rs&lt;/span&gt;

rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-20/src/lib.rs:here}}


&lt;span class=&quot;caption&quot;&gt;Ro‘yxat 17-20: `DraftPost`da `request_review` chaqirib yaratiladigan `PendingReviewPost` va `PendingReviewPost` ni 
nashr qilingan `Post` ga aylantiruvchi `approve` metodi&lt;/span&gt;

`request_review` va `approve` metodlari `self` ni mulkiga oladi, shunday qilib `DraftPost` va `PendingReviewPost` instansiyalarini yo‘q qilib, mos 
ravishda `PendingReviewPost` va nashr qilingan `Post` ga aylantiradi. Shunday qilib, biz `request_review` chaqirgandan so‘ng hech qanday olib qoldirilgan 
`DraftPost` instansiyalariga ega bo‘lmaymiz. `PendingReviewPost` strukturasida `content` metodi aniqlanmagan, shuning uchun uning kontentini o‘qishga 
harakat qilish kompilyator xatosiga olib keladi, bu `DraftPost` bilan bir xil. Bizda `content` metodi aniqlangan nashr qilingan `Post` instansiyasini 
olishning yagona usuli `PendingReviewPost` ustida `approve` metodini chaqirishdir, va `PendingReviewPost` ni olishning yagona usuli `DraftPost` ustida 
`request_review` metodini chaqirishdir, shuning uchun biz blog postlari ish oqimini tur tizimiga kodladik.

Lekin, `main` ga ba'zi kichik o‘zgarishlar kiritishimiz kerak. `request_review` va `approve` metodlari struktura ustida chaqirilganda, o‘zgarmasdan yangi 
instansiyalarni qaytaradi, shuning uchun biz qaytarilgan instansiyalarni saqlash uchun ko‘proq `let post =` qo‘shimcha e'tiborlarga ehtiyojimiz bor. Biz 
draft va kutilayotgan ko‘rib chiqish postlarining kontentlari bo‘sh qator bo‘lmasligi haqida qoidalar belgilashimiz, shuningdek, ularga ehtiyojimiz yo‘q: 
bunday davlatlardagi postlar kontentini qo‘llashi mumkin bo‘lgan kodni kompilyatsiyadan o‘tkazishimiz mumkin emas. `main` da yangilangan kod Ro‘yxat 
17-21 da ko‘rsatilgan:

&lt;span class=&quot;filename&quot;&gt;Fayl nomi: src/main.rs&lt;/span&gt;

rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-21/src/main.rs}}

&nbsp;<span class="caption">Ro‘yxat 17-21: Blog post ish oqimining yangi amalga oshirishiga o‘tish uchun `main` ga kiritilgan o‘zgarishlar</span>

`post` ni qayta tayinlash uchun `main` da kiritgan o‘zgarishlarimiz bu amalga oshirish endi ob&#039;ektga yo‘naltirilgan holat shablonini to‘liq 
kuzatmaydi: holatlar o‘rtasidagi o‘tishlar endi to‘liq `Post` amalga oshirishida yashirilmagan. Biroq, bizning foydamiz shundaki, noaniq holatlar endi 
tur tizimi va kompilyatsiya vaqtida sodir bo‘ladigan tur tekshiruvi tufayli imkonsizdir! Bu, nashr qilinmagan postning kontentini ko‘rsatish kabi ba’zi 
xatolar ishlab chiqarishga o‘tishidan oldin aniqlanishini ta’minlaydi.

Ro‘yxat 17-21 dan keyin `blog` crate-da ushbu bo‘limning boshida taklif qilingan vazifalarni bajarishga harakat qiling va ushbu kodning ushbu 
versiyasining dizayni haqida nima deb o‘ylayotganingizni ko‘ring. Ba’zi vazifalar bu dizaynda allaqachon bajargan bo‘lishingiz mumkin.

Biz Rust ob&#039;ektga yo‘naltirilgan dizayn shablonlarini amalga oshirish qobiliyatiga ega bo‘lganini ko‘rdik, shuningdek, holatni tur tizimiga kodlash 
kabi boshqa shablonlar ham Rustda mavjud. Ushbu shablonlar turli xil o‘zaro ta’sirlarga ega. Ob&#039;ektga yo‘naltirilgan shablonlarga juda tanish bo
‘lishingiz mumkin, ammo muammoni Rustning imkoniyatlaridan foydalanish bilan qayta o‘ylash ba’zi xatolarni kompilyatsiya vaqtida oldini olish kabi 
foydalar taqdim etishi mumkin. Ob&#039;ektga yo‘naltirilgan shablonlar Rustda har doim eng yaxshi yechim bo‘lmasligi mumkin, chunki ob&#039;ektga yo
‘naltirilgan tillarda yo‘q bo‘lgan, masalan, mulk kabi ba’zi xususiyatlar mavjuddir.

## Xulosa

Bu bobni o‘qiganingizdan so‘ng, Rust ob’ektga yo‘naltirilgan tilmi yoki yo‘qligini o‘ylasangiz ham, siz endi Rustda ba'zi ob'ektga yo‘naltirilgan 
xususiyatlarni olish uchun xususiyat ob'ektlaridan foydalanishingiz mumkinligini bilasiz. Dinamik tarqatish sizning kodingizga biroz ishga tushirish 
vaqtida ishlash yaxshilanishi evaziga moslashuvchanlik beradi. Siz bu moslashuvchanlikdan ob'ektga yo‘naltirilgan shablonlarni amalga oshirish uchun 
foydalanishingiz mumkin, bu esa sizning kodingizni saqlashni osonlashtiradi. Rustda ob'ektga yo‘naltirilgan tillarda yo‘q bo‘lgan mulk kabi boshqa 
xususiyatlar ham mavjud. Ob'ektga yo‘naltirilgan shablon Rustning kuchlaridan foydalanishning eng yaxshi yo‘li har doim bo‘lmasligi mumkin, lekin bu bir 
imkoniyatdir.

Keyingi bo‘limda, Rustning ko‘plab moslashuvchanlikni ta’minlovchi yana bir xususiyati bo‘lgan shablonlarga nazar solamiz. Biz kitob davomida ularga 
qisqacha to‘xtalganmiz, lekin ularning barcha imkoniyatlarini ko‘rmaganmiz. Keling, buni amalga oshiraylik!

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch19-06-macros.html#macros
