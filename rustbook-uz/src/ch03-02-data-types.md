## Ma'lumotlar turlari

Rust-dagi har bir qiymat ma'lum bir *ma'lumot turiga* tegishli bo'lib, Rustga qanday ma'lumotlar ko'rsatilayotganligini bildiradi, shuning uchun u ushbu ma'lumotlar bilan qanday ishlashni biladi. Biz ikkita ma'lumotlar turini ko'rib chiqamiz: skalyar va birikma.

Esda tutingki, Rust *statik tarzda yozilgan* tildir, ya'ni kompilyatsiya vaqtida barcha o'zgaruvchilarning turlarini bilishi kerak. Kompilyator odatda qiymat va uni qanday ishlatishimiz asosida biz qaysi turdan foydalanmoqchi ekanligimiz haqida xulosa chiqarishi mumkin.
Ko‘p turlar mumkin bo‘lgan hollarda, masalan, 2-bobdagi [“Tahminni maxfiy raqam bilan solishtirish”][comparing-the-guess-to-the-secret-number]<!-- ignore --> bo‘limidagi `parse` yordamida `String`ni raqamli turga o‘zgartirganimizda, quyidagi turdagi izohni qo‘shishimiz kerak:

```rust
let taxmin: u32 = "42".parse().expect("Raqam emas!");
```

Oldingi kodda ko'rsatilgan `: u32` turidagi izohni qo'shmasak, Rust quyidagi xatoni ko'rsatadi, ya'ni kompilyator bizdan qaysi turdan foydalanishni xohlayotganimizni bilish uchun qo'shimcha ma'lumotga muhtoj:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Boshqa ma'lumotlar turlari uchun turli turdagi izohlarni ko'rasiz.

### Skalyar Turlar

*Skalyar* turi bitta qiymatni ifodalaydi. Rust to'rtta asosiy skalyar turga ega: integerlar, floating-point number, boolean va belgilar. Siz ularni boshqa dasturlash tillaridan bilishingiz mumkin. Keling, ularning Rustda qanday ishlashini ko'rib chiqaylik.

#### Integer Turlari

*Integer* kasr komponenti bo‘lmagan sondir. Biz 2-bobda `u32` tipidagi bitta *integer* sonni ishlatdik. Ushbu turdagi deklaratsiya u bilan bog'langan qiymat 32 bit bo'sh joyni egallagan belgisiz butun son bo'lishi kerakligini bildiradi (Signed integer sonlar `u` o'rniga `i` bilan boshlanadi). 3-1-jadvalda Rust-da o'rnatilgan integer son turlari ko'rsatilgan. Integer son qiymatining turini e'lon qilish uchun biz ushbu variantlardan foydalanishimiz mumkin.

<span class="caption">3-1-jadval: Rustdagi Integer sonlar turlari</span>

| Uzunlik | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Signedlar kichkina `i` harfi bilan boshlanadi, Unsigned esa kichik `u` harfi bilan boshlanadi.

Har bir variant signed yoki unsigned bo'lishi mumkin va aniq o'lchamga ega.
*Signed* va *Unsigned* raqam manfiy boʻlishi mumkinmi yoki yoʻqligini anglatadi, boshqacha qilib aytganda, raqam u bilan birga belgiga ega boʻlishi (signed) boʻlishi kerakmi yoki u faqat ijobiy bo'ladimi va shuning uchun belgisiz (unsigned) ifodalanishi mumkinmi. Bu raqamlarni qog'ozga yozishga o'xshaydi: belgi muhim bo'lsa, raqam ortiqcha yoki minus belgisi bilan ko'rsatiladi; ammo, agar raqamni ijobiy deb hisoblash xavfsiz bo'lsa, u hech qanday belgisiz ko'rsatiladi.
Signed raqamlar [ikkita to'ldiruvchi][twos-complement]<!-- ignore--> ko'rinish yordamida saqlanadi.


Har bir signed variant -(2<sup>n - 1</sup>) dan 2<sup>n -
1</sup> -1 gacha bo'lgan raqamlarni saqlashi mumkin, bu erda *n* variant foydalanadigan bitlar soni.
Shunday qilib, `i8` -(2<sup>7</sup>) dan 2<sup>7</sup> - 1, gacha bo'lgan raqamlarni saqlashi mumkin, bu tengdir -128 dan 127 gacha.
Unsigned variantlar 0 dan 2<sup>n</sup> - 1 gacha raqamlarni saqlashi mumkin, shuning uchun `u8` 0 dan 2<sup>8</sup> - 1 gacha bo'lgan raqamlarni saqlashi mumkin, bu 0 dan 255 gacha.

Bundan tashqari, `isize` va `usize` turlari dasturingiz ishlayotgan kompyuterning arxitekturasiga bog'liq bo'lib, u jadvalda “arch” sifatida ko'rsatilgan: agar siz 64 bitli arxitekturada bo'lsangiz 64 bit va 32 bitli arxitekturada bo'lsangiz 32 bit.

Integer sonlarni 3-2-jadvalda ko'rsatilgan istalgan shaklda yozishingiz mumkin. E'tibor bering, bir nechta raqamli turlar bo'lishi mumkin bo'lgan son harflari turni belgilash uchun `57u8` kabi tur qo'shimchasiga ruxsat beradi. Raqamni o'qishni osonlashtirish uchun `_` dan raqamli harflar ham foydalanishi mumkin, masalan, `1_000`, siz `1000` ni ko'rsatganingizdek bir xil qiymatga ega bo'ladi.

<span class="caption">3-2-jadval: Rustdagi Integer literallar</span>

| Raqamli harflar   | Misol         |
|-------------------|---------------|
| O'nlik            | `98_222`      |
| O'n oltilik       | `0xff`        |
| Sakkizlik         | `0o77`        |
| Ikkilik           | `0b1111_0000` |
| Bayt (faqat "u8") | `b'A'`        |

Xo'sh, qaysi turdagi integer sonni ishlatishni qanday bilasiz? Agar ishonchingiz komil bo'lmasa, Rustning standart sozlamalari odatda boshlash uchun yaxshi joylardir: integer son turlari standart bo'yicha `i32` dir. `isize` yoki `usize` dan foydalanadigan asosiy holat to'plamning bir turini indekslashdir.

> ##### Integer Overflow
>
> Aytaylik, sizda 0 dan 255 gacha bo'lgan qiymatlarni ushlab turadigan `u8` tipidagi o'zgaruvchi bor.
> Agar siz o'zgaruvchini ushbu diapazondan tashqaridagi qiymatga o'zgartirishga harakat qilsangiz,
> masalan, 256, *integer overflow* sodir bo'ladi, bu ikki xatti-harakatdan biriga olib kelishi mumkin.
> Debug mode rejimida kompilyatsiya qilayotganingizda, Rust butun sonlarning to'lib ketishini
> tekshirishni o'z ichiga oladi, bu esa dasturni ishga tushirish vaqtida *panic* chiqaradi. Rust
> dastur xato bilan chiqqanda *panicking* atamasini ishlatadi; Biz panic haqida 9-bobdagi
> [“`panic` bilan tuzatib bo'lmaydigan xatolar”][unrecoverable-errors-with-panic]<!-- ignore -->
> bo'limda batafsil ko'rib chiqamiz
> 
> `--release` buyrug'i bilan reliz rejimida kompilyatsiya qilayotganingizda, Rust
> panic keltirib chiqaradigan butun sonlarni tekshirishni *o'z ichiga olmaydi*.
> overflow occur sodir bo'ladi Rust *ikkitasini to'ldiruvchi wrapni* bajaradi. Qisqa qilib
> aytganda, turdagi maksimal qiymatdan kattaroq qiymatlar, tur ushlab turishi mumkin bo'lgan minimal
> qiymatlargacha "wrap" ni tashkil qiladi. `u8` holatida 256 qiymati 0 ga, 257 qiymati
> 1 ga aylanadi va hokazo. Dastur panic qo'ymaydi, lekin o'zgaruvchi
> siz kutgan qiymatga ega bo'lmaydi. Butun sonlarni wrapga tayanish
> xato hisoblanadi. Owerflow ehtimolini aniq ko'rib chiqish uchun siz prime sonlar uchun
> standart kutubxona tomonidan taqdim etilgan ushbu metodlar oilalaridan foydalanishingiz mumkin:
> 
> * Barcha modelarni `wrapping_*` methodlari bilan oʻrash, masalan, `wrapping_add`.
> * Agar `checked_*` methodlari owerflow boʻlsa, `None` qiymatini qaytaring.
> * Qiymat va boolean qiymatni qaytaring, bu `overflowing_*` methodlari
>   bilan overflow bo'lganini ko'rsatadi.
> * Qiymatning minimal yoki maksimal qiymatlarida `saturating_*`
>   methodllari bilan saturate bo'lgan.

#### Floating-Point Turlari

Rust shuningdek *floating-point raqamlar* uchun ikkita primitive turga ega, ular kasrli raqamlardir.
Rust-ning floating-point turlari `f32` va `f64` bo'lib, ular mos ravishda 32 bit va 64 bit o'lchamga ega.
Standart tur `f64` dir, chunki zamonaviy protsessorlarda u `f32` bilan bir xil tezlikda, lekin aniqroq bo'lishga qodir.
Barcha floating-point turlari signeddir.

Bu yerda harakatdagi floating-point raqamlarni ko'rsatadigan misol:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Floating-point raqamlari IEEE-754 standartiga muvofiq taqdim etiladi. `f32` turi bitta aniqlikdagi floatdir va `f64` ikki tomonlama aniqlikka ega.

#### Raqamli operatsiyalar

Rust barcha turdagi raqamlar uchun kutilgan asosiy matematik operatsiyalarni qo'llab-quvvatlaydi: qo'shish, ayirish, ko'paytirish, bo'lish va qoldiq. Butun sonni bo'lish noldan eng yaqin butun songa qisqaradi. Quyidagi kod `let` iborasida har bir raqamli operatsiyadan qanday foydalanishni ko'rsatadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Ushbu bayonotlardagi har bir ifoda matematik operatordan foydalanadi va bitta qiymatga baholanadi, keyin esa o'zgaruvchiga bog'lanadi. [B ilovasi][appendix_b]<!-- ignore --> da
Rust taqdim etgan barcha operatorlar ro'yxati mavjud.

#### Boolean turi

Ko'pgina boshqa dasturlash tillarida bo'lgani kabi, Rust-da ham Boolean turi ikkita mumkin bo'lgan qiymatga ega: `true` va `false`. Boolean hajmi bir baytga teng.
Rustdagi boolean turi `bool` yordamida belgilanadi. Misol uchun:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean qiymatlardan foydalanishning asosiy metodi shartlardir, masalan, `if` ifodasidir. Rustda `if` iboralari qanday ishlashini [“Control Flow”][control-flow]<!-- ignore --> bo‘limida ko‘rib chiqamiz.

#### Belgilar(Character) turi

Rustning `char` turi tilning eng primitive alifbo turidir. Mana `char` qiymatlarini e'lon qilishning ba`zi misollari:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

E'tibor bering, biz qo'sh tirnoq ishlatadigan satr harflaridan farqli o'laroq, `char` harflarini bitta tirnoq bilan belgilaymiz. Rustning `char` turi to'rt bayt o'lchamga ega va Unicode Scalar qiymatini ifodalaydi, ya'ni u ASCIIdan ko'ra ko'proq narsani anglatishi mumkin.
Urg'uli harflar; Xitoy, yapon va koreys belgilar; emoji; va nol kenglikdagi boʻshliqlar Rust-dagi barcha haqiqiy `char` qiymatlaridir. Unicode Scalar qiymatlari `U+0000`dan `U+D7FF`gacha va `U+E000`dan `U+10FFFF`gacha.
Biroq, “character” aslida Unicode-da tushuncha emas, shuning uchun “character” nima ekanligi haqidagi Rustdagi `char` bilan mos kelmasligi mumkin. Biz ushbu mavzuni 8-bobdagi [“UTF-8 kodlangan matnni satrlar bilan saqlash”][strings]<!-- ignore --> bo'limida batafsil muhokama qilamiz.

### Murakkab turlar

*Murakkab turlar* bir nechta qiymatlarni bir turga to'plashi mumkin.Rust ikkita primitive birikma turiga ega: tuplelar va arraylar.

#### Tuple turi

*tuple* - bu turli xil turlarga ega bo'lgan bir qator qiymatlarni bitta qo'shma turga birlashtirishning umumiy metodi.Tuplelar belgilangan uzunlikka ega: bir marta e'lon qilingandan so'ng, ular o'sishi yoki kichrayishi mumkin emas.

Qavslar ichida vergul bilan ajratilgan qiymatlar ro'yxatini yozish orqali tuple yaratamiz. Tupledagi har bir pozitsiya o'z turiga ega va tupledagi turli qiymatlarning turlari bir xil bo'lishi shart emas. Ushbu misolda biz ixtiyoriy turdagi izohlarni qo'shdik:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

`tup` o'zgaruvchisi butun tuplega bog'lanadi, chunki tuple bitta birikma element hisoblanadi. Tupledan individual qiymatlarni olish uchun biz tuple qiymatini buzish uchun pattern moslashuvidan foydalanishimiz mumkin, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Bu dastur avval tuple yaratadi va uni `tup` o'zgaruvchisiga bog'laydi.Keyin u `tup`ni olish va uni uchta alohida o‘zgaruvchiga, `x`, `y` va `z` ga aylantirish uchun `let` bilan pattern ishlatadi. Bu  *destruktura* deb ataladi, chunki u bitta tupleni uch qismga ajratadi. Nihoyat, dastur `y` qiymatini chop etadi, bu `6,4`.

Shuningdek, biz to'g'ridan-to'g'ri nuqta (`.`) va undan keyin kirishni xohlagan qiymat indeksidan foydalanib, tuple elementiga kirishimiz mumkin. Misol uchun:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Bu dastur `x` tuplesini yaratadi va so'ngra o'z indekslari yordamida tuplening har bir elementiga kiradi. Ko'pgina dasturlash tillarida bo'lgani kabi, tupledagi birinchi indeks 0 ga teng.

Hech qanday qiymatsiz tuple maxsus nomga, *unit* ega. Bu qiymat va unga mos keladigan tur `()` yoziladi va bo'sh qiymat yoki bo'sh qaytish turini ifodalaydi. Ifodalar, agar ular boshqa qiymatni qaytarmasa, bilvosita birlik qiymatini qaytaradi.

#### Array Turi

Bir nechta qiymatlar to'plamiga ega bo'lishning yana bir usuli *array*dir. Tupledan farqli o'laroq, arrayning har bir elementi bir xil turdagi bo'lishi kerak. Ba'zi boshqa tillardagi arraylardan farqli o'laroq, Rustdagi arraylar belgilangan uzunlikka ega.

Biz arraydagi qiymatlarni kvadrat qavslar ichida vergul bilan ajratilgan ro'yxat sifatida yozamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arraylar maʼlumotlaringizni toʻplamga emas, balki stekga ajratishni istasangiz foydali boʻladi (biz [4-bobda][stack-and-heap]<!-- ignore -->) stek va toʻplam haqida koʻproq gaplashamiz yoki sizda har doim maʼlum miqdordagi elementlar mavjudligini taʼminlashni istasangiz).
Array vektor turi kabi moslashuvchan emas. *Vektor* standart kutubxona tomonidan taqdim etilgan o'xshash to'plam turi bo'lib, uning hajmini o'stirish yoki kichraytirishi mumkin. Agar array yoki vektordan foydalanishga ishonchingiz komil bo'lmasa, vektordan foydalanishingiz mumkin.
[8-bobda][vectors]<!-- ignore --> vektorlar batafsilroq muhokama qilinadi.

Biroq, agar elementlar sonini o'zgartirish kerak bo'lmasligini bilsangiz, arraylar foydaliroq bo'ladi. Misol uchun, agar siz dasturda oy nomlaridan foydalansangiz, vektordan ko'ra massivdan foydalanar edingiz, chunki u har doim 12 ta elementdan iborat bo'lishini bilasiz:

```rust
let oylar = ["Yanvar", "Fevral", "Mart", "Aprel", "May", "Iyun", "Iyul",
              "Avgust", "Setabr", "Oktabr", "Noyabr", "Dekabr"];
```

Siz har bir element turi, nuqta-vergul va arraydagi elementlar soni bilan kvadrat qavslar yordamida array turini yozasiz, masalan:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Bu erda `i32` har bir elementning turi. Nuqtali verguldan keyin `5` raqami array beshta elementdan iboratligini bildiradi.

Bundan tashqari, har bir element uchun bir xil qiymatni o'z ichiga olgan arrayni boshlang'ich qiymatdan keyin nuqta-vergul qo'yib, so'ngra bu yerda ko'rsatilgandek kvadrat qavs ichida array uzunligini belgilash orqali ishga tushirishingiz mumkin:

```rust
let a = [3; 5];
```

`a` nomli array dastlab `3` qiymatiga o'rnatiladigan `5` elementni o'z ichiga oladi. Bu `let a = [3, 3, 3, 3, 3];` yozish bilan bir xil, ammo qisqaroq tarzda.

##### Array elementlariga kirish

Array - bu stekda taqsimlanishi mumkin bo'lgan ma'lum, qat'iy o'lchamdagi xotiraning bitta bo'lagi. Siz indekslash yordamida array elementlariga kirishingiz mumkin, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Bu misolda `birinchi` deb nomlangan o‘zgaruvchi `1` qiymatini oladi, chunki bu arraydagi `[0]` indeksidagi qiymatdir. `ikkinchi` deb nomlangan ozgaruvchi arraydagi `[1]` indeksidan `2` qiymatini oladi.

##### Yaroqsiz Array elementlariga kirish

Keling, array oxiridan o‘tgan array elementiga kirishga harakat qilsangiz nima bo‘lishini ko‘rib chiqamiz. Aytaylik, foydalanuvchidan array indeksini olish uchun 2-bobdagi taxminiy o‘yinga o‘xshash ushbu kodni ishlatasiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Ushbu kod muvaffaqiyatli kompilyatsiya qilinadi.Agar siz ushbu kodni `cargo run` yordamida ishga tushirsangiz va `0`, `1`, `2`, `3` yoki `4` kiritsangiz, dastur arraydagi ushbu indeksdagi mos qiymatni chop etadi. Buning o'rniga array oxiridan o'tgan raqamni kiritsangiz, masalan, `10`, siz shunday chiqishni ko'rasiz:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Dastur indekslash operatsiyasida yaroqsiz qiymatdan foydalanish nuqtasida *runtime* xatosiga olib keldi. Dastur xato xabari bilan chiqdi va yakuniy `println!` bayonotini bajarmadi. Indekslash yordamida elementga kirishga harakat qilganingizda, Rust siz ko'rsatgan indeks array uzunligidan kamroq ekanligini tekshiradi. Agar indeks uzunlikdan kattaroq yoki unga teng bo'lsa, Rust panic chiqaradi. Bu tekshirish runtimeda amalga oshirilishi kerak, ayniqsa bu holatda, chunki kompilyator foydalanuvchi kodni keyinroq ishga tushirganda qanday qiymat kiritishini bila olmaydi.

Bu Rustning xotira xavfsizligi tamoyillarining amaldagi namunasidir. Ko'pgina low-leveldagi tillarda bunday tekshirish amalga oshirilmaydi va noto'g'ri indeksni taqdim etganingizda, yaroqsiz xotiraga kirish mumkin. Rust xotiraga kirishga ruxsat berish va davom ettirish o'rniga darhol chiqish orqali sizni bunday xatolardan himoya qiladi. 9-bobda Rust-ning xatolarini qanday hal qilish va siz panic qo'ymaydigan va yaroqsiz xotiraga kirishga ruxsat bermaydigan o'qilishi mumkin bo'lgan xavfsiz kodni qanday yozishingiz mumkinligi muhokama qilinadi.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
