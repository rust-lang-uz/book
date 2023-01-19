## O'zgaruvchilar va o'zgaruvchanlik

[”O'zgaruvchilar bilan qiymatlarni saqlash”][storing-values-with-variables]<!-- ignore --> bo'limida aytib o'tilganidek, standart bo'yicha o'zgaruvchilar o'zgarmasdir.Rust sizga o'z kodingizni Rust taqdim etgan xavfsizlik va qulay parallellikdan foydalanadigan tarzda yozish uchun beradigan ko'plab qulayliklardan biridir. Biroq, siz hali ham o'zgaruvchilaringizni o'zgaruvchan qilish imkoniyatiga egasiz.
Keling, Rust sizni qanday qilib va nima uchun o'zgarmaslikni afzal ko'rishga undashini va nega ba'zan siz undan voz kechishingiz mumkinligini bilib olaylik.

Agar o'zgaruvchi o'zgarmas bo'lsa, qiymat nomga bog'langandan keyin siz bu qiymatni o'zgartira olmaysiz. Buni ko'rsatish uchun `cargo new variables` yordamida *projects* jildingizda *variables* nomli yangi loyihani yarating.

Keyin, yangi *variables* jildida *src/main.rs* ni oching va uning kodini quyidagi kod bilan almashtiring. Bu kod hozircha kompilyatsiya qilinmaydi, biz avval o'zgarmaslik xatosini ko'rib chiqamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Kodni saqlang va dasturni `cargo run` yordamida ishga tushiring. Ushbu chiqishda ko'rsatilganidek, o'zgarmaslik xatosi haqida xato xabarini olishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
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
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Dasturni hozir ishga tushirganimizda, biz quyidagilarni olamiz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut` ishlatilganda `x` ga bog‘langan qiymatni `5` dan `6` ga o‘zgartirishga ruxsat beriladi. Oxir oqibat, o'zgaruvchanlikni qo'llash yoki qilmaslikni hal qilish sizga bog'liq va bu vaziyatda eng aniq deb o'ylagan narsangizga bog'liq.

### Konstantalar

O'zgarmas o'zgaruvchilar singari, *konstantalar* nomga bog'langan va o'zgarishi mumkin bo'lmagan qiymatlardir, lekin konstantalar va o'zgaruvchilar o'rtasida bir nechta farqlar mavjud.

Birinchidan, `mut` dan konstantalar bilan foydalanishga ruxsat berilmagan. Konstantalar standart bo'yicha shunchaki o'zgarmas emas - ular har doim o'zgarmasdir.Siz konstantalarni `let` kalit so'zi o'rniga `const` kalit so'zidan foydalanib e'lon qilasiz va qiymat turiga *annotatsiya qilinishi kerak*. Biz turlar va izohlarni keyingi ["Ma'lumotlar turlari"][data-types]<!-- ignore --> bo'limida ko'rib chiqamiz, shuning uchun hozir tafsilotlar haqida qayg'urmang. Bilingki, siz har doim turga annotate qo'yishingiz kerak.

Konstantalar har qanday miqyosda, shu jumladan global miqyosda e'lon qilinishi mumkin, bu ularni kodning ko'p qismlari bilishi kerak bo'lgan qiymatlar uchun foydali qiladi.

Oxirgi farq shundaki, konstantalar faqat ish vaqtida hisoblanishi mumkin bo'lgan qiymatning natijasi emas, balki faqat konstanta ifodaga o'rnatilishi mumkin.

Mana konstanta deklaratsiyaga misol:

```rust
const SONIYADA_UCH_SOAT: u32 = 60 * 60 * 3;
```

Konstanta nomi `SONIYADA_UCH_SOAT` va uning qiymati 60 ni (bir daqiqadagi soniyalar soni) 60 ga (bir soatdagi daqiqalar soni) 3 ga (biz hisoblamoqchi bo'lgan soatlar soni) ko'paytirish natijasiga o'rnatiladi. Rustning konstantalar uchun nomlash konventsiyasi so'zlar orasida barcha bosh harflarni pastki chiziq bilan ishlatishdir. Kompilyator kompilyatsiya vaqtida cheklangan operatsiyalar to'plamini baholashga qodir, bu bizga ushbu qiymatni 10,800 qiymatiga o'rnatmasdan, tushunish va tekshirish osonroq bo'lgan tarzda yozishni tanlash imkonini beradi.
Konstantalarni e'lon qilishda qanday operatsiyalardan foydalanish mumkinligi haqida qo'shimcha ma'lumot olish
[Rust Referencening konstantalar bo'limiga qarang][const-eval]

Konstantalar dastur ishlayotgan butun vaqt davomida, ular e'lon qilingan doirada amal qiladi. Bu xususiyat dasturning bir nechta qismlari bilishi kerak bo'lgan, masalan, o'yinning har qanday o'yinchisi olishi mumkin bo'lgan maksimal ball soni yoki yorug'lik tezligi kabi, ilova domeningizdagi qiymatlar uchun foydali konstantalarni qiladi.

Dasturingiz davomida ishlatiladigan qattiq kodlangan qiymatlarni konstantalar sifatida nomlash ushbu qiymatning ma'nosini kodning kelajakdagi maintainerlariga yetkazishda foydalidir. Bu, shuningdek, kodingizda faqat bitta joyga ega bo'lishga yordam beradi, agar kelajakda qattiq kodlangan qiymat yangilanishi kerak bo'lsa, o'zgartirishingiz kerak bo'ladi.

### Shadowing

[2-bobdagi Taxmin qilish oʻyini][comparing-the-guess-to-the-secret-number]<!-- ignore --> boʻyicha qoʻllanmada koʻrganingizdek, oldingi oʻzgaruvchi bilan bir xil nomli yangi oʻzgaruvchini eʼlon qilishingiz mumkin.Rustaceanlarning aytishicha, birinchi o'zgaruvchi ikkinchi o'zgaruvchi tomonidan *shadow qilingan* ya'ni ikkinchi o'zgaruvchi o'zgaruvchi nomidan foydalanganda kompilyator ko'radigan narsadir.
Darhaqiqat, ikkinchi o'zgaruvchi birinchisiga shadow qilib, o'zgaruvchi nomidan har qanday foydalanishni uning o'zi shadowli bo'lmaguncha yoki doirasi tugaguncha oladi.
Biz bir xil oʻzgaruvchining nomidan foydalanib, `let` kalit soʻzidan foydalanishni quyidagi tarzda takrorlash orqali oʻzgaruvchini shadow qilishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu dastur avval `x` ni `5` qiymatiga bog'laydi. Keyin u `let x =` ni takrorlab, asl qiymatni olib, `1` qo'shish orqali yangi `x` o'zgaruvchisini yaratadi, shunda `x` qiymati `6` bo'ladi. Keyin, jingalak qavslar bilan yaratilgan ichki doirada uchinchi `let` iborasi ham `x` ga shadow qiladi va yangi o'zgaruvchini yaratadi va oldingi qiymatni `2` ga ko'paytirib, `x` ga `12` qiymatini beradi.
Bu doira tugagach, ichki shadow tugaydi va `x` `6` ga qaytadi.
Ushbu dasturni ishga tushirganimizda, u quyidagilarni chiqaradi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Shadowing o‘zgaruvchini `mut` deb belgilashdan farq qiladi, chunki `let` kalit so‘zidan foydalanmasdan tasodifan ushbu o‘zgaruvchiga qayta tayinlashga harakat qilsak, kompilyatsiya vaqtida xatolikka yo‘l qo‘yamiz. `let` dan foydalanib, biz qiymat bo'yicha bir nechta o'zgarishlarni amalga oshirishimiz mumkin, lekin bu o'zgarishlar tugagandan so'ng o'zgaruvchi o'zgarmas bo'lishi mumkin.

`Mut` va shadow o'rtasidagi boshqa farq shundaki, biz `let` kalit so'zini qayta ishlatganimizda yangi o'zgaruvchini samarali yaratayotganimiz sababli, qiymat turini o`zgartirishimiz mumkin, lekin bir xil nomni qayta ishlatishimiz ham mumkin. Misol uchun, bizning dasturimiz foydalanuvchidan bo'sh joy belgilarini kiritish orqali ba'zi matnlar orasida qancha bo'sh joy bo'lishini ko'rsatishni so'raydi va biz ushbu kiritishni raqam sifatida saqlamoqchimiz:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

Birinchi `joylar` o'zgaruvchisi satr turi, ikkinchi `joylar` o'zgaruvchisi esa raqam turi. Shadowing shu tariqa bizni turli nomlar bilan chiqishdan saqlaydi, masalan, `joylar_str` va `joylar_num`; Buning o'rniga biz oddiyroq `joylar` nomini qayta ishlatishimiz mumkin. Biroq, bu erda ko'rsatilganidek, buning uchun `mut` dan foydalanmoqchi bo'lsak, kompilyatsiya vaqtida xatoga duch kelamiz:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Xato bizga o'zgaruvchining turini mutatsiyaga o'tkazishga ruxsat yo'qligini aytadi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Endi biz o'zgaruvchilar qanday ishlashini o'rganib chiqdik, keling, ular bo'lishi mumkin bo'lgan ko'proq ma'lumotlar turlarini ko'rib chiqaylik.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
