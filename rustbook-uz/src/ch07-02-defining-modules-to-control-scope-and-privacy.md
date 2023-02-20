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
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

Bu yerda `pub mod sabzavotlar;` *src/poliz/sabzavotlar.rs* dagi kod ham kiritilganligini bildiradi. That code is:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

Keling, ushbu qoidalarning tafsilotlari bilan tanishamiz va ularni amalda ko'rsatamiz!

### Modullarda tegishli kodlarni guruhlash

*Modullar* kodni o'qish va qayta foydalanishni osonlashtirish uchun crate ichida tartibga solish imkonini beradi.
Modullar bizga elementlarning *maxfiyligini* boshqarishga ham imkon beradi, chunki modul ichidagi kod standart boʻyicha shaxsiy(private) hisoblanadi. Shaxsiy elementlar tashqi foydalanish uchun mavjud bo'lmagan ichki dastur tafsilotlari. Biz modullar va ulardagi elementlarni hammaga ochiq qilishni tanlashimiz mumkin, bu esa ularni tashqi koddan foydalanish va ularga bog'liq bo'lishiga imkon beradi.

Misol tariqasida, restoranning funksionalligini ta'minlaydigan kutubxona cratesini yozamiz. Biz funksiyalarning signaturelarini aniqlaymiz, lekin restoranni implement qilishga emas, balki kodni tashkil etishga e'tibor qaratish uchun ularning tanasini bo'sh qoldiramiz.

Restoran sanoatida restoranning ba'zi qismlari *uyning old tomoni* va boshqalari *uyning orqa tomoni* deb ataladi. Uyning old tomoni mijozlar joylashgan joy; Bu mezbonlar mijozlarni joylashtiradigan, serverlar buyurtma va to'lovlarni qabul qiladigan va barmenlar ichimliklar tayyorlaydigan joyni o'z ichiga oladi. Uyning orqa tomonida oshpazlar va oshpazlar oshxonada ishlaydi, idishlarni yuvish mashinalari tozalaydi va menejerlar ma'muriy ishlarni bajaradilar.

Cratemizni shu tarzda tuzish uchun biz uning funksiyalarini ichki modullarga ajratishimiz mumkin. `cargo new restoran --lib` ishga tushirish orqali `restoran` nomli yangi kutubxona yarating; keyin ba'zi modullar va funksiya signaturelarini aniqlash uchun 7-1 ro'yxatidagi kodni *src/lib.rs* ichiga kiriting. Mana, uyning old qismi:
<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Roʻyxat 7-1: `uyning_oldi` moduli, keyin funksiyalarni oʻz ichiga olgan boshqa modullarni oʻz ichiga oladi.</span>

Biz modulni `mod` kalit so'zidan keyin modul nomi bilan belgilaymiz (bu holda `uyning_oldi`). Keyin modul tanasi jingalak qavslar ichiga kiradi. Modullar ichida biz boshqa modullarni joylashtirishimiz mumkin, masalan, `xizmat` va `xizmat_korsatish` modullari. Modullar, shuningdek, structlar, enumlar, konstantalar, belgilar va 7-1 ro'yxatdagi kabi boshqa elementlar uchun ta'riflarga ega bo'lishi mumkin.

By using modules, we can group related definitions together and name why
they’re related. Programmers using this code can navigate the code based on the
groups rather than having to read through all the definitions, making it easier
to find the definitions relevant to them. Programmers adding new functionality
to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate
roots. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the *module tree*.

Listing 7-2 shows the module tree for the structure in Listing 7-1.

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">Listing 7-2: The module tree for the code in Listing
7-1</span>

This tree shows how some of the modules nest inside one another; for example,
`hosting` nests inside `front_of_house`. The tree also shows that some modules
are *siblings* to each other, meaning they’re defined in the same module;
`hosting` and `serving` are siblings defined within `front_of_house`. If module
A is contained inside module B, we say that module A is the *child* of module B
and that module B is the *parent* of module A. Notice that the entire module
tree is rooted under the implicit module named `crate`.

The module tree might remind you of the filesystem’s directory tree on your
computer; this is a very apt comparison! Just like directories in a filesystem,
you use modules to organize your code. And just like files in a directory, we
need a way to find our modules.
