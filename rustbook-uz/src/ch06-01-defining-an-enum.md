## Enumni aniqlash

Structlar sizga tegishli maydonlar va ma'lumotlarni, masalan, `kenglik` va `balandlik` bilan `Kvadrat` ni guruhlash usulini beradigan bo'lsa, enumlar qiymatni mumkin bo'lgan qiymatlar to'plamidan biri deb aytish metodini beradi. Masalan, `Kvadrat` bu mumkin bo‘lgan shakllar to‘plamidan biri bo‘lib, `Doira` va `Uchburchak`ni ham o‘z ichiga oladi, demoqchimiz. Buning uchun Rust bizga ushbu imkoniyatlarni enum sifatida kodlash imkonini beradi.

Keling, kodda ifodalashni xohlashimiz mumkin bo'lgan vaziyatni ko'rib chiqaylik va bu holda nima uchun enumlar foydali va structlardan ko'ra mosroq ekanligini bilib olaylik. Aytaylik, biz IP manzillar bilan ishlashimiz kerak. Hozirgi vaqtda IP manzillar uchun ikkita asosiy standart qo'llaniladi: to'rtinchi versiya va oltinchi versiya. Bular bizning dasturimiz duch keladigan IP-manzilning yagona imkoniyatlari bo'lgani uchun biz barcha mumkin bo'lgan variantlarni *enumerate* qilishimiz mumkin, bu yerda enumeration o'z nomini oladi.

Har qanday IP manzil to'rtinchi versiya yoki oltinchi versiya manzili bo'lishi mumkin, lekin ikkalasi bir vaqtning o'zida emas. IP-manzillarning bu xususiyati enum ma'lumotlar structini mos qiladi, chunki enum qiymati faqat uning variantlaridan biri bo'lishi mumkin.
To'rtinchi versiya va oltinchi versiya manzillari hali ham IP-manzillardir, shuning uchun kod har qanday IP-manzilga tegishli vaziyatlarni ko'rib chiqayotganda ular bir xil turdagi sifatida ko'rib chiqilishi kerak.

Biz ushbu kontseptsiyani kodda `IpAddrKind` ro'yxatini belgilash va IP-manzil bo'lishi mumkin bo'lgan `V4` va `V6` turlarini enumeration qilish orqali ifodalashimiz mumkin. Bular enumning variantlari:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` endi biz kodimizning boshqa joylarida foydalanishimiz mumkin bo'lgan maxsus ma'lumotlar turidir.

### Enum qiymatlari

Biz `IpAddrKind` ning ikkita variantining har birining misollarini quyidagicha yaratishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

E'tibor bering, enumning variantlari uning identifikatori ostida nom maydonida joylashgan va biz ikkalasini ajratish uchun qo'sh nuqtadan foydalanamiz. Bu foydali, chunki endi ikkala `IpAddrKind::V4` va `IpAddrKind::V6` qiymatlari bir xil turdagi: `IpAddrKind`. Masalan, biz har qanday `IpAddrKind` ni qabul qiladigan funksiyani aniqlashimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Va biz bu funktsiyani ikkala variant bilan chaqirishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Enumlardan foydalanish yanada ko'proq afzalliklarga ega. Bizning IP manzilimiz turi haqida ko'proq o'ylab ko'rsak, hozirda bizda haqiqiy IP-manzilni *ma'lumotlarni* saqlash imkoni yo'q; biz faqat qanday *turdagi* ekanligini bilamiz. 5-bobda structlar haqida hozirgina bilib olganingizni hisobga olsak, 6-1 ro'yxatda ko'rsatilganidek, bu muammoni structlar yordamida hal qilish istagi paydo bo'lishi mumkin.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 6-1: `struct` yordamida IP manzilining ma'lumotlarini va `IpAddrKind` variantini saqlash</span>

Bu yerda biz ikkita maydonga ega boʻlgan `IpAddr` structini aniqladik: `IpAddrKind` turidagi `tur` maydoni (biz avvalroq belgilagan raqam) va `String` tipidagi `manzil` maydoni. Bizda bu structning ikkita misoli bor. Birinchisi `asosiy` boʻlib, u `127.0.0.1` bogʻlangan manzil maʼlumotlari bilan `tur` sifatida `IpAddrKind::V4` qiymatiga ega. Ikkinchi misol - `orqaga_qaytish`. U `tur` qiymati sifatida `IpAddrKind` ning boshqa variantiga ega, `V6` va u bilan bog'langan `::1` manzili mavjud. Biz `tur` va `manzil` qiymatlarini birlashtirish uchun structdan foydalanganmiz, shuning uchun endi variant qiymat bilan bog'langan.

Shu bilan birga, bir xil kontseptsiyani faqat enum yordamida ifodalash yanada ixchamroqdir: struct ichidagi enum o'rniga, biz ma'lumotlarni to'g'ridan-to'g'ri har bir enum variantiga qo'yishimiz mumkin. `IpAddr` enumining ushbu yangi ta'rifida aytilishicha, `V4` va `V6` variantlari ham associated `String` qiymatlariga ega bo'ladi:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Biz to'g'ridan-to'g'ri enumning har bir variantiga ma'lumotlarni biriktiramiz, shuning uchun qo'shimcha structga ehtiyoj qolmaydi. Bu yerda, shuningdek, enumlar qanday ishlashining yana bir tafsilotini ko'rish osonroq bo'ladi: biz belgilagan har bir enum variantining nomi, shuningdek, enum nusxasini yaratuvchi funktsiyaga aylanadi. Ya'ni, `IpAddr::V4()` funksiya chaqiruvi bo'lib, u `String` argumentini oladi va `IpAddr` tipidagi misolni qaytaradi. Enumni aniqlash natijasida aniqlangan ushbu konstruktor funksiyasini avtomatik ravishda olamiz.

Structdan ko'ra enumdan foydalanishning yana bir afzalligi bor: har bir variantda bog'langan ma'lumotlarning har xil turlari va miqdori bo'lishi mumkin. To'rtinchi versiyada IP-manzillar har doim 0 dan 255 gacha bo'lgan qiymatlarga ega bo'lgan to'rtta raqamli komponentga ega bo'ladi. Agar biz `V4` manzillarini to‘rtta `u8` qiymati sifatida saqlamoqchi bo‘lsak-da, `V6` manzillarini bitta `String` qiymati sifatida ifodalasak, biz struct bilan buni qila olmaymiz. Enumlar bu ishni osonlik bilan hal qiladi:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Biz to'rtinchi versiya va oltinchi versiya IP manzillarini saqlash uchun ma'lumotlar tuzilmalarini aniqlashning bir necha xil usullarini ko'rsatdik. Biroq, ma'lum bo'lishicha, IP-manzillarni saqlash va ularning qaysi turini kodlash istagi shunchalik keng tarqalganki, [standart kutubxonada biz foydalanishimiz mumkin bo'lgan defination mavjud!][IpAddr]<!-- ignore --> . Keling, standart kutubxona `IpAddr` ni qanday aniqlashini ko'rib chiqaylik: u biz aniqlagan va ishlatgan aniq enum va variantlarga ega, lekin u manzil ma'lumotlarini variantlar ichida ikki xil struct shaklida joylashtiradi, har bir variant uchun turlicha belgilanadi:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Ushbu kod har qanday turdagi ma'lumotlarni enum variantiga qo'yish mumkinligini ko'rsatadi: masalan, stringlar, raqamli turlar yoki structlar. Siz hatto boshqa raqamni ham qo'shishingiz mumkin! Bundan tashqari, standart kutubxona turlari ko'pincha siz o'ylab topganingizdan ancha murakkab emas.

E'tibor bering, standart kutubxonada `IpAddr` uchun definition mavjud bo'lsa ham, biz o'z definitionimizni ziddiyatli holda yaratishimiz va foydalanishimiz mumkin, chunki biz standart kutubxonaning definitionini o'z doiramizga kiritmaganmiz. Biz 7-bobda turlarni qamrab olish haqida ko'proq gaplashamiz.

Keling, 6-2 ro'yxatdagi enumning yana bir misolini ko'rib chiqaylik: bu o'z variantlarida ko'p turdagi turlarga ega.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 6-2: `Xabar` enumi, uning variantlari har xil miqdor va qiymat turlarini saqlaydi</span>

Ushbu enum har xil turdagi to'rtta variantga ega:

* `Chiqish`da u bilan bogʻliq hech qanday maʼlumot yoʻq.
* `Kochirish` da struct kabi maydonlarni nomlagan.
* `Yozish` bitta `String` ni o'z ichiga oladi.
* `RangTanla` uchta `i32` qiymatini o'z ichiga oladi.

Enumni 6-2-roʻyxatdagi kabi variantlar bilan belgilash strukturaviy definitionlarning har xil turlarini aniqlashga oʻxshaydi, faqat enum `struct` kalit soʻzidan foydalanmaydi va barcha variantlar `Xabar` turi ostida birlashtiriladi. Quyidagi structlar oldingi enum variantlari bilan bir xil ma'lumotlarni saqlashi mumkin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Lekin biz o'z turlariga ega bo'lgan turli structlardan foydalanganimizda, biz har qanday xabar turini qabul qiladigan funksiyalarni osonlikcha aniqlay olmadik, buni bitta tur bo'lgan 6-2 ro'yxatda e'lon qilingan `Xabar` turini enum bilan amalga oshirish mumkin.

Enumlar va structlar o'rtasida yana bir o'xshashlik bor: biz `impl` yordamida structlarda metodlarni aniqlay olganimizdek, enumlarda ham metodlarni belgilashimiz mumkin. Bu yerda biz `Xabar` enumimizda aniqlashimiz mumkin bo'lgan `chaqiruv` deb nomlangan metod:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Metod tanasi biz metod deb atagan qiymatni olish uchun `self` ishlatadi. Ushbu misolda biz `Xabar::Yozish(String::from("salom"))` qiymatini o'z ichiga olgan `m` o'zgaruvchisini yaratdik va `m.chaqiruv()` ishga tushganda `chaqiruv` metodining tanasida aynan shunday bo`ladi.
Keling, standart kutubxonadagi juda keng tarqalgan va foydali bo'lgan yana bir enumni ko'rib chiqaylik: `Option`.

### `Option` Enum va uning null qiymatlardan ustunligi

Ushbu bo'lim standart kutubxona tomonidan aniqlangan yana bir enum bo'lgan `Option` ning misolini o'rganadi.`Option` turi qiymat nimadir yoki hech narsa bo'lmasligi mumkin bo'lgan juda keng tarqalgan senariyni kodlaydi.

Misol uchun, agar siz bo'sh bo'lmagan ro'yxatdagi birinchi elementni so'rasangiz, qiymat olasiz. Agar siz bo'sh ro'yxatdagi birinchi elementni so'rasangiz, hech narsa olmaysiz.
Ushbu kontseptsiyani turdagi tizim nuqtai nazaridan ifodalash kompilyator siz ko'rib chiqishingiz kerak bo'lgan barcha ishlarni ko'rib chiqqaningizni tekshirishi mumkinligini anglatadi; bu funksiya boshqa dasturlash tillarida juda keng tarqalgan xatolarni oldini oladi.

Dasturlash tilining dizayni ko'pincha siz qaysi xususiyatlarni o'z ichiga olganligingiz nuqtai nazaridan o'ylanadi, ammo siz chiqarib tashlagan xususiyatlar ham muhimdir. Rust ko'plab boshqa tillarda mavjud bo'lgan null xususiyatiga ega emas. *Null* - bu qiymat yo'qligini bildiradi. Null bo'lgan tillarda o'zgaruvchilar har doim ikkita holatdan birida bo'lishi mumkin: null yoki null emas.

2009 yilgi "Null References: The Million Dollar Mistake" taqdimotida null ixtirochisi Tony Hoare shunday deydi:

> Men buni milliard dollarlik xatoyim deb atayman. O'sha paytda men object-oriented language
> referencelar uchun birinchi keng qamrovli turdagi tizimni loyihalashtirgan edim. Mening maqsadim
> referencelardan foydalanishning mutlaqo xavfsiz bo'lishini ta'minlash edi, tekshirish kompilyator
> tomonidan avtomatik ravishda amalga oshiriladi. Lekin men null reference qo'yish vasvasasiga
> qarshi tura olmadim, chunki uni amalga oshirish juda oson edi.
> Bu so'nggi qirq yil ichida bir milliard dollar og'riq va zarar keltirgan
> son-sanoqsiz xatolar, zaifliklar va tizimning ishdan chiqishiga olib keldi.

Null qiymatlari bilan bog'liq muammo shundaki, agar siz null qiymatdan a sifatida foydalanishga harakat qilsangiz
no-null qiymat bo'lsa, siz qandaydir xatoga duch kelasiz. Ushbu null yoki not-null xususiyat keng tarqalganligi sababli, bunday xatoga yo'l qo'yish juda oson.

Biroq, null ifodalamoqchi bo'lgan kontseptsiya hali ham foydalidir: null hozirda yaroqsiz yoki biron sababga ko'ra mavjud bo'lmagan qiymatdir.

Muammo aslida kontseptsiyada emas, balki muayyan amalga oshirishda. Shunday qilib, Rust nulllarga ega emas, lekin u mavjud yoki yo'q qiymat tushunchasini kodlay oladigan enumga ega. Bu enum `Option<T>` bo'lib, u [standart kutubxona tomonidan][option]<!-- ignore --> quyidagicha aniqlanadi: 

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` enumi shunchalik foydaliki, u hatto muqaddimaga ham kiritilgan; uni aniq doiraga kiritishingiz shart emas. Uning variantlari ham muqaddima tarkibiga kiritilgan: `Some` va `None` dan `Option::` prefiksisiz bevosita foydalanishingiz mumkin. `Option<T>` enum hali ham oddiy enum bo'lib, `Some(T)` va `None` hali ham `Option<T>` turidagi variantlardir.

`<T>` sintaksisi Rustning oʻziga xos xususiyati boʻlib, biz hali gaplashmaganmiz. Bu umumiy turdagi parametr va biz 10-bobda genericlarni batafsil ko'rib chiqamiz.
Hozircha siz bilishingiz kerak bo'lgan narsa shuki, `<T>` `Option` enumining `Some` varianti har qanday turdagi ma'lumotlarning bir qismini saqlashi mumkinligini va o'rniga qo'llaniladigan har bir konkret turni bildiradi. `T` umumiy `Option<T>` turini boshqa turga aylantiradi. Raqam turlari va qator turlarini saqlash uchun `Option` qiymatlaridan foydalanishga misollar keltiramiz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

`raqam` turi - `Option<i32>`. `belgi` turi `Option<char>` bo'lib, u boshqa tur. Rust bu turlarni aniqlashi mumkin, chunki biz `Some` variantida qiymat belgilaganmiz. `yoq_raqam` uchun Rust bizdan umumiy `Option` turiga izoh berishimizni talab qiladi: kompilyator faqat `None` qiymatiga qarab mos keladigan `Some` varianti qanday turga ega bo'lishini aniqlay olmaydi. Bu yerda biz Rustga aytamizki, biz `yoq_raqam` `Option<i32>` turida bo'lishini nazarda tutamiz.

Agar bizda `Some` qiymati bo'lsa, biz qiymat mavjud ekanligini va qiymat `Some` ichida saqlanishini bilamiz. Agar bizda `None` qiymati bo'lsa, u qaysidir ma'noda null bilan bir xil narsani anglatadi: bizda haqiqiy qiymat yo'q. Xo'sh, nega `Option<T>` nullga ega bo'lishdan yaxshiroq?

Xulosa qilib aytganda, `Option<T>` va `T` (bu erda `T` har qanday tur bo'lishi mumkin) har xil turdagi, chunki kompilyator bizga `Option<T>` qiymatidan foydalanishga ruxsat bermaydi, go'yo bu haqiqiy qiymat. Masalan, bu kod kompilyatsiya qilinmaydi, chunki u `Option<i8>`ga `i8` qo`shishga harakat qilmoqda:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Agar biz ushbu kodni ishlatsak, biz quyidagi kabi xato xabarini olamiz:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Kuchli! Aslida, bu xato xabari Rust `i8` va `Option<i8>` ni qanday qo'shishni tushunmasligini anglatadi, chunki ular turli xil. Rustda `i8` kabi turdagi qiymatga ega bo'lsak, kompilyator bizda har doim haqiqiy qiymatga ega bo'lishini ta'minlaydi. Ushbu qiymatdan foydalanishdan oldin nullni tekshirmasdan ishonch bilan davom etishimiz mumkin. Faqat bizda `Option<i8>` (yoki qanday turdagi qiymat bilan ishlayotgan bo'lishimizdan qat'iy nazar) mavjud bo'lganda, biz qiymatga ega bo'lmasligimizdan xavotirlanishimiz kerak va kompilyator qiymatdan foydalanishdan oldin bu holatni hal qilishimizga ishonch hosil qiladi.

Boshqacha qilib aytganda, `T` amallarini bajarishdan oldin `Option<T>`ni `T` ga aylantirishingiz kerak. Umuman olganda, bu null bilan bog'liq eng keng tarqalgan muammolardan birini hal qilishga yordam beradi: agar biror narsa bo'lsa, u null emas deb taxmin qilish.

Eliminating the risk of incorrectly assuming a not-null value helps you to be
more confident in your code. In order to have a value that can possibly be
null, you must explicitly opt in by making the type of that value `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn’t an
`Option<T>`, you *can* safely assume that the value isn’t null. This was a
deliberate design decision for Rust to limit null’s pervasiveness and increase
the safety of Rust code.

So how do you get the `T` value out of a `Some` variant when you have a value
of type `Option<T>` so that you can use that value? The `Option<T>` enum has a
large number of methods that are useful in a variety of situations; you can
check them out in [its documentation][docs]<!-- ignore -->. Becoming familiar
with the methods on `Option<T>` will be extremely useful in your journey with
Rust.

In general, in order to use an `Option<T>` value, you want to have code that
will handle each variant. You want some code that will run only when you have a
`Some(T)` value, and this code is allowed to use the inner `T`. You want some
other code to run only if you have a `None` value, and that code doesn’t have a
`T` value available. The `match` expression is a control flow construct that
does just this when used with enums: it will run different code depending on
which variant of the enum it has, and that code can use the data inside the
matching value.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
