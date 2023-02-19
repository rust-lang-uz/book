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

### Grouping Related Code in Modules

*Modules* let us organize code within a crate for readability and easy reuse.
Modules also allow us to control the *privacy* of items, because code within a
module is private by default. Private items are internal implementation details
not available for outside use. We can choose to make modules and the items
within them public, which exposes them to allow external code to use and depend
on them.

As an example, let’s write a library crate that provides the functionality of a
restaurant. We’ll define the signatures of functions but leave their bodies
empty to concentrate on the organization of the code, rather than the
implementation of a restaurant.

In the restaurant industry, some parts of a restaurant are referred to as
*front of house* and others as *back of house*. Front of house is where
customers are; this encompasses where the hosts seat customers, servers take
orders and payment, and bartenders make drinks. Back of house is where the
chefs and cooks work in the kitchen, dishwashers clean up, and managers do
administrative work.

To structure our crate in this way, we can organize its functions into nested
modules. Create a new library named `restaurant` by running `cargo new
restaurant --lib`; then enter the code in Listing 7-1 into *src/lib.rs* to
define some modules and function signatures. Here’s the front of house section:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Listing 7-1: A `front_of_house` module containing other
modules that then contain functions</span>

We define a module with the `mod` keyword followed by the name of the module
(in this case, `front_of_house`). The body of the module then goes inside curly
brackets. Inside modules, we can place other modules, as in this case with the
modules `hosting` and `serving`. Modules can also hold definitions for other
items, such as structs, enums, constants, traits, and—as in Listing
7-1—functions.

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
