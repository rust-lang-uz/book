## Kengaytirilgan parallellik (konkurensiya) bilan `Sinxronlash`(Sync) va  `Jonatish`(Send) xususiyatlari

Qiziqroq qilib aytganda, Rust tilida juda kam konkurentlik talabiga javob bera oladigan xususiyatlar bor. Biz shu vaqtgacha, avvalgi bolimlarda korib
chiqqan deyarli barcha xususiyatlar standard kutubxonaga tegishli xususiyatlar hisoblanadi, tilga(Rustga) emas. Sizning parallellik(konkurensiya) bilan
musobatingiz Rust tiliga yoki standardlashtirilgan kutubxonaga cheklanmagan. Siz oziznikini yozishingiz yoki boshqalar tomonidan yozilganini olishingiz
mumkin.

Ammo, tilga 2 ta parallellik konsepsiya(metodologiya, tushuncha) lari kiritilgan:
`std::marker`  xususiyatlari `Sync`(sinxronlash) va `Send`(jonatish).

### `Send`(Jonatish) orqali egalikni Threadlarga kochirish

`Send`(Yuborish) belgisi xususiyati Jonatishni amalga oshiruvchi turdagi qiymatlarga egalik huquqini threadlar o'rtasida o'tkazish mumkinligini
ko'rsatadi. Deyarli har bir Rust turi `Send`(Yuborish), lekin ba'zi istisnolar mavjud, masalan `Rc<T>`: buni `Send`(Yuborish) bo’lmaydi, chunki agar siz
`Rc<T>` qiymatini klonlashtirsangiz va klonga egalik huquqini boshqa threadga o'tkazmoqchi bo'lsangiz, ikkala threadlar ham reference sonini yangilishi
mumkin. Shu sabablisiz i `Rc<T>`  bitta oqimli holatlarda foydalanish uchun amalga oshiriladi qayerdaki siz jarima (thread-xavfsizligi uchun) tolashni
istamasangiz.

Shu sababli, Rustning turga asoslangan tizimi va belgilar chegaralari hech qachon tasodifan `Rc<T>` qiymatini threadlar bo'ylab xavfsiz tarzda yubora
olmasligingizni ta'minlaydi. Buni 16-14-raqamli roʻyxatda qilishga urinib koʻrganimizda, Rc<Mutex<i32>> uchun Jo’natish(Send)  xususiyati qoʻllanilmagan
degan xatoga duch keldik. `Arc<T>`-ga, ya'ni `Send`-g(Jonatishga)a o'tganimizda, kod tuzildi.

To'liq `Send`(Yuborish) turlaridan tashkil topgan har qanday tur avtomatik ravishda `Send`(Yuborish) sifatida ham belgilanadi. Deyarli barcha ibtidoiy
turlar, biz 19-bobda muhokama qiladigan xom(primitive) ko'rsatkichlardan tashqari, `Send`(Yuborishdir).

### `Sync`(Sinxronizatsiya) bilan bir nechta mavzulardan kirishga ruxsat berish

`Sync`(Sinxronlash) belgisi xususiyati Sinxronizatsiyani amalga oshiruvchi turga bir nechta oqimlardan havola qilish xavfsiz ekanligini ko'rsatadi.
Boshqacha qilib aytganda, agar `&T` (T ga o'zgarmas havola) Send bo'lsa, har qanday `T` turi Sinxronizatsiya hisoblanadi, ya'ni havola boshqa oqimga
xavfsiz yuborilishi mumkin. Yuborishga o'xshab, primitive turlar Sync(Sinxrondir) va to'liq `Sync`(Sinxronlashtirilgan) turlardan tashkil topgan turlar
ham `Sync`(Sinxrondir).

`Rc<T>` aqlli ko'rsatkichi ham xuddi shu sabablarga ko'ra `Sync`(Sinxronlashtirilmaydi), chunki u Send(Yuborish) emas. `RefCell<T>` turi (bu haqda biz
15-bobda gaplashdik) va tegishli `Cell<T>` turlari oilasi `Sync`(Sinxronlash) emas. `RefCell<T>` ish vaqtida bajaradigan qarzni tekshirishni amalga
oshirish thread uchun xavfsiz emas. `Mutex<T>` aqlli ko'rsatkichi `Sync`(Sinxronlashdir) va Bir nechta mavzular o'rtasida `Mutex<T>` almashish bo'limida
ko'rganingizdek, bir nechta oqimlar bilan kirishni almashish uchun ishlatilishi mumkin.

### `Send`(Yuborish) va `Sync`(Sinxronizatsiya) ni qo'lda amalga oshirish xavfli

Send(Yuborish) va Sync(Sinxronlash) xususiyatlaridan tashkil topgan turlar avtomatik ravishda Send(Yuborish) va Sync(Sinxronlash) xususiyatiga ega
boʻlgani uchun biz bu xususiyatlarni qoʻlda amalga oshirishimiz shart emas. Belgilovchi xususiyatlar sifatida ular hatto amalga oshirish uchun hech
qanday usullarga ega emaslar. Ular parallellik bilan bog'liq invariantlarni qo'llash uchun foydalidir.

Ushbu xususiyatlarni qo'lda amalga oshirish xavfli Rust kodini amalga oshirishni o'z ichiga oladi. 19-bobda xavfli Rust kodidan foydalanish haqida
gaplashamiz; Hozircha muhim ma'lumot shundaki, `Send`(Yuborish) va `Sync`(Sinxronlash) qismlaridan iborat bo'lmagan yangi bir vaqtda turlarni yaratish
xavfsizlik kafolatlarini ta'minlash uchun ehtiyotkorlik bilan o'ylashni talab qiladi. "Rustonomicon" ushbu kafolatlar va ularni qanday saqlash kerakligi
haqida ko'proq ma'lumotga ega ular.

## Xulosa

Bu siz ushbu kitobda mos kelishini ko'rgan oxirgi narsa emas: 20-bobdagi loyiha bu bobdagi tushunchalardan bu erda muhokama qilingan kichikroq misollarga
qaraganda realroq vaziyatda foydalanadi.

Yuqorida aytib o'tganimizdek, Rustning parallellikni qanday boshqarishi tilning bir qismi bo'lganligi sababli, ko'plab parallellik echimlari qutilar
sifatida amalga oshiriladi. Ular standart kutubxonaga qaraganda tezroq rivojlanadi, shuning uchun ko'p tarmoqli vaziyatlarda foydalanish uchun joriy, eng
zamonaviy qutilarni onlayn qidirib toping.

Rust standart kutubxonasi bir vaqtda kontekstlarda foydalanish uchun xavfsiz bo'lgan `Mutex<T>` va `Arc<T>` kabi xabarlarni uzatish va aqlli ko'rsatkich
turlari uchun kanallarni taqdim etadi. Tur tizimi va qarz tekshiruvi ushbu echimlardan foydalanadigan kod ma'lumotlar poygasi yoki noto'g'ri havolalar
bilan yakunlanmasligini ta'minlaydi. Kodingizni kompilyatsiya qilish uchun olganingizdan so'ng, u boshqa tillarda tez-tez uchraydigan kuzatilishi qiyin
bo'lgan xatolarsiz bir nechta mavzularda baxtli ishlashiga amin bo'lishingiz mumkin. Bir vaqtning o'zida dasturlash endi qo'rqadigan tushuncha emas:
oldinga boring va dasturlaringizni qo'rqmasdan bir vaqtda qiling!

Keyinchalik, Rust dasturlaringiz kattalashib borishi bilan muammolarni modellashtirish va echimlarni tuzishning idiomatik usullari haqida gaplashamiz.
Bundan tashqari, biz Rustning idiomalari sizga object-oriented programming(ob'ektga yo'naltirilgan dasturlashdan) tanish bo'lgan narsalar bilan qanday
bog'liqligini muhokama qilamiz.

[sharing-a-mutext-between-multiple-threads]:
ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads
[nomicon]: ../nomicon/index.html

[sharing-a-mutext-between-multiple-threads]:
ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads
[nomicon]: ../nomicon/index.html
