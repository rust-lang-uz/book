# Kirish

> Eslatma: Kitobning ushbu nashri [No Starch Press][nsp]-dan bosma va elektron kitob formatida mavjud bo'lgan
> [Rust dasturlash tili][nsprust] bilan bir xil.

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

Rust haqida kirish kitobi bo'lgan *Rust dasturlash tili* ga xush kelibsiz.
Rust dasturlash tili sizga tezroq va ancha ishonchliroq bo'lgan dasturlarni yozishga yordam beradi. High-level ergonomika va low-level boshqaruv mutanosibligi ko'pincha dasturlash tili dizayniga zid keladi. Rust esa bu ziddiyatga barham beradi. Kuchli texnik imkoniyatlar va kod yozish qulayligini muvozanatlash orqali, Rust sizga an'anaviy qiyinchiliklardan xoli bo'lgan holda low-level tafsilotlarni (masalan, xotira sarfi) boshqarish imkonini beradi.

## Rust kim uchun

Rust turli sabablarga ko'ra ko'p odamlar uchun idealdir. Keling, eng muhim guruhlarning bir nechtasini ko'rib chiqaylik.

### Dasturchilar jamoalari

Rust tizim darajasidagi dasturlash bo'yicha yirik dasturchilar guruhlari o'rtasida hamkorlik qilish uchun samarali vosita ekanligini isbotlab kelmoqda. Low-level kod har xil nozik xatolarga moyil bo'lib, ko'pchilik boshqa tillarda ularni faqat keng ko'lamli testlar va tajribali dasturchilarning sinchkov tekshiruvlari orqaligina aniqlash mumkin. Rust-da esa kompilyator ushbu qiyin xatoliklar, jumladan, parallellik xatoliklari mavjud bo'lgan kodni kompilyatsiya qilishni rad etib, qorovul rolini o'ynaydi. Kompilyator yordamida, jamoa xatoliklarni quvishdan ko'ra, ko'proq vaqtini dastur mantig'iga sarflashi mumkin bo'ladi.

Rust shuningdek, tizim dasturlash dunyosiga zamonaviy dasturchilar vositalarini olib kiradi:

* Cargo - dependency menejer va build tool bo'lib, Rust ekotizimida dependency qo'shish, kompilyatsiya qilish va boshqarishni oson va pishiq qiladi.
* Rustfmt formatlash vositasi dasturchilar orasida barqaror kod yozish uslubini ta'minlaydi.
* Rust Language Server kodni toʻldirish va inline xatolik xabarlari uchun Integrated Development Environment (IDE) integratsiyasini ta'minlaydi.

Dasturchilar Rust ekotizimidagi ushbu va boshqa vositalardan foydalangan holda, tizim darajasidagi kodni yozishda samarali bo'lishi mumkin.

### Talabalar

Talabalar va tizim tushunchalarini o'rganishga qiziquvchilar uchun Rust ayni muddao. Rust-dan foydalanib, ko'pgina odamlar operatsion tizimlarni tuzish haqidagi mavzularni o'rganishgan. Jamiyat juda mehmondo'st va talabalar savollariga javob berishdan xursand. Ushbu kitobga ketgani kabi turli sa'y-harakatlar orqali Rust jamoalari tizim tushunchalarini iloji boricha ko'proq odamlarga, ayniqsa yangi dasturchilarga qulayroq qilishni xohlashadi.

### Kompaniyalar

Yuzlab yirik va kichik kompaniyalar Rust-dan CLI dasturlar, veb-xizmatlar, DevOps toollari, embedded qurilmalar, audio va video tahlillari va transkodlar, kriptovalyutalar, bioinformatika, qidiruv tizimlari, Internet of Things ilovalari, machine learning va hatto Firefox veb-brauzerining asosiy qismlarini tuzish kabi turli vazifalar uchun foydalanadilar.

### Open Source dasturchilar

Rust Rust dasturlash tilini, hamjamiyatini, dasturchilar vositalarini va kutubxonalarini yaratmoqchi bo'lgan odamlar uchundir. Rust tiliga o'z hissangizni qo'shishingizni istardik.

### Tezlik va stabillikni qadrlaydigan odamlar

Rust dasturlash tili tezlik va stabillikni xohlaydigan odamlar uchundir. Tezlik deganda biz Rust kodining ishlash tezligi-yu, Rust sizga beradigan dastur yozish tezligini nazarda tutamiz. Rust kompilyatorining tekshiruvlari stabillikni xususiyat qo'shish va refaktoring orqali ta'minlaydi. Bu esa tabiyki, bunday tekshiruvlarga ega bo'lmagan, dasturchilar ko'pincha o'zgartirishdan qo'rqadigan tillardagi xatolikka moyil koddan ko'ra yaxshiroq natija ko'rsatadi. Nol xarajatli abstraksiyalarga, ya'ni high-level kodni xuddi qo'lda yozilgandek samarali bo'lgan low-level kodga kompilyatsiya qilishga intilish orqali, Rust kodni ham xavfsiz, ham tez kod bo'lishiga harakat qiladi.

Rust tili boshqa ko'plab foydalanuvchilarni ham qo'llab-quvvatlashiga umid qiladi; Yuqorida tilga olingan guruhlar faqatgina eng katta manfaatdor guruhlardan biri hisoblanadi. Umuman olganda, Rustning eng katta maqsadi xavfsizlik *va* unumdorlik, tezlik *va* samaradorlikni birdek ta'minlash orqali dasturchilar o'nlab yillar davomida ko'nib kelishga majbur bo'lgan  muammolarni yo'q qilishdir. Rust-ni sinab ko'ring va uning tanlovlari sizga mos keladimi yoki yo'qligini tekshiring.

## Bu kitob kim uchun

Ushbu kitobda siz boshqa dasturlash tilida kod yozgansiz deb taxmin qilinadi, lekin aynan qaysi biri ekanligining ahamiyati yo'q. Biz materialni hamma uchun qulay qilishga harakat qildik. Biz dasturlash nima ekanligi yoki u haqida qanday fikr yuritish haqida ko'p gapirmaymiz. Agar siz dasturlashda mutlaqo yangi bo'lsangiz, bu kitobni o'qishdan oldin, dasturlash bilan tanishtiruvchi kitoblarni o'qishni tavsiya qilamiz.

## Ushbu kitobdan qanday foydalanish kerak

Umuman olganda, bu kitob siz uni bir boshidan ketma-ket o'qiyotganingizni taxmin qiladi. Doim Navbatdagi boblar oldingi boblardagi tushunchalarga asoslanadi va oldingi boblar ma'lum bir mavzu bo'yicha tafsilotlarni o'rganmasa ham, lekin keyingi bobda o'sha mavzuni qayta ko'rib chiqadi.

Ushbu kitobda siz ikki xil bobni topasiz: kontseptsiya boblari va loyiha bo'limlari. Kontseptsiya boblarida siz Rustni nazariy jihatdan o'rganasiz. Loyiha boblarida esa biz hozirgacha o'rganganlaringizni qo'llagan holda kichik dasturlarni birgalikda tuzamiz. 2, 12 va 20-boblar loyiha boblari; qolganlari esa kontseptsiya boblaridir.

1-bobda Rustni qanday o'rnatish, "Hello, world!" dasturi va Cargo, Rust paket menejeri va build tooldan qanday foydalanishni ko'rib chiqamiz. 2-bob Rustda dastur yozish bo'yicha amaliy mashg'ulot bo'lib, siz raqamlarni taxmin qilish o'yinini tuzasiz. Bu yerda biz tushunchalarni sayoz yoritamiz va navbatdagi boblarda chuqurroq tafsilotlar bilan ulashamiz. Agar siz darhol yenglarni shimarmoqchi bo'lsangiz, 2-bob buning uchun ayni muddao. 3-bobda boshqa dasturlash tillariga oʻxshash Rust funksiyalari, 4-bobda esa Rustning ownershp tizimi haqida bilib olasiz. Agar siz keyingisiga o‘tishdan oldin har bir tafsilotni chuqur o‘rganishni afzal ko‘radigan, ya'ni sinchkov o‘quvchi bo‘lsangiz, 2-bobni o‘tkazib yuborib, to‘g‘ridan-to‘g‘ri 3-bobga o‘tishingiz va o'rganganlaringiz loyiha ustida sinab ko'rishni hohlaganingizda 2-bobga qaytishingizni tavsiya qilamiz.

5-bobda structlar va metodlar muhokama qilinadi, 6-bob esa enumlar, `match` expressionlari va `if let` control flow konstruksiyasini qamrab oladi. Rust-da maxsus turlarni yaratish uchun struclar va enumlardan foydalanasiz.

7-bobda siz Rust modul tizimi va uning amaliy dasturlash interfeysini (API) va kodingizni tartiblashdagi maxfiylik qoidalari haqida bilib olasiz. 8-bobda standart kutubxona taqdim etadigan vektorlar, stringlar va hash maplar kabi umumiy ma'lumotlar tuzilmalari muhokama qilinadi. 9-bob Rustning xatolarni hal qilish falsafasi va usullarini o'rganadi.

10-bob generiklar, traitlar va lifetimeni o'rganadi, bular sizga bir nechta turlar bilan ishlay oladigan kod yozish imkonini beradi. 11-bob faqat testlar haqida, unutmang, hatto Rustning xavfsizlik kafolat bersa ham, dasturingiz mantig'ining to'g'riligini ta'minlash uchun testlar zarurdir. 12-bobda biz fayllar ichidagi matnni qidiradigan `grep` vositasidan o'z funksiyalar to'plamimizni yaratamiz. Buning uchun biz oldingi boblarda muhokama qilingan ko'plab tushunchalardan foydalanamiz.

13-bob closure va iteratorlarni o'rganadi: Rustga funktsional dasturlash tillaridan kirib kelgan xususiyatlar. 14-bobda biz Cargoni chuqurroq ko'rib chiqamiz va o'z kutubxonalaringizni boshqalar bilan ulashishning eng yaxshi amaliyotlari haqida gaplashamiz.
15-bobda standart kutubxona taqdim etadigan smart pointerlar va ularning funksionalligini ta'minlaydigan traitlar muhokama qilinadi.

16-bobda concurrent dasturlashning turli modellarini ko'rib chiqamiz va Rust sizga bir nechta thread-larda qo'rqmasdan dasturlashda qanday yordam berishi haqida gaplashamiz.
17-bobda esa Rust idiomlari obyektga yo'naltirilgan dasturlash(OOP) tamoyillari bilan qanday farq qilishini ko'rib chiqamiz.

18-bobda Rust dasturlari bo'ylab g'oyalarni ifodalashning kuchli usullari bo'lgan patternlar va patternlarni Rustga moslashtirish haqida ma'lumot berilgan. 19-bobda ilg'or qiziqarli mavzular, jumladan xavfli Rust, makroslar va boshqa ko'p narsalar mavjud.

20-bobda biz low-level ko'p oqimli veb-server tuzamiz!

Quyidagi ba'zi ilovalarda til haqida anchagina foydali ma'lumotlar mavjud. A ilovasida Rustdagi kalit so'zlari, B ilovasida Rust operatorlari va belgilari, C ilovasi standart kutubxona tomonidan taqdim etilgan traitlarini, D ilovasi esa ba'zi foydali ishlab chiqish vositalarini qamrab oladi va E ilovasida Rust nashrlari tushuntiriladi. F ilovasida siz kitobning tarjimalarini topishingiz mumkin, G ilovasida esa Rust qanday yaratilgani va  nightly Rust nima ekanligini ko'rib chiqamiz.

Ushbu kitobni o'qishning noto'g'ri usuli yo'q: Keyingi mavzuga o'tmoqchimisiz? Bosing! Mabodo yo'lda chalkashliklar uchrasa, avvalgi boblarga qaytish kerak bo'lishi mumkin. Xullas, sizga qaysi qulay bo'lsa, shuni qiling.

<span id="ferris"></span>

Rustni o'rganish jarayonining muhim qismi kompilyator ko'rsatadigan xato xabarlarini o'qishni o'rganishdir: ular sizni to'g'ri kodga boshlaydi.
Kitobda, biz kompilyator har bir vaziyatda sizga ko'rsatadigan xato xabari bilan birga kompilyatsiya qilinmaydigan ko'plab misollarni keltiramiz. Bilingki, agar siz kitobdan tasodifiy misolni kiritsangiz va ishlatsangiz, u kompilyatsiya qilinmasligi mumkin! Ba'zi misollar ataylab xatolik chiqazishga mo'ljallanganligini bilish uchun uni atrofidagi matnlarni o'qing. Ferris, sizga xatolik uchun mo'ljallangan kodni ajratishga yordam beradi:

| Ferris                                                                                                           | Ma'nosi                                         |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | Bu kod kompilyatsiya qilinmaydi!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | Bu kod panic!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | Ushbu kod kerakli xatti-harakatni keltirib chiqarmaydi. |

Aksariyat hollarda biz sizni kompilyatsiya qilinmagan har qanday kodning to'g'ri versiyasi bilan ta'minlaymiz.

## Manba kodi

Ushbu kitob manba fayllarini [GitHub][book]da topishingiz mumkin.

[book]: https://github.com/rust-lang/book/tree/main/src
