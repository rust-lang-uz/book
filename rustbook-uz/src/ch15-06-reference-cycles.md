## Yo'naltiruvchi tsikllar xotirani oqishi mumkin

Rustning xotira xavfsizligi kafolatlari buni qiyinlashtiradi, lekin imkonsiz emas
tasodifan hech qachon tozalanmaydigan xotira yaratish (*xotira oqish* deb nomlanadi).
Xotiraning oqishi to'liq oldini olish Rustning kafolatlaridan biri emas, ya'ni
xotira sızıntıları Rust-da xotira xavfsizdir. Rust xotira oqishiga ruxsat berishini ko'rishimiz mumkin
`Rc<T>` va `RefCell<T>` dan foydalanib: bu yerda havolalar yaratish mumkin.
elementlar siklda bir-biriga ishora qiladi. Bu xotira oqishini yaratadi, chunki
tsikldagi har bir elementning mos yozuvlar soni hech qachon 0 ga etib bormaydi va qiymatlar
hech qachon tashlab ketilmaydi.

### Malumot siklini yaratish

Keling, mos yozuvlar sikli qanday sodir bo'lishi mumkinligini va uni qanday oldini olishni ko'rib chiqaylik,
Listingdagi `List` enum va `tail` usulining ta`rifidan boshlab
15-25:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

<span class="caption">Listing 15-25: A cons list definition that holds a
`RefCell<T>` so we can modify what a `Cons` variant is referring to</span>

Biz 15-5 roʻyxatdagi “Roʻyxat” taʼrifining boshqa variantidan foydalanmoqdamiz. The
`Cons` variantidagi ikkinchi element endi `RefCell<Rc<List>>`, ya`ni
Biz Listingda bo'lgani kabi "i32" qiymatini o'zgartirish imkoniyatiga ega bo'lish o'rniga
15-24, biz 'Kasalliklar' varianti ko'rsatayotgan 'Ro'yxat' qiymatini o'zgartirmoqchimiz.
Bizga kirishni qulay qilish uchun biz "quyruq" usulini ham qo'shmoqdamiz
ikkinchi element, agar bizda "Kasalliklar" varianti bo'lsa.

15-26 roʻyxatda biz “asosiy” funksiyani qoʻshmoqdamiz.
Ro'yxat 15-25. Bu kod `a` ro`yxatini va `b` ga ishora qiluvchi ro`yxatni yaratadi
`a` ichidagi ro'yxat. Keyin u `a` ro`yxatini `b` ga ishora qilib o`zgartiradi va a ni yaratadi
mos yozuvlar aylanishi. Yo'lda nima ekanligini ko'rsatish uchun `println!' iboralari mavjud
mos yozuvlar soni bu jarayonning turli nuqtalarida.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

<span class="caption">Listing 15-26: Creating a reference cycle of two `List`
values pointing to each other</span>

Biz “a” o‘zgaruvchisida “Ro‘yxat” qiymatiga ega “Rc<List>” misolini yaratamiz.
boshlang'ich ro'yxati "5, Nil" bilan. Keyin biz `Rc<List>` misol xoldingini yaratamiz
10 qiymati va nuqtalarni o'z ichiga olgan "b" o'zgaruvchisidagi boshqa "Ro'yxat" qiymati
`a` ro'yxatiga.

Biz "a" ni o'zgartiramiz, shuning uchun u "Nil" o'rniga "b" ga ishora qiladi va sikl hosil qiladi. Biz qilamiz
`RefCell<Rc<List>>` ga havola olish uchun `tail` usuli yordamida
"a" da, biz "link" o'zgaruvchisini qo'yamiz. Keyin biz "borrow_mut" dan foydalanamiz
`RefCell<Rc<List>>` ichidagi qiymatni `Rc<List>`dan oʻzgartirish uchun usul
Bu `b` dagi `Rc<List>` uchun `Nil` qiymatiga ega.

Ushbu kodni ishga tushirganimizda, oxirgi `println!` uchun izoh berilgan
lahzada biz ushbu natijani olamiz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

`a` va `b`dagi `Rc<List>` misollarining mos yozuvlar soni 2 dan keyin
biz `a` ro`yxatini `b` ga ishora qilish uchun o`zgartiramiz. "Asosiy" oxirida Rust ni tushiradi
`b` o'zgaruvchisi, bu `b` `Rc<List>` misolining mos yozuvlar sonini kamaytiradi
2 dan 1 gacha. `Rc<List>` to'plamida bo'lgan xotira o'chirilmaydi.
bu nuqta, chunki uning mos yozuvlar soni 0 emas, 1. Keyin Rust `a` tushiradi, qaysi
`a` `Rc<List>` misolining mos yozuvlar sonini 2 dan 1 gacha kamaytiradi
yaxshi. Bu misolning xotirasini ham tashlab bo'lmaydi, chunki boshqasi
`Rc<List>` misoli hali ham unga ishora qiladi. Ro'yxatga ajratilgan xotira bo'ladi
abadiy yig'ilmagan qoladi. Ushbu mos yozuvlar siklini tasavvur qilish uchun biz yaratdik
15-4-rasmdagi diagramma.

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figure 15-4: A reference cycle of lists `a` and `b`
pointing to each other</span>

Agar siz oxirgi `println!`-ni izohdan olib tashlasangiz va dasturni ishga tushirsangiz, Rust bunga harakat qiladi
bu siklni `a` bilan `b` `a` ga ishora qilib va ​​shunga o`xshash davom etguncha chop eting
stekni to'ldirib yuboradi.

Haqiqiy dunyo dasturi bilan taqqoslaganda, oqibatlar mos yozuvlar aylanishini yaratadi
bu misolda unchalik dahshatli emas: biz mos yozuvlar siklini yaratganimizdan so'ng,
dastur tugaydi. Ammo, agar murakkabroq dasturda ko'p xotira ajratilgan bo'lsa
siklda va uni uzoq vaqt ushlab tursa, dastur ko'proq xotiradan foydalanadi
kerak bo'lganidan ko'ra va tizimni to'sib qo'yishi mumkin, bu esa uning tugashiga olib keladi
mavjud xotira.

Malumot davrlarini yaratish oson emas, lekin bu ham imkonsiz emas.
Agar sizda `Rc<T>` qiymatlari yoki shunga o'xshash ichki o'rnatilgan `RefCell<T>` qiymatlari mavjud bo'lsa
ichki o'zgaruvchanlik va mos yozuvlar hisoblash bilan turlarning kombinatsiyasi, siz kerak
tsikllarni yaratmasligingizga ishonch hosil qiling; ularni qo'lga olish uchun siz Rustga tayanolmaysiz.
Malumot siklini yaratish dasturingizdagi mantiqiy xato bo'lishi mumkin
avtomatlashtirilgan testlar, kodlarni ko'rib chiqish va boshqa dasturiy ta'minotni ishlab chiqish amaliyotlaridan foydalaning
minimallashtirish.

Malumot davrlarini oldini olishning yana bir yechimi maʼlumotlaringizni qayta tashkil etishdir
tuzilmalar shunday qilib, ba'zi havolalar egalik huquqini bildiradi, ba'zi havolalar esa bildirmaydi.
Natijada, siz ba'zi egalik munosabatlaridan tashkil topgan davrlarga ega bo'lishingiz mumkin va
ba'zi mulkiy bo'lmagan munosabatlar va faqat mulkchilik munosabatlari ta'sir qiladi
qiymat tushirilishi mumkinmi yoki yo'qmi. 15-25 ro'yxatda biz har doim "Kasalliklar" ni xohlaymiz
o'z ro'yxatiga egalik qilish variantlari mavjud, shuning uchun ma'lumotlar strukturasini qayta tashkil qilish mumkin emas.
Keling, ota-ona va tugunlardan tashkil topgan grafiklardan foydalangan holda misolni ko'rib chiqaylik
egalik bo'lmagan munosabatlar qachon oldini olish uchun to'g'ri yo'l ekanligini ko'rish
mos yozuvlar davrlari.

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

Hozirgacha biz "Rc::clone" ni chaqirish ko'rsatkichni oshirishini ko'rsatdik
`Rc<T>` misolining `kuchli_hisobchasi` va `Rc<T>` misoli faqat tozalanadi
yuqoriga, agar uning "kuchli_hisoblash" qiymati 0 bo'lsa. Shuningdek, "kuchli_hisob" ga * zaif havola* yaratishingiz mumkin
`Rc<T>` misolidagi qiymatni `Rc::downgrade` deb chaqirish va
`Rc<T>` ga havola. Kuchli havolalar egalik huquqini baham ko'rishingiz mumkin
`Rc<T>` misoli. Zaif havolalar egalik munosabatlarini bildirmaydi,
va `Rc<T>` namunasi tozalanganda ularning soni ta'sir qilmaydi. Ular
mos yozuvlar aylanishiga olib kelmaydi, chunki ba'zi zaif havolalarni o'z ichiga olgan har qanday tsikl
jalb qilingan qiymatlarning kuchli mos yozuvlar soni 0 bo'lsa, buziladi.

`Rc::downgrade` ga qo`ng`iroq qilganingizda, siz `Zaif<T>` tipidagi aqlli ko`rsatgichga ega bo`lasiz.
`Rc<T>` misolidagi `strong_count` ni 1 ga oshirish o`rniga,
`Rc::downgrade` `zaif_hisobni` 1 ga oshiradi. `Rc<T>` turi foydalanadi
Qancha `Zaif<T>` havolalari mavjudligini kuzatish uchun `zaif_hisob`
`kuchli_hisob`. Farqi shundaki, "zaif_hisob" uchun 0 bo'lishi shart emas
`Rc<T>` namunasi tozalanadi.

Chunki `Zaif<T>` havola qiladigan qiymat oʻchirilgan boʻlishi mumkin
`Zaif<T>` ko'rsatayotgan qiymatga ega bo'lgan har qanday narsaga ishonch hosil qilishingiz kerak
qiymati hali ham mavjud. Buni “Zaif<T>” da “yangilash” usulini chaqirish orqali bajaring
misol, bu `Option<Rc<T>>`ni qaytaradi. Siz "Ba'zi" natijasini olasiz
agar "Rc<T>" qiymati hali tushirilmagan bo'lsa va "Yo'q" natijasi, agar
`Rc<T>` qiymati olib tashlandi. Chunki `yangilash` `Option<Rc<T>>`ni qaytaradi,
Rust `Some` ishi va `None` ishi ko'rib chiqilishini ta'minlaydi va
yaroqsiz ko'rsatgich bo'lmaydi.

Misol sifatida, elementlari faqat keyingi haqida biladigan ro'yxatni ishlatish o'rniga
elementi boʻlsa, biz daraxt yaratamiz, uning obʼyektlari oʻz bolalari buyumlari haqida *va*
ularning ota-onalari.

#### Creating a Tree Data Structure: a `Node` with Child Nodes

Boshlash uchun biz ularning tugunlari haqida biladigan tugunlari bo'lgan daraxt quramiz.
Biz o'zining "i32" qiymatiga ega bo'lgan "tugun" nomli tuzilmani yaratamiz.
uning bolalar 'Tugun' qiymatlariga havolalar:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

Biz "Tugun" o'z farzandlariga ega bo'lishini istaymiz va biz bu egalikni baham ko'rmoqchimiz
o'zgaruvchilar, shuning uchun biz daraxtdagi har bir "Tugun" ga to'g'ridan-to'g'ri kirishimiz mumkin. Buning uchun biz
`Vec<T>` elementlarini `Rc<Node>` tipidagi qiymatlar sifatida belgilang. Biz ham xohlaymiz
qaysi tugunlar boshqa tugunning bolalari ekanligini o'zgartiring, shuning uchun bizda `RefCell<T>` mavjud
`Vec<Rc<tugun>>` atrofidagi `bolalar`.

Keyinchalik, biz strukturaning ta'rifidan foydalanamiz va bitta "Tugun" nomini yaratamiz
`barg` qiymati 3 va bolalari yo'q va boshqa misol `filial`
15-27 roʻyxatda koʻrsatilganidek, qiymati 5 va “barg” uning farzandlaridan biri sifatida:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

<span class="caption">Listing 15-27: Creating a `leaf` node with no children
and a `branch` node with `leaf` as one of its children</span>

Biz `Rc<tugun>`ni `barg`da klonlaymiz va uni `filial`da saqlaymiz, ya`ni
Endi “barg”dagi “tugun” ikkita egasiga ega: “barg” va “novda”. dan olishimiz mumkin
'branch.children' orqali 'shox'dan 'barg'ga, lekin undan olishning iloji yo'q
'barg'dan 'filialga'. Sababi, `barg` so`zi `filial` va
aloqadorligini bilmaydi. Biz “barg” “filial” uning ekanligini bilishini istaymiz
ota-ona. Biz buni keyin qilamiz.

#### Bolaning ota-onasiga havolani qo'shish

Bola tugunni ota-onasidan xabardor qilish uchun biz "ota-ona" maydonini qo'shishimiz kerak
bizning "tugun" tuzilmasining ta'rifi. Muammo nima turini tanlashda
"ota-ona" bo'lishi kerak. Biz bilamizki, unda `Rc<T>` bo'lishi mumkin emas, chunki bu bo'lar edi
`leaf.parent` `filial`ga ishora qiluvchi mos yozuvlar siklini yarating va
`barg`ga ishora qiluvchi `filial.bolalar`, bu ularning `kuchli_hisobiga` olib keladi
qiymatlar hech qachon 0 bo'lmasligi kerak.

O'zaro munosabatlar haqida boshqa yo'l bilan o'ylab, ota-ona tuguniga ega bo'lishi kerak
bolalar: agar ota-ona tugunlari tushirilsa, uning tugunlari sifatida tushirilishi kerak
yaxshi. Biroq, bola o'z ota-onasiga egalik qilmasligi kerak: agar biz bola tugunini tashlasak,
ota-ona hali ham mavjud bo'lishi kerak. Bu zaif havolalar uchun holat!

Shunday qilib, "Rc<T>" o'rniga, "ota-ona" turini "Zaif<T>" dan foydalanamiz,
xususan `RefCell<Zaif<tugun>>`. Endi bizning "tugun" tuzilmasining ta'rifi ko'rinadi
shunga o'xshash:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

Tugun o'zining asosiy tuguniga murojaat qilishi mumkin, lekin uning ota-onasiga ega emas.
15-28 ro'yxatda biz ushbu yangi ta'rifdan foydalanish uchun "asosiy" ni yangilaymiz, shuning uchun "barg"
tugun o'zining ota-onasi "filial" ga murojaat qilish usuliga ega bo'ladi:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

<span class="caption">Listing 15-28: A `leaf` node with a weak reference to its
parent node `branch`</span>

“Yaproq” tugunini yaratish 15-27 roʻyxatga oʻxshaydi, bundan tashqari
"ota-ona" maydoni: "barg" ota-onasiz boshlanadi, shuning uchun biz yangisini yaratamiz,
bo'sh `Zaif<tugun>` mos yozuvlar misoli.

Shu nuqtada, biz yordamida `barg` ota-onasiga havola olishga harakat qilganimizda
"yangilash" usulida biz "Yo'q" qiymatini olamiz. Buni biz dan chiqishda ko'ramiz
birinchi `println!` bayonoti:

``` matn
barg ota-onasi = Yo'q
```

Biz `filial` tugunini yaratganimizda, u ham yangi `Zaif<tugun>`ga ega bo'ladi.
"ota" maydonida havola, chunki "filial" da asosiy tugun yo'q.
Bizda hamon “barg” “filial” farzandlaridan biri. Bir marta bizda
“Filial”dagi “tugun” misolida biz “barg”ni “zaif<tugun>” qilish uchun o‘zgartirishimiz mumkin.
uning ota-onasiga havola. Biz "borrow_mut" usulidan foydalanamiz
`barg`ning `ota` maydonida `RefCell<Zaif<tugun>>` va keyin biz
`Rc::downgrade` funksiyasidan `filial`ga `Zaif<tugun>` havolasini yaratish
`filialdagi `Rc<tugun>`

Biz “barg” ning ota-onasini yana chop qilsak, bu safar “Ba’zi” variantini olamiz
"filial" ni ushlab turish: endi "barg" ota-onasiga kira oladi! Biz "barg" ni chop etganda, biz
Shuningdek, oxir-oqibat bizda bo'lgani kabi stekning to'lib ketishi bilan yakunlangan tsikldan qoching
Ro'yxat 15-26; `Zaif<tugun>` havolalari `(Zaif)` sifatida chop etiladi:

``` matn
barg ota = Ba'zi(tugun {qiymat: 5, ota: RefCell {qiymat: (zaif)},
bolalar: RefCell { qiymat: [tugun {qiymat: 3, ota: RefCell {qiymat: (zaif)},
bolalar: RefCell { qiymat: [] } }] } })
```

Cheksiz chiqishning yo'qligi ushbu kod mos yozuvlar yaratmaganligini ko'rsatadi
tsikl. Buni biz qo'ng'iroq qilishdan olgan qadriyatlarimizga qarab ham aytishimiz mumkin
`Rc::strong_count` va `Rc::weak_count`.

#### "Kuchli_hisob" va "zaif_hisob" dagi o'zgarishlarni vizualizatsiya qilish

Keling, `Rc<tugun>` `kuchli_hisoblash` va `zaif_hisoblash` qiymatlari qanday ekanligini ko'rib chiqamiz.
misollar yangi ichki doirani yaratish va yaratishni ko'chirish orqali o'zgaradi
`filial` shu doiraga kiradi. Shunday qilib, biz "filial" bo'lganda nima sodir bo'lishini ko'rishimiz mumkin
yaratilgan va keyin u ko'lamdan chiqib ketganda tushib ketgan. O'zgartirishlar ko'rsatilgan
15-29 ro'yxatda:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

<span class="caption">Listing 15-29: Creating `branch` in an inner scope and
examining strong and weak reference counts</span>

'barg' yaratilgandan so'ng, uning 'Rc<tugun>' kuchli soni 1 va zaif bo'ladi.
0 ning soni. Ichki doirada biz "filial" yaratamiz va uni bog'laymiz.
`barg`, biz hisoblarni chop etganda, `filialdagi `Rc<tugun>`
kuchli soni 1 va zaif soni 1 bo‘ladi (“leaf.parent” ko‘rsatish uchun
`zaif<tugun>` bilan `filial`ga). Hisoblarni “barg”da chop etganimizda, biz ko'ramiz
uning kuchli soni 2 ga teng bo'ladi, chunki `filial` endi kloniga ega
`barg`ning `Rc<tugun>` `branch.children` da saqlanadi, lekin baribir zaif bo'ladi.
soni 0.

Ichki qamrov tugagach, "filial" doiradan chiqib ketadi va kuchli soni
`Rc<Node>` 0 ga kamayadi, shuning uchun uning `Tugun` tushiriladi. Zaif hisob 1
'leaf.parent' dan "tugun" tushirilgan yoki yo'qligiga ta'sir qilmaydi, shuning uchun biz
hech qanday xotira oqishiga yo'l qo'ymang!

Qo'llanish doirasi tugagandan so'ng "barg" ning ota-onasiga kirishga harakat qilsak, biz olamiz
Yana 'Yo'q'. Dastur oxirida `barg`dagi `Rc<tugun>` kuchli
soni 1 va kuchsiz soni 0, chunki "barg" o'zgaruvchisi endi yagona
yana `Rc<tugun>` ga murojaat qiling.

Hisoblash va qiymatni pasaytirishni boshqaradigan barcha mantiq o'rnatilgan
`Rc<T>` va `Zaif<T>` va ularning `Drop` xususiyatini amalga oshirish. tomonidan
bolaning ota-onasiga bo'lgan munosabati a bo'lishi kerakligini ko'rsatib
`Tugun` ta`rifida `zaif<T>` havolasi, siz ota-onaga ega bo`lishingiz mumkin
tugunlar mos yozuvlar siklini yaratmasdan, bola tugunlariga ishora qiladi va aksincha
va xotira oqadi.

## Xulosa

Ushbu bobda turli xil kafolatlar berish uchun aqlli ko'rsatkichlardan qanday foydalanish kerakligi ko'rib chiqildi
Rust odatiy havolalar bilan sukut bo'yicha qiladi. The
`Box<T>` turi ma'lum o'lchamga ega va u yerda ajratilgan ma'lumotlarga ishora qiladi. The
`Rc<T>` turi to'pdagi ma'lumotlarga havolalar sonini kuzatib boradi
bu ma'lumotlar bir nechta egalariga ega bo'lishi mumkin. `RefCell<T>` turi ichki ko'rinishi bilan
o'zgaruvchanlik bizga o'zgarmas tur kerak bo'lganda foydalanishimiz mumkin bo'lgan turni beradi, lekin
ushbu turdagi ichki qiymatni o'zgartirish kerak; u qarz olishni ham majbur qiladi
kompilyatsiya vaqtida emas, balki ish vaqtidagi qoidalar.

Shuningdek, ko'plab imkoniyatlarni beradigan "Deref" va "Drop" xususiyatlari ham muhokama qilindi
aqlli ko'rsatkichlarning funksionalligi. Biz sabab bo'lishi mumkin bo'lgan mos yozuvlar davrlarini o'rganib chiqdik
xotira oqishi va ularni `Zaif<T>` yordamida qanday qilib oldini olish mumkin.

Agar ushbu bob sizni qiziqtirgan bo'lsa va siz o'zingiznikini amalga oshirmoqchi bo'lsangiz
aqlli ko'rsatkichlar, foydaliroq bo'lishi uchun [“The Rustonomicon”][nomicon] ni tekshiring
ma'lumot.

Keyinchalik, biz Rustdagi parallellik haqida gaplashamiz. Siz hatto bir nechta yangi narsalarni bilib olasiz
aqlli ko'rsatkichlar.

[nomicon]: ../nomicon/index.html
