## Faylni o'qish

Now we’ll add functionality to read the file specified in the `file_path` argument. First, we need a sample file to test it with: we’ll use a file with a small amount of text over multiple lines with some repeated words. Listing 12-3 has an Emily Dickinson poem that will work well! Create a file called *poem.txt* at the root level of your project, and enter the poem “I’m Nobody! Who are you?”

<span class="filename">Fayl nomi: olma.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/olma.txt}}
```


<span class="caption">Ro'yxat 12-3: Olma haqidagi she'r yaxshi sinov ishini yaratadi</span>

Matn joyida bo'lgan holda *src/main.rs* ni tahrirlang va 12-4 ro'yxatda ko'rsatilganidek, faylni o'qish uchun kod qo'shing.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```


<span class="caption">Ro'yxat 12-4: Ikkinchi argument tomonidan ko'rsatilgan fayl mazmunini o'qish</span>

Birinchidan, biz standart kutubxonaning tegishli qismini `use` statementi bilan keltiramiz: fayllar bilan ishlash uchun bizga `std::fs` kerak.

`main` da yangi `fs::read_to_string` statementi `fayl_yoli`ni oladi, bu faylni ochadi va fayl mazmunining `std::io::Result<String>` ni qaytaradi.

Shundan so'ng, fayl o'qilgandan keyin `tarkib` qiymatini chop etadigan vaqtinchalik `println!` statementini yana qo'shamiz, shuning uchun dasturning hozirgacha ishlayotganligini tekshirishimiz mumkin.

Keling, ushbu kodni birinchi buyruq qatori argumenti sifatida istalgan qator bilan ishga tushiramiz (chunki biz hali qidiruv qismini amalga oshirmaganmiz) va ikkinchi argument sifatida *olma.txt* fayli:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Great! The code read and then printed the contents of the file. But the code has a few flaws. At the moment, the `main` function has multiple responsibilities: generally, functions are clearer and easier to maintain if each function is responsible for only one idea. The other problem is that we’re not handling errors as well as we could. The program is still small, so these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly. It’s good practice to begin refactoring early on when developing a program, because it’s much easier to refactor smaller amounts of code. We’ll do that next.
