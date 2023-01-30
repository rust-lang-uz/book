# Kirish

> Eslatma: Kitobning ushbu nashri [No Starch Press][nsp]-dan bosma va elektron kitob formatida mavjud bo'lgan
> [Rust dasturlash tili][nsprust] bilan bir xil.

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

*Rust dasturlash tili*ga xush kelibsiz, Rust haqida kirish kitobi.
Rust dasturlash tili tezroq va ishonchli dasturlarni yozishga yordam beradi.
Yuqori darajadagi samaradorlik va low-leveldagi boshqaruv ko'pincha dasturlash tilini loyihalashda bir-biriga zid keladi; Rust bu ziddiyatga qarshi turadi. Kuchli texnik imkoniyatlar va ishlab chiquvchilarning ajoyib tajribasini muvozanatlash orqali Rust sizga an'anaviy ravishda bunday nazorat bilan bog'liq bo'lgan barcha qiyinchiliklarsiz low-leveldagi tafsilotlarni (masalan, xotiradan foydalanish) boshqarish imkoniyatini beradi.

## Rust kim uchun

Rust turli sabablarga ko'ra ko'p odamlar uchun idealdir. Keling, eng muhim guruhlarning bir nechtasini ko'rib chiqaylik.

### Dasturchilar jamoalari

Rust turli darajadagi tizimlarni dasturlash bo'yicha bilimga ega bo'lgan yirik ishlab chiquvchilar guruhlari o'rtasida hamkorlik qilish uchun samarali vosita ekanligini isbotlamoqda. Low-leveldagi kod turli xil nozik xatolarga moyil bo'lib, ko'pchilik boshqa tillarda ularni faqat keng ko'lamli sinov va tajribali ishlab chiquvchilar tomonidan sinchkovlik bilan tekshirish orqali aniqlash mumkin.Rust-da kompilyator ushbu qiyin xatolar, jumladan, parallellik xatolari bilan kodni kompilyatsiya qilishni rad etib, darvozabon rolini o'ynaydi. Kompilyator bilan birga ishlash orqali jamoa xatolarni ta'qib qilishdan ko'ra, vaqtini dastur mantig'iga qaratishga sarflashi mumkin.

Rust shuningdek, tizim dasturlash dunyosiga zamonaviy ishlab chiquvchilar vositalarini olib keladi:

* Cargo  dependency menejeri va build toolni o'z ichiga oladi, Rust ekotizimida bog'liqliklarni qo'shish, kompilyatsiya qilish va boshqarishni qiyinchiliksiz va davomli qiladi.
* Rustfmt formatlash vositasi ishlab chiquvchilar orasida barqaror kodlash uslubini ta'minlaydi.
* Rust Language Server kodni toʻldirish va inline xato xabarlari uchun Integrated Development Environment (IDE) integratsiyasini quvvatlaydi.

Rust ekotizimidagi ushbu va boshqa vositalardan foydalangan holda, ishlab chiquvchilar tizim darajasidagi kodni yozishda samarali bo'lishi mumkin.

### Talabalar

Rust talabalar va tizim tushunchalarini o'rganishga qiziquvchilar uchun. Rust-dan foydalanib, ko'p odamlar operatsion tizimlarni ishlab chiqish kabi mavzular haqida bilib oldilar. Jamiyat juda mehmondo'st va talabalar savollariga javob berishdan xursand. Ushbu kitob kabi sa'y-harakatlar orqali Rust guruhlari tizim tushunchalarini ko'proq odamlar, ayniqsa dasturlash uchun yangi bo'lganlar uchun qulayroq qilishni xohlashadi.

### Kompaniyalar

Yuzlab yirik va kichik kompaniyalar ishlab chiqarishda Rust-dan CLI dasturlar, veb-xizmatlar, DevOps toollari, embedded qurilmalar, audio va video tahlillari va transkodlar, kriptovalyutalar, bioinformatika, qidiruv tizimlari, Internet of Things ilovalari kabi turli vazifalar uchun foydalanadilar. , machine learning va hatto Firefox veb-brauzerining asosiy qismlari.

### Open Source dasturchilar

Rust Rust dasturlash tilini, hamjamiyatini, ishlab chiquvchilar vositalarini va kutubxonalarini yaratmoqchi bo'lgan odamlar uchundir. Rust tiliga o'z hissangizni qo'shishingizni istardik.

### Tezlik va barqarorlikni qadrlaydigan odamlar

Rust dasturlash tili tezlik va barqarorlikni xohlaydigan odamlar uchundir. Tezlik deganda biz Rust kodi qanchalik tez ishlashini va Rust sizga dasturlar yozish imkonini beradigan tezligini nazarda tutamiz. Rust kompilyatorining tekshiruvlari qo'shimcha funksiyalar va refaktoring orqali barqarorlikni ta'minlaydi. Bu ishlab chiquvchilar ko'pincha o'zgartirishdan qo'rqadigan ushbu tekshiruvlarsiz tillardagi mo'rt eski koddan farqli o'laroq. Nol xarajatli abstraktsiyalarga, qo'lda yozilgan kod kabi tezroq lower-leveldagi kodni kompilyatsiya qiladigan higher-leveldagi funktsiyalarga intilish orqali Rust xavfsiz kodni ham tezkor kod qilishga intiladi.

Rust tili boshqa ko'plab foydalanuvchilarni ham qo'llab-quvvatlashga umid qiladi; Bu yerda tilga olinganlar faqat eng katta manfaatdor tomonlardan biri hisoblanadi. Umuman olganda, Rustning eng katta ambitsiyalari xavfsizlik *va* unumdorlik, tezlik *va* samaradorlikni ta'minlash orqali dasturchilar o'nlab yillar davomida qabul qilgan kelishuvlarni yo'q qilishdir. Rust-ni sinab ko'ring va uning tanlovlari sizga mos keladimi yoki yo'qligini tekshiring.

## Bu kitob kim uchun

Ushbu kitobda siz boshqa dasturlash tilida kod yozgansiz deb taxmin qilinadi, lekin qaysi biri haqida hech qanday taxminlar yo'q. Biz materialni turli xil dasturlash tajribasiga ega bo'lganlar uchun keng foydalanishga harakat qildik. Biz dasturlash nima ekanligi yoki u haqida qanday fikr yuritish haqida gapirishga ko'p vaqt sarflamaymiz. Agar siz dasturlashda mutlaqo yangi bo'lsangiz, dasturlash bilan tanishishni ta'minlaydigan kitobni o'qisangiz yaxshi bo'lardi.

## Ushbu kitobdan qanday foydalanish kerak

Umuman olganda, bu kitob siz uni oldindan orqaga ketma-ket o'qiyotganingizni taxmin qiladi. Keyingi boblar oldingi boblardagi tushunchalarga asoslanadi va oldingi boblar ma'lum bir mavzu bo'yicha tafsilotlarni o'rganmasligi mumkin, lekin keyingi bobda mavzuni qayta ko'rib chiqadi.

Ushbu kitobda siz ikki xil bo'limni topasiz: kontseptsiya bo'limlari va loyiha bo'limlari. Kontseptsiya boblarida siz Rustning bir tomoni haqida bilib olasiz. Loyiha bo'limlarida biz hozirgacha o'rganganlaringizni qo'llagan holda kichik dasturlarni birgalikda tuzamiz. 2, 12 va 20-boblar loyiha boblari; qolganlari kontseptsiya boblari.

1-bobda Rustni qanday o'rnatish, "Hello, world!" dasturi va Cargo, Rust paket menejeri va build tooldan qanday foydalanishni ko'rib chiqamiz. 2-bob Rustda dastur yozish bo'yicha amaliy kirish bo'lib, siz raqamlarni taxmin qilish o'yinini tuzasiz. Bu yerda biz tushunchalarni yuqori darajada yoritamiz va keyingi boblarda qo'shimcha tafsilotlar beriladi. Agar siz darhol qo'llaringizni ifloslantirmoqchi bo'lsangiz, 2-bob buning uchun joy. 3-bobda boshqa dasturlash tillariga oʻxshash Rust funksiyalari yoritilgan va 4-bobda siz Rustning ownershp tizimi haqida bilib olasiz. Agar siz keyingisiga o‘tishdan oldin har bir tafsilotni o‘rganishni ma’qul ko‘radigan, ayniqsa sinchkov o‘quvchi bo‘lsangiz, 2-bobni o‘tkazib yuborib, to‘g‘ridan-to‘g‘ri 3-bobga o‘tishingiz va loyiha ustida ishlashni hohlaganingizda 2-bobga qaytishingiz mumkin. siz o'rgangan tafsilotlar.

5-bobda structlar va methodlar muhokama qilinadi, 6-bob esa enumlar, `match` expressionlari va `if let` control flow konstruksiyasini qamrab oladi. Rust-da maxsus turlarni yaratish uchun struclar va enumlardan foydalanasiz.

7-bobda siz Rust modul tizimi va kodingizni va uning umumiy amaliy dasturlash interfeysini (API) tashkil qilish uchun maxfiylik qoidalari haqida bilib olasiz. 8-bobda standart kutubxona taqdim etadigan vektorlar, stringlar va hash maplar kabi umumiy yig'ish ma'lumotlar tuzilmalari muhokama qilinadi. 9-bob Rustning xatolarni hal qilish falsafasi va usullarini o'rganadi.

10-bob generiklar, traitlar va lifetimeni o'rganadi, bu sizga bir nechta turlarga tegishli kodni aniqlash imkoniyatini beradi. 11-bob sinovdan o'tadi, bu hatto Rustning xavfsizlik kafolatlari bilan ham dasturingiz mantig'ining to'g'riligini ta'minlash uchun zarurdir. 12-bobda biz fayllar ichidagi matnni qidiradigan `grep` buyruq qatori vositasidan o'zimizning funksiyalar to'plamini yaratamiz. Buning uchun biz oldingi boblarda muhokama qilgan ko'plab tushunchalardan foydalanamiz.

13-bob yopilishlar va iteratorlarni o'rganadi: Rustning funktsional dasturlash tillaridan kelib chiqadigan xususiyatlari. 14-bobda biz Cargolarni chuqurroq ko'rib chiqamiz va kutubxonalaringizni boshqalar bilan baham ko'rishning eng yaxshi amaliyotlari haqida gaplashamiz.
15-bobda standart kutubxona taqdim etadigan smart pointerlar va ularning funksionalligini ta'minlaydigan traitlar muhokama qilinadi.

16-bobda biz bir vaqtning o'zida dasturlashning turli modellarini ko'rib chiqamiz va Rust sizga bir nechta mavzularda qo'rqmasdan dasturlashda qanday yordam berishi haqida gaplashamiz.
17-bobda Rust idiomlari sizga tanish bo'lishi mumkin bo'lgan obyektga yo'naltirilgan(OOP) dasturlash tamoyillari bilan qanday taqqoslanishi ko'rib chiqiladi.

18-bobda Rust dasturlari bo'ylab g'oyalarni ifodalashning kuchli usullari bo'lgan patternlar va patternlarni moslashtirish haqida ma'lumot berilgan. 19-bobda ilg'or qiziqarli mavzular, jumladan xavfli Rust, makroslar va boshqa ko'p narsalar mavjud.

20-bobda biz low-leveldagi ko'p tarmoqli veb-serverni amalga oshiradigan loyihani yakunlaymiz!

Va nihoyat, ba'zi qo'shimchalarda til haqida foydali ma'lumotlar ko'proq mos yozuvlar formatida mavjud. A ilovasida Rustning kalit so'zlari, B ilovasida Rust operatorlari va belgilari, C ilovasi standart kutubxona tomonidan taqdim etilgan hosila traitlarini o'z ichiga oladi, D ilovasi ba'zi foydali ishlab chiqish vositalarini qamrab oladi va E ilovasida Rust nashrlari tushuntiriladi. F ilovasida siz kitobning tarjimalarini topishingiz mumkin, G ilovasida esa Rust qanday qilinganligi va  nightlyli Rust nima ekanligini ko'rib chiqamiz.

Ushbu kitobni o'qishning noto'g'ri usuli yo'q: agar siz oldinga o'tmoqchi bo'lsangiz, unga boring! Agar chalkashliklarga duch kelsangiz, avvalgi boblarga qaytishingiz kerak bo'lishi mumkin. Lekin siz uchun nima ish qilsa, shuni qiling.

<span id="ferris"></span>

Rustni o'rganish jarayonining muhim qismi kompilyator ko'rsatadigan xato xabarlarini o'qishni o'rganishdir: ular sizni ish kodiga yo'naltiradi.
Shunday qilib, biz kompilyator har bir vaziyatda sizga ko'rsatadigan xato xabari bilan birga kompilyatsiya qilinmaydigan ko'plab misollarni keltiramiz. Bilingki, agar siz tasodifiy misol kiritsangiz va ishlatsangiz, u kompilyatsiya qilinmasligi mumkin! Ishlamoqchi bo'lgan misol xato uchun mo'ljallanganligini bilish uchun atrofdagi matnni o'qiganingizga ishonch hosil qiling. Ferris, shuningdek, ishlash uchun mo'ljallanmagan kodni ajratishga yordam beradi:

| Ferris                                                                                                           | Ma'nosi                                         |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | Bu kod kompilyatsiya qilinmaydi!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | Bu kod panic!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | Ushbu kod kerakli xatti-harakatni keltirib chiqarmaydi. |

Aksariyat hollarda biz sizni kompilyatsiya qilinmagan har qanday kodning to'g'ri versiyasiga olib boramiz.

## Manba kodi

Ushbu kitob yaratilgan manba fayllarni [GitHub][book]da topish mumkin.

[book]: https://github.com/rust-lang/book/tree/main/src
