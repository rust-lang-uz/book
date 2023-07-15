## Buyruqlar qatori argumentlarini qabul qilish

Keling, har doimgidek, `cargo new` bilan yangi loyiha yarataylik. Loyihamizni tizimingizda mavjud boʻlgan `grep` konsol dasturidan farqlash uchun uni `minigrep` deb ataymiz.

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

Birinchi vazifa `minigrep` ni ikkita buyruq qatori argumentlarini qabul qilishdir: fayl yo'li va izlash uchun satr. Ya'ni, biz o'z dasturimizni `cargo run` bilan ishga tushirishni xohlaymiz, quyidagi argumentlar `cargo` uchun emas, balki dasturimizga tegishli ekanligini ko'rsatadigan ikkita tire(qo'shaloq chiziq), qidirish uchun satr va qidiruv uchun faylga yo'l. ichida, shunga o'xshash:

```console
$ cargo run -- qidiruv-matni namuna-fayl.txt
```

Hozirda `cargo new` tomonidan yaratilgan dastur biz bergan argumentlarni qayta ishlay olmaydi. [crates.io](https://crates.io/)-dagi ba'zi mavjud kutubxonalar buyruq qatori argumentlarini qabul qiladigan dastur yozishda yordam berishi mumkin, ammo siz ushbu kontseptsiyani endigina o'rganayotganingiz uchun keling, bu imkoniyatni o'zimiz amalga oshiraylik.

### Argument qiymatlarini o'qish

`minigrep` ga biz o'tadigan buyruq qatori argumentlarining qiymatlarini o'qishni yoqish uchun bizga Rust standart kutubxonasida taqdim etilgan `std::env::args` funksiyasi kerak bo'ladi. Bu funksiya `minigrep` ga uzatilgan buyruq qatori argumentlarining iteratorini qaytaradi. Biz iteratorlarni [13-bobda][ch13]<!-- ignore
--> to'liq ko'rib chiqamiz. Hozircha siz iteratorlar haqida faqat ikkita ma'lumotni bilishingiz kerak: iteratorlar bir qator qiymatlarni ishlab chiqaradi va biz uni vector kabi to'plamga(collection) aylantirish uchun iteratorda `collect` metodini chaqirishimiz mumkin,iterator ishlab chiqaradigan barcha elementlarni o'z ichiga oladi.

12-1 ro'yxatidagi kod `minigrep` dasturingizga unga berilgan har qanday buyruq qatori argumentlarini o'qish va keyin qiymatlarni vectorga yig'ish imkonini beradi.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

<span class="caption">Listing 12-1: Collecting the command line arguments into
a vector and printing them</span>

First, we bring the `std::env` module into scope with a `use` statement so we
can use its `args` function. Notice that the `std::env::args` function is
nested in two levels of modules. As we discussed in [Chapter
7][ch7-idiomatic-use]<!-- ignore -->, in cases where the desired function is
nested in more than one module, we’ve chosen to bring the parent module into
scope rather than the function. By doing so, we can easily use other functions
from `std::env`. It’s also less ambiguous than adding `use std::env::args` and
then calling the function with just `args`, because `args` might easily be
mistaken for a function that’s defined in the current module.

> ### The `args` Function and Invalid Unicode
>
> Note that `std::env::args` will panic if any argument contains invalid
> Unicode. If your program needs to accept arguments containing invalid
> Unicode, use `std::env::args_os` instead. That function returns an iterator
> that produces `OsString` values instead of `String` values. We’ve chosen to
> use `std::env::args` here for simplicity, because `OsString` values differ
> per platform and are more complex to work with than `String` values.

On the first line of `main`, we call `env::args`, and we immediately use
`collect` to turn the iterator into a vector containing all the values produced
by the iterator. We can use the `collect` function to create many kinds of
collections, so we explicitly annotate the type of `args` to specify that we
want a vector of strings. Although we very rarely need to annotate types in
Rust, `collect` is one function you do often need to annotate because Rust
isn’t able to infer the kind of collection you want.

Finally, we print the vector using the debug macro. Let’s try running the code
first with no arguments and then with two arguments:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Notice that the first value in the vector is `"target/debug/minigrep"`, which
is the name of our binary. This matches the behavior of the arguments list in
C, letting programs use the name by which they were invoked in their execution.
It’s often convenient to have access to the program name in case you want to
print it in messages or change behavior of the program based on what command
line alias was used to invoke the program. But for the purposes of this
chapter, we’ll ignore it and save only the two arguments we need.

### Saving the Argument Values in Variables

The program is currently able to access the values specified as command line
arguments. Now we need to save the values of the two arguments in variables so
we can use the values throughout the rest of the program. We do that in Listing
12-2.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

<span class="caption">Listing 12-2: Creating variables to hold the query
argument and file path argument</span>

As we saw when we printed the vector, the program’s name takes up the first
value in the vector at `args[0]`, so we’re starting arguments at index `1`. The
first argument `minigrep` takes is the string we’re searching for, so we put a
reference to the first argument in the variable `query`. The second argument
will be the file path, so we put a reference to the second argument in the
variable `file_path`.

We temporarily print the values of these variables to prove that the code is
working as we intend. Let’s run this program again with the arguments `test`
and `sample.txt`:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Great, the program is working! The values of the arguments we need are being
saved into the right variables. Later we’ll add some error handling to deal
with certain potential erroneous situations, such as when the user provides no
arguments; for now, we’ll ignore that situation and work on adding file-reading
capabilities instead.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
