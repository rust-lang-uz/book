## Salom Olam!

Endi siz Rustni o'rnatdingiz, hozir sizning birinchi Rust dasturingizni yozishning ayni vaqti.
Yangi dasturlash tilini o'rganishda "Salom, Olam!" matnini ekranga chop etuvchi kichik va sodda
dastur tuzish an'anaga aylangan, shunday ekan biz ham sinab ko'ramiz!

> Eslatma: Bu kitob terminal bilan ishlay olishning boshlang'ich ko'nikmalarini
> talab qiladi. Rust sizning kod muxarriringiz foydalanadigan asboblaringiz va
> kodingizni qayerda joylayishi bo'yicha talablar qo'ymaydi, shuning uchun siz
> terminal o'rniga o'zingiz xohlagan kod muxarririni ishlatishingiz mumkin.
> Hozir ko'pchilik kod muxarrirlarida Rust kodini yozish ancha osson. Rust dasturlash
> tili jamoasi kod yozishni ossonlashtirish uchun `rust-analyzer` ni ishlab chiqishgan.
> Qo'shimcha ma'lumot uchun [Appendix D][devtools]<!-- ignore -->ni ko'zdan kechiring.

### Loyiha jildini yaratish

Siz ishni Rust kodingizni joylaytirish uchun jild yaratishdan boshlaysiz.
Rust uchun sizning kodingiz qayerda joylashining ahamiyati yo'q, lekin biz
bu kitobdagi mashq va loyihalarni joylash uchun *projects* nomli jild yaratishingizni
maslahat beramiz.

Terminalni oching va *projects* jildini yaratish va uning ichidan "Salom Olam!" loyihasi
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
bilan tugaydi. Agar siz faylni nomlashda 1dan ortiq so'z ishlatmoqchi bo'lsangiz,
ularni tag chiqiz orqali bog'lashingiz mumkin. Misol uchun: salom_olam.rs

Endi hozirgina yaratgan *main.rs* faylingizni kod muharririda oching.

<span class="filename">Fayl nomi: main.rs</span>

```rust
fn main() {
    println!("Salom, Olam!");
}
```

<span class="caption">Dastur "Salom, Olam!" matnini ekranga chop etadi.</span>

Faylni saqlang va Terminalda *~/projects/hello_world* jildiga qayting.
Linux yoki macOS'da, quyidagi buyruq orqali kompilyatsiya qiling va isha tushiring:

```console
$ rustc main.rs
$ ./main
Salom, Olam!
```

Windowsda `./main` ning o'rniga `.\main.exe` buyrug'ini kiriting:

```powershell
> rustc main.rs
> .\main.exe
Salom, Olam!
```
Siz foydalanyotgan operatsion tizimdan qat'iy nazar, Terminalga "Salom, Olam!" matni
chop etilishi kerak. Agar sizda natija boshqacha bo'lsa, [“Troubleshooting”][troubleshooting]<!-- ignore --> qismini qaytadan ko'rib chiqing.

Agar "Salom, Olam!" chop etilgan bo'lsa, tabriklayman. Siz ramsan Rust dasturi
yozdingiz. Bu sizni Rust dasturchiga aylantiradi, xush kelibsiz!

### Rust dasturining tuzilishi.

Keling "Salom, Olam!" dasturiga chuqurroq nazar solamiz. Boshqotirmaning 1-qismi:

```rust
fn main() {

}
```

Bu qatorlar `main` nomli funksiyani e'lon qiladi. `main` bajariluvchi(executable)
Rust dasturi ishga tushganda ishlaydigan 1-koddir. Birinichi qatorda dastur hech
qanday argumentlarsiz `main` funksiyasini e'lon qiladi va bu u hech narsa qaytarmaydi.
Agar bizga argumentlar kerak bo'lsa, biz ularni qavslar `()` ichiga yozamiz.

Funksiyasing tanasi `{}` bilan o'ralgan. Rust har bir funksiyalarda e'lon qilishda
`{}` dan foydalanishni talab qiladi.

> Eslatma: Agar siz Rust loyihalarda standart usulda kod yozmoqchi bo'lsangiz
> avtomatik tarzda kod formatlab beruvchi `rustfmt` dan foydalanishingiz mumkin
> Note: If you want to stick to a standard style across Rust projects, you can
> [Appendix D][devtools]<!-- ignore -->).Rust dasturlash tili jamoasi
> standar holatda `rustfmt` ni `rustc`dek o'rnatiluvchi tarzda e'lon qilishgan.
> shunday ekan u sizni qurilmangizda allaqachon o'rnatilgan bo'lishi kerak.

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
