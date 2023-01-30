## O'rnatish

Birinchi qadam Rustni o'rnatishdir.Rustni Rust versiyalari va tegishli vositalarni boshqarish uchun buyruq qatori vositasi bo‘lgan `rustup` orqali yuklab olamiz. Yuklab olish uchun sizga internet ulanishi kerak bo'ladi.

> Eslatma: Agar biron sababga ko'ra `rustup` dan foydalanmaslikni xohlasangiz, boshqa variantlar uchun
> [Rustni o'rnatishning boshqa usullari][otherinstall] sahifasiga qarang.

Quyidagi qadamlar Rust kompilyatorining so'nggi barqaror versiyasini o'rnatadi.
Rustning barqarorligi kafolati kitobdagi kompilyatsiya qilingan barcha misollar Rustning yangi versiyalari bilan kompilyatsiya qilishda davom etishini ta'minlaydi. Chiqish versiyalar orasida biroz farq qilishi mumkin, chunki Rust ko'pincha xato xabarlari va ogohlantirishlarni yaxshilaydi. Boshqacha qilib aytadigan bo'lsak, ushbu qadamlar yordamida o'rnatgan har qanday yangi, barqaror Rust versiyasi ushbu kitob mazmuni bilan kutilganidek ishlashi kerak.

> ### Buyruqlar qatori yozuvi
>
> Ushbu bobda va butun kitobda biz terminalda ishlatiladigan ba'zi buyruqlarni ko'rsatamiz.
> Terminalga kiritishingiz kerak bo'lgan barcha qatorlar `$` bilan boshlanadi.
> `$` belgisini kiritishingiz shart emas; bu har bir buyruqning boshlanishini ko'rsatish
> uchun ko'rsatilgan buyruq qatori. `$` bilan boshlanmagan qatorlar odatda oldingi buyruqning
> natijasini ko'rsatadi. Bundan tashqari, PowerShell-ga xos misollarda `$` emas, `>` ishlatiladi.

### Linux yoki macOS-ga  `rustup` o'rnatish

Agar siz Linux yoki macOS dan foydalansangiz, terminalni oching va quyidagi buyruqni kiriting:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Buyruq skriptni yuklab oladi va Rustning eng so'nggi barqaror versiyasini o'rnatadigan `rustup` vositasini o'rnatishni boshlaydi. Sizdan parol so'ralishi mumkin. O'rnatish muvaffaqiyatli bo'lsa, quyidagi qator paydo bo'ladi:

```text
Rust is installed now. Great!
```

Shuningdek, sizga  *linker*, kerak bo'ladi, ya'ni Rust o'zining kompilyatsiya qilingan natijalarini bitta faylga birlashtirish uchun foydalanadigan dastur. Ehtimol,bu sizda allaqachon mavjud. Agar linker xatolarga duch kelsangiz, odatda linkerni o'z ichiga olgan C kompilyatorini o'rnatishingiz kerak. C kompilyatori ham foydalidir, chunki ba'zi umumiy Rust paketlari C kodiga bog'liq va C kompilyatoriga muhtoj bo'ladi.

MacOS-da siz C kompilyatorini ishga tushirish orqali olishingiz mumkin:

```console
$ xcode-select --install
```

Linux foydalanuvchilari odatda distributiv texnik hujjatlariga muvofiq GCC yoki Clang o'rnatishlari kerak. Misol uchun, agar siz Ubuntu'dan foydalansangiz, `build-essential` paketini o'rnatishingiz mumkin.

### Windows-ga `rustup` o'rnatish

Windows tizimida [https://www.rust-lang.org/tools/install][install] saytiga o'ting va Rustni o'rnatish bo'yicha ko'rsatmalarga amal qiling. O'rnatishning bir nuqtasida sizga Visual Studio 2013 yoki undan keyingi versiyalari uchun MSVC yaratish vositalari kerakligi haqida xabar keladi.

Build toolsini olish uchun [Visual Studio 2022][visualstudio] ni o'rnatishingiz kerak bo'ladi. Qaysi ish dasturlarini o'rnatish kerakligi so'ralganda, quyidagilarni  kiriting:

* “Desktop Development with C++”
* TWindows 10 yoki 11 SDK
* Ingliz tili to'plami komponenti va siz tanlagan boshqa tillar to'plami

Ushbu kitobning qolgan qismi *cmd.exe* va PowerShell da ishlaydigan buyruqlardan foydalanadi.
Agar aniq farqlar bo'lsa, qaysi birini ishlatishni tushuntiramiz.

### Muammolarni bartaraf etish

Rust to'g'ri o'rnatilganligini tekshirish uchun shellni oching va quyidagi qatorni kiriting:

```console
$ rustc --version
```

Quyidagi formatda chiqarilgan so‘nggi barqaror versiya uchun versiya raqami, xesh va tasdiqlangan sanani ko‘rishingiz kerak:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Agar siz ushbu ma'lumotni ko'rsangiz, Rustni muvaffaqiyatli o'rnatdingiz! Agar siz ushbu ma'lumotni ko'rmasangiz, Rust `%PATH%` tizim o'zgaruvchingizda quyidagi tarzda ekanligini tekshiring.

Windows CMD-da quyidagilardan foydalaning:

```console
> echo %PATH%
```

PowerShell-da foydalaning:

```powershell
> echo $env:Path
```

Linux va macOS-da quyidagilardan foydalaning:

```console
$ echo $PATH
```

Agar hammasi to'g'ri bo'lsa va Rust hali ham ishlamasa, yordam olishingiz mumkin bo'lgan bir qancha joylar mavjud. Boshqa Rustaceanlar (biz o'zimizni chaqiradigan ahmoqona taxallus) bilan qanday bog'lanishni [hamjamiyat sahifasida][community] bilib oling.

### Yangilash va o'chirish

Rust `rustup` orqali o'rnatilgandan so'ng, yangi chiqarilgan versiyaga yangilash oson. Shelldan quyidagi yangilash skriptini ishga tushiring:

```console
$ rustup update
```

Rust va  `rustup`-ni o'chirish uchun shelldan quyidagi o'chirish skriptini ishga tushiring:

```console
$ rustup self uninstall
```

### Mahalliy texnik hujjatlar

Rust-ning o'rnatilishi texnik hujjatlarning mahalliy nusxasini ham o'z ichiga oladi, shunda siz uni oflayn rejimda o'qishingiz mumkin. Brauzeringizda mahalliy texnik hujjatlarni ochish uchun `rustup doc` dasturini ishga tushiring.

Istalgan vaqtda standart kutubxona tomonidan tur yoki funksiya taqdim etilsa va siz u nima qilishini yoki undan qanday foydalanishni bilmasangiz, bilish uchun amaliy dasturlash interfeysi (API) texnik hujjatlaridan foydalaning!

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/downloads/
[community]: https://www.rust-lang.org/community
