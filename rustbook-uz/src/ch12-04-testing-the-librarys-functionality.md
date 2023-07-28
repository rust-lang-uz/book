## Testga asoslangan ishlab chiqish bilan kutubxonaning funksionalligini rivojlantirish

Endi biz mantiqni *src/lib.rs* ga chiqardik va argumentlarni yig‘ish va xatolarni qayta ishlashni *src/main.rs* da qoldirdik, kodimizning asosiy funksionalligi uchun testlarni yozish ancha osonlashdi. Biz turli xil argumentlar bilan funksiyalarni to'g'ridan-to'g'ri chaqirishimiz va buyruq satridan binaryga murojaat qilmasdan qaytish(return) qiymatlarini tekshirishimiz mumkin.

Ushbu bo'limda biz quyidagi bosqichlar bilan test-driven development (TDD) jarayonidan foydalangan holda `minigrep` dasturiga qidiruv mantig'ini qo'shamiz:

1. Muvaffaqiyatsiz bo'lgan testni yozing va siz kutgan sabab tufayli muvaffaqiyatsiz bo'lishiga ishonch hosil qilish uchun uni ishga tushiring.
2. Yangi testdan o'tish uchun yetarli kodni yozing yoki o'zgartiring.
3. Siz qo'shgan yoki o'zgartirgan kodni qayta tiklang(refaktoring) va testlar o'tishda davom etayotganiga ishonch hosil qiling.
4. Repeat from step 1!

Garchi bu dasturiy ta'minotni yozishning ko'p usullaridan biri bo'lsa-da, TDD kod dizaynini boshqarishga yordam beradi. Testdan o'tishni ta'minlaydigan kodni yozishdan oldin testni yozish jarayon davomida yuqori sinov qamrovini saqlashga yordam beradi.

Biz fayl tarkibidagi so'rovlar qatorini qidirishni amalga oshiradigan va so'rovga mos keladigan qatorlar ro'yxatini tuzadigan funksiyani amalga oshirishni sinovdan o'tkazamiz. Biz bu funksiyani `qidiruv` funksiyasiga qo‘shamiz.

### Muvaffaqiyatsiz test yozish

Bizga endi ular kerak emasligi sababli, dasturning harakatini tekshirish uchun foydalanilgan *src/lib.rs* va *src/main.rs* dan `println!` statementlarini olib tashlaymiz. Keyin, *src/lib.rs* da, [11-bobda][ch11-anatomy]<!-- ignore --> qilganimizdek, test funksiyasiga ega `tests` modulini qo'shing. Test funksiyasi biz `qidirish` funksiyasiga ega bo'lishini xohlagan xatti-harakatni belgilaydi: u so'rov va izlash uchun matnni oladi va u so'rovni o'z ichiga olgan matndan faqat satrlarni qaytaradi. 12-15 ro'yxatda ushbu test ko'rsatilgan, u hali kompilyatsiya bo'lmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">12-15 roʻyxat: `qidiruv` funksiyasi uchun muvaffaqiyatsiz test yaratish</span>

Bu test `marali` qatorini qidiradi.Biz izlayotgan matn uchta qatordan iborat bo‘lib, ulardan faqat bittasi `marali`ni o‘z ichiga oladi (E’tibor bering, qo‘sh qo‘shtirnoqning ochilishidan keyingi teskari chiziq Rustga ushbu satr literalining boshiga yangi qator belgisini qo‘ymaslikni bildiradi). `qidiruv` funksiyasidan qaytarilgan qiymat faqat biz kutgan qatorni o'z ichiga oladi, deb ta'kidlaymiz.

Biz hali bu testni bajara olmaymiz va uning muvaffaqiyatsizligini kuzata olmaymiz, chunki test hatto kompilyatsiya ham qilmaydi: `qidiruv` funksiyasi hali mavjud emas! TDD tamoyillariga muvofiq, biz 12-16 roʻyxatda koʻrsatilganidek, har doim boʻsh vektorni qaytaruvchi `qidiruv` funksiyasining definitionni qoʻshish orqali testni kompilyatsiya qilish va ishga tushirish uchun yetarli kodni qoʻshamiz. Keyin test kompilyatsiya qilinishi va muvaffaqiyatsiz bo'lishi kerak, chunki bo'sh vektor `"xavfsiz, tez, samarali."` qatorini o'z ichiga olgan vektorga mos kelmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 12-16: `qidiruv` funksiyasini yetarli darajada aniqlash, shuning uchun testimiz kompilyatsiya bo'ladi</span>

E'tibor bering, biz `qidiruv` signaturesida `'a` aniq lifetimeni belgilashimiz va bu lifetimeni `tarkib` argumenti va qaytarish(return) qiymati bilan ishlatishimiz kerak. [10-bobda][ch10-lifetimes]<!-- ignore -->  esda tutingki, lifetime parametrlari qaysi argumentning lifetime(ishlash muddati) qaytariladigan qiymatning lifetime bilan bog'liqligini belgilaydi. Bunday holda, qaytarilgan vektorda `tarkib` argumentining bo'laklariga (`sorov` argumenti o'rniga) reference qiluvchi string bo'laklari bo'lishi kerakligini ko'rsatamiz.

Boshqacha qilib aytganda, biz Rustga aytamizki, `qidiruv` funksiyasi tomonidan qaytarilgan maʼlumotlar `tarkib` argumentida `qidiruv` funksiyasiga oʻtgan maʼlumotlar shuncha vaqtgacha yashaydi. Bu muhim! Murojaatlar haqiqiy bo'lishi uchun bo'laklar(slice) bo'yicha reference qilingan ma'lumotlar ham haqiqiy bo'lishi kerak; agar kompilyator biz `tarkib` emas, balki `sorov` ning satr bo'laklarini(string slice) yaratmoqda deb hisoblasa, u xavfsizlik tekshiruvini noto'g'ri bajaradi.

Agar biz lifetime izohlarni(annotation) unutib, ushbu funksiyani kompilyatsiya qilishga harakat qilsak, biz ushbu xatoni olamiz:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust can’t possibly know which of the two arguments we need, so we need to tell
it explicitly. Because `contents` is the argument that contains all of our text
and we want to return the parts of that text that match, we know `contents` is
the argument that should be connected to the return value using the lifetime
syntax.

Other programming languages don’t require you to connect arguments to return
values in the signature, but this practice will get easier over time. You might
want to compare this example with the [“Validating References with
Lifetimes”][validating-references-with-lifetimes]<!-- ignore --> section in
Chapter 10.

Now let’s run the test:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

Great, the test fails, exactly as we expected. Let’s get the test to pass!

### Writing Code to Pass the Test

Currently, our test is failing because we always return an empty vector. To fix
that and implement `search`, our program needs to follow these steps:

* Iterate through each line of the contents.
* Check whether the line contains our query string.
* If it does, add it to the list of values we’re returning.
* If it doesn’t, do nothing.
* Return the list of results that match.

Let’s work through each step, starting with iterating through lines.

#### Iterating Through Lines with the `lines` Method

Rust has a helpful method to handle line-by-line iteration of strings,
conveniently named `lines`, that works as shown in Listing 12-17. Note this
won’t compile yet.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">Listing 12-17: Iterating through each line in `contents`
</span>

The `lines` method returns an iterator. We’ll talk about iterators in depth in
[Chapter 13][ch13-iterators]<!-- ignore -->, but recall that you saw this way
of using an iterator in [Listing 3-5][ch3-iter]<!-- ignore -->, where we used a
`for` loop with an iterator to run some code on each item in a collection.

#### Searching Each Line for the Query

Next, we’ll check whether the current line contains our query string.
Fortunately, strings have a helpful method named `contains` that does this for
us! Add a call to the `contains` method in the `search` function, as shown in
Listing 12-18. Note this still won’t compile yet.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">Listing 12-18: Adding functionality to see whether the
line contains the string in `query`</span>

At the moment, we’re building up functionality. To get it to compile, we need
to return a value from the body as we indicated we would in the function
signature.

#### Storing Matching Lines

To finish this function, we need a way to store the matching lines that we want
to return. For that, we can make a mutable vector before the `for` loop and
call the `push` method to store a `line` in the vector. After the `for` loop,
we return the vector, as shown in Listing 12-19.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">Listing 12-19: Storing the lines that match so we can
return them</span>

Now the `search` function should return only the lines that contain `query`,
and our test should pass. Let’s run the test:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Our test passed, so we know it works!

At this point, we could consider opportunities for refactoring the
implementation of the search function while keeping the tests passing to
maintain the same functionality. The code in the search function isn’t too bad,
but it doesn’t take advantage of some useful features of iterators. We’ll
return to this example in [Chapter 13][ch13-iterators]<!-- ignore -->, where
we’ll explore iterators in detail, and look at how to improve it.

#### Using the `search` Function in the `run` Function

Now that the `search` function is working and tested, we need to call `search`
from our `run` function. We need to pass the `config.query` value and the
`contents` that `run` reads from the file to the `search` function. Then `run`
will print each line returned from `search`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

We’re still using a `for` loop to return each line from `search` and print it.

Now the entire program should work! Let’s try it out, first with a word that
should return exactly one line from the Emily Dickinson poem, “frog”:

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Cool! Now let’s try a word that will match multiple lines, like “body”:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

And finally, let’s make sure that we don’t get any lines when we search for a
word that isn’t anywhere in the poem, such as “monomorphization”:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Excellent! We’ve built our own mini version of a classic tool and learned a lot
about how to structure applications. We’ve also learned a bit about file input
and output, lifetimes, testing, and command line parsing.

To round out this project, we’ll briefly demonstrate how to work with
environment variables and how to print to standard error, both of which are
useful when you’re writing command line programs.

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
