## Xato xabarlarini standart chiqish oqimi o'rniga xato oqimiga yozish

Ayni paytda biz `println!` funksiyasidan foydalangan holda terminalga barcha 
chiqishlarimizni yozib olamiz. Ko'pgina terminallarda ikkita chiqish turi 
mavjud: umumiy ma'lumot uchun standart chiqish oqimi ( `stdout` ) 
va xato xabarlari uchun standart xato oqimi (`stderr`).
Ushbu farq foydalanuvchilarga dasturning muvaffaqiyatli chiqishini faylga yo'naltirishni 
tanlashga imkon beradi, ammo shu bilan birga xato 
xabarlarini ekranga chiqaradi.

`println!` funktsiyasi (makrosi) faqat standart chiqishda chop etish mumkin,
shuning uchun biz standart xatolar oqimiga chop etish uchun boshqa misollarda 
ko'rib chiqaylik.

### Xatolar yozilgan joyni tekshirish

Birinchidan, keling, `minigrep`-dan chop etilgan kontent hozirda standart chiqishga
qanday yozilishini, shu jumladan biz standart xato oqimiga yozmoqchi bo'lgan har qanday
xato xabarlarini ko'rib chiqaylik. Biz buni standart chiqish oqimini faylga yo'naltirish
va ataylab xatoga yo'l qo'yish orqali qilamiz. Biz standart xatolar oqimiga yo'naltirmaganimiz uchun,
standart xatolar oqimiga yuborilgan har qanday tarkib ekranda paydo bo'lishda davom etadi.

Buyruq qatorining (cmd) dasturlari xato xabarlarini standart xato oqimiga yuborishi kutilmoqda,
shuning uchun biz standart chiqish oqimini faylga yo'naltirsak ham, ekrandagi xato xabarlarini
ko'rishimiz mumkin. Bizning dasturimiz hozirda to'g'ri ishlamayapti:
biz xato xabari chiqishini faylga saqlayotganini ko'ramiz.

Ushbu xatti-harakatni namoyish qilish uchun biz dasturni `>` va *output.txt* fayl nomi
bilan ishga tushiramiz. Biz standart chiqish oqimini yo'naltirishni xohlaymiz.
Biz hech qanday dalil (argument) keltirmaymiz, bu xatoga olib kelishi kerak:

```console
$ cargo run > output.txt
```

`>` sintaksisi qobiqqa (shellga) *output.txt*-ga ekran o'rniga standart chiqish (standard output) tarkibini yozishni buyuradi.
Biz ekranda ko'rishni kutgan xato xabarini ko'rmadik, shuning uchun u faylda bo'lishi kerak.
Yuqorida keltirilgan *output.txt*-ning holati:

```text
Problem parsing arguments: not enough arguments
```

Ha, bizning xato xabarimiz standart chiqishda (standard outputga) ko'rsatiladi. 
Bunday xato xabarlarini standart xatolar oqimiga kiritish ancha foydali,
shuning uchun faqat muvaffaqiyatli ishga tushirish ma'lumotlari faylga kiradi.
Biz buni keyinchalik o'zgartiramiz.

### Xatolarni standard xato oqimiga chop etish (print qilish)

Xato xabarlarini chiqarish usulini o'zgartirish uchun biz 12-24-ro'yxatdagi koddan 
foydalanamiz. Ushbu bobda ilgari qilgan refaktoring tufayli xato xabarlarini chop
etadigan barcha kodlar bitta `main` funktsiyada joylashgan. Rust standart kutubxonasi `eprintln!` makrosini
taqdim etadi va bu makro standart xato oqimiga kiradi, shuning uchun `println!` bilan chaqirilgan joyda,
keling uning o'rniga `eprintln!` makrosini ishlatamiz.

<span class="filename">Fayl: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-24: `println!` yordamida Standard Output o'rniga Standard Error - da xato xabarlarini yozish!</span>

Keling, dasturni xuddi shu tarzda, hech qanday dalilsiz (argumentsiz) qayta ishga
tushiramiz va standart chiqishni (outputni) `>` bilan qayta yo'naltiramiz:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Endi biz ekranda xatoni ko'rishimiz mumkin va *output.txt* esa bo'sh.
Bu esa aynan buyruq qatoridan (cmd-dan) biz kutgan holat.

Keling, dasturni xatoga olib kelmaydigan argumentlar bilan qayta ishga tushiramiz,
ammo baribir standart chiqish faylga yo'naltiradi, misol:

```console
$ cargo run -- to poem.txt > output.txt
```

Biz terminalga hech qanday chiqishni ko'rmaymiz. Va *output.txt* esa
quydagi natijalarni o'z ichiga oladi:

<span class="filename">Fayl: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

Yani, biz vaziyatga qarab, muvaffaqiyatli chiqish (output) uchun 
standart chiqish oqimidan va xatolarni chiqarish uchun standart xato oqimidan foydalanamiz.

## Xulosa

Ushbu bobda siz hozirgacha o'rgangan ba'zi asosiy tushunchalar takrorlangan va 
Rustda muntazam I/O operatsiyalarini qanday bajarish kerakligi aytilgan.
Buyruq qatori argumentlari (command line argumentlari), fayllar,
atrof-muhit o'zgaruvchilari va `println!` makrosi yordamida xatolarni
ishlatgan holda, endi siz buyruq qatori (CLI) dasturlarini yozishga tayyormiz.
Oldingi boblardagi tushunchalar bilan birgalikda sizning kodingiz yaxshi tartibga solinadi,
ma'lumotlarni tegishli tuzilmalarda samarali saqlaydi, xatolarni yaxshi qayta ishlaydi va yaxshi
sinovdan o'tkaziladi.

Keyinchalik, biz funktsional tillar ta'sirida bo'lgan Rust-ning
ba'zi xususiyatlarini ko'rib chiqamiz: yopilishlar (closures) va iteratorlar.