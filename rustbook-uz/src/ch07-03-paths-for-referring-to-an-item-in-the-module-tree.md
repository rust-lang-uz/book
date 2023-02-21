## Modul daraxtidagi elementga murojaat qilish yo'llari

Rust-ga modul daraxtidagi elementni qayerdan topish mumkinligini ko'rsatish uchun biz fayl tizimida harakat qilishda qanday yo'l(path) ishlatgan bo'lsak, xuddi shunday yo'ldan foydalanamiz. Funksiyani chaqirish uchun biz uning yo'lini bilishimiz kerak.

Yo'l ikki shaklda bo'lishi mumkin:

* *Absolyut yo'l* - bu crate ildizidan boshlanadigan to'liq yo'l; tashqi cretedagi kod uchun mutlaq yo'l crate nomidan boshlanadi va joriy cratedagi kod uchun esa `crate` bilan boshlanadi..
* *N   isbiy yo‘l* joriy moduldan boshlanadi va joriy modulda `self`, `super` yoki identifikatordan foydalanadi.

Mutlaq va nisbiy yo‘llardan keyin ikki nuqta (`::`) bilan ajratilgan bir yoki bir nechta identifikatorlar keladi.

7-1 ro'yxatiga qaytsak, biz `navbat_listiga_qoshish` funksiyasini chaqirmoqchimiz deylik.
Bu so'rash bilan bir xil: `navbat_listiga_qoshish` funksiyasining yo'li nima?
7-3 ro'yxatda 7-1 ro'yxati mavjud bo'lib, ba'zi modullar va funksiyalar olib tashlangan.

Biz crate ildizida belgilangan yangi `restoranda_ovqatlanish` funksiyasidan `navbat_listiga_qoshish` funksiyasini chaqirishning ikkita usulini ko‘rsatamiz. Bu yoʻllar toʻgʻri, ammo bu misolni avvalgidek tuzishga toʻsqinlik qiladigan yana bir muammo bor. Sababini birozdan keyin tushuntiramiz.

`restoranda_ovqatlanish` funksiyasi kutubxonamizning umumiy API-ning bir qismidir, shuning uchun biz uni `pub` kalit so'zi bilan belgilaymiz. [“`pub` kalit so'zi bilan yo'llarni ochish”][pub]<!-- ignore --> bo‘limida biz `pub` haqida batafsilroq to‘xtalib o'tamiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-3: `navbat_listiga_qoshish` funksiyasini mutlaq va nisbiy yo'llar yordamida chaqirish</span>

Biz birinchi marta `restoranda_ovqatlanish` ichida `navbat_listiga_qoshish` funksiyasini chaqirganimizda mutlaq yo'ldan foydalanamiz. `navbat_listiga_qoshish` funksiyasi `restoranda_ovqatlanish` bilan bir xil crateda belgilangan, ya'ni mutlaq yoʻlni boshlash uchun `crate` kalit soʻzidan foydalanishimiz mumkin. Keyin biz `navbat_listiga_qoshish` ga o'tgunimizcha ketma-ket modullarning har birini o'z ichiga olamiz. Siz bir xil strukturaga ega fayl tizimini tasavvur qilishingiz mumkin: biz `navbat_listiga_qoshish` dasturini ishga tushirish uchun `/uyning_oldi/xizmat/navbat_listiga_qoshish` yo'lini belgilaymiz; crate ildizidan boshlash uchun `crate` nomidan foydalanish shelldagi fayl tizimi ildizidan boshlash uchun `/` dan foydalanishga o'xshaydi.

Biz `restoranda_ovqatlanish` ichida `navbat_listiga_qoshish` ni ikkinchi marta chaqirganimizda nisbiy yo'ldan foydalanamiz. Yo'l `uyning_oldi` bilan boshlanadi, modul nomi `restoranda_ovqatlanish` bilan bir xil modul daraxti darajasida belgilangan. Bu yerda fayl tizimi ekvivalenti `uyning_oldi/xizmat/navbat_listiga_qoshish` yo'lidan foydalaniladi. Modul nomi bilan boshlash yo'l nisbiy ekanligini bildiradi.

Nisbiy yoki mutlaq yo‘ldan foydalanishni tanlash loyihangiz asosida qabul qilinadigan qaror bo‘lib, element definitioni kodini elementdan foydalanadigan koddan alohida yoki birga ko‘chirish ehtimoli ko‘proq ekanligiga bog‘liq.
Masalan, `uyning_oldi` moduli va `restoranda_ovqatlanish` funksiyasini `mijoz_tajribasi` nomli modulga o‘tkazsak, mutlaq yo‘lni `navbat_listiga_qoshish`ga yangilashimiz kerak bo‘ladi, lekin nisbiy yo‘l baribir amal qiladi.
Biroq, agar biz `restoranda_ovqatlanish` funksiyasini `ovqatlanish` nomli modulga alohida ko'chirsak, `restoranda_ovqatlanish` chaqiruvining mutlaq yo'li bir xil bo'lib qoladi, lekin nisbiy yo'l yangilanishi kerak bo'ladi. Umuman olganda, bizning afzal ko'rganimiz mutlaq yo'llarni belgilashdir, chunki biz kod definitionlari va element chaqiruvlarini bir-biridan mustaqil ravishda ko'chirishni xohlaymiz.

Keling, 7-3 ro'yxatini kompilatsiya qilishga harakat qilaylik va nima uchun u hali kompilatsiya bo'lmaganligini bilib olaylik! Biz olgan xato 7-4 ro'yxatda ko'rsatilgan.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Ro'yxat 7-4: 7-3 ro'yxatdagi kodni kompilyatsiya qilishda kompilyator xatolari</span>

Xato xabarlari `xizmat` moduli shaxsiy ekanligini aytadi. Boshqacha qilib aytadigan bo'lsak, bizda `xizmat` moduli va `navbat_listiga_qoshish` funksiyasi uchun to'g'ri yo'llar mavjud, ammo Rust ulardan foydalanishimizga ruxsat bermaydi, chunki u shaxsiy bo'limlarga kirish imkoniga ega emas. Rust-da barcha elementlar (funktsiyalar, metodlar, structlar, enumlar, modullar va konstantalar) standart bo'yicha ota-modullar uchun shaxsiydir. Agar siz funksiya yoki struktura kabi elementni yaratmoqchi bo'lsangiz, uni modulga joylashtirasiz.

Ota-moduldagi elementlar ichki modullar ichidagi shaxsiy elementlardan foydalana olmaydi, lekin bolalar modullaridagi elementlar o'zlarining ota-modullaridagi elementlardan foydalanishi mumkin. Buning sababi shundaki, bolalar modullari o'zlarining amalga oshirish tafsilotlarini o'rab oladi va yashiradi, lekin bolalar modullari ular aniqlangan kontekstni ko'rishlari mumkin. Bizning metaforamizni davom ettirish uchun, maxfiylik qoidalarini restoranning orqa ofisi kabi tasavvur qiling: u erda nima sodir bo'layotgani restoran mijozlari uchun shaxsiy, ammo ofis menejerlari o'zlari ishlayotgan restoranda hamma narsani ko'rishlari va qilishlari mumkin.

Rust modul tizimining shu tarzda ishlashini tanladi, shuning uchun ichki dastur tafsilotlarini yashirish standart bo'yichadir. Shunday qilib, siz ichki kodning qaysi qismlarini tashqi kodni buzmasdan o'zgartirishingiz mumkinligini bilasiz. Biroq, Rust sizga obyektni hammaga ochiq qilish uchun `pub` kalit so'zidan foydalanib, tashqi ajdod modullariga ichki modullar kodining ichki qismlarini ochish imkoniyatini beradi.

### `pub` kalit so'zi bilan yo'llarni ochish

Keling, 7-4 ro'yxatdagi xatoga qaytaylik, bu bizga `xizmat` moduli shaxsiy ekanligini aytdi. Biz ota-moduldagi `restoranda_ovqatlanish` funksiyasi bolalar modulidagi `navbat_listiga_qoshish` funksiyasiga kirishini xohlaymiz, shuning uchun biz `xizmat` modulini `pub` kalit so'zi bilan belgilaymiz, ro'yxat 7-5da ko`rsatilganidek.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Ro'yxat 7-5: `xizmat` modulini `restoranda_ovqatlanish` dan foydalanish uchun `pub` deb e'lon qilish</span>

Afsuski, 7-5 ro'yxatdagi kod hali ham 7-6 ro'yxatda ko'rsatilganidek xatolikka olib keladi.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Ro'yxat 7-6: 7-5 ro'yxatdagi kodni build qilishda kompilyator xatolari</span>

What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we can access `front_of_house`, we can
access `hosting`. But the *contents* of `hosting` are still private; making the
module public doesn’t make its contents public. The `pub` keyword on a module
only lets code in its ancestor modules refer to it, not access its inner code.
Because modules are containers, there’s not much we can do by only making the
module public; we need to go further and choose to make one or more of the
items within the module public as well.

The errors in Listing 7-6 say that the `add_to_waitlist` function is private.
The privacy rules apply to structs, enums, functions, and methods as well as
modules.

Let’s also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listing 7-7: Adding the `pub` keyword to `mod hosting`
and `fn add_to_waitlist` lets us call the function from
`eat_at_restaurant`</span>

Now the code will compile! To see why adding the `pub` keyword lets us use
these paths in `add_to_waitlist` with respect to the privacy rules, let’s look
at the absolute and the relative paths.

In the absolute path, we start with `crate`, the root of our crate’s module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn’t public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub` and we can access its parent
module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the
first step: rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!

If you plan on sharing your library crate so other projects can use your code,
your public API is your contract with users of your crate that determines how
they can interact with your code. There are many considerations around managing
changes to your public API to make it easier for people to depend on your
crate. These considerations are out of the scope of this book; if you’re
interested in this topic, see [The Rust API Guidelines][api-guidelines].

> #### Best Practices for Packages with a Binary and a Library
>
> We mentioned a package can contain both a *src/main.rs* binary crate root as
> well as a *src/lib.rs* library crate root, and both crates will have the
> package name by default. Typically, packages with this pattern of containing
> both a library and a binary crate will have just enough code in the binary
> crate to start an executable that calls code with the library crate. This
> lets other projects benefit from the most functionality that the package
> provides, because the library crate’s code can be shared.
>
> The module tree should be defined in *src/lib.rs*. Then, any public items can
> be used in the binary crate by starting paths with the name of the package.
> The binary crate becomes a user of the library crate just like a completely
> external crate would use the library crate: it can only use the public API.
> This helps you design a good API; not only are you the author, you’re also a
> client!
>
> In [Chapter 12][ch12]<!-- ignore -->, we’ll demonstrate this organizational
> practice with a command-line program that will contain both a binary crate
> and a library crate.

### Starting Relative Paths with `super`

We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax. Using
`super` allows us to reference an item that we know is in the parent module,
which can make rearranging the module tree easier when the module is closely
related to the parent, but the parent might be moved elsewhere in the module
tree someday.

Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` defined in the `back_of_house` module calls the
function `deliver_order` defined in the parent module by specifying the path to
`deliver_order` starting with `super`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listing 7-8: Calling a function using a relative path
starting with `super`</span>

The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `deliver_order` and find it.
Success! We think the `back_of_house` module and the `deliver_order` function
are likely to stay in the same relationship to each other and get moved
together should we decide to reorganize the crate’s module tree. Therefore, we
used `super` so we’ll have fewer places to update code in the future if this
code gets moved to a different module.

### Making Structs and Enums Public

We can also use `pub` to designate structs and enums as public, but there are a
few details extra to the usage of `pub` with structs and enums. If we use `pub`
before a struct definition, we make the struct public, but the struct’s fields
will still be private. We can make each field public or not on a case-by-case
basis. In Listing 7-9, we’ve defined a public `back_of_house::Breakfast` struct
with a public `toast` field but a private `seasonal_fruit` field. This models
the case in a restaurant where the customer can pick the type of bread that
comes with a meal, but the chef decides which fruit accompanies the meal based
on what’s in season and in stock. The available fruit changes quickly, so
customers can’t choose the fruit or even see which fruit they’ll get.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Listing 7-9: A struct with some public fields and some
private fields</span>

Because the `toast` field in the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice that we can’t use the `seasonal_fruit` field in
`eat_at_restaurant` because `seasonal_fruit` is private. Try uncommenting the
line modifying the `seasonal_fruit` field value to see what error you get!

Also, note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast` didn’t
have such a function, we couldn’t create an instance of `Breakfast` in
`eat_at_restaurant` because we couldn’t set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.

In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword, as shown in Listing 7-10.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listing 7-10: Designating an enum as public makes all its
variants public</span>

Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad`
variants in `eat_at_restaurant`.

Enums aren’t very useful unless their variants are public; it would be annoying
to have to annotate all enum variants with `pub` in every case, so the default
for enum variants is to be public. Structs are often useful without their
fields being public, so struct fields follow the general rule of everything
being private by default unless annotated with `pub`.

There’s one more situation involving `pub` that we haven’t covered, and that is
our last module system feature: the `use` keyword. We’ll cover `use` by itself
first, and then we’ll show how to combine `pub` and `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
