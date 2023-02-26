## `use` kalit so'zi bilan yo'llarni doiraga kiritish

Funksiyalarni chaqirish yo'llarini yozishga to'g'ri kelishi noqulay va takroriy tuyulishi mumkin. 7-7-Ro'yxatda `navbat_listiga_qoshish` funksiyasiga mutlaq yoki nisbiy yoʻlni tanladikmi, har safar `navbat_listiga_qoshish` funksiyasiga murojat qilmoqchi boʻlganimizda, `uyning_oldi` va `xizmat`ni ham belgilashimiz kerak edi. Yaxshiyamki, bu jarayonni soddalashtirishning bir usuli bor: biz bir marta `use` kalit so‘zi bilan yo‘lga nom yaratishimiz mumkin, so‘ngra boshqa hamma joyda qisqaroq nomdan foydalanishimiz mumkin.

7-11 ro'yxatda biz `crate::uyning_oldi::xizmat` modulini `restoranda_ovqatlanish` funksiyasi doirasiga kiritamiz, shuning uchun `restoranda_ovqatlanish`dagi `navbat_listiga_qoshish` funksiyasini chaqirish uchun faqat `xizmat::navbat_listiga_qoshish` ni belgilashimiz kerak.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-11: Modulni `use` bilan qamrab olish</span>

`use` va sohaga yo'lni qo'shish fayl tizimida ramziy havola yaratishga o'xshaydi. Crate ildiziga `use crate::uyning_oldi::xizmat` ni qo‘shish orqali `xizmat` endi bu doirada haqiqiy nom bo‘lib qoladi, xuddi `xizmat` moduli crate ildizida aniqlangandek. `use` doirasiga kiritilgan yo'llar boshqa yo'llar kabi maxfiylikni ham tekshiradi.

E'tibor bering, `use` faqat `use` ishlaydigan aniq doira uchun yorliqni yaratadi. 7-12 roʻyxat `restoranda_ovqatlanish` funksiyasini `mijoz` nomli yangi bolalar moduliga oʻtkazadi, bu keyinchalik `use` statementidan farq qiladi, shuning uchun funksiyaning tanasi kompilyatsiya qilinmaydi:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-12: `use` statementi faqat u joylashgan doirada qo'llaniladi</span>

Kompilyator xatosi yorliq endi `mijoz` modulida qo'llanilmasligini ko'rsatadi:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

E'tibor bering, `use` endi uning doirasida qo'llanilmasligi haqida ogohlantirish ham bor! Bu muammoni hal qilish uchun `use` ni `mijoz` moduliga ham o‘tkazing yoki `mijoz` modulidagi `super::xizmat` bilan ota-moduldagi yorliqlarga murojaat qiling.

### `use` bilan idiomatik yo'llarni yaratish

7-11 ro'yxatda siz shunday deb hayron bo'lishingiz mumkin,Nima uchun biz bir xil natijaga erishish uchun `navbat_listiga_qoshish` funksiyasigacha toʻliq yoʻlni ishlatish oʻrniga, `crate::uyning_oldi::xizmat` ni ishlatishni belgilab qoʻydik va keyin `restoranda_ovqatlanish` ichidagi `xizmat::navbat_listiga_qoshish` ga murojat qildik, 7-13 ro'yxatdagi kabi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-13: `navbat_listiga_qoshish` funksiyasini `use` bilan qamrab olish, bu unidiomatikdir</span>

Garchi 7-11 va 7-13 ro'yxatlari bir xil vazifani bajarsa-da, 7-11 ro'yxat funksiyani `use` bilan qamrab olishning idiomatik usulidir. Funksiyaning ota-modulini `use` bilan qamrab olish funksiyani chaqirishda ota-modulni belgilashimiz kerakligini anglatadi. Funksiyani chaqirishda ota-modulni ko'rsatish, to'liq yo'lning takrorlanishini minimallashtirish bilan birga, funksiya mahalliy sifatida aniqlanmaganligini aniq ko'rsatadi. 7-13 ro'yxatda `navbat_listiga_qoshish` qayerda aniqlangani aniq emas.

Boshqa tomondan, `use` bilan structlar, enumlar va boshqa elementlarni keltirishda to'liq yo'lni ko'rsatish idiomatikdir. 7-14 ro'yxat standart kutubxonaning `HashMap` structini binary crate doirasiga olib kirishning idiomatik usulini ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Ro'yxat 7-14: `HashMap` ni idiomatik tarzda qamrab olish</span>

Bu idioma ortida hech qanday yaxshi sabab yo'q: Bu shunchaki konventsiya paydo bo'ldi va odamlar Rust kodini shu tarzda o'qish va yozishga o'rganib qolgan.

Bu idiomadan istisno shundaki, biz bir xil nomdagi ikkita elementni `use` statementi yordamida doiraga kiritganimizda - Rust bunga yo'l qo'ymaydi. 7-15 ro'yxatda bir xil nomga ega, ammo har xil ota-modullarga ega bo'lgan ikkita `Result` turini qanday ko'rinishga kiritish va ularga qanday murojaat qilish kerakligi ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 7-15: Bir xil nomdagi ikkita turni bir xil doiraga kiritish uchun ularning ota-modullaridan foydalanish talab etiladi.</span>

Ko'rib turganingizdek, ota-modullardan foydalanish ikkita `Result` turini ajratib turadi.
Buning o'rniga `use std::fmt::Result` va `us std::io::Result` ni belgilagan bo'lsak, bizda bir xil miqyosda ikkita `Result` turi bo'lar edi va Rust `Result` dan foydalanganda qaysi birini nazarda tutganimizni bilmas edi.

### `as` kalit so'zi bilan yangi nomlarni taqdim etish

Bir xil nomdagi ikkita turni `use` bilan bir xil doiraga olib kirish muammosining yana bir yechimi bor: yoʻldan soʻng biz `as` va yangi mahalliy nom yoki tur uchun *taxallus* belgilashimiz mumkin. 7-16 ro'yxatda ikkita `Result` turidan birini `as` yordamida qayta nomlash orqali 7-15 ro'yxatdagi kodni yozishning yana bir usuli ko'rsatilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 7-16: `as` kalit so'zi bilan qamrovga kiritilgan tur nomini o'zgartirish</span>

Ikkinchi `use` statementida biz `std::io::Result` turi uchun yangi `IoResult` nomini tanladik, bu endi `std::fmt` dan `Result` turiga zid kelmaydi, u ham doiraga kiradi. 7-15 va 7-16 ro'yxatlar idiomatik hisoblanadi, shuning uchun tanlov sizga bog'liq!

### `pub use` bilan nomlarni qayta eksport(re-eksport) qilish

`use` kalit so'zidan foydalanib, nomni qamrovga kiritganimizda, yangi doirada mavjud bo'lgan nom shaxsiy bo'ladi. Bizning kodimizni chaqiradigan kodni xuddi shu kod doirasida aniqlangandek ushbu nomga murojaat qilishini yoqish uchun biz `pub` va `use` ni birlashtira olamiz. Bu usul *re-eksport* deb nomlanadi, chunki biz obyektni qamrovga kiritmoqdamiz, lekin elementni boshqa qamrovlarga kiritish uchun ham mavjud qilamiz.

7-17 ro'yxatda 7-11 ro'yxatdagi kod ko'rsatilgan, ildiz modulidagi `use` `pub use` ga o'zgartirilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-17. `pub use` bilan yangi doiradagi istalgan kod tomonidan foydalanish uchun nom berish</span>

Ushbu o'zgarishdan oldin tashqi kod `restoran::uyning_oldi::xizmat::navbat_listiga_qoshish()` yo'lidan foydalanib, `navbat_listiga_qoshish` funksiyasini chaqirishi kerak bo'ladi. Endi bu `pub use` `xizmat` modulini ildiz modulidan qayta eksport qilgan bo‘lsa, tashqi kod endi `restoran::xizmat::navbat_listiga_qoshish()` yo‘lidan foydalanishi mumkin..

Qayta eksport qilish sizning kodingizning ichki tuzilishi sizning kodingizni chaqirayotgan dasturchilarning domen haqida o'ylashlaridan farq qilganda foydali bo'ladi. Misol uchun, ushbu restoran metaforasida restoranni boshqaradigan odamlar "uyning old tomoni" va "uyning orqasi" haqida o'ylashadi. Ammo restoranga tashrif buyurgan mijozlar, ehtimol, restoranning qismlari haqida o'ylamaydilar. `pub use` bilan biz kodimizni bitta struct bilan yozishimiz mumkin, lekin boshqa structni ko'rsatamiz. Bu bizning kutubxonamizni kutubxonada ishlaydigan dasturchilar va kutubxonaga murojat qilayotgan dasturchilar uchun uchun yaxshi tashkil etilgan holda saqlaydi. Biz 14-bobning [“`pub use` bilan qulay umumiy APIni eksport qilish”][ch14-pub-use]<!-- ignore --> bo‘limida `pub use`ning yana bir misolini va uning cratengiz hujjatlariga qanday ta’sir qilishini ko‘rib chiqamiz.


### Tashqi paketlardan foydalanish

2-bobda biz tasodifiy raqamlarni olish uchun `rand` deb nomlangan tashqi paketdan foydalangan holda taxminiy o'yin loyihasini dasturlashtirdik. Loyihamizda `rand` dan foydalanish uchun biz ushbu qatorni *Cargo.toml* ga qo'shdik:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Fayl nomi: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

*Cargo.toml*-ga `rand`ni dependency sifatida qo'shish Cargo-ga [crates.io](crates.io)-dan `rand` paketini va har qanday bog'liqliklarni yuklab olishni va `rand`ni loyihamiz uchun ishlatishni aytadi.

Keyin, `rand` ta'riflarini paketimiz doirasiga kiritish uchun biz crate nomidan boshlanadigan `use` qatorini qo'shdik, `rand` va biz qamrab olmoqchi bo'lgan elementlarni sanab o'tdik. Eslatib o‘tamiz, 2-bobdagi [“Tasodifiy raqamni yaratish”][rand]<!-- ignore --> bo‘limida biz `Rng` traitini qamrab oldik va `rand::thread_rng` funksiyasini chaqirdik:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust hamjamiyatining a'zolari [crates.io](crates.io) saytida ko'plab paketlarni taqdim etishdi va ulardan birini o'z paketingizga olish xuddi shu bosqichlarni o'z ichiga oladi: ularni paketingizning *Cargo.toml* faylida roʻyxatga kiriting va `use` dan foydalanib, ularni cratelaridagi elementlarni qamrab oling.

E'tibor bering, standart `std` kutubxonasi bizning paketimizdan tashqarida joylashgan cratedir. Standart kutubxona Rust tili bilan birga kelganligi sababli, biz *Cargo.toml* ni `std` qo'shish uchun o'zgartirishimiz shart emas. Ammo biz u yerdan elementlarni paketimiz doirasiga olib kirish uchun `use` bilan murojaat qilishimiz kerak. Masalan, `HashMap` bilan biz ushbu qatordan foydalanamiz:

```rust
use std::collections::HashMap;
```

Bu standart kutubxona cratesining nomi bo'lgan `std` bilan boshlanadigan mutlaq yo'ldir.

### Uzun `use` ro'yxatini qisqartirish uchun ichki yo'llardan foydalanish

Agar biz bir xil crate yoki bir xil modulda belgilangan bir nechta elementlardan foydalansak, har bir elementni o'z qatoriga qo'yish bizning fayllarimizda juda ko'p vertikal joy egallashi mumkin. Masalan, 2-4 roʻyxatdagi raqamlarni taxmin qilish dasturida mavjud boʻlgan ushbu ikkita `use` statementi `std` dagi elementlarni qamrab oladi:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Buning o'rniga, biz bir xil elementlarni bir qatorga kiritish uchun ichki yo'llardan foydalanishimiz mumkin. Buni 7-18 roʻyxatda koʻrsatilganidek, yoʻlning umumiy qismini, keyin ikkita nuqta qoʻyib, soʻngra yoʻllarning bir-biridan farq qiladigan qismlari roʻyxati atrofida jingalak qavslarni belgilash orqali qilamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Ro'yxat 7-18. Qo'llash sohasiga bir xil prefiksli bir nechta elementlarni qo'shish uchun ichki yo'lni belgilash</span>

Kattaroq dasturlarda bir xil crate yoki moduldan ko'plab elementlarni o'rnatilgan yo'llar yordamida qamrab olish juda ko'p talab qilinadigan alohida `use` statementlari sonini kamaytirishi mumkin!

Siz har qanday darajadagi ichki yo'ldan foydalanishingiz mumkin, bu yo'l qismini ulashuvchi ikkita `use` statementini birlashtirishda foydalidir. Masalan, 7-19 ro'yxat ikkita `use` statementini ko'rsatadi: biri `std::io` ni qamrab oladi va ikkinchisi `std::io::Write` ni qamrab oladi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-19: biri ikkinchisining bir qismi bo'lgan ikkita `use` statementi</span>

Ushbu ikkita yo'lning umumiy qismi `std::io` va to'liq birinchi yo'ldir. Ushbu ikkita yo'lni bitta `use` statementiga birlashtirish uchun biz 7-20 ro'yxatda ko'rsatilganidek, ichki yo'lda `self` dan foydalanishimiz mumkin.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-20: Ro'yxat 7-19dagi yo'llarni bitta `use` statementiga birlashtirish</span>

Bu satr `std::io` va `std::io::Write` ni qamrab oladi.

### Glob operatori

Agar biz yo'lda belgilangan *barcha* umumiy elementlarni qamrovga kiritmoqchi bo'lsak, biz `*` glob operatori tomonidan keyingi yo'lni belgilashimiz mumkin:

```rust
use std::collections::*;
```

Ushbu `use` statementi `std::collections` da aniqlangan barcha ommaviy elementlarni joriy doiraga olib keladi. Glob operatoridan foydalanganda ehtiyot bo'ling! Glob qaysi nomlar qamrovda ekanligini va dasturingizda ishlatiladigan nom qayerda aniqlanganligini aniqlashni qiyinlashtirishi mumkin.

Glob operatori ko'pincha sinovdan o'tgan hamma narsani `tests` moduliga kiritish uchun test paytida ishlatiladi; biz bu haqda 11-bobdagi ["Testlarni qanday yozish kerak"][writing-tests]<!-- ignore --> bo'limida gaplashamiz. Glob operatori ba'zan prelude patterning bir qismi sifatida ham qo'llaniladi: ushbu pattern haqida qo'shimcha ma'lumot olish uchun [standart kutubxona texnik hujjatlariga](../std/prelude/index.html#other-preludes)<!-- ignore --> qarang.


[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
