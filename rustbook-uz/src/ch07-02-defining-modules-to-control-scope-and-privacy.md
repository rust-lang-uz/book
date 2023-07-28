## Qo'llanish doirasi va maxfiylikni nazorat qilish uchun modullarni aniqlash

In this section, we’ll talk about modules and other parts of the module system, namely *paths* that allow you to name items; the `use` keyword that brings a path into scope; and the `pub` keyword to make items public. We’ll also discuss the `as` keyword, external packages, and the glob operator.

First, we’re going to start with a list of rules for easy reference when you’re organizing your code in the future. Then we’ll explain each of the rules in detail.

### Modullar qo'llanmasi

Here we provide a quick reference on how modules, paths, the `use` keyword, and the `pub` keyword work in the compiler, and how most developers organize their code. We’ll be going through examples of each of these rules throughout this chapter, but this is a great place to refer to as a reminder of how modules work.

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually *src/lib.rs* for a library crate or *src/main.rs* for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
  - Inline, jingalak qavs ichida `mod poliz` dan keyingi nuqta-vergul o'rnini egallaydi
  - *src/poliz.rs* faylida
  - *src/poliz/mod.rs* faylida
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in *src/garden.rs*. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, to'g'ridan-to'g'ri `mod sabzavotlar` dan keyin, nuqta-vergul o'rniga jingalak qavslar ichida
  - *src/poliz/sabzavotlar.rs* faylida
  - *src/poliz/sabzavotlar/mod.rs* faylida
- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- **Private vs public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

Here we create a binary crate named `backyard` that illustrates these rules. The crate’s directory, also named `backyard`, contains these files and directories:

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

Here, `pub mod vegetables;` means the code in *src/garden/vegetables.rs* is included too. That code is:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/poliz/sabzavotlar.rs}}
```

Keling, ushbu qoidalarning tafsilotlari bilan tanishamiz va ularni amalda ko'rsatamiz!

### Modullarda tegishli kodlarni guruhlash

*Modules* let us organize code within a crate for readability and easy reuse. Modules also allow us to control the *privacy* of items, because code within a module is private by default. Private items are internal implementation details not available for outside use. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

As an example, let’s write a library crate that provides the functionality of a restaurant. We’ll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than the implementation of a restaurant.

In the restaurant industry, some parts of a restaurant are referred to as *front of house* and others as *back of house*. Front of house is where customers are; this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in this way, we can organize its functions into nested modules. Create a new library named `restaurant` by running `cargo new
restaurant --lib`; then enter the code in Listing 7-1 into *src/lib.rs* to define some modules and function signatures. Here’s the front of house section:

<span class="filename">Roʻyxat 7-1: `uyning_oldi` moduli, keyin funksiyalarni oʻz ichiga olgan boshqa modullarni oʻz ichiga oladi.</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```


<span class="caption">7-2 ro'yxat: 7-1 ro'yxatdagi kod uchun modul daraxti</span>

We define a module with the `mod` keyword followed by the name of the module (in this case, `front_of_house`). The body of the module then goes inside curly brackets. Inside modules, we can place other modules, as in this case with the modules `hosting` and `serving`. Modules can also hold definitions for other items, such as structs, enums, constants, traits, and—as in Listing 7-1—functions.

By using modules, we can group related definitions together and name why they’re related. Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the *module tree*.

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


<span class="caption">Listing 7-2: The module tree for the code in Listing 7-1</span>

This tree shows how some of the modules nest inside one another; for example, `hosting` nests inside `front_of_house`. The tree also shows that some modules are *siblings* to each other, meaning they’re defined in the same module; `hosting` and `serving` are siblings defined within `front_of_house`. If module A is contained inside module B, we say that module A is the *child* of module B and that module B is the *parent* of module A. Notice that the entire module tree is rooted under the implicit module named `crate`.

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.
