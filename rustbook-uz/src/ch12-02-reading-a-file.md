## Faylni o'qish

Endi biz  `fayl_yoli` argumentida koʻrsatilgan faylni oʻqish funksiyasini qoʻshamiz. Birinchidan, uni sinab ko'rish uchun bizga namuna fayli kerak: biz bir nechta takroriy so'zlar bilan bir nechta satrlarda kichik hajmdagi matnli fayldan foydalanamiz. 12-3 ro'yxatda Olma haqida she'r bor, u yaxshi ishlaydi! Loyihangizning root darajasida *olma.txt* nomli fayl yarating va “Olma” she’rini kiriting.

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

First, we bring in a relevant part of the standard library with a `use`
statement: we need `std::fs` to handle files.

In `main`, the new statement `fs::read_to_string` takes the `file_path`, opens
that file, and returns a `std::io::Result<String>` of the file’s contents.

After that, we again add a temporary `println!` statement that prints the value
of `contents` after the file is read, so we can check that the program is
working so far.

Let’s run this code with any string as the first command line argument (because
we haven’t implemented the searching part yet) and the *poem.txt* file as the
second argument:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Great! The code read and then printed the contents of the file. But the code
has a few flaws. At the moment, the `main` function has multiple
responsibilities: generally, functions are clearer and easier to maintain if
each function is responsible for only one idea. The other problem is that we’re
not handling errors as well as we could. The program is still small, so these
flaws aren’t a big problem, but as the program grows, it will be harder to fix
them cleanly. It’s good practice to begin refactoring early on when developing
a program, because it’s much easier to refactor smaller amounts of code. We’ll
do that next.
