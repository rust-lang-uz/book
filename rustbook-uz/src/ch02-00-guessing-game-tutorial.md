# Taxmin qilish o'yinini dasturlash

Keling, birgalikda amaliy loyiha orqali Rustga o'taylik! Ushbu bob sizni bir nechta umumiy Rust tushunchalari bilan tanishtirib, ulardan haqiqiy dasturda qanday foydalanishni ko'rsatib beradi.  Siz `let`, `match`, usullari, bog'langan funksiyalar, external cratelardan foydalanish va boshqalar haqida bilib olasiz! Keyingi boblarda biz ushbu fikrlarni batafsilroq ko'rib chiqamiz. Ushbu bobda siz faqat asoslarni mashq qilasiz.

Biz klassik boshlang'ich dasturlash muammosini amalga oshiramiz: taxmin qilish o'yini. Bu qanday ishlaydi: dastur 1 dan 100 gacha tasodifiy butun son hosil qiladi. Keyin u o'yinchini taxmin qilishni taklif qiladi.Tahmin kiritilgandan so'ng, dastur taxmin kichik yoki katta ekanligini ko'rsatadi. Agar taxmin to'g'ri bo'lsa, o'yin tabrik xabarini chop etadi va chiqadi.

## Yangi loyiha yaratish

Yangi loyihani o'rnatish uchun 1-bobda yaratgan *projects* jildiga o'ting va Cargo-dan foydalanib yangi loyiha yarating, masalan:

```console
$ cargo new taxminiy_raqam
$ cd taxminiy_raqam
```

Birinchi `cargo new` buyrug'i birinchi argument sifatida loyiha nomini (`taxminiy_raqam`)ni oladi. Ikkinchi buyruq yangi loyiha jildiga kiradi.

Yaratilgan *Cargo.toml* fayliga qarang:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

1-bobda ko'rganingizdek, `cargo new` siz uchun “Hello, world!” so'zini chop etadigan dastur yaratadi. *src/main.rs* faylini tekshiring:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Keling, ushbu "Hello, world!" dasturni yarating va `cargo run` buyrug'i yordamida ishga tushiring :

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` buyrug‘i loyihani tezda takrorlash kerak bo‘lganda foydali bo‘ladi, biz bu o‘yinda qilganimizdek, keyingisiga o‘tishdan oldin har bir iteratsiyani tezda sinab ko‘ramiz.

*src/main.rs* faylini qayta oching. Siz ushbu fayldagi barcha kodlarni yozasiz.

## Taxmin qilish o'yiniga ishlov berish

Taxmin qilish o'yini dasturining birinchi qismi foydalanuvchi kiritishini so'raydi, ushbu kiritishni qayta ishlaydi va kirish kutilgan shaklda ekanligini tekshiradi. Boshlash uchun biz o'yinchiga taxmin kiritishga ruxsat beramiz. 2-1 ro'yxatdagi kodni *src/main.rs* ichiga kiriting.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Ro'yxat 2-1: Foydalanuvchi tomonidan taxmin qilinadigan va uni chop etadigan kod</span>

Ushbu kod juda ko'p ma'lumotlarni o'z ichiga oladi, shuning uchun uni satrga o'tkazamiz. Foydalanuvchi kiritishini olish va natijani chiqish sifatida chop etish uchun biz `io` input/output kutubxonasini qamrab olishimiz kerak. `io` kutubxonasi `std` deb nomlanuvchi standart kutubxonadan keladi:

```rust,ignore
use std::io;
```

Odatda, Rust standart kutubxonada belgilangan elementlar to'plamiga ega bo'lib, u har bir dastur doirasiga kiradi. Ushbu to'plam *prelude* deb ataladi va siz undagi hamma narsani [standart kutubxona texnik hujjatlarida][prelude] ko'rishingiz mumkin.

Agar siz foydalanmoqchi bo'lgan tur preludeda bo'lmasa, siz ushbu turni `use` iborasi bilan aniq kiritishingiz kerak. `std::io` kutubxonasidan foydalanish sizga bir qator foydali xususiyatlarni, jumladan, foydalanuvchi kiritishini qabul qilish imkoniyatini beradi.

1-bobda ko'rganingizdek, `main` funksiya dasturga kirish nuqtasidir:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` sintaksisi yangi funktsiyani e'lon qiladi; Qavslar, `()`, hech qanday parametr yo'qligini bildiradi; va jingalak qavs, `{`, funksiyaning asosiy qismini boshlaydi.

1-bobda ham bilib olganingizdek, `println!` bu ekranga satrni chop etuvchi makros:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Ushbu kod o'yin nima ekanligini ko'rsatuvchi va foydalanuvchidan ma'lumot so'rashni chop etadi.

### O'zgaruvchilar bilan qiymatlarni saqlash

Keyinchalik, foydalanuvchi ma'lumotlarini saqlash uchun *o'zgaruvchi* yaratamiz, masalan:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Endi dastur qiziqarli bo'lib bormoqda! Bu kichik satrda juda ko'p narsa bor. O'zgaruvchini yaratish uchun `let` iborasidan foydalanamiz. Mana yana bir misol:

```rust,ignore
let olmalar = 5;
```

Bu qator `olmalar` nomli yangi o‘zgaruvchini yaratadi va uni 5 qiymatiga bog‘laydi. Rustda o'zgaruvchilar standard bo'yicha o'zgarmasdir, ya'ni o'zgaruvchiga qiymat berganimizdan keyin qiymat o'zgarmaydi.Biz ushbu kontseptsiyani 3-bobdagi [”O'zgaruvchilar va O'zgaruvchanlik”][variables-and-mutability]<!-- ignore --> bo'limida batafsil muhokama qilamiz. Oʻzgaruvchini oʻzgaruvchan qilish uchun oʻzgaruvchi nomidan oldin `mut` qoʻshamiz:

```rust,ignore
let olmalar = 5; // o'zgarmas
let mut bananlar = 5; // o'zgaruvchan
```

> Eslatma: `//` sintaksisi satr oxirigacha davom etadigan izohni
> boshlaydi. Rust izohlarda hamma narsani e'tiborsiz qoldiradi.
> Izohlarni [3-bobda][comments]<!-- ignore --> batafsilroq muhokama qilamiz.

Taxmin qilish o'yin dasturiga qaytsak, endi bilasizki, `let mut taxmin` `taxmin` nomli o'zgaruvchan o'zgaruvchini kiritadi. Teng belgisi (`=`) Rustga biz hozir biror narsani oʻzgaruvchiga bogʻlamoqchi ekanligimizni bildiradi. Tenglik belgisining o'ng tomonida `taxmin` bog'langan qiymat joylashgan bo'lib, u `String::new` funksiyasini chaqirish natijasidir, bu `String`ning yangi nusxasini qaytaradi.
[String][string]<!-- ignore --> standart kutubxona tomonidan taqdim etilgan string turi bo'lib, u rivojlantirib boriladigan, UTF-8 kodlangan matn bitidir.

`::new` qatoridagi `::` sintaksisi `new` `String` tipidagi bog`langan funksiya ekanligini bildiradi. *Assosiatsiyalangan funksiya* bu funksiya
turida amalga oshiriladi, bu holda `String`. Ushbu `new` funksiya yangi, bo'sh qatorni yaratadi. Siz ko'p turdagi `new` funksiyani topasiz, chunki u qandaydir yangi qiymatni yaratadigan funksiyaning umumiy nomi.

To'liq `let mut taxmin = String::new();` qatori hozirda `String` ning yangi, bo'sh nusxasiga bog'langan o'zgaruvchan o'zgaruvchini yaratadi.

### Foydalanuvchi ma'lumotlarini qabul qilish

Eslatib o'tamiz, biz dasturning birinchi qatoriga `use std::io;` bilan standart kutubxonadan input/output funksiyasini kiritgan edik. Endi biz `io` modulidan `stdin` funksiyasini chaqiramiz, bu bizga foydalanuvchi kiritishini boshqarish imkonini beradi:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Agar biz dasturning boshida `use std::io;` bilan `io` kutubxonasini import qilmagan bo'lsak, biz ushbu funktsiya chaqiruvini `std::io::stdin` sifatida yozish orqali funksiyadan foydalanishimiz xam mumkin. `stdin` funksiyasi [`std::io::Stdin`][iostdin]<!-- ignore --> misolini qaytaradi, bu sizning terminalingiz uchun standart kirish uchun asosni ifodalovchi tur.

Keyinchalik, `.read_line(&mut taxmin)` qatori foydalanuvchidan ma'lumot olish uchun standart kiritish nuqtasidagi [`read_line`][read_line]<!--ignore --> usulini chaqiradi.
Shuningdek, foydalanuvchi kiritgan maʼlumotlarni qaysi qatorda saqlash kerakligini aytish uchun `read_line` ga argument sifatida `&mut taxmin` ni oʻtkazamiz. `read_line` ning toʻliq vazifasi foydalanuvchi nima yozganidan qatʼiy nazar standart kiritishga olish va uni satrga qoʻshishdir (uning mazmunini qayta yozmasdan), shuning uchun biz bu qatorni argument sifatida beramiz. String argumenti o'zgaruvchan bo'lishi kerak, shuning uchun usul string tarkibini o'zgartirishi mumkin.

`&` bu argument reference(havola) ekanligini bildiradi, bu sizga kodingizning bir nechta qismlariga ushbu ma'lumotni xotiraga bir necha marta nusxalash kerak bo'lmasdan bitta ma'lumotga kirish imkonini beradi. Referencelar murakkab xususiyat bo'lib, Rustning asosiy afzalliklaridan biri havolalardan foydalanish qanchalik xavfsiz va oson ekanligidir. Ushbu dasturni tugatish uchun ko'p bilimlrga ega bo'lishingiz shart emas. Hozircha siz bilishingiz kerak bo'lgan narsa shundaki, o'zgaruvchilar singari, havolalar ham standard bo'yicha o'zgarmasdir. Demak, uni oʻzgaruvchan qilish uchun `&taxmin` oʻrniga `&mut taxmin` yozish kerak. (4-bobda havolalar ko'proq va yaxshiroq tushuntiriladi)

<!-- Old heading. Do not remove or links may break. -->
<a id="handling-potential-failure-with-the-result-type"></a>

### Potensial nosozlikni `Result` turi bilan hal qilish

Biz hali ham ushbu kod qatori ustida ishlayapmiz. Biz hozir matnning uchinchi qatorini muhokama qilmoqdamiz, lekin u hali ham bitta mantiqiy kod qatorining bir qismi ekanligini unutmang. Keyingi qism bu method:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Biz ushbu kodni quyidagicha yozishimiz mumkin edi:

```rust,ignore
io::stdin().read_line(&mut taxmin).expect("Satrni o‘qib bo‘lmadi");
```

Biroq, bitta uzun qatorni o'qish qiyin, shuning uchun uni bo'lish yaxshidir. `.method_name()` sintaksisi bilan methodni chaqirganda uzun qatorlarni ajratishga yordam berish uchun yangi qator va boshqa bo'shliqlarni kiritish ko'pincha oqilona. Endi bu kod nima qilishini muhokama qilaylik.

Yuqorida aytib o'tilganidek, `read_line` foydalanuvchi kiritgan narsani biz unga o'tkazadigan qatorga qo'yadi, lekin u `Result` qiymatini ham qaytaradi. [`Result`][result]<!-- ignore --> - ko'pincha *enum* deb ataladigan [*enumeration*][enums]<!-- ignore -->, bu bir nechta mumkin bo'lgan holatlardan birida bo'lishi mumkin bo'lgan tur. Har bir mumkin bo'lgan holatni *variant* deb ataymiz.

[6-bobda][enums]<!-- ignore --> enumlar batafsilroq yoritiladi. Ushbu `Result` turlarining maqsadi xatolarni qayta ishlash ma'lumotlarini kodlashdir.

`Result` variantlari `Ok` va `Err`. `Ok` varianti operatsiya muvaffaqiyatli bo'lganligini bildiradi va `Ok` ichida muvaffaqiyatli yaratilgan qiymat.
`Err` varianti operatsiya bajarilmaganligini bildiradi va `Err` operatsiya qanday yoki nima uchun bajarilmagani haqida maʼlumotni oʻz ichiga oladi.

`Result` turidagi qiymatlar, har qanday turdagi qiymatlar kabi, ularda aniqlangan usullarga ega. `Result` misolida siz murojat qilishingiz mumkin bo'lgan [`expect` methodi][expect]<!-- ignore --> mavjud. Agar `Result` ning ushbu namunasi `Err` qiymati bo'lsa, `expect` dasturning ishlamay qolishiga olib keladi va `expect` ga argument sifatida siz uzatgan xabarni ko'rsatadi. Agar `read_line` usuli `Err`ni qaytarsa, bu asosiy operatsion tizimdan kelgan xato natijasi bo'lishi mumkin.

Agar `Result`ning ushbu namunasi `Ok` qiymati bo‘lsa, `expect` `Ok` ushlab turgan qaytarish qiymatini oladi va siz undan foydalanishingiz uchun aynan shu qiymatni sizga qaytaradi.
Bunday holda, bu qiymat foydalanuvchi kiritishidagi baytlar soni.

Agar siz `expect` ga murojat qilmasangiz, dastur kompilyatsiya qilinadi, lekin siz ogohlantirish olasiz:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust `read_line` dan qaytarilgan `Result` qiymatini ishlatmaganligingiz haqida ogohlantiradi, bu dastur mumkin bo'lgan xatoni hal qilmaganligini ko'rsatadi.

Ogohlantirishni yo'qotishning to'g'ri yo'li aslida xatolarni qayta ishlash kodini yozishdir, ammo bizning holatlarimizda muammo yuzaga kelganda biz ushbu dasturni ishdan chiqarishni xohlaymiz, shuning uchun biz `expect` dan foydalanishimiz mumkin. Xatolarni tiklash haqida [9-bobda]recover]<!-- ignore --> bilib olasiz.

### Qiymatlarni `println!`  bilan chop etish

Yopuvchi jingalak qavsdan tashqari, kodda hozirgacha muhokama qilinadigan yana bitta satr mavjud:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Ushbu satr foydalanuvchi kiritishini o'z ichiga olgan qatorni chop etadi. `{}` jingalak qavslar to'plami o'rnini egallaydi: `{}` qiymatini joyida ushlab turadigan qisqichbaqa qisqichlari deb tasavvur qiling. O'zgaruvchining qiymatini chop etishda o'zgaruvchi nomi jingalak qavslar ichiga kirishi mumkin. Ifodani baholash natijasini chop etishda format satriga bo'sh jingalak qavslarni joylashtiring, so'ngra har bir bo'sh jingalak qavs o'rnini egallagan holda bir xil tartibda chop etish uchun vergul bilan ajratilgan iboralar ro'yxati bilan format qatoriga amal qiling. O‘zgaruvchini va ifoda natijasini `println!` ga bitta chaqiruvda chop etish quyidagicha ko‘rinadi:

```rust
let x = 5;
let y = 10;

println!("x = {x} va y + 2 = {}", y + 2);
```

Bu kod `x = 5 va y = 12` ni chop etadi.

### Birinchi qismni sinovdan o'tkazish

Keling, taxmin qilish o'yinining birinchi qismini sinab ko'raylik. Uni `cargo run` yordamida ishga tushiring:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/taxminiy_raqam`
Raqamni topish o'yini!
Iltimos, taxminingizni kiriting.
6
Sizni taxminingiz: 6
```

Shu nuqtada, o'yinning birinchi qismi tugadi: biz klaviaturadan ma'lumotlarni olamiz va keyin uni chop etamiz.

## Yashirin raqam yaratish

Keyinchalik, foydalanuvchi taxmin qilishga harakat qiladigan maxfiy raqamni yaratishimiz kerak. Yashirin raqam har safar boshqacha bo'lishi kerak, shuning uchun o'yinni bir necha marta o'ynash qiziqarli bo'ladi. O'yin juda qiyin bo'lmasligi uchun biz 1 dan 100 gacha bo'lgan tasodifiy raqamdan foydalanamiz. Rust hali o'zining standart kutubxonasida tasodifiy raqamlar funksiyasini o'z ichiga olmaydi. Biroq, Rust jamoasi ushbu funksiyaga [`rand` crate][randcrate]i taqdim etadi.

### Ko'proq funksionallikka ega bo'lish uchun Cratedan foydalanish

Esda tutingki, crate Rust manba kodi fayllari to'plamidir. Biz qurayotgan loyiha *binary crate* bo'lib, u bajariladigan. `rand` crate boshqa dasturlarda foydalanish uchun moʻljallangan va mustaqil ravishda bajarib boʻlmaydigan kodni oʻz ichiga olgan *library crate*.

Cargoning tashqi cratelarni muvofiqlashtirishi bu erda Cargp haqiqatan ham ishlaydi. `rand` dan foydalanadigan kodni yozishdan oldin, biz *Cargo.toml* faylini `rand` cratesini dependency sifatida qo‘shish uchun o‘zgartirishimiz kerak. Hozir o‘sha faylni oching va Cargo siz uchun yaratgan`[dependencies]` bo‘limi sarlavhasi ostiga quyidagi qatorni qo‘shing.`rand` ni aynan bizda boʻlganidek, ushbu versiya raqami bilan belgilaganingizga ishonch hosil qiling, aks holda ushbu qoʻllanmadagi kod misollari ishlamasligi mumkin:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

*Cargo.toml* faylida sarlavhadan keyingi hamma narsa boshqa bo'lim boshlanmaguncha davom etadigan bo'limning bir qismidir. `[dependencies]` da siz Cargo loyihangiz qaysi tashqi cratelarga bog'liqligini va bu cratelarning qaysi versiyalari kerakligini aytasiz. Bunday holda, biz `rand` crateni `0.8.5` semantik versiya spetsifikatsiyasi bilan belgilaymiz. Cargo versiya raqamlarini yozish uchun standart bo'lgan [Semantic Versioning][semver]<!-- ignore -->ni (ba'zan *SemVer* deb ataladi) tushunadi. `0.8.5` spetsifikatsiyasi aslida `^0.8.5` ning qisqartmasi boʻlib, kamida 0.8.5, lekin 0.9.0 dan past boʻlgan har qanday versiyani bildiradi.

Cargo ushbu versiyalarni 0.8.5 versiyasiga mos keladigan umumiy API-larga ega deb hisoblaydi va bu spetsifikatsiya sizga ushbu bobdagi kod bilan tuziladigan so‘nggi patch versiyasini olishingizni kafolatlaydi. 0.9.0 yoki undan kattaroq versiyalar quyidagi misollar ishlatadigan API bilan bir xil bo'lishi kafolatlanmaydi.

Endi, hech qanday kodni o'zgartirmasdan, 2-2 ro'yxatda ko'rsatilganidek, loyihani build qilaylik.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Ro'yxat 2-2: rand cratesini dependency sifatida qo'shgandan so'ng `cargo build` dan olingan natija</span>

Siz turli xil versiya raqamlarini (lekin ularning barchasi SemVer tufayli kod bilan mos keladi!) va turli xil satrlarni (operatsion tizimga qarab) ko'rishingiz mumkin va satrlar boshqa tartibda bo'lishi mumkin.

Biz tashqi dependency qo'shganimizda, Cargo [Crates.io][cratesio] ma'lumotlarining nusxasi bo'lgan  *registry* dan dependency uchun zarur bo'lgan barcha narsalarning so'nggi versiyalarini oladi.Crates.io - bu Rust ekotizimidagi odamlar o'zlarining ochiq manbali Rust loyihalarini boshqalar foydalanishi uchun joylashtiradigan joy.

registrni yangilagandan so'ng, Cargo  `[dependencies]`  bo'limini tekshiradi va ro'yxatda hali yuklab olinmagan cratelarni yuklab oladi. Bu holatda, garchi biz faqat `rand` ni dependency sifatida ko'rsatgan bo'lsak-da, Cargo `rand` ishlashga bog'liq bo'lgan boshqa cratelarni ham oldi. Cratelarni yuklab olgandan so'ng, Rust ularni kompilyatsiya qiladi va keyin mavjud bo'lgan dependency bilan loyihani tuzadi.

Agar siz hech qanday o'zgartirishlarsiz darhol `cargo build` ni qayta ishga tushirsangiz, `Finished` qatoridan boshqa hech qanday natija olmaysiz. Cargo allaqachon dependencylarni yuklab olganini va kompilyatsiya qilganini biladi va siz *Cargo.toml* faylida ular haqida hech narsani o'zgartirmagansiz. Cargo, shuningdek, kodingiz haqida hech narsani o'zgartirmaganligingizni biladi, shuning uchun u ham uni qayta kompilyatsiya qilmaydi. Hech narsa qilmasdan, u shunchaki chiqib ketadi.

Agar siz *src/main.rs* faylini ochsangiz, ahamiyatsiz o'zgarishlarni amalga oshirsangiz va keyin uni saqlab va qayta build qilsangiz, siz faqat ikkita chiqish qatorini ko'rasiz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Bu satrlar shuni ko'rsatadiki, Cargo faqat *src/main.rs* fayliga kichik o'zgartirishingiz bilan buildni yangilaydi. Sizning dependencylaringiz o'zgarmadi, shuning uchun Cargo allaqachon yuklab olingan va ular uchun tuzilgan narsadan qayta foydalanishi mumkinligini biladi..

#### *Cargo.lock* fayli bilan qayta tiklanadigan tuzilmalarni ta'minlash

Cargoda siz yoki boshqa birov kodingizni har safar yaratganingizda bir xil artefaktni qayta tiklashingiz mumkinligini ta'minlaydigan mexanizm mavjud: Siz aksini ko'rsatmaguningizcha, cargo faqat siz ko'rsatgan dependency versiyalaridan foydalanadi. Masalan, kelasi hafta `rand` cratening 0.8.6 versiyasi chiqadi va bu versiyada muhim xatoliklar tuzatilgan, lekin u sizning kodingizni buzadigan regressiyani ham o‘z ichiga oladi. Buni hal qilish uchun Rust birinchi marta  `cargo build` dasturini ishga tushirganingizda *Cargo.lock* faylini yaratadi, shuning uchun biz endi bu *guessing_game* jildida mavjud.

Loyihani birinchi marta yaratganingizda, Cargo mezonlarga mos keladigan dependencylarning barcha versiyalarini aniqlaydi va keyin ularni *Cargo.lock* fayliga yozadi. Keyingi loyihangizni yaratganingizda, Cargo *Cargo.lock* fayli mavjudligini ko'radi va versiyalarni qayta aniqlash uchun barcha ishlarni bajarishdan ko'ra, u erda ko'rsatilgan versiyalardan foydalanadi. Bu sizga avtomatik ravishda takrorlanadigan tuzilishga ega bo'lish imkonini beradi. Boshqacha qilib aytganda, *Cargo.lock* fayli tufayli loyihangiz aniq yangilanmaguningizcha 0.8.5 da qoladi.
*Cargo.lock* fayli qayta tiklanadigan tuzilmalar uchun muhim bo'lgani uchun u ko'pincha loyihangizdagi kodning qolgan qismi bilan manba nazoratida tekshiriladi.

#### Yangi versiyani olish uchun Crateni yangilash

Crateni yangilamoqchi bo'lsangiz, Cargo `update` buyrug'ini beradi, bu buyruq *Cargo.lock* faylini e'tiborsiz qoldiradi va *Cargo.toml* dagi texnik xususiyatlaringizga mos keladigan barcha so'nggi versiyalarni aniqlaydi. Keyin Cargo ushbu versiyalarni *Cargo.lock* fayliga yozadi. Aks holda, standart bo'yicha, Cargo faqat 0.8.5 dan katta va 0.9.0 dan kichik versiyalarni qidiradi. Agar `rand` cratesi ikkita yangi 0.8.6 va 0.9.0 versiyalarini chiqargan bo'lsa, `cargo update` ni ishga tushirgan bo'lsangiz, quyidagilarni ko'rasiz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```

Cargo 0.9.0 versiyasiga e'tibor bermaydi. Bu vaqtda siz *Cargo.lock* faylingizda oʻzgarishlarni ham sezasiz, bunda siz hozir foydalanayotgan `rand`  cratesi versiyasi 0.8.6. `rand` 0.9.0 versiyasidan yoki 0.9.*x* seriyasining istalgan versiyasidan foydalanish uchun *Cargo.toml* faylini quyidagi koʻrinishda yangilashingiz kerak boʻladi:

```toml
[dependencies]
rand = "0.9.0"
```

Keyingi safar `cargo build`ni ishga tushirganingizda, Cargo mavjud cratelar reestrini yangilaydi va siz ko‘rsatgan yangi versiyaga muvofiq `rand` talablaringizni qayta baholaydi.

[Cargo][doccargo]<!-- ignore --> va uning [ekotizimlari][doccratesio]<!-- ignore --> haqida ko'p gapirish mumkin, biz ularni 14-bobda muhokama qilamiz, ammo hozircha bilishingiz kerak bo'lgan narsa shu. Cargo kutubxonalarni qayta ishlatishni juda osonlashtiradi, shuning uchun Rustaceans bir nechta paketlardan yig'ilgan kichikroq loyihalarni yozishga qodir.

### Tasodifiy raqamni yaratish

Keling, taxmin qilish uchun raqam yaratishda `rand` dan foydalanishni boshlaylik. Keyingi qadam 2-3 ro'yxatda ko'rsatilganidek *src/main.rs* ni yangilashdir.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Ro'yxat 2-3: Tasodifiy raqam yaratish uchun kod qo'shiladi</span>

Avval `use rand::Rng;` qatorini qo'shamiz. `Rng` traiti tasodifiy sonlar generatorlari qo'llaydigan usullarni belgilaydi va biz ushbu usullardan foydalanishimiz uchun bu trait mos bo'lishi kerak. 10-bobda traitlar batafsil yoritiladi.

Keyin o'rtada ikkita qator qo'shamiz. Birinchi qatorda biz `rand::thread_rng` funksiyasini chaqiramiz, bu bizga biz foydalanmoqchi bo'lgan tasodifiy sonlar generatorini beradi: joriy bajarilish oqimi uchun mahalliy bo'lgan va operatsion tizim tomonidan ekilgan. Keyin tasodifiy sonlar generatorida `gen_range` usulini chaqiramiz. Bu usul biz `use rand::Rng;`  iborasi bilan qamrab olgan `Rng` traiti bilan aniqlanadi. `gen_range` usuli argument sifatida diapazon ifodasini oladi va diapazonda tasodifiy son hosil qiladi. Biz bu yerda foydalanayotgan diapazon ifodasi turi `start..=end`  shaklini oladi va pastki va yuqori chegaralarni qamrab oladi, shuning uchun biz 1 va 100 oralig‘idagi raqamni so‘rash uchun `1..=100` ni belgilashimiz kerak. .


> Eslatma: Siz faqat qaysi traitlardan foydalanishni va qaysi methodlar va funktsiyalarni
> cratedan chaqirishni bila olmaysiz, shuning uchun har bir crateda foydalanish bo'yicha
> ko'rsatmalar mavjud. Cargo-ning yana bir qulay xususiyati shundaki, `cargo doc --open` buyrug'ini
> ishga tushirish sizning barcha dependencylar tomonidan taqdim etilgan texnik hujjatlarni
> mahalliy sifatida tuzadi va uni brauzeringizda ochadi. Agar siz `rand` cratedagi boshqa
> funksiyalarga qiziqsangiz, masalan, `cargo doc --open` ni ishga tushiring va chap tomondagi
> yon paneldagi `rand` tugmasini bosing.

Ikkinchi yangi qator maxfiy raqamni chop etadi. Bu dasturni ishlab chiqishda uni sinab ko'rishimiz uchun foydalidir, lekin biz uni oxirgi versiyadan o'chirib tashlaymiz. Agar dastur boshlanishi bilanoq javobni chop etsa, bu unchalik o'yin emas!

Dasturni bir necha marta ishga tushirishga harakat qiling:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/taxminiy_raqam`
Raqamni topish o'yini!
Yashirin raqam: 7
Iltimos, taxminingizni kiriting.
4
Siznig taxminingiz: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/taxminiy_raqam`
Raqamni topish o'yini!
Yashirin raqam: 83
Iltimos, taxminingizni kiriting.
5
Siznig taxminingiz: 5
```

Siz turli xil tasodifiy raqamlarni olishingiz kerak va ularning barchasi 1 dan 100 gacha raqamlar bo'lishi kerak. Ajoyib ish!

## Taxminni maxfiy raqam bilan solishtirish

Endi bizda foydalanuvchi kiritishi va tasodifiy raqam bor, biz ularni solishtirishimiz mumkin. Ushbu qadam 2-4 ro'yxatda ko'rsatilgan. E'tibor bering, bu kod hozircha kompilatsiya bo'lmaydi, biz tushuntiramiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Ro'yxat 2-4: Ikki raqamni solishtirishning mumkin bo'lgan qaytish qiymatlarini boshqarish</span>

Avval biz standart kutubxonadan `std::cmp::Ording` deb nomlangan turni olib keladigan yana bir `use` iborasini qo'shamiz. `Ordering` turi boshqa raqam boʻlib, `Less`, `Greater` va `Equal` variantlariga ega. Bu ikkita qiymatni solishtirganda mumkin bo'lgan uchta natijadir.

Keyin pastki qismida `Ordering` turidan foydalanadigan beshta yangi qator qo'shamiz. `cmp` usuli ikkita qiymatni solishtiradi va uni solishtirish mumkin bo'lgan har qanday narsani chaqirish mumkin. Siz solishtirmoqchi bo'lgan narsaga reference kerak: bu erda `taxmin` bilan `yashirin_raqam` solishtiriladi. Keyin u biz `use`  iborasi bilan qamrab olgan `Ordering`  raqamining variantini qaytaradi. Biz `taxmin` va `yashirin_raqam` qiymatlari bilan `cmp` ga murojatdan `Ordering` ning qaysi varianti qaytarilganiga qarab, keyin nima qilish kerakligini hal qilish uchun [`match`][match]<!-- ignore --> ifodasidan foydalanamiz.

`Match` ifodasi *arms* dan tuzilgan. Arm mos keladigan *pattern* va agar `match` ga berilgan qiymat armning patterniga mos kelsa, bajarilishi kerak bo'lgan koddan iborat. Rust `match` ga berilgan qiymatni oladi va har bir armning patternini o'z navbatida ko'rib chiqadi. Patternlar va `match` konstruksiyasi Rust-ning kuchli xususiyatlari hisoblanadi: ular sizning kodingiz duch kelishi mumkin bo'lgan turli vaziyatlarni ifodalash imkonini beradi va ularning barchasini boshqarishingizga ishonch hosil qiladi. Bu xususiyatlar mos ravishda 6-bobda va 18-bobda batafsil yoritiladi.

Keling, bu yerda ishlatadigan `match` iborasi bilan bir misolni ko'rib chiqaylik. Aytaylik, foydalanuvchi 50 ni taxmin qilgan va bu safar tasodifiy yaratilgan maxfiy raqam 38 ni tashkil qiladi.

Kod 50 ni 38 ga solishtirganda, `cmp` methodi `Ordering::Greater` ni qaytaradi, chunki 50 38 dan katta. `match` ifodasi `Ordering::Greater` qiymatini oladi va har bir armning patternini tekshirishni boshlaydi. U birinchi armning `Ordering::Less` patternini koʻrib chiqadi va `Ordering::Greater` qiymati `Ordering::Less` qiymatiga mos kelmasligini koʻradi, shuning uchun u armdagi kodga eʼtibor bermaydi va keyingi armga oʻtadi. Keyingi armning namunasi `Ordering::Greater` boʻlib, `Ordering::Greater` bilan *does* match  keladi! Oʻsha armdagi bogʻlangan kod ishga tushadi va ekranga `Raqam katta!` deb chop etiladi. `match` iborasi birinchi muvaffaqiyatli o'yindan keyin tugaydi, shuning uchun bu senariydagi oxirgi armni ko'rib chiqmaydi.

Biroq, 2-4 ro'yxatdagi kod hali kompilyatsiya qilinmaydi. Keling, sinab ko'raylik:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Xatoning asosi *mos kelmaydigan turlar* mavjudligini bildiradi. Rust kuchli, statik turdagi tizimga ega. Biroq, u ham turdagi inference ega. Biz `let mut taxmin = String::new()` deb yozganimizda, Rust `taxmin` `String` bo'lishi kerak degan xulosaga keldi va bizni turni yozishga majburlamadi. Boshqa tomondan, `yashirin_raqam` raqam turidir. Rust raqamlarining bir nechta turlari 1 dan 100 gacha qiymatga ega bo'lishi mumkin: `i32`, 32 bitli raqam; `u32`, unsigned 32-bitli raqam; `i64`, 64-bitli raqam; boshqalar kabi. Agar boshqacha koʻrsatilmagan boʻlsa, Rust standart boʻyicha `i32` ga oʻrnatiladi, bu `yashirin_raqam` turiga, agar siz Rustning boshqa raqamli turini chiqarishiga olib keladigan turdagi maʼlumotlarni boshqa joyga qoʻshmasangiz. Xatoning sababi shundaki, Rust string va raqam turini taqqoslay olmaydi.

Oxir-oqibat, biz dastur tomonidan kiritilgan `String` ni haqiqiy son turiga aylantirmoqchimiz, shuning uchun uni raqamli raqam bilan yashirin raqam bilan solishtirishimiz mumkin.Buni `main` funksiya tanasiga ushbu qatorni qo'shish orqali qilamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Satr

```rust,ignore
let taxmin: u32 = taxmin.trim().parse().expect("Iltimos, raqam yozing!");
```

Biz `taxmin` nomli o'zgaruvchini yaratamiz. Ammo shoshilmang, dasturda allaqachon `taxmin` nomli o'zgaruvchi mavjud emasmi? Bu shunday, lekin foydali Rust bizga `taxmin` ning oldingi qiymatini yangisi bilan ergashtirish imkonini beradi. *Shadowing* bizga ikkita noyob oʻzgaruvchini yaratish oʻrniga, `taxmin` oʻzgaruvchi nomidan qayta foydalanish imkonini beradi, masalan, `taxmin_str` va `taxmin`. Biz buni [3-bobda][shadowing]<!-- ignore --> batafsil ko'rib chiqamiz, ammo hozircha shuni bilingki, bu xususiyat ko'pincha qiymatni bir turdan boshqa turga aylantirmoqchi bo'lganingizda ishlatiladi.

Biz bu yangi o'zgaruvchini `taxmin.trim().parse()` ifodasiga bog'laymiz. Ifodadagi `taxmin` matni qator sifatida kiritilgan asl `taxmin` o'zgaruvchisiga ishora qiladi. `String` misolidagi `trim` usuli boshida va oxiridagi har qanday bo‘shliqni yo‘q qiladi, bu qatorni faqat raqamli ma’lumotlarni o‘z ichiga olishi mumkin bo‘lgan `u32` bilan solishtirishimiz uchun buni qilishimiz kerak. Foydalanuvchi `read_line` ni to'ldirish uchun <span class="keystroke">enter</span>tugmasini bosib, ularni kiritishi kerak
satrga yangi satr belgisini qo'shadigan taxmin. Masalan, agar foydalanuvchi <span class="keystroke">5</span> raqamini kiritsa va va <span class="keystroke">enter</span> tugmasini bossa `taxmin` shunday ko'rinadi: `5\n`.
`\n` “yangi qator”ni bildiradi. (Windows tizimida <span class="keystroke">enter</span> tugmasini bosish natijasida carriage qaytariladi va yangi qator `\r\n` chiqadi.)
 `trim` methodi `\n` yoki `\r\n`ni yo'q qiladi, natijada atigi `5` bo`ladi.

Satrlardagi [`parse` methodi][parse]<!-- ignore --> qatorni boshqa turga aylantiradi.
Bu yerda biz uni stringdan raqamga aylantirish uchun foydalanamiz. Biz Rustga `let taxmin: u32` yordamida kerakli raqam turini aytishimiz kerak. `taxmin` dan keyin ikki nuqta (`:`) Rustga o'zgaruvchining turiga izoh berishimizni aytadi. Rust bir nechta o'rnatilgan raqam turlariga ega; Bu yerda koʻrilgan `u32` unsigned, 32-bitli butun son.
Bu kichik ijobiy raqam uchun yaxshi standart tanlovdir. Boshqa raqamlar turlari haqida [3-bobda][integers]<!-- ignore --> bilib olasiz.

Bundan tashqari, ushbu misol dasturidagi `u32` annotation va `yashirin_raqam` bilan taqqoslash Rust `yashirin_raqam` ham `u32` bo'lishi kerak degan xulosaga keladi. Shunday qilib, endi taqqoslash bir xil turdagi ikkita qiymat o'rtasida bo'ladi!

`parse` methodii faqat mantiqiy ravishda raqamlarga aylantirilishi mumkin bo'lgan belgilarda ishlaydi va shuning uchun osongina xatolarga olib kelishi mumkin. Agar, masalan, satrda `A👍%` bo'lsa, uni raqamga aylantirishning hech qanday usuli bo'lmaydi. Muvaffaqiyatsiz bo'lishi mumkinligi sababli, `parse` methodii `read_line` usuli kabi `Result` turini qaytaradi (oldingi ["`Result` bilan potentsial muvaffaqiyatsizlikni ko'rib chiqish"] bo'limida muhokama qilingan)(#handling-potential-failure-with-result)<!-- ignore-->). Biz ushbu `Result` ga yana `expect` methodini qo'llash orqali xuddi shunday munosabatda bo'lamiz. Agar `parse` qatordan raqam yarata olmagani uchun `Err` `Result` variantini qaytarsa, `expect` chaqiruvi o‘yinni buzadi va biz bergan xabarni chop etadi.
Agar `parse` qatorni raqamga muvaffaqiyatli aylantira olsa, u `Result`ning `Ok` variantini qaytaradi va `expect` biz xohlagan raqamni `Ok` qiymatidan qaytaradi.

Endi dasturni ishga tushiramiz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/taxminiy_raqam`
Raqamni topish o'yini!
Yashirin raqam: 58
Iltimos, taxminingizni kiriting.
  76
Sizning taxminingiz: 76
Raqam katta!
```

Yaxshi! Tahmindan oldin bo'shliqlar qo'shilgan bo'lsa ham, dastur foydalanuvchi 76 ni taxmin qilganini aniqladi. Har xil turdagi kirishlar bilan turli xatti-harakatlarni tekshirish uchun dasturni bir necha marta ishga tushiring: raqamni to'g'ri taxmin qiling, katta raqamni taxmin qiling va kichik raqamni taxmin qiling.

Hozir bizda o'yinning ko'p qismi ishlayapti, lekin foydalanuvchi faqat bitta taxmin qila oladi. Keling, buni loop qo'shish orqali o'zgartiraylik!

## Loop bilan bir nechta taxminlarga ruxsat berish

`loop` kalit so'zi cheksiz tsiklni yaratadi. Biz foydalanuvchilarga raqamni taxmin qilishda ko'proq imkoniyat berish uchun tsikl qo'shamiz:
<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Ko'rib turganingizdek, biz hamma narsani taxminiy kiritish so'rovidan boshlab tsiklga o'tkazdik. Ilova ichidagi satrlarni har birida yana to'rtta bo'sh joydan o'tkazganingizga ishonch hosil qiling va dasturni qayta ishga tushiring. Dastur endi boshqa bir taxminni abadiy yani har doim so'raydi, bu aslida yangi muammoni keltirib chiqaradi. Foydalanuvchi chiqa olmaydiganga o'xshaydi!

Foydalanuvchi har doim <span class="keystroke">ctrl-c</span> klaviatura yorlig'i yordamida dasturni to'xtatishi mumkin. Ammo bu to'yib bo'lmaydigan yirtqich hayvondan qochishning yana bir yo'li bor, [“Taxminni maxfiy raqam bilan solishtirish“](#comparing-the-guess-to-the-secret-number)<!--ignore -->: mavzusidagi `parse` muhokamasida aytib o'tilganidek, agar foydalanuvchi raqam bo'lmagan javobni kiritsa, dastur buziladi. Bu yerda ko'rsatilganidek, foydalanuvchiga chiqishga ruxsat berish uchun undan foydalanishimiz mumkin

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/taxminiy_raqam`
Raqamni topish o'yini!
Yashirin raqam: 59
Iltimos, taxminingizni kiriting.
45
Sizning taxminingiz: 45
Raqam Kichik!
Iltimos, taxminingizni kiriting.
60
Sizning taxminingiz: 60
Raqam katta!
Iltimos, taxminingizni kiriting.
59
Sizning taxminingiz: 59
Siz yutdingiz!
Iltimos, taxminingizni kiriting.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`quit` deb yozsangiz, o‘yin tugaydi, lekin siz ko‘rganingizdek, boshqa raqam bo‘lmagan ma’lumotlarni kiritish ham shunday bo‘ladi. Bu, eng kamida, suboptimaldir; Biz to'g'ri raqam taxmin qilinganda ham o'yin to'xtashini xohlaymiz.

### To'g'ri taxmindan keyin chiqish

Keling, foydalanuvchi g'alaba qozonganida `break` iborasini qo'shish orqali o'yinni to'xtatish uchun dasturlashtiramiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`Siz yutdingiz!` so‘ng `break` qatorini qo‘shish foydalanuvchi maxfiy raqamni to‘g‘ri taxmin qilganda dasturni tsikldan chiqadi. Loopdan chiqish dasturdan chiqishni ham anglatadi, chunki sikl `main` ning oxirgi qismidir.

### Noto'g'ri kiritish

O'yinning xatti-harakatlarini yanada yaxshilash uchun, foydalanuvchi raqamlardan boshqa belgilar kiritganda dasturni ishdan chiqargandan ko'ra, foydalanuvchi taxmin qilishni davom ettirishi uchun o'yinni raqam bo'lmagan belgilarga e'tibor bermaslikka harakat qildiraylik. Buni 2-5 roʻyxatda koʻrsatilganidek, `taxmin` satrdan `u32` ga aylantirilgan qatorni oʻzgartirish orqali amalga oshirishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Ro'yxat 2-5: Raqamsiz taxminga e'tibor bermaslik va dasturni ishdan chiqarish o'rniga boshqa taxminni so'rash</span>

Xato ustida ishlamay qolishdan xatoni hal qilishga o‘tish uchun biz `expect` chaqiruvidan `match` ifodasiga o‘tamiz. Esda tutingki, `parse` `Result` turini qaytaradi, `Result` esa `Ok` va `Err` variantlariga ega bo'lgan raqamdir. Biz bu yerda `cmp` usulining `Ordering` natijasi bilan bo‘lgani kabi `match` ifodasidan foydalanmoqdamiz.

Agar `parse` qatorni raqamga muvaffaqiyatli aylantira olsa, natijada olingan raqamni o'z ichiga olgan `Ok` qiymatini qaytaradi. Bu `Ok` qiymati birinchi armning patterniga mos keladi va `match` ifodasi `parse` ishlab chiqarilgan va `Ok` qiymatiga qo'ygan `num` qiymatini qaytaradi. Bu raqam biz yaratayotgan yangi `taxmin`  o'zgaruvchisida biz xohlagan joyda tugaydi

Agar `parse` satrni raqamga aylantira olmasa xato haqida qo'shimcha ma'lumotni o'z ichiga olgan `Err` qiymatini qaytaradi. `Err` qiymati birinchi `match` bo‘limidagi `Ok(num)` patterniga mos kelmaydi, lekin ikkinchi armdagi `Err(_)` patterniga mos keladi. Pastki chiziq, `_`, diqqatga sazovor qiymatdir; bu misolda biz barcha `Err` qiymatlariga, ular ichida qanday ma'lumotlar bo'lishidan qat'iy nazar, mos kelmoqchimiz deymiz. Shunday qilib, dastur ikkinchi armning `continue` kodini bajaradi, bu dasturga `loop` ning keyingi iteratsiyasiga o'tishni va boshqa taxminni so'rashni aytadi. Shunday qilib, dastur `parse` duch kelishi mumkin bo'lgan barcha xatolarga e'tibor bermaydi!

Endi dasturdagi hamma narsa kutilganidek ishlashi kerak. Keling, sinab ko'raylik:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling taxminiy_raqam v0.1.0 (file:///projects/taxminiy_raqam)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/taxminiy_raqam`
Raqamni topish o'yini!
Yashirin raqam: 61
Iltimos, taxminingizni kiriting.
10
Sizning taxminingiz: 10
Raqam Kichik!
Iltimos, taxminingizni kiriting.
99
Sizning taxminingiz: 99
Raqam katta!
Iltimos, taxminingizni kiriting.
foo
Iltimos, taxminingizni kiriting.
61
Sizning taxminingiz: 61
Siz yutdingiz!
```

Ajoyib! Bitta kichik so'nggi tweak bilan biz taxmin qilish o'yinini tugatamiz. Eslatib o'tamiz, dastur hali ham maxfiy raqamni chop etmoqda. Bu sinov uchun yaxshi ishladi, lekin o'yinni buzadi. Maxfiy raqamni chiqaradigan `println!`ni o'chirib tashlaymiz. 2-6 ro'yxat yakuniy kodni ko'rishingiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Ro'yxat 2-6: To'liq taxmin qilish o'yin kodini</span>

Shu nuqtada, siz taxmin qilish o'yinini muvaffaqiyatli yaratdingiz. Tabriklaymiz!

## Xulosa

Ushbu loyiha sizni Rustning ko'plab yangi tushunchalari bilan tanishtirishning amaliy usuli bo'ldi: `let`, `match`, funktsiyalar, tashqi cratelardan foydalanish va boshqalar. Keyingi bir necha boblarda siz ushbu tushunchalar haqida batafsilroq bilib olasiz. 3-bob ko'pchilik dasturlash tillarida mavjud bo'lgan o'zgaruvchilar, ma'lumotlar turlari va funktsiyalari kabi tushunchalarni qamrab oladi va ulardan Rustda qanday foydalanishni ko'rsatadi. 4-bobda Rust tilini boshqa tillardan ajratib turadigan egalik huquqi o‘rganiladi. 5-bobda tuzilmalar va methodlar sintaksisi muhokama qilinadi va 6-bobda enumlar qanday ishlashi tushuntiriladi.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
