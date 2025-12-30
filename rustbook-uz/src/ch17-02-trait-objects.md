## Turli xildagi qiymatlarni qabul qila oladigan `Trait` ya'ni xususiyat obyektlaridan foydalanish

8-bobda vektorlarning faqat bir turdagi elementlarni saqlash
imkoniyatiga ega ekanligini ta’kidlagan edik. 8-9-ro‘yxatda
butun sonlar, kasr sonlar va matnlarni saqlash uchun variantlarga
ega bo‘lgan ’SpreadsheetCell’ nomli sanab o‘tish turini yaratib,
bu muammoni hal qilish ko'rsatilgan edi. Bu usul har bir katakda
turli xil ma’lumotlarni saqlash va shu bilan birga kataklar
qatorini ifodalovchi vektorga ega bo‘lish imkonini berdi. Agar
o‘zaro almashtirilishi mumkin bo‘lgan elementlarni kodda tuzilayotgan
paytda ma’lum bo‘lgan turlarning belgilangan to‘plamidan iborat
bo‘lsa, bu juda yaxshi yechim hisoblanadi.

Biroq, ba’zida kutubxonamiz foydalanuvchisi o‘zi uchun mos bo‘lgan, muayyan vaziyatda
ishlatilishi mumkin bo‘lgan turlar to‘plamini kengaytira olishini xohlaymiz. Bu qanday
amalga oshirilishini ko‘rsatish uchun, grafik foydalanuvchi interfeysi (GUI) vositasi
misolini yaratamiz. Ushbu vosita elementlar ro‘yxatidan o‘tadi va har bir element uchun
`draw` metodini chaqiradi. Bu GUI vositalarida keng qo‘llaniladigan uslubdir.`gui`
nomli kutubxona crate yaratiladi. Ushbu crate GUI kutubxonasining asosiy tuzilmasini o‘z
ichiga oladi. Unda, masalan, `Button` yoki `TextField` kabi foydalanishga tayyor ayrim
turlarni taqdim qilishi mumkin. Shu bilan birga, `gui` foydalanuvchilari o‘zlarining
chizilishi mumkin bo‘lgan turlarini ham yaratmoqchi bo‘lishadi: masalan, bir dasturchi
`Image` turini qo‘shsa, boshqasi `SelectBox` turini qo‘shishi mumkin.

Ushbu misolda to‘laqonli grafik interfeyslik (GUI) kutubxona yozilmaydi lekin
qismlar bir-biri bilan qanday ulanishini ko‘rsatiladi. Kutubxona yozish vaqtida
boshqa dasturchilar nima va qanday qilib yozishini oldindan bilib bo‘lmaydi.
Lekin bilamizki, `gui` imkon qadar ko‘p turlar qiymatidan xabardor bo‘lishi
kerak, va u `draw` (ya’ni chizish) metodini ana shu turlarning har birida
chaqirishi lozim. `draw` metodini chaqirgan vaqtida aynan nima ish sodir
bo‘lishini `gui` bilishi kerak emas, faqatgina `draw` metodi chaqirish uchun
mavjudligini biladi xolos.

Obyektga yo‘naltirilgan tillarda (misol uchun Java, C# va h.k.), `Component` nomli
klass ichida `draw` nomli metod bilan ifoda etiladi. `Button`, `Image` va
`SelectBox` kabi klasslar `Component`dan nasil olishadi va shu tufayli ular
ham `draw` metodini ifodalashadi. Ular, albatta, o‘zgacha `draw` metodini
e’lon qilishlari mumkin lekin dasturlash tili ularni xuddi `Component`dek
ko‘rishadi va `draw`ni chaqira olishadi. Rust dasturlash tilida nasl olish
imkoniyati yo‘q, vaholanki `gui` kutubxonasi foydalanuvchilari uni
kengaytira olishi uchun kutubxona boshqacha tuzilishi lozim.

### Umumiy xatti-harakatlar uchun `Trait` ni aniqlash ya'ni xususiyatni

`Gui` uchun kerakli xatti-harakatni amalga oshirish maqsadida, `Draw` nomli
trait'ni belgilaymiz. Bu trait `draw` deb nomlangan yagona usulni o‘z ichiga
oladi. Shundan so‘ng _trait object_ ni qabul qiladigan vektorni aniqlash mumkin.
Trait obyekti ko‘rsatilgan xususiyatni amalga oshiruvchi turning nusxasiga ham,
ishlash vaqtida ushbu turdagi trait usullarini qidirish uchun ishlatiladigan
jadvalga ham ishora qiladi. Qandaydir ko‘rsatkichni, masalan `&` havola yoki
`Box<T>` aqlli ko‘rsatkichni, so‘ngra `dyn` kalit so‘zini va tegishli trait'ni
ko‘rsatish orqali trait obyektini yaratamiz. (Trait obyektlarining nima uchun
ko‘rsatkich ishlatishi kerakligi haqida 19-bobning
["Dinamik o‘lchamliturlar va ’Sized’ belgisi"][dinamik-olchamli]

<!-- e’tiborsiz qoldirish --> qismida batafsil to‘xtalamiz.) Trait

obyektlarini `generic` ya'ni turdosh yoki aniq tur o‘rnida ishlatishimiz mumkin.
Trait obyektini qayerda ishlatishimizdan qat'iy nazar, Rustning turlar tizimi
kompilyatsiya vaqtida ushbu kontekstda ishlatiladigan har qanday qiymat
trait obyektining trait'ini amalga oshirishini ta’minlaydi. Natijada
kompilyatsiya vaqtida barcha mumkin bo‘lgan turlarni bilish shart emas.

Rust dasturlash tilida structlar va enumlar “obyekt” deb atalmaydi.
Bunday yondashuv, ularni boshqa dasturlash tillaridagi obyekt tushunchasidan
farqlash maqsadida qo‘llaniladi. Rust tilida struct yoki enum tarkibidagi
ma’lumotlar (ya’ni, maydonlar) va xatti-harakatlar `impl` bloklarida
alohida saqlanadi. Aksariyat boshqa dasturlash tillarida esa ma’lumotlar va
xatti-harakatlar yagona tuzilma sifatida birlashtirilib, odatda “obyekt” deb
ataladi. Biroq trait obyektlar (trait objects) boshqa dasturlash tillaridagi
obyektlarga o‘xshashlik kasb etadi. Chunki ular ma’lumot va xatti-harakatni
birgalikda ifodalash imkonini beradi. Shunga qaramay, trait obyektlar an’anaviy
obyektlardan farq qiladi: ular tarkibiga yangi ma’lumotlar qo‘shishga imkon
bermaydi. Shu bois, trait obyektlar boshqa tillardagi obyektlar kabi keng
maqsadlarda emas, balki faqat umumiy xatti-harakatni abstraktsiyalash,
ya’ni umumiy funksionallik asosida turli obyektlar bilan ishlash imkoniyatini
yaratish uchun qo‘llaniladi.

17-3-ro'yxat `Draw` trait'ini `draw` metodi bilan birga ta'riflash ko'rsatib beradi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<span class="caption">Ro'yxat 17-3: `Draw` trait'ining ta'rifi</span>

Ushbu sintaksis bizning 10-bo'limda bo'lib o'tgan Traitlarni joriy etish
suhbatimizdan keyin tanish bo'lishi kerak. Keyingisi esa yana yangi sintaksis:
17-4-ro'yxat `Screen` nomli `components` nomi ostidagi vekotr o'z ichiga olgan
structni ta'riflaydi. Ushbu vektor `Box<dyn Draw>` turidan, ya'ni trait obyekt (bu
`Box` ichida `Draw` tratini joriy etuvchi istalgan turga solishtiriluvchi).

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-4: `Screen` structidagi `components` maydoni
bir vektorda joylashgan va `Draw` tratini joriy etgan obyektlarni ushlab turibdi
</span>

`Screen` struktida, biz 17-5-ro'yxatda ko'rsatilganiday, `draw` metodini har
bir `components` ustidan chaqiradigan `run` nomli metod yaratamiz:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-5: `Screen` da har bir komponent ustidan
`draw` metodini chaqiradigan `run` metodi</span>

Bu generik tur ko'rsatkichi va trait cheklanmalardan farqli boshqacha
ishlaydi. Generik tur parametr bir vaqt o'zida faqat bitta tur qabul qiladi,
trait obyektlar esa boshqa tarafdan ko'plab konkret turlar ishlash vaqtidagi
trait obyektlarni to'ldirib berish uchun ishlatsa bo'ladi. Misol uchun,
`Screen` struktini 17-6-ro'yxatda ko'rsatilganiday generik tur va trait
cheklanmalari bilan ta'riflasa bo'ladi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-6: `Screen` strukti va uning `run` metodining
generik va trait cheklanmalarini ishlatgandagi alternativ ta'rifi.</span>

Bu faqat `Button` yoki faqat `TextField` turidagi komponentlar ro‘yxatiga
ega bo‘lgan `Screen` nusxasi bilan cheklaydi. Agar sizda faqat bir xil
to‘plamlar bo‘lsa, `generic` umumiy va `trait` xususiyat chegaralaridan
foydalanish afzalroq, chunki aniq turlardan foydalanish uchun ta’riflar
tuzish vaqtida har bir tur uchun birlashtiradi.

Boshqa tomondan, `trait` obyektlaridan foydalanadigan usul bilan bitta
`Screen` nusxasi `Box<Button>`, shuningdek `Box<TextField>` ni o‘z ichiga
olgan `Vec<T>` ni saqlash imkoniyatiga ega bo‘ladi. Keling, bu qanday
ishlashini ko‘rib chiqaylik, so‘ngra dasturning ishlash vaqtidagi
unumdorlik ta’sirlari haqida suhbatlashamiz.

### Traitni amalga oshirish

Endi `Draw` traitini amalga oshiradigan ba'zi turlarni qo‘shamiz. `Button`
turini taqdim etamiz. Yana, haqiqiy GUI kutubxonasini yaratish kitobimiz
doirasidan tashqarida, shuning uchun `draw` metodi tanasida hech qanday
foydali amalga oshirish bo‘lmaydi. Amalga oshirish qanday ko‘rinishi
mumkinligini tasavvur qilish uchun, `Button` tuzilmasi `width` (kenglik),
`height` (bo‘yi) va `label` (yorliq) kabi maydonlarga ega bo‘lishi mumkin,
bu 17-7 ro'yxatdada ko‘rsatilgan:

<span class="filename">Faylnomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-7: `Draw` traitini amalga oshiradigan `Button` strukti</span>

`Button`ning `width`, `height` va `label` maydonlari boshqa komponentlarga
nisbatan farq qiladi; misol uchun `TextField`ning turi avvalgi maydonlar va
qo‘shimcha `placeholder` maydonidan tashkil topgan bo‘lishi mumkin. Har bir
ekranga chizilishi kerak bo‘lgan turlar `Draw` trait’ini joriy qilishadi ammo
ularning `draw` metodlari bir-birlaridan farq qiladi chunki har bir turning
o‘ziga xos shakli yoki boshqa xususiyati chizilishi mumkin. Misol uchun
`Button` bosganda sodir bo‘ladigan metod `impl` bloki ichida qo‘shimcha
kiritilishi mumkin. `TextField` uchun esa bunday funksional talab etilmaydi.

`width`, `height` va `options` maydonlardan tashkil topgan `SelectBox`ni
amalga oshirmoqchi bo‘lgan dasturchi `SelectBox` uchun `Draw` trait’ini
ham Ro‘yxat 17-8 da ko‘rsatilgandek yozishi kerak bo‘ladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<span class="caption">Ro‘yxat 17-8: `gui`dan foydalanuvchi va undagi `Draw`
trait’ini `SelectBox` struct’i uchun amalga oshiradigan crate</span>

Kutubxona foydalanuvchisi endi o‘z `main` funksiyasi ichida `Screen` nusxasini
yaratish imkoniga ega. `Screen` nusxasiga foydalanuvchi `SelectBox` va `Button`
nusxalarini har birini `Box<T>` ichiga joylash orqali ularni trait’ga o‘girib
qo‘shib chiqishi mumkin. Undan keyin `Screen` nusxasida `run` metodi chaqirgan
vaqtda har bir ichki komponentlarning `draw` metodi birma-bir chaqiriladi.
Ro‘yxat 17-9 xuddi shuni namoyish etadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<span class="caption">Ro‘yxat 17-9: Bir trait’ni amalga oshiruvchi turli xil
turlarni saqlash uchun trait obyektlar ishlatilishi</span>

When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.

Bu tushuncha — ya’ni, qiymatning aniq tipi emas, balki qanday xabarlarga
javob bera olishi muhim bo‘lishi — dinamik tiplangan tillardagi _duck typing_
tushunchasiga o‘xshaydi: agar u o‘rdakdek yursa va o‘rdakdek ovoz chiqarsa,
demak u o‘rdak! 17-5-ro‘yxatdagi `Screen` uchun `run` funksiyasi
implementatsiyasida `run` har bir komponentning aniq tipi nima ekanini
bilishga muhtoj emas. Komponent `Button` yoki `SelectBox` ekanligini
tekshirmaydi, shunchaki uning `draw` metodini chaqiradi. `components` vektoridagi
qiymatlar turi sifatida `Box<dyn Draw>`ni ko‘rsatish orqali,`Screen`dan `draw`
metodini chaqira olishimiz mumkin bo‘lgan qiymatlarni talab qiladigan qilib belgiladik.

Trait obyektlar va Rustning turlar tizimidan foydalanib, `duck typing` uslubiga
o‘xshash kod yozishning afzalligi shundaki, qiymatning ma’lum bir usulni
bajarishini ishga tushirish vaqtida tekshirishimiz shart emas. Bundan tashqari,
agar qiymat usulni amalga oshirmasa-yu, lekin u chaqirilsa ham, xatolar yuzaga
kelishidan xavotirlanishimizga hojat yo‘q. Agar qiymatlar trait obyektlariga
kerak bo'lgan trait'larni amalga oshirmasa, Rust bu kodni kompilatsiya qilmaydi.

Masalan, 17-10-roʻyxatda `String` komponentli `Screen` yaratishga harakat
qilsa, nima sodir bo‘lishi ko‘rsatilgan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

<span class="caption">17-10-ro'yxat: Trait obyektining xususiyatini amalga
oshirmaydigan turdan foydalanishga urinish</span>

`String` funksiyasi `Draw` trait'ni amalga oshirmagani uchun quyidagi xatolik yuz berdi:

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

Bu xato bizga shuni bildiradi: biz `Screen`ga yubormoqchi bo‘lmagan narsani
yuboryapmiz va shuning uchun boshqa turni o'tkazishimiz kerakligini yoki
`String` ustiga `Draw`ni qo'llashimiz kerakligini bildiradi, shunda `Screen`
`draw`ni chaqira oladi.

### Trait Objects Perform Dynamic Dispatch

["Generiklar yordamida kodning ishlashi"][performance-of-code-using-generics]<!-- ignore-->
bo‘limidagi muhokamadagi kompilyator monomorfizatsiya qismini eslaymiz. Kompilyator
umumiy turlarning trait chegaralari ustida ishlash jarayonida u har bir umumiy tur
o‘rnida ishlatilgan aniq turlarni funksiya va metodlarga joriy etadi.
Monomorfizatsiya natijasidagi kod _static dispatch_ (ya’ni statik yo‘naltirish) deb
ataladi. Bu degani kompilyatsiya vaqtida kompilyator qaysi turga oid funksiya yoki
metod qayerda chaqirilishini biladi. Kompilyator bilmagan holat esa dynamic dispatch
(ya’ni dinamik yo‘naltirish) deb ataladi va kompilyatsiya jarayonida dastur o‘zi
ishga tushish vaqtida yo‘naltira oladigan kod yaratiladi.

Rust-da trait obyektlaridan foydalanganda, dinamik dispatch ishlatiladi.
Kompilyator kodda qaysi turdagi qiymatlar ishlatilishini oldindan bilmaydi,
shuning uchun qaysi turdagi metod chaqirilishini ham bilmaydi. Buning o‘rniga,
bajarilish vaqtida (runtime) Rust trait obyektining ichidagi ko‘rsatkichlardan
(pointer) qaysi metodni chaqirish kerakligini aniqlaydi. Bu esa statik dispatchiga
nisbatan bajarilish vaqtida qo‘shimcha xarajatlarni keltirib chiqaradi. Shuningdek,
dinamik dispatchi kompilyatorga metod kodini inline qilish imkonini bermaydi, bu esa
ba’zi optimallashtirishlarni cheklaydi. Biroq, biz ro‘yxat 17-5 yozgan kodimizda
qo‘shimcha moslashuvchanlikka ega bo‘ldik va ro‘yxat 17-9 da qo‘llab-quvvatlay oldik,
shuning uchun buni hisobga olish kerak.

[performance-of-code-using-generics]: ch10-01-syntax.html#generiklar-yordamida-kodning-ishlashi
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
