## UTF-8 kodlangan matnni String bilan saqlash

Biz 4-bobda stringlar haqida gapirgan edik, ammo hozir ularni batafsil ko'rib chiqamiz.
Yangi Rustaceanlar odatda uchta sababning kombinatsiyasiga ko'ra stringlarga yopishib qolishadi: Rustning mumkin bo'lgan xatolarni ochishga moyilligi, stringlar ko'plab dasturchilar tushunganidan ko'ra murakkabroq ma'lumotlar tuzilishi va UTF-8. Bu omillar shunday birlashadiki, agar siz boshqa dasturlash tillaridan kelgan bo'lsangiz, mavzu murakkab ko'rinishi mumkin.

To'plamlar kontekstida stringlarni muhokama qilish foydalidir, chunki stringlar baytlar to'plami sifatida amalga oshiriladi, shuningdek, bu baytlar matn sifatida talqin qilinganda foydali funksiyalarni ta'minlashning ba'zi usullari. Ushbu bo'limda biz `String` bo'yicha har bir to'plam turiga ega bo'lgan yaratish, yangilash va o'qish kabi operatsiyalar haqida gapiramiz. Shuningdek, biz `String` ning boshqa to'plamlardan qanday farq qilishini, ya'ni `String` ga indekslash odamlar va kompyuterlarning `String` ma'lumotlarini qanday talqin qilishlari o'rtasidagi farqlar tufayli qanday murakkablashishini muhokama qilamiz.

### String nima?

Biz birinchi navbatda *string* atamasi bilan nimani nazarda tutayotganimizni aniqlaymiz. Rust asosiy tilda faqat bitta string turiga ega, bu `str` qator slice boʻlib, odatda uning `&str` shaklida koʻrinadi. 4-bobda biz boshqa joyda saqlangan ba'zi UTF-8 kodlangan string ma'lumotlariga referencelar bo'lgan *string slicelar* haqida gaplashdik. Masalan, satr literallari dasturning binary tizimida saqlanadi va shuning uchun satr slicedir.

Rust standart kutubxonasi tomonidan taqdim etilgan `String` turi asosiy tilga o'rnatilmagan va kengaytiriladigan, o'zgaruvchan, ega bo'lgan, UTF-8 kodlangan string turidir. Rustaceanlar Rust tilidagi "stringlar" ga murojaat qilganda, ular bu turlardan birini emas, balki `String` yoki string slice `&str` turlarini nazarda tutishi mumkin. Garchi bu bo'lim asosan String haqida bo'lsa-da, ikkala tur ham Rust standart kutubxonasida ko'p qo'llaniladi, `String` va string slicelari UTF-8 da kodlangan.

### Yangi String yaratish

`Vec<T>` bilan mavjud bo'lgan bir xil amallarning ko'pchiligi `String` bilan ham mavjud, chunki `String` aslida qo'shimcha kafolatlar, cheklovlar va imkoniyatlarga ega baytlar vectori atrofida o'rash sifatida amalga oshiriladi. `Vec<T>` va `String` bilan bir xil ishlaydigan funksiyaga misol qilib, 8-11 ro'yxatda ko'rsatilgan yangi turdagi misolni yaratuvchi `new` funksiyadir.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-11: Yangi, bo'sh `String` yaratish</span>

Ushbu satr `s` deb nomlangan yangi bo'sh qatorni yaratadi, biz keyin unga ma'lumotlarni yuklashimiz mumkin. Ko'pincha, biz stringni boshlamoqchi bo'lgan dastlabki ma'lumotlarga ega bo'lamiz. Buning uchun biz string literallari kabi `Display` traittini amalga oshiradigan har qanday turda mavjud bo'lgan `to_string` metotidan foydalanamiz. Ro'yxat 8-12 ikkita misolni ko'rsatadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-12: string literalidan `String` yaratish uchun `to_string` metodidan foydalanish</span>

Bu kod `dastlabki tarkib`ni o‘z ichiga olgan stringni yaratadi.

Satr literalidan `String` yaratish uchun `String::from` funksiyasidan ham foydalanishimiz mumkin. 8-13 ro'yxatdagi kod `to_string` funksiyasidan foydalanadigan 8-12 ro'yxatdagi kodga teng:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-13: string literalidan `String` yaratish uchun `String::from` funksiyasidan foydalanish</span>

Stringlar juda ko'p narsalar uchun ishlatilganligi sababli, biz stringlar uchun juda ko'p turli xil umumiy API'lardan foydalanishimiz mumkin, bu bizga juda ko'p imkoniyatlarni taqdim etadi. Ulardan ba'zilari ortiqcha bo'lib tuyulishi mumkin, ammo ularning barchasi o'z joylariga ega! Bunday holda, `String::from` va `to_string` bir xil ishni bajaradi, shuning uchun tanlov sizga eng yoqqan uslubga bog'liq.

Yodda tutingki, stringlar UTF-8 bilan kodlangan, shuning uchun 8-14 ro'yxatda ko'rsatilganidek, biz ularga har qanday to'g'ri kodlangan ma'lumotlarni kiritishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-14: Salom so'zini turli tillarda stringlarda saqlash</span>

Bularning barchasi yaroqli `String` qiymatlari.

### Stringni yangilash

Agar siz unga ko'proq ma'lumot kiritsangiz, `String` hajmi kattalashishi mumkin va uning tarkibi `Vec<T>` tarkibidagi kabi o'zgarishi mumkin. Bundan tashqari, `String` qiymatlarini birlashtirish uchun `+` operatori yoki `format!` makrosidan qulay foydalanish mumkin.

#### `push_str` va `push` yordamida stringga biriktirish

Biz 8-15 roʻyxatda koʻrsatilganidek, string boʻlagini qoʻshish uchun `push_str` metodidan foydalanib `String`ni kengaytirishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-15: `push_str` metodi yordamida `String` ga satr boʻlagini qoʻshish</span>

Ushbu ikki qatordan keyin `s` tarkibida `dasturchi` bo'ladi. `push_str` metodi string bo'lagini oladi, chunki biz parametrga egalik qilishni xohlamaymiz. Masalan, 8-16 roʻyxatdagi kodda biz uning mazmunini `s1` ga qoʻshgandan soʻng `s2` dan foydalanish imkoniyatiga ega boʻlishni xohlaymiz.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-16: Tarkibni `String` ga qo'shgandan so'ng, string bo'lagidan foydalanish</span>

Agar `push_str` metodi `s2` ga egalik qilgan bo‘lsa, biz uning qiymatini oxirgi satrda chop eta olmaymiz. Biroq, bu kod biz kutgandek ishlaydi!

`push` metodi parametr sifatida bitta belgini oladi va uni `String` ga qo'shadi. 8-17 ro'yxatda `push` metodi yordamida `String` ga `v` harfi qo'shiladi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-17: `push` yordamida `String` qiymatiga bitta belgi qoʻshish</span>

Natijada, `s` tarkibida `suv` bo'ladi.

#### `+` operatori yoki `format!` makrosidan foydalanib satrlarni birlashtirish

Ko'pincha siz ikkita mavjud satrni birlashtirishni xohlaysiz. Buning usullaridan biri 8-18 ro'yxatda ko'rsatilganidek, `+` operatoridan foydalanishdir.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">Roʻyxat 8-18: Ikkita `String` qiymatini yangi `String` qiymatiga birlashtirish uchun `+` operatoridan foydalanish</span>

`s3` qatorida `Salom, Rust!` bo'ladi. Qo‘shishdan keyin `s1` ning endi haqiqiy emasligi va `s2`ga referenceni qo‘llaganimiz sababi `+` operatoridan foydalanganda chaqirilayotgan metodning imzosi bilan bog‘liq.
`+` operatori `add` metodidan foydalanadi, uning imzosi quyidagicha ko'rinadi:

```rust,ignore
fn add(self, s: &str) -> String {
```

Standart kutubxonada siz umumiy va tegishli turlar yordamida aniqlangan `add`ni ko'rasiz. Bu erda biz aniq turlarni almashtirdik, bu metodni `String` qiymatlari bilan chaqirganimizda sodir bo'ladi. Biz 10-bobda generiklarni muhokama qilamiz.
Ushbu imzo bizga `+` operatorining murakkab bitlarini tushunishimiz kerak bo'lgan maslahatlarni beradi.

Birinchidan, `s2` `&` belgisiga ega, ya'ni biz birinchi satrga ikkinchi satrning *reference*ni qo'shmoqdamiz. Buning sababi `add` funksiyasidagi `s` parametri: biz faqat `String`ga `&str` qo'shishimiz mumkin; biz ikkita `String` qiymatini qo'sha olmaymiz. Lekin kuting – `&s2` turi `add` uchun ikkinchi parametrda ko‘rsatilganidek, `&str` emas, `&String`dir. Xo'sh, nima uchun 8-18 ro'yxatdagi kod kompilyatsiya bo'ladi?

`add` chaqiruvida `&s2` dan foydalanishimiz sababi shundaki, kompilyator `&String` argumentini `&str` ga *majburlashi(coerce)* mumkin. Biz `add` metodini chaqirganimizda Rust *deref coercion* dan foydalanadi, bu erda `&s2` ni `&s2[..]` ga aylantiradi.
Biz 15-bobda coercion haqida batafsilroq gaplashamiz. `add` `s` parametriga egalik qilmaganligi sababli, `s2` bu amaldan keyin ham haqiqiy `String` bo'lib qoladi.

Ikkinchidan, imzoda `add` `self` egalik qilishini ko'rishimiz mumkin, chunki `self`da `&` *yo'q*. Bu shuni anglatadiki, 8-18-sonli ro'yxatdagi `s1` `add` chaqiruviga o'tkaziladi va bundan keyin endi yaroqsiz bo'ladi. Shunday qilib, `let s3 = s1 + &s2;` har ikkala satrdan nusxa ko'chiradigan va yangisini yaratadiganga o'xshasa-da, bu statement aslida `s1`ga egalik qiladi va `s2` mazmunining nusxasini qo'shadi, va keyin natijaga egalik huquqini qaytaradi. Boshqacha qilib aytganda, u juda ko'p nusxa ko'chirayotganga o'xshaydi, lekin unday emas; implement qilish nusxalashdan ko'ra samaraliroq.

Agar biz bir nechta satrlarni birlashtirishimiz kerak bo'lsa, `+` operatorining xatti-harakati noqulay bo'ladi:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Bu vaqtda `s` `tic-tac-toe` bo'ladi. Barcha `+` va `"` belgilar bilan nima sodir bo'layotganini ko'rish qiyin. Murakkab qatorlarni birlashtirish uchun biz `format!` makrosidan foydalanishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Ushbu kod `s` ni `tic-tac-toe` ga ham o'rnatadi. `format!` makrosi `println!` kabi ishlaydi, lekin natijani ekranga chop etish o'rniga mazmuni bilan `String`ni qaytaradi. Kodning `format!` dan foydalanilgan versiyasini o‘qish ancha oson va `format!` makrosi tomonidan yaratilgan kod bu chaqiruv uning parametrlaridan birortasiga egalik qilmasligi uchun havolalardan foydalanadi.

### Stringlarni indekslash

Ko'pgina boshqa dasturlash tillarida stringdagi alohida belgilarga indeks bo'yicha murojaat qilish orqali kirish to'g'ri va keng tarqalgan operatsiya hisoblanadi. Biroq, agar siz Rust-da indekslash sintaksisidan foydalanib, `String` qismlariga kirishga harakat qilsangiz, xatoga duch kelasiz. 8-19 ro'yxatdagi noto'g'ri kodni ko'rib chiqing.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-19: String bilan indekslash sintaksisidan foydalanishga urinish</span>

Ushbu kod quyidagi xatoga olib keladi:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

Xato va eslatma Rust indekslashni qo'llab-quvvatlamasligini aytadi. Lekin nega yo'q? Bu savolga javob berish uchun Rust stringlarni xotirada qanday saqlashini muhokama qilishimiz kerak.

#### Ichki vakillik

`String` turi - bu `Vec<u8>` turidagi wrapper. Keling, 8-14 ro'yxatdagi to'g'ri kodlangan UTF-8 misol stringlarini ko'rib chiqaylik. Birinchidan, bu:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

Bunday holda, `len` 4 bo'ladi, ya'ni "Hola" qatorini saqlaydigan vektor 4 bayt uzunlikda. Bu harflarning har biri UTF-8 da kodlanganda 1 baytni oladi. Biroq, keyingi qator sizni hayratda qoldirishi mumkin. (E'tibor bering, bu qator arabcha 3 raqami emas, kirill alifbosining bosh harfi Ze bilan boshlanadi.)

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

String uzunligi so'ralganda, siz 12 deb aytishingiz mumkin. Aslida, Rustning javobi 24: bu UTF 8 da “Здравствуйте” ni kodlash uchun zarur bo'lgan baytlar soni, chunki bu satrdagi har bir Unicode skalyar qiymati 2 bayt xotirani oladi. Shuning uchun, satr baytlaridagi indeks har doim ham haqiqiy Unicode skalyar qiymatiga mos kelmaydi. Namoyish qilish uchun ushbu yaroqsiz Rust kodini ko'rib chiqing:

```rust,ignore,does_not_compile
let salom = "Здравствуйте";
let javob = &salom[0];
```

Siz allaqachon bilasizki, `javob` birinchi harf bo'lgan `З` bo'lmaydi. UTF-8 da kodlanganda, `З` birinchi bayti `208`, ikkinchisi esa `151`, shuning uchun `javob` aslida `208` bo'lishi kerakdek tuyuladi, lekin `208` o'z-o'zidan haqiqiy belgi emas. Agar foydalanuvchi ushbu qatorning birinchi harfini so'ragan bo'lsa, `208` ni qaytarish, ehtimol bu emas; ammo, bu Rust bayt indeksi 0 bo'lgan yagona ma'lumotdir. Foydalanuvchilar odatda bayt qiymatini qaytarishni xohlamaydilar, hatto satrda faqat lotin harflari bo‘lsa ham: agar `&“hello”[0]` bayt qiymatini qaytaruvchi yaroqli kod bo‘lsa, u `h` emas, `104`ni qaytaradi. .

Javob shundaki, kutilmagan qiymatni qaytarmaslik va darhol topilmasligi mumkin bo'lgan xatolarni keltirib chiqarmaslik uchun Rust ushbu kodni umuman kompilyatsiya qilmaydi va ishlab chiqish jarayonida tushunmovchiliklarning oldini oladi.

#### Baytlar va skalyar qiymatlar va grafema klasterlari!

UTF-8 bilan bog'liq yana bir nuqta shundaki, Rust nuqtai nazaridan satrlarga qarashning uchta mos usuli mavjud: baytlar, skalyar qiymatlar va grafema klasterlari (biz *harflar* deb ataydigan narsaga eng yaqin narsa).

Devanagari skriptida yozilgan hindcha “नमस्ते” so'ziga qarasak, u `u8` qiymatlari vektori sifatida saqlanadi, bu quyidagicha ko'rinadi:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Bu 18 bayt va kompyuterlar oxir-oqibat bu ma'lumotlarni qanday saqlaydi. Agar biz ularni Rustning `char` turiga ega bo'lgan Unicode skalyar qiymatlari sifatida qarasak, bu baytlar quyidagicha ko'rinadi:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

Bu yerda oltita `char` qiymati bor, lekin to'rtinchi va oltinchi harflar emas: ular o'z-o'zidan ma'noga ega bo'lmagan diakritikdir. Nihoyat, agar baytlarni grafema klasterlari sifatida ko'rib chiqsak, inson to'rt harfli hindcha so'z deb ataydigan narsani olamiz:

```text
["न", "म", "स्", "ते"]
```

Rust kompyuterlar saqlaydigan ma'lumotlar qatorini talqin qilishning turli usullarini taqdim etadi, shunda ma'lumotlar qaysi inson tilida bo'lishidan qat'i nazar, har bir dastur kerakli talqinni tanlashi mumkin.

Rust bizga belgini olish uchun `String` ga indekslashga ruxsat bermasligining yakuniy sababi shundaki, indekslash operatsiyalari doimo konstanta(doimiy) vaqtni oladi (O(1)). Ammo `Strin`g bilan ishlashni kafolatlab bo‘lmaydi, chunki Rust qancha to‘g‘ri belgilar mavjudligini aniqlash uchun tarkibni boshidan indeksgacha bosib o‘tishi kerak edi.

### String bo'laklari

Satrni indekslash ko'pincha noto'g'ri fikrdir, chunki satrni indekslash operatsiyasining qaytish turi qanday bo'lishi kerakligi aniq emas: bayt qiymati, belgi, grafema klasteri yoki satr bo'lagi. Agar chindan ham string bo'laklarini yaratish uchun indekslardan foydalanish kerak bo'lsa, Rust sizdan aniqroq bo'lishingizni so'raydi.

Bitta raqam bilan `[]` yordamida indeksatsiya qilish o'rniga, muayyan baytlarni o'z ichiga olgan string bo'laklarini yaratish uchun diapazon bilan `[]` dan foydalanishingiz mumkin:

```rust
let salom = "Здравствуйте";

let s = &salom[0..4];
```

Bu erda `s` qatorning dastlabki 4 baytini o'z ichiga olgan `&str` bo'ladi.
Avvalroq, biz ushbu belgilarning har biri 2 baytdan iborat bo'lganligini aytib o'tgan edik, ya'ni `s` `Зд` bo'ladi.

Agar biz `&salom[0..1]` kabi belgi baytlarining faqat bir qismini kesishga harakat qilsak, Rust ish runtimeda xuddi vektordagi yaroqsiz indeksga kirish kabi panic qo'yadi:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Ehtiyotkorlik bilan string bo'laklarni yaratish uchun diapazonlardan foydalanishingiz kerak, chunki bu dasturni buzishi mumkin.

### Stringlarni takrorlash usullari

String bo'laklari bilan ishlashning eng yaxshi usuli - belgilar yoki baytlarni xohlaysizmi, aniq bo'lishdir. Unicode skalyar qiymatlari uchun `chars` metodidan foydalaning. “Зд” da `chars`ni chaqirish `char` turidagi ikkita qiymatni ajratib turadi va qaytaradi va har bir elementga kirish uchun natijani takrorlashingiz mumkin:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Ushbu kod quyidagilarni chop etadi:

```text
З
д
```

Shu bilan bir qatorda, `bytes` metodi boshqa domenga mos kelishi mumkin bo'lgan har bir baytni qaytaradi:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Ushbu kod ushbu satrni tashkil etuvchi to'rt baytni chop etadi:

```text
208
151
208
180
```

Lekin esda tutingki, joriy Unicode skalyar qiymatlari 1 baytdan ortiq bo'lishi mumkin.

Devanagari skriptidagi kabi satrlardan grafema klasterlarini olish juda murakkab, shuning uchun bu funksiya standart kutubxona tomonidan ta'minlanmagan. Agar sizga ushbu funksiya kerak bo'lsa, [crates.io](https://crates.io/)<!-- ignore --> saytida cratelar mavjud.

### Stringlar unchalik oddiy emas

Xulosa qilib aytganda, satrlar murakkab. Turli xil dasturlash tillari ushbu murakkablikni dasturchiga qanday taqdim etish bo'yicha turli xil tanlovlar qiladi. Rust `String` ma'lumotlarini to'g'ri ishlashni barcha Rust dasturlari uchun standart xatti-harakatga aylantirishni tanladi, bu esa dasturchilar UTF-8 ma'lumotlari bilan ishlash haqida oldindan ko'proq o'ylashlari kerakligini anglatadi. Ushbu o'zaro kelishuv boshqa dasturlash tillarida ko'rinadiganidan ko'ra ko'proq stringlarning murakkabligini ochib beradi, lekin keyinchalik ishlab chiqish jarayonida paydo bo'lishi mumkin bo'lgan ASCII bo'lmagan belgilar xatolarini qayta ishlash zaruratini oldini oladi.

Yaxshi xabar shundaki, standart kutubxona ushbu murakkab vaziyatlarni to'g'ri hal qilishga yordam beradigan `String` va `&str` turlaridan iborat ko'plab funksiyalarni taklif etadi. Satrda qidirish uchun `contains` va qator qismlarini boshqa satr bilan almashtirish uchun `replace` kabi foydali metodlar uchun texnik hujjatlarni ko'rib chiqing.

Keling, biroz murakkabroq narsaga o'taylik: HashMap!
