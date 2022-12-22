## O'zgaruvchilar va o'zgaruvchanlik

[”O'zgaruvchilar bilan qiymatlarni saqlash”][storing-values-with-variables]<!-- ignore --> bo'limida aytib o'tilganidek, standart bo'yicha o'zgaruvchilar o'zgarmasdir.Rust sizga o'z kodingizni Rust taqdim etgan xavfsizlik va qulay parallellikdan foydalanadigan tarzda yozish uchun beradigan ko'plab qulayliklardan biridir. Biroq, siz hali ham o'zgaruvchilaringizni o'zgaruvchan qilish imkoniyatiga egasiz.
Keling, Rust sizni qanday qilib va nima uchun o'zgarmaslikni afzal ko'rishga undashini va nega ba'zan siz undan voz kechishingiz mumkinligini bilib olaylik.

Agar o'zgaruvchi o'zgarmas bo'lsa, qiymat nomga bog'langandan keyin siz bu qiymatni o'zgartira olmaysiz. Buni ko'rsatish uchun `cargo new variables` yordamida *projects* jildingizda *o'zgaruvchilar* nomli yangi loyihani yarating.

Keyin, yangi *o'zgaruvchilar* jildida *src/main.rs* ni oching va uning kodini quyidagi kod bilan almashtiring. Bu kod hozircha kompilyatsiya qilinmaydi, biz avval o'zgarmaslik xatosini ko'rib chiqamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let x = 5;
    println!("x qiymati: {x}");
    x = 6;
    println!("x qiymati: {x}");
}
```

Kodni saqlang va dasturni `cargo run` yordamida ishga tushiring. Ushbu chiqishda ko'rsatilganidek, o'zgarmaslik xatosi haqida xato xabarini olishingiz kerak:

```console
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("x qiymati: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

Ushbu misol kompilyator sizning dasturlaringizdagi xatolarni topishga qanday yordam berishini ko'rsatadi.
Kompilyatordagi xatolar sizni asabiylashtirishi mumkin, lekin aslida ular sizning dasturingiz hali siz xohlagan narsani xavfsiz bajarmayotganligini anglatadi; ular sizning yaxshi dasturchi emasligingizni bildirmaydi! Tajribali Rustaceanlar hali ham kompilyator xatolariga duch kelishadi.

Siz oʻzgarmas `x` oʻzgaruvchisiga ikkinchi qiymatni belgilashga harakat qilganingiz uchun ````x` oʻzgaruvchisiga ikki marta tayinlab boʻlmaydi``` xato xabarini oldingiz.

O'zgarmas deb belgilangan qiymatni o'zgartirishga urinayotganda kompilyatsiya vaqtida xatolarga duch kelishimiz muhim, chunki bu holat xatolarga olib kelishi mumkin.Agar bizning kodimizning bir qismi qiymat hech qachon o'zgarmasligi haqidagi faraz asosida ishlayotgan bo'lsa va kodimizning boshqa qismi bu qiymatni o'zgartirsa, kodning birinchi qismi uni bajarish uchun mo'ljallangan narsani qilmasligi mumkin. Bunday xatoning sababini aniqlash qiyin bo'lishi mumkin, ayniqsa kodning ikkinchi qismi faqat *ba'zan* qiymatini o'zgartirganda. Rust kompilyatori qiymat o'zgarmasligini bildirganingizda, u haqiqatan ham o'zgarmasligini kafolatlaydi, shuning uchun uni o'zingiz kuzatib borishingiz shart emas. Shunday qilib, kodingizni tushunish osonroq.

Ammo o'zgaruvchanlik juda foydali bo'lishi mumkin va kodni yozishni qulayroq qilishi mumkin.
Garchi oʻzgaruvchilar standart boʻyicha oʻzgarmas boʻlsa-da, [2-bobda][storing-values-with-variables]<!-- ignore --> boʻlgani kabi oʻzgaruvchi nomi oldiga `mut` qoʻshish orqali ularni oʻzgaruvchan qilish mumkin. `mut` qo'shilishi, shuningdek, kodning boshqa qismlari ushbu o'zgaruvchining qiymatini o'zgartirishini ko'rsatib, kelajakdagi kod o'quvchilariga niyatni bildiradi.

Masalan, *src/main.rs* ni quyidagiga o'zgartiramiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn main() {
    let mut x = 5;
    println!("x qiymati: {x}");
    x = 6;
    println!("x qiymati: {x}");
}
```

Dasturni hozir ishga tushirganimizda, biz quyidagilarni olamiz:

```console
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
x qiymati: 5
x qiymati: 6
```

`mut` ishlatilganda `x` ga bog‘langan qiymatni `5` dan `6` ga o‘zgartirishga ruxsat beriladi. Oxir oqibat, o'zgaruvchanlikni qo'llash yoki qilmaslikni hal qilish sizga bog'liq va bu vaziyatda eng aniq deb o'ylagan narsangizga bog'liq.

### Konstantalar

O'zgarmas o'zgaruvchilar singari, *konstantalar* nomga bog'langan va o'zgarishi mumkin bo'lmagan qiymatlardir, lekin konstantalar va o'zgaruvchilar o'rtasida bir nechta farqlar mavjud.

Birinchidan, `mut` dan doimiylar bilan foydalanishga ruxsat berilmagan. Doimiylar standart bo'yicha shunchaki o'zgarmas emas - ular har doim o'zgarmasdir.Siz konstantalarni `let` kalit so'zi o'rniga `const` kalit so'zidan foydalanib e'lon qilasiz va qiymat turiga *annotatsiya qilinishi kerak*. Biz turlar va izohlarni keyingi ["Ma'lumotlar turlari"][data-types]<!-- ignore --> bo'limida ko'rib chiqamiz, shuning uchun hozir tafsilotlar haqida qayg'urmang. Bilingki, siz har doim turga izoh qo'yishingiz kerak.

Konstantalar har qanday miqyosda, shu jumladan global miqyosda e'lon qilinishi mumkin, bu ularni kodning ko'p qismlari bilishi kerak bo'lgan qiymatlar uchun foydali qiladi.

Oxirgi farq shundaki, konstantalar faqat ish vaqtida hisoblanishi mumkin bo'lgan qiymatning natijasi emas, balki faqat doimiy ifodaga o'rnatilishi mumkin.

Mana doimiy deklaratsiyaga misol:

```rust
const SONIYADA_UCH_SOAT: u32 = 60 * 60 * 3;
```

Konstanta nomi `SONIYADA_UCH_SOAT` va uning qiymati 60 ni (bir daqiqadagi soniyalar soni) 60 ga (bir soatdagi daqiqalar soni) 3 ga (biz hisoblamoqchi bo'lgan soatlar soni) ko'paytirish natijasiga o'rnatiladi. Rustning doimiylar uchun nomlash konventsiyasi so'zlar orasida barcha bosh harflarni pastki chiziq bilan ishlatishdir. Kompilyator kompilyatsiya vaqtida cheklangan operatsiyalar to'plamini baholashga qodir, bu bizga ushbu qiymatni 10,800 qiymatiga o'rnatmasdan, tushunish va tekshirish osonroq bo'lgan tarzda yozishni tanlash imkonini beradi.
Konstantalarni e'lon qilishda qanday operatsiyalardan foydalanish mumkinligi haqida qo'shimcha ma'lumot olish
[Rust Referencening konstantalar bo'limiga qarang][const-eval]

Konstantalar dastur ishlayotgan butun vaqt davomida, ular e'lon qilingan doirada amal qiladi. Bu xususiyat dasturning bir nechta qismlari bilishi kerak bo'lgan, masalan, o'yinning har qanday o'yinchisi olishi mumkin bo'lgan maksimal ball soni yoki yorug'lik tezligi kabi, ilova domeningizdagi qiymatlar uchun foydali konstantalarni qiladi.

Dasturingiz davomida ishlatiladigan qattiq kodlangan qiymatlarni doimiylar sifatida nomlash ushbu qiymatning ma'nosini kodning kelajakdagi maintainerlariga yetkazishda foydalidir. Bu, shuningdek, kodingizda faqat bitta joyga ega bo'lishga yordam beradi, agar kelajakda qattiq kodlangan qiymat yangilanishi kerak bo'lsa, o'zgartirishingiz kerak bo'ladi.

### Shadowing

As you saw in the guessing game tutorial in [Chapter
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, you can declare a
new variable with the same name as a previous variable. Rustaceans say that the
first variable is *shadowed* by the second, which means that the second
variable is what the compiler will see when you use the name of the variable.
In effect, the second variable overshadows the first, taking any uses of the
variable name to itself until either it itself is shadowed or the scope ends.
We can shadow a variable by using the same variable’s name and repeating the
use of the `let` keyword as follows:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

This program first binds `x` to a value of `5`. Then it creates a new variable
`x` by repeating `let x =`, taking the original value and adding `1` so the
value of `x` is then `6`. Then, within an inner scope created with the curly
brackets, the third `let` statement also shadows `x` and creates a new
variable, multiplying the previous value by `2` to give `x` a value of `12`.
When that scope is over, the inner shadowing ends and `x` returns to being `6`.
When we run this program, it will output the following:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Shadowing is different from marking a variable as `mut` because we’ll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword. By using `let`, we can perform a few transformations
on a value but have the variable be immutable after those transformations have
been completed.

The other difference between `mut` and shadowing is that because we’re
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name. For example, say our
program asks a user to show how many spaces they want between some text by
inputting space characters, and then we want to store that input as a number:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

The first `spaces` variable is a string type and the second `spaces` variable
is a number type. Shadowing thus spares us from having to come up with
different names, such as `spaces_str` and `spaces_num`; instead, we can reuse
the simpler `spaces` name. However, if we try to use `mut` for this, as shown
here, we’ll get a compile-time error:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

The error says we’re not allowed to mutate a variable’s type:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Now that we’ve explored how variables work, let’s look at more data types they
can have.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
