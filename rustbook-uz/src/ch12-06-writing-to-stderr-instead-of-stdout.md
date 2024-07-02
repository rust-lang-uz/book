## Xatolik Xabarlarini Standart Error'ga, Standart Output O'rniga Yozish

Hozirda, biz barcha chiqishlarni `println!` macro. orqali terminalga yozmoqdamiz.
Ko'pgina terminallarda ikkita turdagi chiqish mavjud: *standard
output* (`stdout`) umumiy ma'lumot uchun va *standard error* (`stderr`)  xato xabarlar uchun. Bu farq foydalanuvchilarga dasturning muvaffaqiyatli
natijalarini faylga yozish, lekin xatolik xabarlarini ekranga chiqarish imkonini beradi.

The `println!` faqat standart output'ga chop etish qobiliyatiga ega, shuning uchun standart error'ga chop etish uchun boshqa biror narsadan
foydalanishimiz kerak.

### Xatolar Qayerda Yozilganligini Tekshirish

Avvalo, `minigrep` tomonidan chop etilgan kontentning hozirda standart output'ga yozilishini, shuningdek, standart error'ga yozishimiz kerak
bo'lgan xatolik xabarlarini ko'rib chiqaylik. Buni standart output oqimini faylga yo'naltirib va qasddan xatolik yuzaga keltirib amalga oshiramiz.
Standart error oqimini yo'naltirmaymiz, shuning uchun standart error'ga yuborilgan har qanday kontent ekranda ko'rsatilishda davom etadi.

Buyruq satri dasturlari xatolik xabarlarini standart error oqimiga yuborishi
kutilyapti, shunda standart output oqimini faylga yo'naltirgan bo'lsak ham, xatolik
xabarlarini ekranda ko'rishimiz mumkin. Bizning dasturimiz hozirda yaxshi
ishlamayapti: xatolik xabarlarini faylga saqlashini ko'rib chiqamiz!

Ushbu xatti-harakatni namoyish etish uchun, dasturini `>` va standart output oqimini
yo'naltirmoqchi bo'lgan fayl yo'li, masalan, `output.txt`, bilan ishga tushiramiz.
Hech qanday argumentlar bermaymiz, bu xatolik yuzaga kelishiga olib kelishi kerak:

```console
$ cargo run > output.txt
```

`>` sintaksisi shell'ga standart output'ning mazmunini ekranga emas, balki
`output.txt` faylga yozishni bildiradi. Ekranda kutgan xatolik xabarini ko'rmadik,
demak, u faylda bo'lishi kerak. Bu `output.txt` faylida mavjud bo'lgan ma'lumot:

```text
Problem parsing arguments: not enough arguments
```

Ha, xatolik xabarimiz standart output'ga chop etilmoqda. Xatolik xabarlarining bunday
holatda standart error'ga chop etilishi ancha foydaliroq, shunda faqat muvaffaqiyatli
ishga tushirilgan ma'lumotlar faylda saqlanadi. Biz buni o'zgartiramiz.

### Xatoliklarni Standart Error'ga Chop Etish

Xatolik xabarlarini chop etish usulini o'zgartirish uchun Listing 12-24'dagi kodni
ishlatamiz. Bu bobda avvalgi refaktoring tufayli, xatolik xabarlarini chop etuvchi
barcha kod bir funktsiyada, ya'ni `main`da joylashgan. Standart kutubxona `eprintln!`
makrosini taqdim etadi, bu standart error oqimiga chop etadi, shuning uchun
`println!`ni xatoliklarni chop etish uchun ishlatilgan ikki joyni `eprintln!` bilan 
o'zgartiramiz.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

<span class="caption">Listing 12-24: Xatolik xabarlarini standart output o'rniga
standart error'ga `eprintln!` yordamida yozish!</span>

Endi dasturini avvalgi kabi, hech qanday argument bermay va standart output'ni `>`
bilan yo'naltirib ishga tushiramiz:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Endi xatolik ekranda ko'rinadi va *output.txt* hech narsa o'z ichiga olmaydi, bu
buyruq satri dasturlaridan kutayotgan xatti-harakatdir.

Dasturini yana xatolik yuzaga keltirmaydigan, lekin standart output'ni faylga
yo'naltiradigan argumentlar bilan ishga tushiramiz, quyidagicha:

```console
$ cargo run -- to poem.txt > output.txt
```

Terminalda hech qanday chiqishni ko'rmaymiz va *output.txt* natijalarimizni o'z ichiga
oladi:

<span class="filename">Filename: output.txt</span>

```text
Siz ham hech kim emasmisiz?
Qanday zerikarli bo'lish kimdir!
```

Bu muvaffaqiyatli chiqish uchun standart output va xatolik chiqishi uchun standart
error'ni to'g'ri ishlatayotganimizni ko'rsatadi.

## Xulosa

Bu bob siz hozirgacha o'rgangan ba'zi asosiy tushunchalarni qayta ko'rib chiqdi va 
Rust'da umumiy I/O operations qanday bajarishni ko'rsatdi. Buyruq satri argumentlari,
fayllar, atrof-muhit o'zgaruvchilari va xatoliklarni chop etish uchun `eprintln!`
makrosidan foydalanib, siz hozirda buyruq satri ilovalarini yozishga tayyorsiz. Oldingi
boblardagi tushunchalar bilan birga, sizning kodingiz yaxshi tashkil etilgan,
ma'lumotlarni tegishli ma'lumot tuzilmalarida samarali saqlaydi, xatoliklarni yaxshi
boshqaradi va yaxshi sinovdan o'tgan bo'ladi. 

Keyinchalik, funktsional tillar ta'sirida bo'lgan ba'zi Rust xususiyatlarini
o'rganamiz: closures va iterators.
