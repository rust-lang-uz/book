## Traitlar: umumiy xatti-harakatni aniqlash

*trait* ma'lum bir turga ega bo'lgan va boshqa turlar bilan bo'lishishi mumkin bo'lgan funksionallikni belgilaydi. Biz umumiy xatti-harakatni mavhum tarzda aniqlash uchun traitlardan foydalanishimiz mumkin. Generik tur ma'lum xatti-harakatlarga ega bo'lgan har qanday tur bo'lishi mumkinligini aniqlash uchun *trait (bound)chegaralari* dan foydalanishimiz mumkin.

> Eslatma: Traitlar ba'zi farqlarga ega bo'lsa-da, ko'pincha boshqa tillarda
> *interfeyslar* deb ataladigan xususiyatga o'xshaydi.

### Traitni aniqlash

Turning xatti-harakati biz ushbu turga murojaat qilishimiz mumkin bo'lgan metodlardan iborat. Agar biz ushbu turlarning barchasida bir xil metodlarni chaqira olsak, har xil turlar bir xil xatti-harakatlarga ega. Trait ta'riflari - bu qandaydir maqsadga erishish uchun zarur bo'lgan xatti-harakatlar to'plamini aniqlash uchun metod imzolarini birgalikda guruhlash usuli.

Misol uchun, bizda turli xil va hajmdagi matnlarni o'z ichiga olgan bir nechta structlar mavjud deylik: ma'lum bir joyda joylashtirilgan yangiliklarni o'z ichiga olgan `YangiMaqola` structi va eng ko'pi 280 belgidan iborat bo'lishi mumkin bo'lgan `Maqola` yangi post, retpost yoki boshqa postga javob ekanligini ko'rsatadigan metama'lumotlar.

Biz `YangiMaqola` yoki `Maqola` misolida saqlanishi mumkin bo‘lgan ma’lumotlarning qisqacha mazmunini ko‘rsata oladigan `aggregator` nomli media agregator kutubxonasini yaratmoqchimiz. Buni amalga oshirish uchun bizga har bir tur bo'yicha xulosa kerak bo'ladi va biz ushbu xulosani misolda `umumiy_xulosa` metodini chaqirish orqali so'raymiz. 10-12 ro'yxatda ushbu xatti-harakatni ifodalovchi umumiy `Xulosa` traitining ta'rifi ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">Roʻyxat 10-12: `umumiy_xulosa` metodi bilan taʼminlangan xatti-harakatlardan iborat `Xulosa` traiti</span>

Bu yerda biz `trait` kalit so'zidan foydalanib traitni e'lon qilamiz, so'ngra belgi nomi, bu holda `Xulosa`. Shuningdek, biz ushbu traitni `pub` deb e’lon qildik, shunda bu cratega bog‘liq bo‘lgan cratelar ham bu traitdan foydalanishi mumkin, buni bir necha misollarda ko‘ramiz. Jingalak qavslar ichida biz ushbu traitni amalga oshiradigan turlarning xatti-harakatlarini tavsiflovchi metod imzolarini e'lon qilamiz, bu holda `fn umumiy_xulosa(&self) -> String`.

Metod imzosidan so'ng, jingalak qavslar ichida amalga oshirish o'rniga, biz nuqta-verguldan foydalanamiz. Ushbu traitni amalga oshiradigan har bir tur metod tanasi uchun o'ziga xos xatti-harakatni ta'minlashi kerak. Kompilyator `Xulosa` traitiga ega boʻlgan har qanday turda aynan shu imzo bilan aniqlangan `umumiy_xulosa` metodi boʻlishini talab qiladi.

Traitining tanasida bir nechta metodlar bo'lishi mumkin: metod imzolari har bir satrda bittadan ko'rsatilgan va har bir satr nuqtali vergul bilan tugaydi.

### Turga xos traitni amalga oshirish

Endi biz `Xulosa` traiti metodlarining kerakli imzolarini aniqlaganimizdan so‘ng, uni media agregatorimizdagi turlarga qo‘llashimiz mumkin. 10-13 roʻyxat sarlavhadan foydalanadigan `YangiMaqola` structidagi `Xulosa` traitining amalga oshirilishini koʻrsatadi, muallif va `umumiy_xulosa` qaytish qiymatini yaratish uchun joy. `Maqola` structi uchun biz `umumiy_xulosa`ni foydalanuvchi nomi va undan keyin maqolaning butun matni sifatida belgilaymiz, maqola mazmuni allaqachon 280 belgi bilan cheklangan deb hisoblaymiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">Roʻyxat 10-13: `Xulosa` traitini `YangiMaqola` va `Maqola` turlariga joriy qilish</span>

Traitni turga tatbiq etish odatiy usullarni amalga oshirishga o'xshaydi. Farqi shundaki, `impl` dan so'ng biz amalga oshirmoqchi bo'lgan trait nomini qo'yamiz, so'ng `for` kalit so'zidan foydalanamiz va keyin traitni amalga oshirmoqchi bo'lgan tur nomini belgilaymiz. `impl` blokida biz trait ta'rifi belgilagan metod imzolarini qo'yamiz. Har bir imzodan keyin nuqta-vergul qo'yish o'rniga, biz jingalak qavslardan foydalanamiz va metod tanasini o'ziga xos xatti-harakat bilan to'ldiramiz, biz traitning metodlari ma'lum bir turga ega bo'lishini xohlaymiz.

Kutubxona `YangiMaqola` va `Maqola`da `Xulosa` traitini joriy qilganligi sababli, crate foydalanuvchilari `YangiMaqola` va `Maqola` misollaridagi xususiyat metodlarini biz odatdagi metodlar deb ataganimizdek chaqirishlari mumkin. Yagona farq shundaki, foydalanuvchi o'ziga xos traitni turlari bilan bir qatorda qamrab olishi kerak. Binary crate bizning `aggregator` kutubxonamiz cratesidan qanday foydalanishi mumkinligiga misol:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Bu kod `1 ta yangi xabar: ismoilovdev: Rust kitobi juda foydali ekan, men juda ko'p bilimlarni o'zlashtirdim` chop etadi.

`aggregator` cratesiga bog'liq bo'lgan boshqa cratelar ham `Xulosa` traitini o'z turlari bo'yicha `Xulosa`ni amalga oshirish uchun qamrab olishi mumkin. E'tiborga olish kerak bo'lgan cheklashlardan biri shundaki, biz trait yoki turning hech bo'lmaganda bittasi bizning cratemiz uchun mahalliy(local) bo'lsa, biz traitni turga qo'llashimiz mumkin. Misol uchun, biz `Maqola` kabi maxsus turdagi `Display` kabi standart kutubxona traitlarini `aggregator` crate funksiyamizning bir qismi sifatida amalga oshirishimiz mumkin, chunki `Maqola` turi `aggregator` cratemiz uchun mahalliydir. Shuningdek, biz  `Vec<T>` da `Xulosa`ni `aggregator` cratemizda ham qo‘llashimiz mumkin, chunki `Xulosa` traiti `aggregator` cratemiz uchun mahalliydir.

Ammo biz tashqi turlarga tashqi traitlarni amalga oshira olmaymiz. Masalan, biz `aggregator` cratemiz ichida `Vec<T>` da `Display` traitini amalga oshira olmaymiz, chunki `Display` va `Vec<T>` ikkalasi ham standart kutubxonada belgilangan va bizning `aggregator` cratemiz uchun mahalliy emas. Bu cheklash *kogerentlik(coherence)* deb nomlangan xususiyatning bir qismi va aniqrog'i *yetim qoidasi(orphan rule)*, chunki ota-ona turi mavjud emasligi sababli shunday nomlangan. Bu qoida boshqa odamlarning kodi sizning kodingizni buzmasligini ta'minlaydi va aksincha. Qoidalarsiz ikkita crate bir xil turdagi bir xil traitni amalga oshirishi mumkin edi va Rust qaysi dasturdan foydalanishni bilmaydi.

### Standart ilovalar

Ba'zan har bir turdagi barcha metodlarni amalga oshirishni talab qilish o'rniga, traitdagi ba'zi yoki barcha metodlar uchun standart xatti-harakatlarga ega bo'lish foydali bo'ladi.
Keyin, biz traitni ma'lum bir turga qo'llaganimizda, har bir metodning standart xatti-harakatlarini saqlab qolishimiz yoki bekor qilishimiz mumkin.

Roʻyxat 10-14da biz 10-12 ro'yxatda bo'lgani kabi faqat metod imzosini belgilash o'rniga `Xulosa` traitining `umumiy_xulosa` metodi uchun standart qatorni belgilaymiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">Roʻyxat 10-14: `Xulosa` traitini `umumiy_xulosa` metodini standart boʻyicha amalga oshirish bilan aniqlash</span>

`YangiMaqola` misollarini umumlashtirish uchun standart ilovadan foydalanish uchun biz bo'sh `impl` blokini `impl Xulosa for YangiMaqola {}` bilan belgilaymiz.

Biz `YangiMaqola`da to‘g‘ridan-to‘g‘ri `umumiy_xulosa` metodini endi aniqlamasak ham, biz standart bo‘yicha dasturni taqdim etdik va `YangiMaqola` `Xulosa` traitini amalga oshirishini belgilab oldik. Natijada, biz hali ham `YangiMaqola` misolida `umumiy_xulosa` metodini quyidagicha chaqirishimiz mumkin:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Bu kod `Yangi maqola mavjud! (Batafsil...)`ni chop etadi.

Standart dasturni yaratish bizdan 10-13 roʻyxatdagi `Maqola`dagi `Xulosa`ni amalga oshirish haqida biror narsani oʻzgartirishimizni talab qilmaydi. Buning sababi, standart dasturni bekor qilish sintaksisi standart dasturga ega bo'lmagan trait metodini amalga oshirish sintaksisi bilan bir xil.

Standart ilovalar bir xil traitga ega bo'lgan boshqa metodlarni chaqirishi mumkin, hatto bu boshqa metodlarda standart dastur bo'lmasa ham. Shunday qilib, trait juda ko'p foydali funksiyalarni taqdim etishi mumkin va amalga oshiruvchilardan faqat uning kichik qismini ko'rsatishni talab qiladi. Misol uchun, biz `Xulosa` traitini amalga oshirish zarur bo'lgan `muallif_haqida` metodiga ega bo'lish uchun belgilashimiz va keyin `muallif_haqida` metodini chaqiradigan standart amalga oshirishga ega bo'lgan `umumiy_xulosa` metodini belgilashimiz mumkin:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

`Xulosa` ning ushbu versiyasidan foydalanish uchun biz faqat bir turdagi traitni amalga oshirganimizda `muallif_haqida` ni aniqlashimiz kerak:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

`muallif_haqida` ni aniqlaganimizdan so'ng, biz `Maqola` structi misollarida `umumiy_xulosa` deb atashimiz mumkin va `umumiy_xulosa` standart bajarilishi biz taqdim etgan `muallif_haqida` ta'rifini chaqiradi. Biz `muallif_haqida` ni qo'llaganimiz sababli, `Xulosa` traiti bizga boshqa kod yozishni talab qilmasdan `umumiy_xulosa` metodining harakatini berdi.

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Bu kod `1 ta yangi xabar: (Batafsil: @ismoilovdev...)` ni chop etadi.

Shuni esda tutingki, xuddi shu metodni bekor qilish orqali standart dasturni chaqirish mumkin emas.

### Traitlar parametr sifatida

Endi siz traitlarni qanday aniqlash va amalga oshirishni bilganingizdan so'ng, biz ko'plab turlarni qabul qiladigan funksiyalarni aniqlash uchun traitlardan qanday foydalanishni o'rganishimiz mumkin. Biz 10-13 roʻyxatdagi `YangiMaqola` va `Maqola` turlari uchun joriy qilingan `Xulosa` traitidan foydalanamiz, uning `element` parametri boʻyicha umumlashtirish metodlini chaqiradigan `xabar_berish` funksiyasini belgilaymiz, u `Xulosa` traitini amalga oshiradi. Buning uchun biz `impl Trait` sintaksisidan foydalanamiz, masalan:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

`element` parametri uchun aniq tur o'rniga biz `impl` kalit so'zini va trait nomini belgilaymiz. Ushbu parametr belgilangan traitni amalga oshiradigan har qanday turni qabul qiladi. `xabar_berish` qismida biz `Xulosa` traitidan kelib chiqadigan `element` bo‘yicha har qanday metodlarni chaqirishimiz mumkin, masalan, `umumiy_xulosa`. Biz `xabar_berish` ga chaiqruv  qilishimiz va `YangiMaqola` yoki `Maqola` ning istalgan misolida o'tishimiz mumkin. Funksiyani `String` yoki `i32` kabi boshqa har qanday turdagi chaqiruvchi kod kompilyatsiya qilinmaydi, chunki bu turlar `Xulosa` ni amalga oshirmaydi.

<!-- Old headings. Do not remove or links may break. -->
<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Traitlarni cheklash sintaksisi

`impl Trait` sintaksisi oddiy holatlar uchun ishlaydi, lekin aslida *trait bound* deb nomlanuvchi uzunroq shakl uchun sintaksis shakaridir; bu shunday ko'rinadi:

```rust,ignore
pub fn xabar_berish<T: Xulosa>(element: &T) {
    println!("Tezkor xabarlar! {}", element.umumiy_xulosa());
}
```

Ushbu uzunroq shakl oldingi bo'limdagi misolga teng, ammo batafsilroq. Trait chegaralarini ikki nuqta va ichki burchakli qavslardan keyin umumiy tur parametri e'lon qilingan holda joylashtiramiz.

`impl Trait` sintaksisi qulay va oddiy holatlarda ixchamroq kodni yaratadi, to'liqroq traitlar bilan bog'langan sintaksisi esa boshqa holatlarda ko'proq murakkablikni ifodalashi mumkin. Misol uchun, bizda `Xulosa` ni amalga oshiradigan ikkita parametr bo'lishi mumkin. Buni `impl Trait` sintaksisi bilan bajarish quyidagicha ko'rinadi:

```rust,ignore
pub fn xabar_berish(element1: &impl Xulosa, element2: &impl Xulosa) {
```

Agar biz ushbu funksiya `element1` va `element2` turli xil turlarga ega bo'lishini istasak, `impl Trait` dan foydalanish maqsadga muvofiqdir (agar ikkala tur ham `Xulosa`ni qo'llasa). Agar biz ikkala parametrni bir xil turga ega bo'lishga majburlamoqchi bo'lsak, quyidagi kabi trait bounddan foydalanishimiz kerak:

```rust,ignore
pub fn xabar_berish<T: Xulosa>(element1: &T, element2: &T) {
```

`element1` va `element2` parametrlarining turi sifatida belgilangan umumiy `T` turi funksiyani shunday cheklaydiki, `element1` va `element2` uchun argument sifatida berilgan qiymatning aniq turi bir xil bo`lishi kerak.

#### `+` sintaksisi bilan bir nechta trait chegaralarini belgilash

Bundan tashqari, biz bir nechta traitlarni belgilashimiz mumkin. Aytaylik, biz `xabar_berish` funksiyasidan display formatlash hamda `element` bo‘yicha `umumiy_xulosa`dan foydalanishni xohladik: biz `xabar_berish` ta'rifida `element` `Display` va `Xulosa` ni ham amalga oshirishi kerakligini belgilaymiz. Buni `+` sintaksisi yordamida amalga oshirishimiz mumkin:

```rust,ignore
pub fn xabar_berish(element: &(impl Xulosa + Display)) {
```

`+` sintaksisi generik turdagi belgilar chegaralari bilan ham amal qiladi:

```rust,ignore
pub fn xabar_berish<T: Xulosa+ Display>(element: &T) {
```

Belgilangan ikkita trait chegarasi bilan `xabar_berish` asosiy qismi `umumiy_xulosa` deb chaqirishi va `element`ni formatlash uchun `{}` dan foydalanishi mumkin.

#### `where` bandlari bilan aniqroq trait bounds(chegaralari)

Haddan tashqari ko'p belgilar boundlaridan foydalanish o'zining salbiy tomonlariga ega. Har bir generikning o'ziga xos trait boundlari bor, shuning uchun bir nechta umumiy turdagi parametrlarga ega funksiyalar funksiya nomi va uning parametrlar ro'yxati o'rtasida ko'plab belgilar bilan bog'liq ma'lumotlarni o'z ichiga olishi mumkin, bu funksiya imzosini o'qishni qiyinlashtiradi. Shu sababli, Rust funksiya imzosidan keyin `where` bandida trait boundlarini belgilash uchun muqobil sintaksisga ega.

```rust,ignore
fn boshqa_funksiya<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

biz `where` bandidan foydalanishimiz mumkin, masalan:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

Bu funksiya imzosi kamroq chalkash: funksiya nomi, parametrlar ro'yxati va qaytish turi bir-biriga yaqin bo'lib, ko'p trait boundlari bo'lmagan funksiyaga o'xshaydi.

### Traitlarni amalga oshiradigan Return(qaytaruvchi) turlar

Bu yerda ko'rsatilganidek, traitni amalga oshiradigan ba'zi turdagi qiymatni qaytarish(return) uchun `impl Trait` sintaksisini return holatida ham ishlatishimiz mumkin:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Qaytish(return) turi uchun `impl Xulosa` dan foydalanib, biz `return_xulosa` funksiyasi aniq turga nom bermasdan `Xulosa` traitini amalga oshiradigan ba'zi turlarni qaytarishini aniqlaymiz. Bunday holda, `return_xulosa` `Maqola` ni qaytaradi, lekin bu funksiyani chaqiruvchi kod buni bilishi shart emas.

Qaytish turini faqat u amalga oshiradigan traitga ko'ra belgilash qobiliyati, ayniqsa, biz 13-bobda ko'rib chiqiladigan closurelar va iteratorlar kontekstida foydalidir.  Closures va iteratorlar faqat kompilyator biladigan turlarni yoki belgilash uchun juda uzoq turlarni yaratadi. `impl Trait` sintaksisi sizga funksiya juda uzun turni yozishga hojat qoldirmasdan `Iterator` traitini amalga oshiradigan ba'zi turlarni qaytarishini qisqacha belgilash imkonini beradi.

Biroq, faqat bitta turni qaytarayotgan bo'lsangiz, `impl Trait` dan foydalanishingiz mumkin. Masalan, `YangiMaqola` yoki `Maqola`ni qaytaruvchi `impl Xulosa` sifatida ko‘rsatilgan qaytarish turiga ega bo‘lgan bu kod ishlamaydi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

`YangiMaqola` yoki `Maqola`ni qaytarishga `impl Trait` sintaksisi kompilyatorda qanday amalga oshirilishi bilan bog‘liq cheklovlar tufayli ruxsat berilmaydi. Ushbu xatti-harakat bilan funksiyani qanday yozishni biz 17-bobning ["Turli turdagi qiymatlarga ruxsat beruvchi trait ob'ektlaridan foydalanish"][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> bo'limida ko'rib chiqamiz.

### Metodlarni shartli ravishda amalga oshirish uchun Trait Boundlardan foydalanish

Umumiy turdagi parametrlardan foydalanadigan `impl` bloki bilan trait bounddan foydalanib, biz belgilangan traitlarni amalga oshiradigan turlar uchun metodlarni shartli ravishda amalga oshirishimiz mumkin. Masalan, 10-15-ro'yxatdagi `Pair<T>` turi har doim yangi `Pair<T>` nusxasini qaytarish uchun `new` funksiyasini amalga oshiradi (5-bobning [“Metodlarni aniqlash”][methods]<!-- ignore -->  boʻlimidan eslaylikki, `Self` bu `impl` bloki turiga tegishli turdagi taxallus(alias) boʻlib, bu holda `Pair<T>` boʻladi). Ammo keyingi `impl` blokida `Pair<T>` faqat `cmp_display` metodini qo'llaydi, uning ichki turi(inner type) `T` taqqoslash imkonini beruvchi `PartialOrd` traitini *va* chop etish imkonini beruvchi `Display` traittini amalga oshiradi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

<span class="caption">Ro'yxat 10-15: Trait boundga qarab generik tur bo'yicha shartli ravishda qo'llash metodlari</span>

Biz shartli ravishda boshqa traitni amalga oshiradigan har qanday tur uchun traitni amalga oshirishimiz mumkin. Trait boundlarni qondiradigan har qanday turdagi tarittni amalga oshirish *blanket implementations* deb nomlanadi va Rust standart kutubxonasida keng qo'llaniladi. Masalan, standart kutubxona `Display` traitini amalga oshiradigan har qanday turdagi `ToString` traitini amalga oshiradi. Standart kutubxonadagi `impl` bloki ushbu kodga o'xshaydi:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Standart kutubxonada bu keng qamrovli dastur mavjud bo'lganligi sababli, biz `Display` traitini amalga oshiradigan har qanday turdagi `ToString` traiti bilan aniqlangan `to_string` metodini chaqirishimiz mumkin. Masalan, biz butun sonlarni mos keladigan `String` qiymatlariga shunday aylantirishimiz mumkin, chunki butun sonlar `Display`ni amalga oshiradi:

```rust
let s = 3.to_string();
```

Blanket implementationlari "Implementors" bo'limidagi trait uchun texnik hujjatlarda ko'rinadi.

Traitlar va trait boundlar takrorlanishni kamaytirish uchun generik turdagi parametrlardan foydalanadigan kod yozishga imkon beradi, shuningdek, generik turning o'ziga xos xatti-harakatlariga ega bo'lishini kompilyatorga ko'rsatishga imkon beradi. Keyin kompilyator trait bilan bog'langan ma'lumotlardan bizning kodimiz bilan qo'llaniladigan barcha aniq turlar to'g'ri xatti-harakatni ta'minlaydiganligini tekshirish uchun foydalanishi mumkin. Dinamik ravishda tuzilgan tillarda, agar biz metodni aniqlamagan turdagi metodni chaqirsak, runtimeda xatoga yo'l qo'yamiz. Ammo Rust bu xatolarni vaqtni kompilyatsiya qilish uchun ko'chiradi, shuning uchun biz kodimiz ishga tushgunga qadar muammolarni hal qilishga majbur bo'lamiz. Bundan tashqari, biz runtimeda xatti-harakatni tekshiradigan kod yozishimiz shart emas, chunki biz kompilyatsiya vaqtida allaqachon tekshirganmiz.
Bu generiklarning moslashuvchanligidan voz kechmasdan ishlashni yaxshilaydi.

[using-trait-objects-that-allow-for-values-of-different-types]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[methods]: ch05-03-method-syntax.html#defining-methods
