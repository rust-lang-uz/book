## Referencelarni lifetime bilan tekshirish

Lifetimelar - biz allaqachon uchratgan generiklarning yana bir turi. Turning biz xohlagan xatti-harakatga ega bo'lishini ta'minlash o'rniga, lifetime referencelar biz uchun kerak bo'lganda haqiqiyligini ta'minlaydi.

4-bobdagi [“Referencelar va Borrowing”]([references-and-borrowing]<!-- ignore -->) bo‘limida biz muhokama qilmagan bir tafsilot shundan iboratki, Rust-dagi har bir referenceda o‘sha referencening amal qilish doirasi *lifetime* bo‘ladi. Ko'pincha, lifetimelar yashirin va inferred bo'ladi,
ko'p hollarda bo'lgani kabi, turlar ham inferred qilinadi.Biz faqat bir nechta tur mumkin bo'lganda turlarga izoh berishimiz kerak. Shunga o'xshab, biz referencelarning lifetime bir necha xil yo'llar bilan bog'lanishi mumkin bo'lgan lifetimelarini izohlashimiz kerak. Rust bizdan runtimeda ishlatiladigan haqiqiy referencelar haqiqiy bo'lishini ta'minlash uchun generik lifetime parametrlaridan foydalangan holda munosabatlarga izoh berishimizni talab qiladi.

Lifetimeni izohlash boshqa dasturlash tillarining ko'pchiligida mavjud bo'lgan tushuncha ham emas, shuning uchun bu notanish tuyuladi. Garchi biz ushbu bobda lifetimeni to'liq qamrab olmasak ham, kontseptsiyadan qulay bo'lishingiz uchun lifetime sintaksisga duch kelishingiz mumkin bo'lgan umumiy usullarni muhokama qilamiz.

### Lifetimeda dangling referencelarni oldini olish

Lifetimening asosiy maqsadi dasturga reference qilish uchun mo'ljallangan ma'lumotlardan boshqa ma'lumotlarga reference qilishiga olib keladigan *dangling referencelar* ning oldini olishdir.
10-16 ro'yxatdagi dasturni ko'rib chiqing, uning tashqi va ichki ko'lami(tashqi va ichki ishlash doirasi) bor.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

<span class="caption">Ro'yxat 10-16: Qiymati ishlash doiradan chiqib ketgan referencedan foydalanishga urinish</span>

> Eslatma: 10-16, 10-17 va 10-23 ro'yxatlardagi misollar o'zgaruvchilarni
> ularga boshlang'ich qiymat bermasdan e'lon qiladi, shuning uchun o'zgaruvchi nomi
> tashqi doirada mavjud. Bir qarashda, bu Rustning null qiymatlari yo'qligiga zid
> bo'lib tuyulishi mumkin. Biroq, agar biz o'zgaruvchiga qiymat berishdan oldin
> foydalanmoqchi bo'lsak, biz kompilyatsiya vaqtida xatoga duch kelamiz, bu Rust
> haqiqatan ham null qiymatlarga ruxsat bermasligini ko'rsatadi.

Tashqi qamrov boshlang‘ich qiymati bo‘lmagan `r` nomli o‘zgaruvchini, ichki qamrov esa boshlang‘ich qiymati 5 bo‘lgan `x` nomli o‘zgaruvchini e’lon qiladi. Ichki doirada(qamrov) biz `x` ga reference sifatida `r` qiymatini belgilashga harakat qilamiz. Keyin ichki qamrov tugaydi va biz qiymatni `r` da chop etishga harakat qilamiz. Ushbu kod kompilyatsiya qilinmaydi, chunki biz undan foydalanishga urinishdan oldin `r` qiymati ko'rib chiqilmaydi. Mana xato xabari:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

`x` o'zgaruvchisi "yetarlicha uzoq umr ko'rmaydi". Sababi, 7-qatorda ichki qamrov tugashi bilan `x` amaldan tashqarida bo'ladi. Lekin `r` tashqi doira uchun hamon amal qiladi; uning qamrovi kengroq bo'lgani uchun biz uni "uzoq yashaydi" deymiz. Agar Rust ushbu kodning ishlashiga ruxsat bergan bo'lsa, `r` `x` doiradan chiqib ketganda ajratilgan xotiraga reference bo'ladi va biz `r` bilan qilishga uringan har qanday narsa to'g'ri ishlamaydi. Xo'sh, Rust bu kodning yaroqsizligini qanday aniqlaydi?
Bu borrow(qarz) tekshiruvidan foydalanadi.

### Borrow tekshiruvchisi

Rust kompilyatorida barcha borrowlar to'g'ri yoki yo'qligini aniqlash uchun ko'lamlarni taqqoslaydigan *borrow tekshiruvi(borrow checker)* mavjud. 10-17 ro'yxat 10-16 ro'yxati bilan bir xil kodni ko'rsatadi, ammo o'zgaruvchilarning lifetime(ishlash muddatini) ko'rsatadigan izohlar bilan.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

<span class="caption">Roʻyxat 10-17: `r` va `x` ning mos ravishda `a` va `b` nomlari bilan ishlash lifetimening izohlari</span>

Bu yerda biz `r`ning lifetimeni `a` bilan va `x`ning lifetimeni `b` bilan izohladik. Ko'rib turganingizdek, ichki `b` bloki tashqi `'a` lifetime blokdan ancha kichik. Kompilyatsiya vaqtida Rust ikki lifetimening o'lchamini solishtiradi va `r` ning lifetime `'a` ekanligini, lekin u `'b` lifetime(umr bo'yi) xotiraga ishora qilishini ko'radi. Dastur rad etildi, chunki `'b` `'a` dan qisqaroq: reference mavzusi reference kabi uzoq vaqt yashamaydi.

Ro'yxat 10-18 kodni tuzatadi, shuning uchun u dangling referencega ega emas va hech qanday xatosiz kompilyatsiya qilinadi.

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

<span class="caption">Ro'yxat 10-18: To'g'ri reference, chunki referencelar mos yozuvlardan ko'ra uzoqroq lifetimega ega</span>

Bu erda `x` `'b` muddatiga ega, bu holda `'a` dan kattaroqdir. Bu `r` `x` ga murojaat qilishi mumkin degan ma'noni anglatadi, chunki Rust `r` dagi reference har doim `x` amalda bo`lishini biladi.

Endi siz referencelarning amal qilish muddati qayerda ekanligini va referencelar har doim haqiqiy boʻlishini taʼminlash uchun Rust lifetimeni qanday tahlil qilishini bilganingizdan soʻng, keling, funksiyalar kontekstida parametrlarning generik lifetime va qiymatlarni qaytarishni koʻrib chiqaylik.

### Funksiyalarning generik lifetime

Biz ikkita satr bo'lagining uzunligini qaytaradigan funksiyani yozamiz. Bu funksiya ikkita satr bo'lagini oladi va bitta satr bo'lagini qaytaradi. `eng_uzun` funksiyasini amalga oshirganimizdan so'ng, 10-19 ro'yxatdagi kod `Eng uzun satr - abcd` ni chop etishi kerak.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

<span class="caption">Ro'yxat 10-19: Ikki qator boʻlagining uzunini topish uchun `eng_uzun` funksiyani chaqiruvchi `main` funksiya</span>

E'tibor bering, biz funksiya satrlarni emas, referencelar bo'lgan satr bo'laklarini olishni xohlaymiz, chunki biz `eng_uzun` funksiya uning parametrlariga egalik qilishni xohlamaymiz. 10 19 roʻyxatda biz foydalanadigan parametrlar nima uchun biz xohlagan parametrlar ekanligi haqida koʻproq muhokama qilish uchun 4-bobdagi [“String slicelari parametr sifatida”][string-slices-as-parameters]<!-- ignore --> boʻlimiga qarang.

Agar biz 10-20 ro'yxatda ko'rsatilganidek, `eng_uzun` funksiyasini amalga oshirishga harakat qilsak, u kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-20: Ikki qatorli boʻlakning uzunroq qismini qaytaradigan, lekin hali kompilyatsiya qilinmagan `eng_uzun` funksiyaning amalga oshirilishi</span>

Buning o'rniga biz lifetime haqida gapiradigan quyidagi xatoni olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Yordam matni shuni ko'rsatadiki, return(qaytarish) turiga umumiy lifetime parametri kerak, chunki Rust qaytarilayotgan reference `x` yoki `y` ga tegishli ekanligini aniqlay olmaydi. Aslida, biz ham bilmaymiz, chunki bu funksiyaning asosiy qismidagi `if` bloki `x` ga referenceni, `else` bloki esa `y` ga referenceni qaytaradi!

Ushbu funksiyani aniqlaganimizda, biz ushbu funksiyaga o'tadigan aniq qiymatlarni bilmaymiz, shuning uchun `if` yoki `else` ishi bajarilishini bilmaymiz. Shuningdek, biz uzatiladigan referencelarning aniq amal qilish muddatini bilmaymiz, shuning uchun biz qaytaradigan(return) lifetime har doim haqiqiy bo'lishini aniqlash uchun 10-17 va 10-18 ro'yxatlarda bo'lgani kabi qamrovni ko'rib chiqa olmaymiz. Borrow tekshiruvchisi buni ham aniqlay olmaydi, chunki u `x` va `y` ning ishlash lifetime qaytarilgan qiymatning lifetime(ishlash muddati) bilan qanday bog'liqligini bilmaydi. Ushbu xatoni tuzatish uchun biz referencelar o'rtasidagi munosabatni aniqlaydigan umumiy lifetime parametrlarini qo'shamiz, shunda borrow tekshiruvi tahlilini amalga oshirishi mumkin.

### Lifetime annotation sintaksisi

Lifetime annotationlar referencelarning qancha yashashini ko'rishini o'zgartirmaydi. Aksincha, ular lifetimega ta'sir qilmasdan, bir-biriga ko'plab murojaatlarning umrbod lifetimelar munosabatlarini tasvirlaydi. Signature generik turdagi parametrni ko'rsatsa, funksiyalar har qanday turni qabul qilishi mumkin bo'lgani kabi, funksiyalar ham umumiy lifetime parametrini belgilash orqali har qanday xizmat muddati bilan murojaatlarni qabul qilishi mumkin.

Lifetime annotationlar biroz noodatiy sintaksisga ega: lifetime parametrlarining nomlari apostrof (`'`) bilan boshlanishi kerak va odatda generik turlar kabi kichik va juda qisqa bo'ladi. Ko'pchilik lifetime annotation birinchi izoh uchun `'a` nomidan foydalanadi. Annotationi reference turidan ajratish uchun boʻsh joydan foydalanib, biz lifetime parametr annotationlarini referencening `&` belgisidan keyin joylashtiramiz.

Mana bir nechta misollar: lifetime parametri bo'lmagan `i32` ga reference, `'a` nomli lifetime parametriga ega `i32` ga reference va lifetime `'a` bo'lgan `i32` ga o'zgaruvchan reference.

```rust,ignore
&i32        // reference
&'a i32     // aniq lifetimega ega bo'lgan reference
&'a mut i32 // aniq lifetimega ega o'zgaruvchan reference
```

Bir umrlik lifetime annotatsiyaning o'zi katta ma'noga ega emas, chunki annotatsiyalar Rustga bir nechta referencelalarning lifetime generik parametrlari bir-biriga qanday bog'liqligini aytib berish uchun mo'ljallangan. Keling, `eng_uzun` funksiya kontekstida lifetime annotatsiyalarning bir-biriga qanday bog'liqligini ko'rib chiqaylik.

### Funksiya signaturelaridagi lifetime annotatsiyalar

Funksiya signaturelarida lifetime annotatsiyalardan foydalanish uchun biz generik *tur* parametrlari bilan qilganimiz kabi, funksiya nomi va parametrlar ro'yxati o'rtasida burchak qavslar ichida generik *lifetime* parametrlarini e'lon qilishimiz kerak.

Biz signature quyidagi cheklovni ifodalashini istaymiz: qaytarilgan(return) reference ikkala parametr ham to'g'ri bo'lsa, haqiqiy bo'ladi. Bu parametrlarning lifetime(ishlash muddati) va qaytariladigan(return) qiymat o'rtasidagi bog'liqlikdir. 10-21 ro'yxatda ko'rsatilganidek, biz lifetimega `'a` deb nom beramiz va keyin uni har bir referencega qo'shamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-21: Signaturedagi barcha referencelar bir xil lifetimega(ishlash muddati) ega bo'lishi kerakligini ko'rsatuvchi `eng_uzun` funksiya ta'rifi `'a`</span>

Ushbu kod 10-19-sonli ro'yxatdagi `main` funksiyadan foydalanganda biz xohlagan natijani kompilyatsiya qilishi va ishlab chiqarishi kerak.

Funktsiya signaturesi endi Rustga ma'lum bir lifetimeda `'a` funksiyasi ikkita parametrni qabul qilishini aytadi, ularning har ikkalasi ham kamida lifetime `'a` bo'lgan string bo'laklaridir. Funktsiya signaturesi, shuningdek, Rustga funksiyadan qaytarilgan string bo'lagi hech bo'lmaganda `'a` lifetimegacha yashashini aytadi.
Amalda, bu `eng_uzun` funksiya tomonidan qaytarilgan referencening lifetime, funksiya argumentlari bilan bog'liq bo'lgan qiymatlarning eng kichik lifetimesi bilan bir xil ekanligini anglatadi. Bu munosabatlar Rust ushbu kodni tahlil qilishda foydalanishini xohlaydigan narsadir.

Esda tutingki, biz ushbu funksiya signaturesida lifetime parametrlarini belgilaganimizda, biz kiritilgan yoki qaytarilgan qiymatlarning lifetimeni o'zgartirmaymiz. Aksincha, biz borrowni tekshiruvchi(borrow checker) ushbu cheklovlarga rioya qilmaydigan har qanday qiymatlarni rad etishi kerakligini ta'kidlaymiz. Shuni esda tutingki, `eng_uzun` funksiya `x` va `y` qancha vaqt ishlashini aniq bilishi shart emas, faqat ushbu signatureni qondiradigan `'a` ga baʼzi bir qamrovni almashtirish mumkin.

Funksiyalarda lifetimeni izohlashda annotationlar funksiya tanasida emas, balki funksiya signaturesida bo'ladi. Signaturedagi turlar singari, lifetime annotationlar funksiya shartnomasining bir qismiga aylanadi. Funktsiya signaturelari lifetime shartnomani o'z ichiga oladi, degan ma'noni anglatadi Rust kompilyatori tahlil qilish osonroq bo'lishi mumkin. Agar funksiyaga izoh berish yoki uni chaqirish bilan bog'liq muammo bo'lsa, kompilyator xatolari kodimizning bir qismiga va cheklovlarga aniqroq ishora qilishi mumkin. Buning o'rniga, Rust kompilyatori biz lifetime munosabatlari haqida ko'proq taxminlar qilgan bo'lsa, kompilyator faqat muammoning sababidan bir necha qadam uzoqda bizning kodimizdan foydalanishni ko'rsatishi mumkin.

Biz `eng_uzun` ga aniq referencelar berganimizda, `'a` o‘rniga qo‘yilgan aniq lifetime `x` doirasining `y` doirasiga to‘g‘ri keladigan qismidir. Boshqacha qilib aytadigan bo'lsak, `'a` generik lifetimesi `x` va `y` ning eng kichik lifetimaga teng bo'lgan aniq lifetimeni oladi. Biz qaytarilgan(return) referencega bir xil lifetime parametri `'a` bilan izoh berganimiz sababli, qaytarilgan reference `x` va `y` lifetimening kichikroq uzunligi uchun ham amal qiladi.

Keling, turli xil aniq lifetimelarga ega bo'lgan referencelarni o'tkazish orqali `eng_uzun` funksiyani qanday cheklashini ko'rib chiqaylik. Ro'yxat 10-22 - bu oddiy misol.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-22: `eng_uzun` funksiyasidan foydalanish, turli xil aniq lifetimega ega `String` qiymatlariga referencelar</span>

Bu misolda `string1` tashqi qamrov oxirigacha amal qiladi, `string2` ichki qamrov oxirigacha amal qiladi va `natija` ichki doiraning oxirigacha amal qiladigan narsaga ishora qiladi. Ushbu kodni ishga tushiring va siz borrowni tekshiruvchi tasdiqlaganini ko'rasiz; u kompilyatsiya qiladi va `Eng uzun satr - uzundan uzun string` ni yaratadi.

Keyinchalik, `natija`dagi referencening lifetime ikkita argumentning kichikroq lifetime bo'lishi kerakligini ko'rsatadigan misolni ko'rib chiqaylik. Biz `natija` o'zgaruvchisi deklaratsiyasini ichki doiradan tashqariga o'tkazamiz, lekin qiymatni belgilashni `string2` bilan doiradagi `natija` o'zgaruvchisiga qoldiramiz. Keyin, `natija`ni ishlatadigan `println!`ni ichki doira tugagandan so‘ng, ichki doiradan tashqariga o‘tkazamiz. 10-23 ro'yxatdagi kod kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-23: `string2` dan keyin `natija` dan foydalanishga urinish</span>

Ushbu kodni kompilyatsiya qilmoqchi bo'lganimizda, biz quyidagi xatoni olamiz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

Xato shuni ko'rsatadiki, `natija` `println!` bayonoti uchun haqiqiy bo'lishi uchun `string2` tashqi doiraning oxirigacha amal qilishi kerak. Rust buni biladi, chunki biz funksiya parametrlarining lifetimeni(ishlash muddati) va qiymatlarni bir xil `'a` parametridan foydalangan holda izohladik.

Inson sifatida biz ushbu kodni ko'rib chiqamiz va `string1` `string2` dan uzunroq ekanligini ko'rishimiz mumkin va shuning uchun `natija` `string1` ga referenceni o'z ichiga oladi.
`string1` hali amaldan tashqariga chiqmaganligi sababli, `string1`ga reference `println!` bayonoti uchun amal qiladi. Biroq, kompilyator bu holatda reference haqiqiy ekanligini ko'ra olmaydi. Biz Rustga aytdikki, `eng_uzun` funksiya tomonidan qaytarilgan referencening lifetime uzatilgan referencelarning lifetimesidan kichikroq vaqt bilan bir xil. Shuning uchun, borrowni tekshirish vositasi 10-23 ro'yxatdagi kodga ruxsat bermaydi, chunki noto'g'ri reference mavjud.

`eng_uzun` funksiyaga oʻtkazilgan referencelarning qiymatlari va amal lifetime va qaytarilgan(return) referencedan qanday foydalanishni oʻzgartiruvchi koʻproq tajribalar ishlab chiqishga harakat qiling. Kompilyatsiya qilishdan oldin tajribalaringiz borrow tekshiruvidan o'tadimi yoki yo'qmi haqida faraz qiling; keyin siz haq ekanligingizni tekshiring!

### Lifetime nuqtai nazaridan fikrlash

Lifetime parametrlarini belgilashingiz kerak bo'lgan metod sizning funksiyangiz nima qilayotganiga bog'liq. Misol uchun, agar biz `eng_uzun` funksiyasini amalga oshirishni har doim eng uzun satr bo'lagini emas, balki birinchi parametrni qaytarish uchun o'zgartirgan bo'lsak, `y` parametrida lifetimeni belgilashimiz shart emas. Quyidagi kod kompilyatsiya qilinadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

Biz `x` parametri va qaytarish(return) turi uchun lifetime `'a` parametrini belgiladik, lekin `y` parametri uchun emas, chunki `y` ning lifetimesi `x` yoki qaytarish qiymati bilan hech qanday aloqasi yo'q.

Funksiyadan mos yozuvlar qaytarilganda, qaytarish turi uchun lifetime parametri parametrlardan birining lifetime parametriga mos kelishi kerak. Agar qaytarilgan reference parametrlardan biriga tegishli bo'lmasa, u ushbu funksiya doirasida yaratilgan qiymatga murojaat qilishi kerak. Biroq, bu dangling reference bo'ladi, chunki funksiya oxirida qiymat doiradan chiqib ketadi.
Kompilyatsiya qilmaydigan `eng_uzun` funksiyani amalga oshirishga urinishlarni ko'rib chiqing:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

Bu erda, biz qaytish turi uchun lifetime parametr `'a` ni belgilagan bo'lsak ham, bu dastur kompilyatsiya qilinmaydi, chunki qaytish qiymatining lifetime parametrlarning lifetime bilan umuman bog'liq emas. Mana biz olgan xato xabari:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

Muammo shundaki, `natija` ishchi ko'lamdan tashqariga chiqadi va `eng_uzun` funksiya oxirida tozalanadi. Shuningdek, biz funksiyadan `natija`ga referenceni qaytarishga harakat qilyapmiz. Dangling referenceni o'zgartiradigan lifetime parametrlarini belgilashning iloji yo'q va Rust bizga dangling reference yaratishga ruxsat bermaydi. Bunday holda, eng yaxshi tuzatish mos yozuvlar emas, balki tegishli referencelar turini qaytarish bo'ladi, shuning uchun chaqiruv funksiyasi qiymatni tozalash uchun javobgar bo'ladi.

Oxir oqibat, lifetime sintaksisi turli parametrlarning ishlash muddatini va funktsiyalarning qaytish qiymatlarini bog'lashdir. Ular ulangandan so'ng, Rust xotira xavfsizligini ta'minlaydigan operatsiyalarga ruxsat berish va dangling pointerlarni yaratish yoki xotira xavfsizligini boshqa tarzda buzadigan operatsiyalarga ruxsat berish uchun yetarli ma'lumotga ega.

### Struktura ta'riflarida lifetime annotationlar

Hozirgacha biz belgilagan structlar barcha egalik turlariga ega. Biz referencelarni saqlash uchun structlarni belgilashimiz mumkin, ammo bu holda structning ta'rifidagi har bir referencega lifetime annotation qo'shishimiz kerak bo'ladi. 10-24 roʻyxatda `ImportantExcerpt` nomli struktura mavjud boʻlib, u string sliceni saqlaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

<span class="caption">Ro'yxat 10-24: Referencega ega bo'lgan struct, lifetime annotationni talab qiladi</span>

Bu structda bir satr boʻlagini oʻz ichiga oluvchi `qism` maydoni mavjud boʻlib, bu referencelardir. Generik(umumiy) ma'lumotlar turlarida bo'lgani kabi, biz structning nomidan keyin burchakli qavslar ichida generik lifetime parametrining nomini e'lon qilamiz, shuning uchun biz structning ta'rifi tanasida lifetime parametridan foydalanishimiz mumkin. Bu izoh `ImportantExcerpt` namunasi oʻzining `qism` maydonidagi referencedan uzoqlasha olmasligini bildiradi.

Bu yerda `main` funksiyasi `roman` oʻzgaruvchisiga tegishli `String`ning birinchi jumlasiga referenceni oʻz ichiga olgan `ImportantExcerpt` strukturasining namunasini yaratadi. `roman`dagi ma'lumotlar `ImportantExcerpt` misoli yaratilishidan oldin mavjud. Bundan tashqari, `roman` `ImportantExcerpt` ishchi doirasi tashqariga chiqmagunicha, ishchi doiradan chiqib ketmaydi, shuning uchun `ImportantExcerpt` misolidagi reference haqiqiy hisoblanadi.

### Lifetime Elision

Siz har bir referencening lifetime(ishlash muddati) borligini va referencelardan foydalanadigan funksiyalar yoki structlar uchun lifetime parametrlarini belgilashingiz kerakligini bilib oldingiz. Biroq, 4-bobda biz 4-9-ro'yxatdagda funksiyaga ega bo'ldik, u keyin yana 10-25 ro'yxatda ko'rsatiladi, unda kod lifetime annotationsiz tuzilgan.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-25: Biz 4-9 ro'yxatda aniqlagan funksiya, parametr va qaytish(return) turi referencelar bo'lsa ham, lifetime annotationsiz(umrbod lifetime) tuzilgan.</span>

Ushbu funktsiyaning lifetime annotationlarsiz kompilyatsiya qilinishining sababi tarixiydir: Rust-ning dastlabki versiyalarida (1.0-dan oldingi) bu kod kompilyatsiya bo'lmagan bo'lardi, chunki har bir reference aniq ishlash muddatini talab qiladi. O'sha paytda funktsiya signaturesi quyidagicha yozilgan bo'lar edi:

```rust,ignore
fn birinchi_belgi<'a>(s: &'a str) -> &'a str {
```

Rust-da juda ko'p kod yozgandan so'ng, Rust jamoasi Rust dasturchilari muayyan vaziyatlarda bir xil lifetime annotatiolarni qayta-qayta kiritayotganini aniqladilar. Bu vaziyatlarni oldindan aytish mumkin edi va bir nechta deterministik patternlarga amal qildi. Ishlab chiquvchilar ushbu patternlarni kompilyator kodiga dasturlashtirdilar, shuning uchun borrow tekshiruvi ushbu vaziyatlarda lifetimeni(ishlash muddatini) aniqlay oladi va aniq izohlarga muhtoj bo'lmaydi.

Rust tarixining ushbu qismi dolzarbdir, chunki ko'proq deterministik patternlar paydo bo'lishi va kompilyatorga qo'shilishi mumkin. Kelajakda undan ham kamroqlifetime annotationlar talab qilinishi mumkin.

Rustning referencelarni tahlil qilishda dasturlashtirilgan patternlar *lifetime elision qoidalari(lifetime elision rules)* deb ataladi. Bu dasturchilar rioya qilishi kerak bo'lgan qoidalar emas; ular kompilyator ko'rib chiqadigan muayyan holatlar to'plamidir va agar sizning kodingiz ushbu holatlarga mos keladigan bo'lsa, lifetime vaqtlarini aniq yozishingiz shart emas.

Elision qoidalari to'liq xulosa chiqarmaydi. Agar Rust qoidalarni qat'iy qo'llasa, lekin referencelarning qancha vaqt ishlashi(lifetime) haqida hali ham noaniqlik mavjud bo'lsa, kompilyator qolgan referencelarning lifetime(ishlash muddati) qancha bo'lishi kerakligini taxmin qila olmaydi. Taxmin qilish o'rniga, kompilyator sizga lifetime annotationlarni qo'shish orqali hal qilishingiz mumkin bo'lgan xatoni beradi.

Funksiya yoki metod parametrlari bo‘yicha lifetime *kirish lifetime (input lifetimes)*, qaytariladigan(return) qiymatlar bo‘yicha lifetime *chiqish lifetimei (output lifetimes)* deb ataladi.

Aniq izohlar(annotation) bo'lmasa, kompilyator referencelarning lifetimeni aniqlash uchun uchta qoidadan foydalanadi. Birinchi qoida kirish lifetimega(input lifetimes), ikkinchi va uchinchi qoidalar esa chiqish lifetimega(output lifetimes) tegishli. Agar kompilyator uchta qoidaning oxiriga yetib borsa va hali ham lifetimeni(foydalanish muddati) aniqlay olmaydigan referencelar mavjud bo'lsa, kompilyator xato bilan to'xtaydi.
Bu qoidalar `fn` ta'riflari hamda `impl` bloklari uchun amal qiladi.

Birinchi qoida shundaki, kompilyator mos yozuvlar bo'lgan har bir parametrga lifetime parametrni belgilaydi.

Ikkinchi qoida shuki, agar aynan bitta kirish lifetime(input) parametri mavjud boʻlsa, u lifetime barcha chiqish(output) lifetime parametrlariga tayinlanadi: `fn foo<'a>(x: &'a i32) -> &'a i32`.

Uchinchi qoida shundaki, agar kirish lifetime bir nechta parametrlar mavjud bo'lsa, lekin ulardan biri `&self` yoki `&mut self` bo'lsa, chunki bu metod bo'lsa, `self` lifetime barcha chiqish lifetime parametrlariga tayinlanadi. Ushbu uchinchi qoida metodlarni o'qish va yozishni ancha yaxshi qiladi, chunki kamroq belgilar kerak.

Tasavvur qilaylik, biz kompilyatormiz. 10-25 roʻyxatdagi `birinchi_belgi` funksiyasi signaturesidagi referencelarning lifetimeni(amal qilish muddati) aniqlash uchun biz ushbu qoidalarni qoʻllaymiz. Signature referencelalar bilan bog'liq bo'lmagan lifetimesiz(muddatsiz) boshlanadi:

```rust,ignore
fn birinchi_belgi(s: &str) -> &str {
```

Keyin kompilyator birinchi qoidani qo'llaydi, bu har bir parametr o'z lifetimesini oladi. Biz uni odatdagidek `'a` deb ataymiz, shuning uchun endi signature quyidagicha:

```rust,ignore
fn birinchi_belgi<'a>(s: &'a str) -> &str {
```

Ikkinchi qoida amal qiladi, chunki aynan bitta kirish lifetime mavjud. Ikkinchi qoida bitta kirish(input) parametrining lifetime chiqish lifetimesiga tayinlanishini bildiradi, shuning uchun signature endi quyidagicha:

```rust,ignore
fn birinchi_belgi<'a>(s: &'a str) -> &'a str {
```

Endi ushbu funksiya signaturesidagi barcha referencelar lifetimesiga ega va kompilyator dasturchiga ushbu funksiya signaturesidagi lifetimeni izohlashiga hojat qoldirmasdan tahlilini davom ettirishi mumkin.

Keling, yana bir misolni ko'rib chiqaylik, bu safar biz 10 20 ro'yxatda ishlashni boshlaganimizda lifetime parametrlarga ega bo'lmagan `eng_uzun` funksiyadan foydalangan holda:

```rust,ignore
fn eng_uzun(x: &str, y: &str) -> &str {
```

Keling, birinchi qoidani qo'llaymiz: har bir parametr o'z lifetimeni oladi. Bu safar bizda bitta emas, ikkita parametr bor, shuning uchun bizda ikkita lifetime bor:

```rust,ignore
fn eng_uzun<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Siz ikkinchi qoida qo'llanilmasligini ko'rishingiz mumkin, chunki bir nechta kirish lifetime mavjud. Uchinchi qoida ham qo'llanilmaydi, chunki `eng_uzun` - bu metod emas, balki funksiya, shuning uchun parametrlarning hech biri `self` emas. Barcha uchta qoidani ko'rib chiqqandan so'ng, biz qaytish(return) turining lifetime nima ekanligini hali aniqlay olmadik. Shuning uchun biz 10-20 ro'yxatdagi kodni kompilyatsiya qilishda xatoga yo'l qo'ydik: kompilyator lifetime elision qoidalari bo'yicha ishladi, lekin signaturedagi referencelarning butun lifetimeni aniqlay olmadi.

Uchinchi qoida haqiqatan ham faqat metod signaturelarida amal qilganligi sababli, biz ushbu kontekstda lifetimeni ko'rib chiqamiz, nima uchun uchinchi qoida biz metod signaturelariga tez-tez izoh qo'yishimiz shart emasligini tushunish uchun.

### Metod ta'riflarida(defination) Lifetime Annotationlar

Biz lifetime bo'lgan strukturada metodlarni qo'llaganimizda, biz 10-11 ro'yxatda ko'rsatilgan generik turdagi parametrlar bilan bir xil sintaksisdan foydalanamiz. Lifetime parametrlarini qayerda e'lon qilishimiz va ishlatishimiz ularning struktura maydonlari yoki metod parametrlari va qaytish(return) qiymatlari bilan bog'liqligiga bog'liq.

Struct maydonlarining lifetime nomlari har doim `impl` kalit so'zidan keyin e'lon qilinishi va keyin structning nomidan keyin ishlatilishi kerak, chunki bu lifetimelar strukturaning bir qismidir.

`impl` blokidagi metod signaturelarida referencelar struct maydonlaridagi referencelar lifetimega bog'langan bo'lishi mumkin yoki ular mustaqil bo'lishi mumkin. Bundan tashqari, lifetime elision qoidalari ko'pincha metod signaturelarida lifetime annotationlar kerak bo'lmasligi uchun shunday qiladi. 10-24 ro'yxatda biz aniqlagan `ImportantExcerpt` nomli strukturadan foydalanib, ba'zi misollarni ko'rib chiqaylik.

Birinchidan, biz `daraja` deb nomlangan metoddan foydalanamiz, uning yagona parametri `self` ga reference va qaytariladigan qiymati `i32` bo‘lib, hech narsaga reference emas:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

`impl` dan keyin lifetime parametr deklaratsiyasi va tur nomidan keyin foydalanish talab qilinadi, lekin biz birinchi elision qoida tufayli `self` ga referencening lifetimeni izohlashimiz shart emas.

Mana uchinchi umr bo'yi elision qoida qo'llaniladigan misol:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Ikkita kirish(input) lifetime bor, shuning uchun Rust birinchi lifetime elision qoidasini qo'llaydi va `&self` va `elon_qilish` ga ham o'z lifetimeni beradi. Keyin, parametrlardan biri `&self` bo'lgani uchun qaytarish(return) turi `&self` lifetimeni oladi va barcha lifetimelar hisobga olingan.

### Statik Lifetime

Muhokama qilishimiz kerak bo'lgan maxsus lifetime bu `'static` bo'lib, bu ta'sirlangan reference dasturning butun muddati davomida yashashi mumkinligini bildiradi. Barcha satr literallari `'static` lifetimega ega, biz ularga quyidagicha izoh berishimiz mumkin:

```rust
let s: &'static str = "Mening statik lifetimem bor.";
```

Ushbu satrning matni to'g'ridan-to'g'ri dasturning binary faylida saqlanadi, u har doim mavjud. Shunday qilib, barcha satr literallarining lifetime `'static` dir.

Xato xabarlarida `'static` lifetimedan foydalanish bo'yicha takliflarni ko'rishingiz mumkin. Biroq, `'static` ni referencening lifetime sifatida belgilashdan oldin, sizda mavjud bo'lgan reference haqiqatan ham dasturingizning butun ish vaqti davomida yashaydimi yoki yo'qmi va buni xohlaysizmi, deb o'ylab ko'ring. Ko'pincha, `'static` lifetimeni ko'rsatadigan xato xabari dangling reference yaratishga urinish yoki mavjud bo'lgan lifetimelarning mos kelmasligi natijasida paydo bo'ladi. Bunday hollarda, yechim `'static` lifetimeni ko'rsatmasdan, bu muammolarni tuzatishdir.

## Generik tur parametrlari, Trait boundlar va birgalikdagi lifetime

Keling, generik turdagi parametrlarni, trait boundlarini va lifetimeni bitta funksiyada belgilash sintaksisini qisqacha ko'rib chiqaylik!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Bu 10-21 roʻyxatdagi `eng_uzun` funksiya boʻlib, u ikki qatorning uzunroq qismini qaytaradi. Ammo endi u `where` bandida ko'rsatilgandek `Display` traitini amalga oshiradigan har qanday tur tomonidan to'ldirilishi mumkin bo'lgan `T` generik turidagi `ann` nomli qo'shimcha parametrga ega. Ushbu qo'shimcha parametr `{}` yordamida chop etiladi, shuning uchun `Display` trait boundi(trait chegarasi) zarur. Lifetimelar generik tur bo'lganligi sababli, lifetime parametri `'a` va generik turdagi parametr `T` funksiya nomidan keyin burchakli qavslar ichida bir xil ro'yxatda joylashgan.

## Xulosa

Biz ushbu bobda juda ko'p narsalarni ko'rib chiqdik! Endi siz generik(umumiy) turdagi parametrlar, traitlar va trait boundlari(trait chegaralari) va generik lifetime parametrlari haqida bilganingizdan so'ng, siz ko'p turli vaziyatlarda ishlaydigan kodni takrorlashsiz yozishga tayyorsiz.
Generik turdagi parametrlar kodni turli turlarga qo'llash imkonini beradi. Traitlar va traitlar boundlari(chegara) turlar generik(umumiy) bo'lsa ham, ular kodga kerak bo'lgan xatti-harakatlarga ega bo'lishini ta'minlaydi. Ushbu moslashuvchan kodda hech qanday dangling referencelar bo'lmasligini ta'minlash uchun lifetime annotationlardan qanday foydalanishni o'rgandingiz. Va bu tahlillarning barchasi kompilyatsiya vaqtida sodir bo'ladi, bu runtimening ishlashiga ta'sir qilmaydi!

Ishoning yoki ishonmang, biz ushbu bobda muhokama qilgan mavzular bo'yicha ko'p narsalarni o'rganishimiz mumkin: 17-bobda traitlardan foydalanishning yana bir usuli bo'lgan trait ob'ektlari muhokama qilinadi. Bundan tashqari,lifetime annotationlarni o'z ichiga olgan murakkab stsenariylar ham mavjud, ular sizga faqat juda ilg'or stsenariylarda kerak bo'ladi; ular uchun siz [Rust Reference][reference] ni o'qishingiz kerak. Ammo keyin siz Rust-da testlarni qanday yozishni o'rganasiz, shunda kodingiz kerakli tarzda ishlayotganiga ishonch hosil qilishingiz mumkin.

[references-and-borrowing]:
ch04-02-references-and-borrowing.html#references-and-borrowing
[string-slices-as-parameters]:
ch04-03-slices.html#string-slices-as-parameters
[reference]: ../reference/index.html
