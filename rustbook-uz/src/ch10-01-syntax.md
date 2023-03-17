## Generik ma'lumotlar turlari

Funksiya imzolari yoki structlar kabi elementlar uchun definitionlarni yaratish uchun biz generik(umumiy) ma'lumotlardan foydalanamiz, keyin ularni turli xil aniq ma'lumotlar turlari bilan ishlatishimiz mumkin. Keling, avval generiklar yordamida funksiyalar, structlar, enumlar va metodlarni qanday aniqlashni ko'rib chiqaylik. Keyin biz generiklar kod ishlashiga qanday ta'sir qilishini muhokama qilamiz.

### Funksiya ta'riflarida

Generiklardan foydalanadigan funksiyani belgilashda biz generiklarni funksiya imzosiga joylashtiramiz, u yerda biz odatda parametrlarning ma'lumotlar turlarini va qiymatni qaytaramiz. Bu bizning kodimizni yanada moslashuvchan qiladi va kodning takrorlanishining oldini olish bilan birga funksiyamizni chaqiruvchilarga ko'proq funksionallik beradi.

`eng_katta` funksiyamizni davom ettirsak, 10-4 roʻyxatda ikkalasi ham boʻlakdagi eng katta qiymatni topadigan ikkita funksiya koʻrsatilgan. Keyin biz ularni generiklardan foydalanadigan yagona funksiyaga birlashtiramiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">Roʻyxat 10-4: Ikki funksiya faqat nomlari va imzolaridagi turlari bilan farqlanadi</span>

`eng_katta_i32` funksiyasi biz 10-3 roʻyxatda ajratib olingan funksiya boʻlib, u boʻlakdagi eng katta `i32`ni topadi. `eng_katta_char` funksiyasi bo‘lakdagi eng katta `char`ni topadi. Funksiya organlari bir xil kodga ega, shuning uchun bitta funksiyaga generik turdagi parametrni kiritish orqali takrorlanishni bartaraf qilaylik.

Yangi bitta funksiyada turlarni parametrlash uchun, biz funksiyaning qiymat parametrlari uchun qilganimiz kabi, tur parametrini nomlashimiz kerak. Tur parametri nomi sifatida istalgan identifikatordan foydalanishingiz mumkin. Lekin biz `T` dan foydalanamiz, chunki Rust-dagi parametr nomlari odatda qisqa, koʻpincha harfdan iborat boʻladi va Rustning tur nomlash konventsiyasi UpperCamelCase hisoblanadi. “type(tur)” so'zining qisqartmasi `T`, Rust dasturchilarining ko'pchiligining standart tanlovidir.

Funksiya tanasida parametrdan foydalanganda, biz imzoda parametr nomini e'lon qilishimiz kerak, shunda kompilyator bu nom nimani anglatishini biladi.
Xuddi shunday, biz funktsiya imzosida tup parametri nomini ishlatganimizda, uni ishlatishdan oldin parametr nomini e'lon qilishimiz kerak. Generik `eng_katta` funksiyani aniqlash uchun burchakli qavslar ichida `<>` nomi deklaratsiyasini funksiya nomi va parametrlar ro'yxati orasiga qo'ying, masalan:

```rust,ignore
fn eng_katta<T>(list: &[T]) -> &T {
```

Biz bu taʼrifni shunday oʻqiymiz: `eng_katta` funksiyasi `T` turiga nisbatan umumiydir. Bu funksiya `list` nomli bitta parametrga ega, bu `T` turidagi qiymatlar bo'lagidir. `eng_katta` funksiya bir xil turdagi `T` qiymatiga referenceni qaytaradi.

10-5 ro'yxatda imzodagi umumiy ma'lumotlar turidan foydalangan holda birlashtirilgan `eng_katta` funksiya ta'rifi ko'rsatilgan. list shuningdek, funktsiyani `i32` yoki `char` qiymatlari bilan qanday chaqirishimiz mumkinligini ko'rsatadi. E'tibor bering, bu kod hali kompilyatsiya qilinmaydi, ammo biz uni ushbu bobda keyinroq tuzatamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">Ro'yxat 10-5: Generik turdagi parametrlardan foydalangan holda `eng_katta` funksiya; bu hali kompilyatsiya qilinmagan</span>

Agar dasturni hozir kompilyatsiya qilsak, biz quyidagi xatolikni olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

Yordam matnida `std::cmp::PartialOrd` qayd etilgan, bu *trait* va biz keyingi bo'limda traitlar haqida gaplashamiz. Hozircha shuni bilingki, bu xato `eng_katta` tanasi `T` bo'lishi mumkin bo'lgan barcha mumkin bo'lgan turlar uchun ishlamasligini bildiradi. Kod tanasidagi `T` turidagi qiymatlarni solishtirmoqchi bo'lganimiz uchun biz faqat qiymatlari ordere qilinadigan turlardan foydalanishimiz mumkin. Taqqoslashni yoqish uchun standart kutubxona `std::cmp::PartialOrd` traitiga ega bo'lib, uni turlarga tatbiq etishingiz mumkin (bu trait haqida batafsil ma'lumot uchun C ilovasiga qarang). Yordam matnining taklifiga amal qilib, biz `T` uchun amal qiladigan turlarni faqat `PartialOrd`-ni qo'llaydiganlar bilan cheklaymiz va bu misol kompilyatsiya qilinadi, chunki standart kutubxona `PartialOrd`ni ham `i32` va `char` da qo'llaydi.

### Struktura Definitionlarida

Shuningdek, biz `<>` sintaksisi yordamida bir yoki bir nechta maydonlarda generik turdagi parametrlardan foydalanish uchun structlarni belgilashimiz mumkin. Ro'yxat 10-6 har qanday turdagi `x` va `y` koordinata qiymatlarini saqlash uchun `Point<T>` structni belgilaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">10-6 roʻyxat: `T` turidagi `x` va `y` qiymatlarini oʻz ichiga olgan `Point<T>` structi</span>

Struktura ta'riflarida generiklardan foydalanish sintaksisi funksiya ta'riflarida qo'llaniladigan sintaksisiga juda oʻxshaydi. Birinchidan, burchakli qavslar ichida strukturaning nomidan keyin tur parametrining nomini e'lon qilamiz. Keyin biz aniq ma'lumotlar turlarini ko'rsatadigan struct ta'rifida generik turdan foydalanamiz.

Esda tutingki, biz `Point<T>`ni aniqlash uchun faqat bitta generik turdan foydalanganmiz, bu taʼrifda aytilishicha, `Point<T>` structi ba'zi bir `T` turiga nisbatan umumiy boʻlib, `x` va `y` maydonlari qaysi turdagi boʻlishidan qatʼi nazar bir xil turdagi dir. Agar biz 10-7 ro'yxatdagi kabi har xil turdagi qiymatlarga ega bo'lgan `Point<T>` nusxasini yaratsak, bizning kodimiz kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">Roʻyxat 10-7: `x` va `y` maydonlari bir xil turdagi boʻlishi kerak, chunki ikkalasi ham bir xil umumiy maʼlumotlar turi `T`ga ega.</span>

Ushbu misolda, biz `x` ga 5 butun qiymatini belgilaganimizda, kompilyatorga `T` generik turi `Point<T>` misoli uchun butun son bo'lishini bildiramiz. Keyin biz `x` bilan bir xil turga ega ekanligini aniqlagan `y` uchun 4.0 ni belgilaganimizda, biz quyidagi turdagi nomuvofiqlik xatosini olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

`x` va `y` ikkalasi ham generik bo'lgan, lekin har xil turlarga ega bo'lishi mumkin bo'lgan `Point` strukturasini aniqlash uchun biz bir nechta generik turdagi parametrlardan foydalanishimiz mumkin. Masalan, 10-8 roʻyxatda biz `Point` taʼrifini `T` va `U` turlari boʻyicha umumiy qilib oʻzgartiramiz, bunda `x` `T` turiga, `y` esa `U` turiga tegishli.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">Roʻyxat 10-8: `x` va `y` har xil turdagi qiymatlar boʻlishi uchun ikki turdagi umumiy `Point<T, U>`.</span>

Endi ko'rsatilgan `Point` ning barcha misollariga ruxsat berilgan! Ta'rifda siz xohlagancha turdagi parametrlardan generik foydalanishingiz mumkin, lekin bir nechtadan ko'proq foydalanish kodingizni o'qishni qiyinlashtiradi. Agar siz kodingizda ko'plab generik turlar kerakligini aniqlasangiz, bu sizning kodingizni kichikroq qismlarga qayta qurish kerakligini ko'rsatishi mumkin.

### Enum Definitionlarida

Structlar bilan qilganimizdek, ularning variantlarida generik ma'lumotlar turlarini saqlash uchun enumlarni belgilashimiz mumkin. Biz 6-bobda foydalanilgan standart kutubxona taqdim etadigan `Option<T>` enumini yana bir ko'rib chiqamiz:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Bu ta'rif endi siz uchun yanada ma'noli bo'lishi kerak. Ko'rib turganingizdek, `Option<T>` enum `T` turiga nisbatan generik va ikkita variantga ega: `T` turidagi bitta qiymatga ega `Some` va hech qanday qiymatga ega bo'lmagan `None` varianti.
`Option<T>` enum yordamida biz ixtiyoriy qiymatning mavhum kontseptsiyasini ifodalashimiz mumkin va `Option<T>` umumiy bo'lgani uchun biz ixtiyoriy qiymatning turi qanday bo'lishidan qat`i nazar, bu abstraktsiyadan foydalanishimiz mumkin.

Enumlar bir nechta generik turlardan ham foydalanishi mumkin. Biz 9-bobda aytib o'tgan `Result` enumining ta'rifi ushbu foydalanishga misoldir:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` enumlari ikki xil, `T` va `E` uchun generikdir va ikkita variantga ega: `T` turidagi qiymatga ega `OK` va `E` turidagi qiymatga ega bo'lgan `Err`. Bu taʼrif `Result` enumidan bizda muvaffaqiyatli boʻlishi mumkin boʻlgan (`T` turidagi qiymatni qaytarish) yoki muvaffaqiyatsiz boʻlishi mumkin boʻlgan (`E` turidagi xatolikni qaytarish) istalgan joyda foydalanishni qulay qiladi. Aslida, biz 9-3 ro'yxatdagi faylni shunday ochar edik, bu yerda fayl muvaffaqiyatli ochilganda `T` `std::fs::File` turi bilan to'ldirilgan va faylni ochishda muammolar yuzaga kelganda `E` `std::io::Error` turi bilan to`ldirilgan.

Kodingizdagi vaziyatlarni faqat ular ega bo'lgan qiymatlar turlarida farq qiluvchi bir nechta struct yoki enum ta'riflari bilan tanib olganingizda, uning o'rniga generik turlardan foydalanish orqali takrorlanishdan qochishingiz mumkin.

### Metod Definitionlarida

Biz structlar va enumlar bo'yicha metodlarni qo'llashimiz mumkin (5-bobda qilganimiz kabi) va ularning ta'riflarida generik turlardan ham foydalanishimiz mumkin. 10-9 ro'yxatda biz 10-6 ro'yxatda belgilagan `Point<T>` structi ko'rsatilgan va unda `x` nomli metod qo'llaniladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">Roʻyxat 10-9: `Point<T>` structida `x` nomli metodni qo'llash, bu `T` turidagi `x` maydoniga referenceni qaytaradi</span>

Bu yerda biz `Point<T>` da `x` nomli metodni belgilab oldik, u `x` maydonidagi ma`lumotlarga referenceni qaytaradi.

Esda tutingki, biz `impl` dan keyin `T` ni e'lon qilishimiz kerak, shuning uchun biz `Point<T>` turidagi metodlarni amalga oshirayotganimizni aniqlash uchun `T` dan foydalanishimiz mumkin. `T` ni `impl` dan keyin generik tur sifatida e'lon qilish orqali Rust `Point` dagi burchak qavslaridagi tur aniq tur emas, balki generik tur ekanligini aniqlay oladi. Biz ushbu umumiy parametr uchun struct taʼrifida eʼlon qilingan generik parametrdan boshqa nom tanlashimiz mumkin edi, lekin bir xil nomdan foydalanish odatiy hisoblanadi. Generik turni e'lon qiladigan `impl` ichida yozilgan metodlar, generik turdagi o'rnini bosadigan aniq turdagi qanday bo'lishidan qat'i nazar, har qanday turdagi namunada aniqlanadi.

Tur bo'yicha metodlarni belgilashda generik turlarga cheklovlarni ham belgilashimiz mumkin. Biz, masalan, har qanday generik turdagi `Point<T>` misollarida emas, balki faqat `Point<f32>` misollarida metodlarni amalga oshirishimiz mumkin. 10-10 ro'yxatda biz `f32` aniq turidan foydalanamiz, ya'ni `impl` dan keyin hech qanday turni e'lon qilmaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">Roʻyxat 10-10: `impl` bloki, faqat `T` generik tur parametri uchun ma`lum bir aniq turdagi strukturaga tegishli.</span>

Bu kod `Point<f32>` turi `kelib_chiqishidan_masofa` metodiga ega bo'lishini bildiradi; `T` `f32` turiga tegishli bo'lmagan `Point<T>` ning boshqa misollarida bu metod aniqlanmaydi. Metod bizning pointimizning koordinatadagi nuqtadan qanchalik uzoqligini o'lchaydi (0,0, 0,0) va faqat floating point turlari uchun mavjud bo'lgan matematik operatsiyalardan foydalanadi.

Struct taʼrifidagi generik turdagi parametrlar har doim ham oʻsha structning metod imzolarida foydalanadigan parametrlar bilan bir xil boʻlavermaydi. 10-11 roʻyxatda misolni aniqroq qilish uchun `Point`  structsi uchun `X1` va `Y1` va `aralashtirish` metodi imzosi uchun `X2` `Y2` generik turlari qoʻllaniladi. Metod yangi `Point` misolini yaratadi
`self` `Point` (`X1` turidagi) `x` qiymati va o'tkazilgan `Point` (`Y2` turidagi) `y` qiymati.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">Ro'yxat 10-11: O'zining strukturasi ta'rifidan farqli generik turlardan foydalanadigan metod</span>

`main`da biz `x` uchun `i32` (`5` qiymati bilan) va `y` uchun `f64` (`10,4` qiymati bilan) bo'lgan `Point` ni aniqladik. `p2` o'zgaruvchisi bu `Point` structi bo'lib, `x` (`Salom` qiymati bilan) va `y` (`c` qiymati bilan) uchun `char` bo'lagiga ega. `p1` da `aralashtirish`ni `p2` argumenti bilan chaqirish bizga `p3`ni beradi, bunda `x` uchun `i32` bo‘ladi, chunki `x` `p1` dan kelgan. `p3` o‘zgaruvchisi `y` uchun `char`ga ega bo‘ladi, chunki `y` `p2` dan kelgan. `println!` makro chaqiruvi `p3.x = 5, p3.y = c` ni chop etadi.

Ushbu misolning maqsadi ba'zi generik parametrlar `impl` bilan e'lon qilingan va ba'zilari metod ta'rifi bilan e'lon qilingan vaziyatni ko'rsatishdir. Bu erda `X1` va `Y1` generik parametrlari `impl` dan keyin e'lon qilinadi, chunki ular struct ta'rifiga mos keladi. `X2` va `Y2` generik parametrlari `fn aralashtirish` dan keyin e'lon qilinadi, chunki ular faqat metodga tegishli.

### Generiklar yordamida kodning ishlashi

Generik turdagi parametrlardan foydalanganda ish vaqti narxi bor yoki yo'qligini sizni qiziqtirgan bo'lishi mumkin. Yaxshi xabar shundaki, generik turlardan foydalanish dasturingizning aniq turlariga qaraganda sekinroq ishlashiga olib kelmaydi.

Rust buni kompilyatsiya vaqtida generiklar yordamida kodni monomorfizatsiya qilish orqali amalga oshiradi. *Monomorfizatsiya* - bu kompilyatsiya paytida ishlatiladigan aniq turlarni to'ldirish orqali generik kodni maxsus kodga aylantirish jarayoni. Ushbu jarayonda kompilyator biz 10-5 ro'yxatdagi generik funksiyani yaratishda qo'llagan qadamlarning teskarisini bajaradi: kompilyator generik kod chaqiriladigan barcha joylarni ko'rib chiqadi va generik kod chaqirilgan aniq turlar uchun kod ishlab chiqaradi.

Keling, bu standart kutubxonaning umumiy `Option<T>` enum yordamida qanday ishlashini ko'rib chiqaylik:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Rust ushbu kodni kompilyatsiya qilganda, u monomorfizatsiyani amalga oshiradi. Ushbu jarayon davomida kompilyator `Option<T>` misollarida ishlatilgan qiymatlarni o'qiydi va ikki xil `Option<T>`ni aniqlaydi: biri `i32`, ikkinchisi esa `f64`. Shunday qilib, u `Option<T>` ning umumiy ta'rifini `i32` va `f64` uchun ixtisoslashgan ikkita ta'rifga kengaytiradi va shu bilan umumiy ta'rifni o'ziga xos ta'riflar bilan almashtiradi.

Kodning monomorflashtirilgan versiyasi quyidagiga o'xshaydi (kompilyator biz tasvirlash uchun ishlatayotganimizdan boshqa nomlardan foydalanadi):

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Generik `Option<T>` kompilyator tomonidan yaratilgan maxsus ta`riflar bilan almashtiriladi. Rust generik kodni har bir misolda turni belgilaydigan kodga kompilyatsiya qilganligi sababli, biz generiklardan foydalanish uchun hech qanday ish vaqti to'lamaymiz. Kod ishga tushganda, agar biz har bir ta'rifni qo'lda takrorlagan bo'lsak, xuddi shunday ishlaydi. Monomorfizatsiya jarayoni Rust generiklarini runtimeda juda samarali qiladi.
