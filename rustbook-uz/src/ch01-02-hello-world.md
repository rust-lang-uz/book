## Hello, World!

Endi siz Rustni o'rnatdingiz, hozir sizning birinchi Rust dasturingizni yozishning ayni vaqti.
Yangi dasturlash tilini o'rganishda `Hello, World!` matnini ekranga chop etuvchi kichik va sodda
dastur tuzish an'anaga aylangan, shunday ekan biz ham sinab ko'ramiz!

> Eslatma: Bu kitob terminal bilan ishlay olishning boshlang'ich ko'nikmalarini
> talab qiladi. Rust sizning kod muxarriringiz foydalanadigan asboblaringiz va
> kodingizni qayerda joylayishi bo'yicha talablar qo'ymaydi, shuning uchun agar siz
> terminal o'rniga integratsiyalashgan ishlab chiqish muhitidan (IDE) foydalanishni afzal ko'rsangiz,
> o'zingizning sevimli IDE-dan foydalaning. Ko'pgina IDElar endi ma'lum darajada
> Rust-ni qo'llab-quvvatlaydi; tafsilotlar uchun IDE hujjatlarini tekshiring.
> Rust jamoasi `rust-analyzer` orqali ajoyib IDE yordamini ta'minlashga e'tibor qaratdi.
> Batafsil ma’lumot uchun [D ilovasi][devtools]<!-- ignore -->ni ko'zdan kechiring.

### Loyiha jildini yaratish

Siz ishni Rust kodingizni joylaytirish uchun jild yaratishdan boshlaysiz.
Rust uchun sizning kodingiz qayerda joylashining ahamiyati yo'q, lekin biz
bu kitobdagi mashq va loyihalarni joylash uchun *projects* nomli jild yaratishingizni
maslahat beramiz.

Terminalni oching va *projects* jildini yaratish va uning ichidan “Hello, world!” loyihasi
jildini yaratish uchun quyidagi buyruqlarni kiriting.

Linux, macOS va Windows Powershell uchun:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD uchun:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Rust dasturi yozish va ishga tushirish.

Endi, *main.rs* nomli yangi fayl yarating. Rust kodlar har doim *.rs* kengaytmasi
bilan tugaydi. Agar fayl nomida bir nechta so'zlardan foydalansangiz, ularni ajratish uchun pastki chiziqdan foydalanish shart. Masalan, *helloworld.rs* o'rniga *hello_world.rs* dan foydalaning.

Endi hozirgina yaratgan *main.rs* faylingizni kod muharririda oching.

<span class="filename">Fayl nomi: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<span class="caption">Listing 1-1: `Hello, world!` ni chop etuvchi dastur</span>

Faylni saqlang va Terminalda *~/projects/hello_world* jildiga qayting.
Linux yoki macOS da faylni kompilyatsiya qilish va ishga tushirish uchun quyidagi buyruqlarni kiriting:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Windowsda `./main` ning o'rniga `.\main.exe` buyrug'ini kiriting:

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```
Operatsion tizimingizdan qat'i nazar, terminalda `Hello, world!` qatori chop etilishi kerak.Agar siz ushbu chiqishni ko'rmasangiz, yordam olish usullari uchun O'rnatish bo'limining [”Muammolarni bartaraf etish”][troubleshooting]<!-- ignore --> bo'limiga qayting.

Agar `Hello, world!` chop etilgan bo'lsa, tabriklaymiz! Siz rasmiy ravishda Rust dasturini yozdingiz. Bu sizni Rust dasturchisiga aylantiradi - xush kelibsiz!

### Rust dasturining tuzilishi.

Keling "Salom, Olam!" dasturiga chuqurroq nazar solamiz. Boshqotirmaning 1-qismi:

```rust
fn main() {

}
```

Bu qatorlar `main` nomli funksiyani e'lon qiladi. `main` funksiyasi alohida: u har doim bajariladigan Rust dasturida ishlaydigan birinchi koddir. Bu yerda birinchi satr hech qanday parametrga ega boʻlmagan va hech narsani qaytarmaydigan `main` funksiyasini eʼlon qiladi.
Agar parametrlar mavjud bo'lsa, ular `()` qavslar ichiga kiradi.

Funksiyasing tanasi `{}` bilan o'ralgan. Rust har bir funksiyalarda e'lon qilishda
`{}` dan foydalanishni talab qiladi.

> Eslatma: Agar siz Rust loyihalarda standart usulda kod yozmoqchi bo'lsangiz
> kodingizni maʼlum bir uslubda formatlash uchun `rustfmt` nomli avtomatik formatlash vositasidan
> foydalanishingiz mumkin (batafsilroq `rustfmt` [D ilovasi][devtools]<!-- ignore --> -da)
> Rust jamoasi ushbu vositani standart Rust distributiviga kiritdi,
> chunki `rustc` kabi, u allaqachon kompyuteringizga o'rnatilgan bo'lishi kerak!

`main` funksiyaning tanasi quyidagi kodni saqlaydi:

```rust
    println!("Hello, world!");
```

Shu bir qator kod shu kichik dasturdagi barcha ishni amalga oshiardi: u
matnni ekranga chop etadi.Bu yerda ahamiyat qaratish zarur bo'lgan
to'rtta muhim narsalar bor.

Birinchidan, Rust stili 4ta bo'sh joydan iborat 1ta tabdan emas.

Ikkinchidan, `println!` Rust macro deb ataladi. Agar u funksiya deb atalganda
`println` (`!`siz) tarzda kiritilardi. Biz Rust makrolari haqida 19-bobda batafsil
gaplashamiz. Siz hozir bilishingiz kerak bo'lgan narsa, `!`dan foydalanish 
funksiya deb emas, makro deb ataladi.

Uchinchidan, siz "Salom, Olam!" matnini ko'rdingiz. Biz `println!`ga argument
sifatida matn berdik va matn ekranga chop etildi.

To'rtinchidan, biz qatorni semikolon(`;`) bilan tugatdik, u hozirgi jarayon tugab,
keyingi jarayonning boshlanishini anglatadi. Rust kodlarining katta qismi semikolon
bilan tugaydi.


### Kompilyatsiya qilish va ishga tushirish alohida bosqichlar hisoblanadi.

Siz hozirgina yangi dastur tuzdingiz, keling jarayonning har bir qadamini
ko'rib chiqamiz.

Rust dasturini ishga tushirishdan oldin, uni `rustc` buyrug'i orqali Rust kompilyatodan
foydalangan holda dastur nomini kiritib kompilyatsiya qilishingiz zarur. Namuna:

```console
$ rustc main.rs
```

Agar siz C yoki C++ bilan ishlagan bo'lsangiz, bu `gcc` yoki `clang` bilan bir
xil ekanligini payqaysiz. Muvoffaqiyatli kompilyatsiyadan so'ng, Rust bina bajaruvchi
faylni yaratadi.

Linux, macOS va Windows PowerShellda, siz bajariluvchi faylni `ls` buyrug'i
orqali ko'rishingiz mumkin:


```console
$ ls
main  main.rs
```

Linux va macOSda, siz 2ta fayllarni ko'rasiz. Windows Powershhellda siz
Windows CMD yordamida ko'ra oladigan 3ta faylni ko'rasiz.


```cmd
> dir /B %= the /B faqat fayl nomlarini ko'rish uchun =%
main.exe
main.pdb
main.rs
```

Bu sizga *.rs* kengaytmali kod faylini, bajariluvchi faylni(Windowsda `main.exe`
boshqa barcha tizimlarda `main`), va Windowsdan foydalanayotganingizda, debugging 
ma'lumotlarini o'z ichida saqlovchi *.pdb* kengaytmali faylni ko'rsatadi.

Endi, siz *main* yoki *main.exe*ni quyidagicha ishga tushira olasiz:

```console
$ ./main # yoki Windowsda ./main.exe
```

Agar sizning *main.rs* faylingiz "Salom, Olam!" dasturi bo'lsa, bu dastur
ekranga "Salom, Olam!" matnini chop etadi.

Agar siz Ruby, Python yoki JavaScript kabi dinamik tillar bilan tanish bo'lsangiz,
siz dasturni kompilyatsiya qilish va bir necha qadamlarga bo'lish orqali ishga
tushirmagan bo'lishingiz mumkin. Rust *yozish davomida kompilyatsiya qilinga* til,
ya'ni siz Rust kodni yozib uni kompilyatsiya qilib, ishga tushiriluvchi faylni boshqa
odamga jo'natsangiz, u kompyuterida Rust o'rnatilmagan holda dasturni ishga tushrish
imkoniyatiga ega bo'ladi. Agar siz kimgadir *.rb*, *.py* yoki *.js* faylini uzatsangiz
ular dasturni ishga tushirishi uchun Ruby, Python yoki JavaScript ishga tushiruvchilari
o'rnatishlari zarur. Lekin bu tillarda dasturni kompilyatsiya qilish va ishga tushirish 
uchun birgina buyruq yetarli bo'ladi.

Kichikroq dasturlarni `rustc` orqali kompilyatsiya qilish yaxshi, lekin sizni loyihangiz 
kengaygani sari, siz ishga tushirish jarayonini osonlashtirishni xohlaysiz.
Endi, biz siz bilan haqiqiy Rust dasturlarini tuzishda qulaylik yaratuvchi
Cargo yordamchisi bilan tanishamiz.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.md
