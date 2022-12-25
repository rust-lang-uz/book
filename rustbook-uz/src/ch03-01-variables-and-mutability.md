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

[2-bobdagi Guessing oʻyini][comparing-the-guess-to-the-secret-number]<!-- ignore --> boʻyicha qoʻllanmada koʻrganingizdek, oldingi oʻzgaruvchi bilan bir xil nomli yangi oʻzgaruvchini eʼlon qilishingiz mumkin. Rustaceans say that the
Rustaceanlarning aytishicha, birinchi o'zgaruvchi ikkinchi o'zgaruvchi tomonidan **shadow qilingan* ya'ni ikkinchi o'zgaruvchi o'zgaruvchi nomidan foydalanganda kompilyator ko'radigan narsadir.
Darhaqiqat, ikkinchi o'zgaruvchi birinchisiga shadow qilib, o'zgaruvchi nomidan har qanday foydalanishni uning o'zi shadowli bo'lmaguncha yoki doirasi tugaguncha oladi.
Biz bir xil oʻzgaruvchining nomidan foydalanib, `let` kalit soʻzidan foydalanishni quyidagi tarzda takrorlash orqali oʻzgaruvchini shadow qilishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x ning ichki doiradagi qiymati: {x}");
    }

    println!("x qiymati: {x}");
}

```

Bu dastur avval `x` ni `5` qiymatiga bog'laydi. Keyin u `let x =` ni takrorlab, asl qiymatni olib, `1` qo'shish orqali yangi `x` o'zgaruvchisini yaratadi, shunda `x` qiymati `6` bo'ladi. Keyin, jingalak qavslar bilan yaratilgan ichki doirada uchinchi `let` iborasi ham `x` ga shadow qiladi va yangi o'zgaruvchini yaratadi va oldingi qiymatni `2` ga ko'paytirib, `x` ga `12` qiymatini beradi.
Bu doira tugagach, ichki shadow tugaydi va `x` `6` ga qaytadi.
Ushbu dasturni ishga tushirganimizda, u quyidagilarni chiqaradi:

```console
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
x ning ichki doiradagi qiymati: 12
x qiymati: 6
```

Shadowing o‘zgaruvchini `mut` deb belgilashdan farq qiladi, chunki `let` kalit so‘zidan foydalanmasdan tasodifan ushbu o‘zgaruvchiga qayta tayinlashga harakat qilsak, kompilyatsiya vaqtida xatolikka yo‘l qo‘yamiz. `let` dan foydalanib, biz qiymat bo'yicha bir nechta o'zgarishlarni amalga oshirishimiz mumkin, lekin bu o'zgarishlar tugagandan so'ng o'zgaruvchi o'zgarmas bo'lishi mumkin.

`Mut` va shadow o'rtasidagi boshqa farq shundaki, biz `let` kalit so'zini qayta ishlatganimizda yangi o'zgaruvchini samarali yaratayotganimiz sababli, qiymat turini o`zgartirishimiz mumkin, lekin bir xil nomni qayta ishlatishimiz ham mumkin. Misol uchun, bizning dasturimiz foydalanuvchidan bo'sh joy belgilarini kiritish orqali ba'zi matnlar orasida qancha bo'sh joy bo'lishini ko'rsatishni so'raydi va biz ushbu kiritishni raqam sifatida saqlamoqchimiz:

```rust
    let joylar = "   ";
    let joylar = joylar.len();
```

Birinchi `joylar` o'zgaruvchisi satr turi, ikkinchi `joylar` o'zgaruvchisi esa raqam turi. Shadowing shu tariqa bizni turli nomlar bilan chiqishdan saqlaydi, masalan, `joylar_str` va `joylar_num`; Buning o'rniga biz oddiyroq `joylar` nomini qayta ishlatishimiz mumkin. Biroq, bu erda ko'rsatilganidek, buning uchun `mut` dan foydalanmoqchi bo'lsak, kompilyatsiya vaqtida xatoga duch kelamiz:

```rust,ignore,does_not_compile
    let mut joylar = "   ";
    joylar = joylar.len();
```

Xato bizga o'zgaruvchining turini mutatsiyaga o'tkazishga ruxsat yo'qligini aytadi:

```console
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut joylar = "   ";
  |                      ----- expected due to this value
3 |     joylar = joylar.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error

```
Endi biz o'zgaruvchilar qanday ishlashini o'rganib chiqdik, keling, ular bo'lishi mumkin bo'lgan ko'proq ma'lumotlar turlarini ko'rib chiqaylik.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
