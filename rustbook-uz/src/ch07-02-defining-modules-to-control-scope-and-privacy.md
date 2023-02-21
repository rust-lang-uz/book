## Qo'llanish doirasi va maxfiylikni nazorat qilish uchun modullarni aniqlash

Ushbu bo'limda biz modullar va modul tizimining boshqa qismlari haqida gapiramiz, ya'ni elementlarni nomlash imkonini beruvchi *pathlar(yo'llar)*; pathni qamrab oluvchi `use` kalit so'zi; va obyektlarni hammaga ochiq qilish(public) uchun `pub` kalit so'zi. Shuningdek, biz `as` kalit so'zini, tashqi paketlarni va glob operatorini muhokama qilamiz.

Birinchidan, kelajakda kodingizni tartibga solishda qulay foydalanish uchun qoidalar ro'yxatidan boshlaymiz. Keyin har bir qoidalarni batafsil tushuntiramiz.

### Modullar qo'llanmasi

Bu yerda biz modullar, pathlar, `use` kalit soʻzi va `pub` kalit soʻzining kompilyatorda qanday ishlashi va koʻpchilik ishlab chiquvchilar oʻz kodlarini qanday tashkil qilishlari haqida qisqacha maʼlumot beramiz. Ushbu bobda biz ushbu qoidalarning har biriga misollarni ko'rib chiqamiz va modullar qanday ishlashini takrorlash uchun yaxshi vaqt.
work.

- **Cratening ildizidan boshlang**: Crateni kompilyatsiya qilishda kompilyator kodni kompilyatsiya qilish uchun avval cratening ildiz fayliga (odatda kutubxona cratesi uchun *src/lib.rs* yoki binary crate uchun *src/main.rs*) qaraydi.
- **Modullarni e'lon qilish**: Cratening ildiz faylida siz yangi modullarni e'lon qilishingiz mumkin; aytaylik, siz `mod poliz` bilan `poliz` modulini e'lon qilasiz; 
Kompilyator modul kodini quyidagi joylarda qidiradi:
  - Inline, jingalak qavs ichida `mod poliz` dan keyingi nuqta-vergul o'rnini egallaydi
  - *src/poliz.rs* faylida
  - *src/poliz/mod.rs* faylida
- **Submodullarni e'lon qilish**: Crate ildizidan boshqa har qanday faylda siz submodullarni e'lon qilishingiz mumkin. Masalan, *src/poliz.rs* da `mod sabzavotlar;` deb e`lon qilishingiz mumkin. Kompilyator quyi modul kodini quyidagi joylarda ota-modul uchun nomlangan jilddan qidiradi:
  - Inline, to'g'ridan-to'g'ri `mod sabzavotlar` dan keyin, nuqta-vergul o'rniga jingalak qavslar ichida
  - *src/poliz/sabzavotlar.rs* faylida
  - *src/poliz/sabzavotlar/mod.rs* faylida
- **Modullarda kodlash yo'llari**: Modul sizning cratengizning bir qismi bo'lgandan so'ng, maxfiylik qoidalari ruxsat bergan bo'lsa, kod yo'lidan foydalanib, xuddi shu cratening istalgan joyidan ushbu moduldagi kodga murojaat qilishingiz mumkin. Misol uchun, poliz sabzavotlari modulidagi `Pomidor` turi `Crate::poliz::sabzavotlar::Pomidor` da topiladi.
- **Shaxsiy va ommaviy**: Modul ichidagi kod standart bo'yicha uning ota-modullaridan maxfiydir. Modulni ommaviy qilish uchun uni `mod` o'rniga `pub mod` bilan e’lon qiling. Ommaviy moduldagi elementlarni ham hammaga ochiq qilish uchun ularni e'lon qilishdan oldin `pub` dan foydalaning.
- **`use` kalit so'zi**: Bir doirada `use` kalit so'zidan foydalanish uzoq yo'llarning takrorlanishini kamaytirish uchun elementlar uchun taxalluslarni yaratadi. `Crate::poliz::sabzavotlar::Pomidor` ga murojaat qilishi mumkin bo'lgan har qanday sohada siz `use crate::poliz::sabzavotlar::Pomidor;` bilan taxallus yaratishingiz mumkin va shundan so'ng siz ushbu turdagi ushbu doirada foydalanish uchun `Pomidor `deb yozishingiz kerak.

Bu erda biz ushbu qoidalarni aks ettiruvchi `orqa_hovli` nomli binary crate yaratamiz. Crate jildi, shuningdek, `orqa_hovli` deb nomlangan, quyidagi fayllar va jildlarni o'z ichiga oladi:

```text
orqa_hovli
├── Cargo.lock
├── Cargo.toml
└── src
    ├── poliz
    │   └── sabzavotlar.rs
    ├── poliz.rs
    └── main.rs
```

Bu holda cratening ildiz fayli *src/main.rs* bo'lib, u quyidagilarni o'z ichiga oladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```
`pub mod poliz;` qatori kompilyatorga *src/poliz.rs* da topilgan kodni kiritishni aytadi, ya'ni:

<span class="filename">Fayl nomi: src/poliz.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/poliz.rs}}
```

Bu yerda `pub mod sabzavotlar;` *src/poliz/sabzavotlar.rs* dagi kod ham kiritilganligini bildiradi. That code is:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/poliz/sabzavotlar.rs}}
```

Keling, ushbu qoidalarning tafsilotlari bilan tanishamiz va ularni amalda ko'rsatamiz!

### Modullarda tegishli kodlarni guruhlash

*Modullar* kodni o'qish va qayta foydalanishni osonlashtirish uchun crate ichida tartibga solish imkonini beradi.
Modullar bizga elementlarning *maxfiyligini* boshqarishga ham imkon beradi, chunki modul ichidagi kod standart boʻyicha shaxsiy(private) hisoblanadi. Shaxsiy elementlar tashqi foydalanish uchun mavjud bo'lmagan ichki dastur tafsilotlari. Biz modullar va ulardagi elementlarni hammaga ochiq qilishni tanlashimiz mumkin, bu esa ularni tashqi koddan foydalanish va ularga bog'liq bo'lishiga imkon beradi.

Misol tariqasida, restoranning funksionalligini ta'minlaydigan kutubxona cratesini yozamiz. Biz funksiyalarning signaturelarini aniqlaymiz, lekin restoranni implement qilishga emas, balki kodni tashkil etishga e'tibor qaratish uchun ularning tanasini bo'sh qoldiramiz.

Restoran sanoatida restoranning ba'zi qismlari *uyning old tomoni* va boshqalari *uyning orqa tomoni* deb ataladi. Uyning old tomoni mijozlar joylashgan joy; Bu mezbonlar mijozlarni joylashtiradigan, serverlar buyurtma va to'lovlarni qabul qiladigan va barmenlar ichimliklar tayyorlaydigan joyni o'z ichiga oladi. Uyning orqa tomonida oshpazlar va oshpazlar oshxonada ishlaydi, idishlarni yuvish mashinalari tozalaydi va menejerlar ma'muriy ishlarni bajaradilar.

Cratemizni shu tarzda tuzish uchun biz uning funksiyalarini ichki modullarga ajratishimiz mumkin. `cargo new restoran --lib` ishga tushirish orqali `restoran` nomli yangi kutubxona yarating; keyin ba'zi modullar va funksiya signaturelarini aniqlash uchun 7-1 ro'yxatidagi kodni *src/lib.rs* ichiga kiriting. Mana, uyning old qismi:
<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Roʻyxat 7-1: `uyning_oldi` moduli, keyin funksiyalarni oʻz ichiga olgan boshqa modullarni oʻz ichiga oladi.</span>

Biz modulni `mod` kalit so'zidan keyin modul nomi bilan belgilaymiz (bu holda `uyning_oldi`). Keyin modul tanasi jingalak qavslar ichiga kiradi. Modullar ichida biz boshqa modullarni joylashtirishimiz mumkin, masalan, `xizmat` va `xizmat_korsatish` modullari. Modullar, shuningdek, structlar, enumlar, konstantalar, belgilar va 7-1 ro'yxatdagi kabi boshqa elementlar uchun ta'riflarga ega bo'lishi mumkin.

Modullardan foydalanib, biz bir-biriga bog'liq definitionlarni guruhlashimiz va ular nima uchun bog'liqligini nomlashimiz mumkin. Ushbu koddan foydalanadigan dasturchilar barcha definitionlarni o'qib chiqmasdan, guruhlarga asoslangan kodni boshqarishi mumkin, bu ularga tegishli definitionlarni topishni osonlashtiradi. Ushbu kodga yangi funksiya qo'shadigan dasturchilar dasturni tartibli saqlash uchun kodni qayerga joylashtirishni bilishadi.

Yuqorida aytib o'tganimizdek, *src/main.rs* va *src/lib.rs* fayllari crate ildiz modullari deb ataladi. Ularning nomlanishining sababi shundaki, bu ikki faylning birortasining mazmuni *modul daraxti* deb nomlanuvchi crate modul strukturasining ildizida joylashgan `crate` nomli modulni tashkil qiladi.

7-2 ro'yxatda 7-1 ro'yxatdagi strukturaning modul daraxti ko'rsatilgan.

```text
crate
 └── uyning_oldi
     ├── xizmat
     │   ├── navbat_listiga_qoshish
     │   └── stolga_otirish
     └── xizmat_korsatish
         ├── buyurtma_olish
         ├── buyurtma_berish
         └── tolov_qilish
```

<span class="caption">7-2 ro'yxat: 7-1 ro'yxatdagi kod uchun modul daraxti</span>

Bu daraxt ba'zi modullar bir-birining ichida qanday joylashishini ko'rsatadi; masalan, `xizmat` uyasi `uyning_oldi` ichida. Daraxt shuningdek, ba'zi modullar bir-birining aka-uka ekanligini, ya'ni ular bir modulda aniqlanganligini ko'rsatadi; `xizmat` va `xizmat_korsatish` `uyning_oldi` ichida belgilangan aka-ukalardir. Agar A moduli B modulida joylashgan bo'lsa, biz A moduli B modulining *bolasi* va B moduli A modulining *otasi* deb aytamiz. E'tibor bering, butun modul daraxti `Crate` nomli yashirin modul ostida joylashgan.

Modul daraxti sizga kompyuteringizdagi fayl tizimining jildlar daraxtini eslatishi mumkin; bu juda to'g'ri taqqoslash! Fayl tizimidagi jildlar singari, siz kodingizni tartibga solish uchun modullardan foydalanasiz. Va xuddi jilddagi fayllar singari, bizga modullarimizni topish usuli kerak.
