## Funksiyalar

Funksiyalar Rust kodida keng tarqalgan. Siz allaqachon tildagi eng muhim funksiyalardan birini ko'rgansiz: ko'plab dasturlarning kirish nuqtasi bo'lgan `main` funksiya. Siz yangi funksiyalarni e'lon qilish imkonini beruvchi `fn` kalit so'zini ham ko'rdingiz.

Rust kodi funksiya va oʻzgaruvchilar nomlari uchun anʼanaviy uslub sifatida *snake case* dan foydalanadi, unda barcha harflar kichik va alohida soʻzlarning tagiga chiziladi.
Mana, misol funksiya ta'rifini o'z ichiga olgan dastur:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust-da funksiyani `fn` so'ng funksiya nomi va qavslar to'plamini kiritish orqali aniqlaymiz. Jingalak qavslar kompilyatorga funktsiya tanasi qayerda boshlanishi va tugashini bildiradi.

Biz belgilagan har qanday funktsiyani uning nomidan keyin qavslar to'plamini kiritish orqali chaqirishimiz mumkin. Dasturda `boshqa_funksiya` ni aniqlanganligi sababli uni `main` funksiya ichidan chaqirish mumkin. E'tibor bering, biz `boshqa_funksiya` ni manba kodidagi `main` funksiyadan keyin belgilaganmiz; uni avval ham belgilashimiz mumkin edi. Rust sizning funksiyalaringizni qayerda belgilashingizning ahamiyati yo'q, faqat ular so'rov yuboruvchi tomonidan ko'rinadigan doirada aniqlangan.

Keling, funksiyalarni ko'proq o'rganish uchun *funktsiyalar* nomli yangi binary loyihani boshlaylik. `boshqa_funksiya` misolini *src/main.rs* ga joylashtiring va uni ishga tushiring.Quyidagi chiqishni ko'rishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Qatorlar `main` funksiyada paydo bo'ladigan tartibda bajariladi.
Avvaliga "Hello, world!" xabar chop etiladi, keyin `boshqa_funksiya` chaqiriladi va uning xabari chop etiladi.

### Parametrlar

Biz funksiyalarni *parametrlari* bo'lishi uchun belgilashimiz mumkin, ular funksiya imzosining bir qismi bo'lgan maxsus o'zgaruvchilardir. Agar funksiya parametrlarga ega bo'lsa, siz unga ushbu parametrlar uchun aniq qiymatlarni berishingiz mumkin. Texnik jihatdan aniq qiymatlar *argumentlar* deb ataladi, ammo tasodifiy suhbatda odamlar funksiya taʼrifidagi oʻzgaruvchilar yoki funksiyani chaqirganingizda qabul qilingan aniq qiymatlar uchun *parametr* va *argument* soʻzlarini bir-birining oʻrniga ishlatishga moyildirlar.

`boshqa_funksiya` ning ushbu versiyasida biz parametr qo'shamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Ushbu dasturni ishga tushirishga harakat qiling; quyidagi chiqishni olishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`boshqa_funksiya` deklaratsiyasi `x` nomli bitta parametrga ega. `x` turi `i32` sifatida belgilangan. Biz `5`ni `boshqa_funksiya`ga o‘tkazganimizda, `println!` makros `5` ni `x`ni o‘z ichiga olgan jingalak qavslar juftligi format satrida joylashgan joyga qo‘yadi.

Funksiya signaturelarda siz har bir parametr turini e'lon qilishingiz kerak. Bu Rust dizaynidagi ataylab qabul qilingan qaror: funksiya taʼriflarida turdagi izohlarni talab qilish kompilyatorga qaysi turni nazarda tutayotganingizni tushunish uchun ularni kodning boshqa joylarida ishlatishingizga deyarli hech qachon ehtiyoj sezmasligini anglatadi. Kompilyator, shuningdek, funksiya qanday turlarni kutayotganini bilsa, yanada foydali xato xabarlarini berishi mumkin.

Bir nechta parametrlarni belgilashda parametr deklaratsiyasini vergul bilan ajrating, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Ushbu misol ikkita parametrli `belgilangan_vaqt` nomli funksiyani yaratadi. Birinchi parametr `value` deb nomlangan va `i32` dir. Ikkinchisi `unit_label` deb nomlanadi va `char` turidir. Keyin funksiya `value` va ``unit_label` ni o‘z ichiga olgan matnni chop etadi.

Keling, ushbu kodni ishga tushirishga harakat qilaylik. Hozirda *funksiyalar* loyihangizning *src/main.rs* faylidagi dasturni oldingi misol bilan almashtiring va uni `cargo run` yordamida ishga tushiring:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Biz funksiyani `value` qiymati sifatida `5` va `unit_label` qiymati sifatida `'h'` deb ataganimiz sababli, dastur chiqishi ushbu qiymatlarni o`z ichiga oladi.

### Statementlar va  Expressionlar

Funksiya qismlari ixtiyoriy ravishda statement bilan tugaydigan bir qator expressionlardan iborat. Hozircha biz ko'rib chiqqan funktsiyalar yakuniy expressionni o'z ichiga olmagan, lekin siz expressionni statementning bir qismi sifatida ko'rdingiz. Rust expressionga asoslangan til bo'lganligi sababli, bu tushunish uchun muhim farqdir. Boshqa tillar bir xil farqlarga ega emas, shuning uchun keling, qanday statementlar va expressionlar ekanligini va ularning farqlari funksiyalar tanasiga qanday ta'sir qilishini ko'rib chiqaylik.

* **Statementlar** ba'zi amallarni bajaradigan va qiymat qaytarmaydigan ko'rsatmalardir.
* **Expressionlar** qiymatga baholanadi. Keling, ba'zi misollarni ko'rib chiqaylik.

Biz allaqachon statementlar va expressionlarni ishlatganmiz. O'zgaruvchini yaratish va unga `let` kalit so'zi bilan qiymat berish - bu statement. 3-1 ro'yxatda `let y = 6;` - bu statement.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Ro'yxat 3-1: Bitta statementni o'z ichiga olgan `main` funktsiya deklaratsiyasi</span>

Funksiya definitionlari ham statementlardir; oldingi misol o'z-o'zidan bir statementdir.

Statementlar qiymatlarni qaytarmaydi. Shuning uchun siz boshqa o'zgaruvchiga `let` iborasini tayinlay olmaysiz, chunki quyidagi kod bunga harakat qiladi; siz xatoga duch kelasiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Ushbu dasturni ishga tushirganingizda, sizda paydo bo'ladigan xato quyidagicha ko'rinadi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` statementi qiymat qaytarmaydi, shuning uchun `x` bog'lanishi uchun hech narsa yo'q. Bu boshqa tillarda sodir bo'ladigan narsadan farq qiladi, masalan, C va Ruby, bu yerda assignment assignmentning qiymatini qaytaradi. Bu tillarda siz `x = y = 6` yozishingiz mumkin va `x` va `y` ham `6` qiymatiga ega; Rustda bunday emas.

Expressionlar qiymatga baholanadi va siz Rust-da yozadigan kodning qolgan qismini tashkil qiladi. `5 + 6` kabi matematik amalni ko'rib chiqing, bu `11` qiymatini beruvchi expressiondir. Expressionlar statementlarning bir qismi bo'lishi mumkin: 3-1 ro'yxatdagi `let y = 6;` ifodasidagi `6`, `6` qiymatini beruvchi expressiondir. Funksiyani chaqirish expressiondir. Makroni chaqirish expressiondir. Jingalak qavslar bilan yaratilgan yangi qamrov bloki expressiondir, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Bu expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

blok bo'lib, bu holda `4` ga evaluate bo'ladi. Bu qiymat `let` statementining bir qismi sifatida `y` ga bog'lanadi. E'tibor bering, "`x + 1` qatorining oxirida nuqta-vergul yo'q, bu siz ko'rgan ko'pgina qatorlardan farqli o'laroq. Expressionlar yakuniy nuqtali vergullarni o'z ichiga olmaydi. Ifodaning oxiriga nuqtali vergul qo'shsangiz, uni statementga aylantirasiz va u keyinchalik qiymatni qaytarmaydi. Keyingi funksiyani qaytarish qiymatlari va expressionlarini o'rganayotganda buni yodda tuting.

### Return qiymatlari bilan funksiyalar

Functions can return values to the code that calls them. We don’t name return
values, but we must declare their type after an arrow (`->`). In Rust, the
return value of the function is synonymous with the value of the final
expression in the block of the body of a function. You can return early from a
function by using the `return` keyword and specifying a value, but most
functions return the last expression implicitly. Here’s an example of a
function that returns a value:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

There are no function calls, macros, or even `let` statements in the `five`
function—just the number `5` by itself. That’s a perfectly valid function in
Rust. Note that the function’s return type is specified too, as `-> i32`. Try
running this code; the output should look like this:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

The `5` in `five` is the function’s return value, which is why the return type
is `i32`. Let’s examine this in more detail. There are two important bits:
first, the line `let x = five();` shows that we’re using the return value of a
function to initialize a variable. Because the function `five` returns a `5`,
that line is the same as the following:

```rust
let x = 5;
```

Second, the `five` function has no parameters and defines the type of the
return value, but the body of the function is a lonely `5` with no semicolon
because it’s an expression whose value we want to return.

Let’s look at another example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Running this code will print `The value of x is: 6`. But if we place a
semicolon at the end of the line containing `x + 1`, changing it from an
expression to a statement, we’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Compiling this code produces an error, as follows:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

The main error message, `mismatched types`, reveals the core issue with this
code. The definition of the function `plus_one` says that it will return an
`i32`, but statements don’t evaluate to a value, which is expressed by `()`,
the unit type. Therefore, nothing is returned, which contradicts the function
definition and results in an error. In this output, Rust provides a message to
possibly help rectify this issue: it suggests removing the semicolon, which
would fix the error.
