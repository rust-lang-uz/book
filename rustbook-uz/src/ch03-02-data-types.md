## Ma'lumotlar turlari

Rust-dagi har bir qiymat ma'lum bir *ma'lumot turiga* tegishli bo'lib, Rustga qanday ma'lumotlar ko'rsatilayotganligini bildiradi, shuning uchun u ushbu ma'lumotlar bilan qanday ishlashni biladi. Biz ikkita ma'lumotlar turini ko'rib chiqamiz: skalyar va birikma.

Esda tutingki, Rust *statik tarzda yozilgan* tildir, ya'ni kompilyatsiya vaqtida barcha o'zgaruvchilarning turlarini bilishi kerak. Kompilyator odatda qiymat va uni qanday ishlatishimiz asosida biz qaysi turdan foydalanmoqchi ekanligimiz haqida xulosa chiqarishi mumkin.
Ko‘p turlar mumkin bo‘lgan hollarda, masalan, 2-bobdagi [“Tahminni maxfiy raqam bilan solishtirish”][comparing-the-guess-to-the-secret-number]<!-- ignore --> bo‘limidagi `parse` yordamida `String`ni raqamli turga o‘zgartirganimizda, quyidagi turdagi izohni qo‘shishimiz kerak:

```rust
let taxmin: u32 = "42".parse().expect("Raqam emas!");
```

Oldingi kodda ko'rsatilgan `: u32` turidagi izohni qo'shmasak, Rust quyidagi xatoni ko'rsatadi, ya'ni kompilyator bizdan qaysi turdan foydalanishni xohlayotganimizni bilish uchun qo'shimcha ma'lumotga muhtoj:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Boshqa ma'lumotlar turlari uchun turli turdagi izohlarni ko'rasiz.

### Skalyar Turlar

*Skalyar* turi bitta qiymatni ifodalaydi. Rust to'rtta asosiy skalyar turga ega: integerlar, floating-point number, boolean va belgilar. Siz ularni boshqa dasturlash tillaridan bilishingiz mumkin. Keling, ularning Rustda qanday ishlashini ko'rib chiqaylik.

#### Integer Turlari

*Integer* kasr komponenti bo‘lmagan sondir. Biz 2-bobda `u32` tipidagi bitta *integer* sonni ishlatdik. Ushbu turdagi deklaratsiya u bilan bog'langan qiymat 32 bit bo'sh joyni egallagan belgisiz butun son bo'lishi kerakligini bildiradi (Signed integer sonlar `u` o'rniga `i` bilan boshlanadi). 3-1-jadvalda Rust-da o'rnatilgan integer son turlari ko'rsatilgan. Integer son qiymatining turini e'lon qilish uchun biz ushbu variantlardan foydalanishimiz mumkin.

<span class="caption">3-1-jadval: Rustdagi Integer sonlar turlari</span>

| Uzunlik | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Signedlar kichkini `i` harfi bilan boshlanadi, Unsigned esa kichik `u` harfi bilan boshlanadi.

Har bir variant signed yoki unsigned bo'lishi mumkin va aniq o'lchamga ega.
*Signed* va *Unsigned* raqam manfiy boʻlishi mumkinmi yoki yoʻqligini anglatadi, boshqacha qilib aytganda, raqam u bilan birga belgiga ega boʻlishi (signed) boʻlishi kerakmi yoki u faqat ijobiy bo'ladimi va shuning uchun belgisiz (unsigned) ifodalanishi mumkinmi. Bu raqamlarni qog'ozga yozishga o'xshaydi: belgi muhim bo'lsa, raqam ortiqcha yoki minus belgisi bilan ko'rsatiladi; ammo, agar raqamni ijobiy deb hisoblash xavfsiz bo'lsa, u hech qanday belgisiz ko'rsatiladi.
Signed raqamlar [ikkita to'ldiruvchi][twos-complement]<!-- ignore--> ko'rinish yordamida saqlanadi.


Har bir signed variant -(2<sup>n - 1</sup>) dan 2<sup>n -
1</sup> -1 gacha bo'lgan raqamlarni saqlashi mumkin, bu erda *n* variant foydalanadigan bitlar soni.
Shunday qilib, `i8` -(2<sup>7</sup>) dan 2<sup>7</sup> - 1, gacha bo'lgan raqamlarni saqlashi mumkin, bu tengdir -128 dan 127 gacha.
Unsigned variantlar 0 dan 2<sup>n</sup> - 1 gacha raqamlarni saqlashi mumkin, shuning uchun `u8` 0 dan 2<sup>8</sup> - 1 gacha bo'lgan raqamlarni saqlashi mumkin, bu 0 dan 255 gacha.

Bundan tashqari, `isize` va `usize` turlari dasturingiz ishlayotgan kompyuterning arxitekturasiga bog'liq bo'lib, u jadvalda “arch” sifatida ko'rsatilgan: agar siz 64 bitli arxitekturada bo'lsangiz 64 bit va 32 bitli arxitekturada bo'lsangiz 32 bit.

Integer sonlarni 3-2-jadvalda ko'rsatilgan istalgan shaklda yozishingiz mumkin. E'tibor bering, bir nechta raqamli turlar bo'lishi mumkin bo'lgan son harflari turni belgilash uchun `57u8` kabi tur qo'shimchasiga ruxsat beradi. Raqamni o'qishni osonlashtirish uchun `_` dan raqamli harflar ham foydalanishi mumkin, masalan, `1_000`, siz `1000` ni ko'rsatganingizdek bir xil qiymatga ega bo'ladi.

<span class="caption">3-2-jadval: Rustdagi Integer literallar</span>

| Raqamli harflar   | Misol         |
|-------------------|---------------|
| O'nlik            | `98_222`      |
| O'n oltilik       | `0xff`        |
| Sakkizlik         | `0o77`        |
| Ikkilik           | `0b1111_0000` |
| Bayt (faqat "u8") | `b'A'`        |

Xo'sh, qaysi turdagi integer sonni ishlatishni qanday bilasiz? Agar ishonchingiz komil bo'lmasa, Rustning standart sozlamalari odatda boshlash uchun yaxshi joylardir: integer son turlari standart bo'yicha `i32` dir. `isize` yoki `usize` dan foydalanadigan asosiy holat to'plamning bir turini indekslashdir.

> ##### Integer Overflow
>
> Aytaylik, sizda 0 dan 255 gacha bo'lgan qiymatlarni ushlab turadigan `u8` tipidagi o'zgaruvchi bor.
> Agar siz o'zgaruvchini ushbu diapazondan tashqaridagi qiymatga o'zgartirishga harakat qilsangiz,
> masalan, 256, *integer overflow* sodir bo'ladi, bu ikki xatti-harakatdan biriga olib kelishi mumkin.
> Debug mode rejimida kompilyatsiya qilayotganingizda, Rust butun sonlarning to'lib ketishini
> tekshirishni o'z ichiga oladi, bu esa dasturni ishga tushirish vaqtida *panic* chiqaradi. Rust
> dastur xato bilan chiqqanda *panicking* atamasini ishlatadi; Biz panic haqida 9-bobdagi
> [“`panic` bilan tuzatib bo'lmaydigan xatolar”][unrecoverable-errors-with-panic]<!-- ignore -->
> bo'limda batafsil ko'rib chiqamiz
> 
> `--release` buyrug'i bilan reliz rejimida kompilyatsiya qilayotganingizda, Rust
> panic keltirib chiqaradigan butun sonlarni tekshirishni *o'z ichiga olmaydi*.
> overflow occur sodir bo'ladi Rust *ikkitasini to'ldiruvchi wrapni* bajaradi. Qisqa qilib
> aytganda, turdagi maksimal qiymatdan kattaroq qiymatlar, tur ushlab turishi mumkin bo'lgan minimal
> qiymatlargacha "wrap" ni tashkil qiladi. `u8` holatida 256 qiymati 0 ga, 257 qiymati
> 1 ga aylanadi va hokazo. Dastur panic qo'ymaydi, lekin o'zgaruvchi
> siz kutgan qiymatga ega bo'lmaydi. Butun sonlarni wrapga tayanish
> xato hisoblanadi. Owerflow ehtimolini aniq ko'rib chiqish uchun siz prime sonlar uchun
> standart kutubxona tomonidan taqdim etilgan ushbu usullar oilalaridan foydalanishingiz mumkin:
> 
> * Barcha modelarni `wrapping_*` methodlari bilan oʻrash, masalan, `wrapping_add`.
> * Agar `checked_*` methodlari owerflow boʻlsa, `None` qiymatini qaytaring.
> * Qiymat va boolean qiymatni qaytaring, bu `overflowing_*` methodlari
>   bilan overflow bo'lganini ko'rsatadi.
> * Qiymatning minimal yoki maksimal qiymatlarida `saturating_*`
>   methodllari bilan saturate bo'lgan.

#### Floating-Point Turlari

Rust shuningdek *floating-point raqamlar* uchun ikkita primitive turga ega, ular kasrli raqamlardir.
Rust-ning floating-point turlari `f32` va `f64` bo'lib, ular mos ravishda 32 bit va 64 bit o'lchamga ega.
Standart tur `f64` dir, chunki zamonaviy protsessorlarda u `f32` bilan bir xil tezlikda, lekin aniqroq bo'lishga qodir.
Barcha floating-point turlari signeddir.

Bu yerda harakatdagi floating-point raqamlarni ko'rsatadigan misol:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Floating-point raqamlari IEEE-754 standartiga muvofiq taqdim etiladi. `f32` turi bitta aniqlikdagi floatdir va `f64` ikki tomonlama aniqlikka ega.

#### Raqamli operatsiyalar

Rust barcha turdagi raqamlar uchun kutilgan asosiy matematik operatsiyalarni qo'llab-quvvatlaydi: qo'shish, ayirish, ko'paytirish, bo'lish va qoldiq. Butun sonni bo'lish noldan eng yaqin butun songa qisqaradi. Quyidagi kod `let` iborasida har bir raqamli operatsiyadan qanday foydalanishni ko'rsatadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Ushbu bayonotlardagi har bir ifoda matematik operatordan foydalanadi va bitta qiymatga baholanadi, keyin esa o'zgaruvchiga bog'lanadi. [B ilovasi][appendix_b]<!-- ignore --> da
Rust taqdim etgan barcha operatorlar ro'yxati mavjud.

#### Boolean turi

Ko'pgina boshqa dasturlash tillarida bo'lgani kabi, Rust-da ham Boolean turi ikkita mumkin bo'lgan qiymatga ega: `true` va `false`. Boolean hajmi bir baytga teng.
Rustdagi boolean turi `bool` yordamida belgilanadi. Misol uchun:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean qiymatlardan foydalanishning asosiy usuli shartlardir, masalan, `if` ifodasidir. Rustda `if` iboralari qanday ishlashini [“Control Flow”][control-flow]<!-- ignore --> bo‘limida ko‘rib chiqamiz.

#### Belgilar(Character) turi

Rustning `char` turi tilning eng primitive alifbo turidir. Mana `char` qiymatlarini e'lon qilishning ba`zi misollari:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

E'tibor bering, biz qo'sh tirnoq ishlatadigan satr harflaridan farqli o'laroq, `char` harflarini bitta tirnoq bilan belgilaymiz. Rustning `char` turi to'rt bayt o'lchamga ega va Unicode Scalar qiymatini ifodalaydi, ya'ni u ASCIIdan ko'ra ko'proq narsani anglatishi mumkin.
Urg'uli harflar; Xitoy, yapon va koreys belgilar; emoji; va nol kenglikdagi boʻshliqlar Rust-dagi barcha haqiqiy `char` qiymatlaridir. Unicode Scalar qiymatlari `U+0000`dan `U+D7FF`gacha va `U+E000`dan `U+10FFFF`gacha.
Biroq, “character” aslida Unicode-da tushuncha emas, shuning uchun “character” nima ekanligi haqidagi Rustdagi `char` bilan mos kelmasligi mumkin. Biz ushbu mavzuni 8-bobdagi [“UTF-8 kodlangan matnni satrlar bilan saqlash”][strings]<!-- ignore --> bo'limida batafsil muhokama qilamiz.

### Murakkab turlar

*Murakkab turlar* bir nechta qiymatlarni bir turga to'plashi mumkin.Rust ikkita primitive birikma turiga ega: tuplelar va arraylar.

#### Tuple turi

*tuple* - bu turli xil turlarga ega bo'lgan bir qator qiymatlarni bitta qo'shma turga birlashtirishning umumiy usuli.Tuplelar belgilangan uzunlikka ega: bir marta e'lon qilingandan so'ng, ular o'sishi yoki kichrayishi mumkin emas.

Qavslar ichida vergul bilan ajratilgan qiymatlar ro'yxatini yozish orqali tuple yaratamiz. Tupledagi har bir pozitsiya o'z turiga ega va tupledagi turli qiymatlarning turlari bir xil bo'lishi shart emas. Ushbu misolda biz ixtiyoriy turdagi izohlarni qo'shdik:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

`tup` o'zgaruvchisi butun tuplega bog'lanadi, chunki tuple bitta birikma element hisoblanadi. Tupledan individual qiymatlarni olish uchun biz tuple qiymatini buzish uchun pattern moslashuvidan foydalanishimiz mumkin, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring* because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.

We can also access a tuple element directly by using a period (`.`) followed by
the index of the value we want to access. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

This program creates the tuple `x` and then accesses each element of the tuple
using their respective indices. As with most programming languages, the first
index in a tuple is 0.

The tuple without any values has a special name, *unit*. This value and its
corresponding type are both written `()` and represent an empty value or an
empty return type. Expressions implicitly return the unit value if they don’t
return any other value.

#### The Array Type

Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Unlike arrays in
some other languages, arrays in Rust have a fixed length.

We write the values in an array as a comma-separated list inside square
brackets:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in [Chapter
4][stack-and-heap]<!-- ignore -->) or when you want to ensure you always have a
fixed number of elements. An array isn’t as flexible as the vector type,
though. A *vector* is a similar collection type provided by the standard
library that *is* allowed to grow or shrink in size. If you’re unsure whether
to use an array or a vector, chances are you should use a vector. [Chapter
8][vectors]<!-- ignore --> discusses vectors in more detail.

However, arrays are more useful when you know the number of elements will not
need to change. For example, if you were using the names of the month in a
program, you would probably use an array rather than a vector because you know
it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You write an array’s type using square brackets with the type of each element,
a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number `5`
indicates the array contains five elements.

You can also initialize an array to contain the same value for each element by
specifying the initial value, followed by a semicolon, and then the length of
the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

The array named `a` will contain `5` elements that will all be set to the value
`3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a
more concise way.

##### Accessing Array Elements

An array is a single chunk of memory of a known, fixed size that can be
allocated on the stack. You can access elements of an array using indexing,
like this:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

In this example, the variable named `first` will get the value `1` because that
is the value at index `[0]` in the array. The variable named `second` will get
the value `2` from index `[1]` in the array.

##### Invalid Array Element Access

Let’s see what happens if you try to access an element of an array that is past
the end of the array. Say you run this code, similar to the guessing game in
Chapter 2, to get an array index from the user:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

This code compiles successfully. If you run this code using `cargo run` and
enter `0`, `1`, `2`, `3`, or `4`, the program will print out the corresponding
value at that index in the array. If you instead enter a number past the end of
the array, such as `10`, you’ll see output like this:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The program resulted in a *runtime* error at the point of using an invalid
value in the indexing operation. The program exited with an error message and
didn’t execute the final `println!` statement. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than or equal to the length,
Rust will panic. This check has to happen at runtime, especially in this case,
because the compiler can’t possibly know what value a user will enter when they
run the code later.

This is an example of Rust’s memory safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling and how you can
write readable, safe code that neither panics nor allows invalid memory access.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
