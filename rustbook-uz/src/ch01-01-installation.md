## O'rnatish

The first step is to install Rust. We’ll download Rust through `rustup`, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.

> Eslatma: Agar biron sababga ko'ra `rustup` dan foydalanmaslikni xohlasangiz, boshqa variantlar uchun [Rustni o'rnatishning boshqa usullari][otherinstall] sahifasiga qarang.

The following steps install the latest stable version of the Rust compiler. Rust’s stability guarantees ensure that all the examples in the book that compile will continue to compile with newer Rust versions. The output might differ slightly between versions because Rust often improves error messages and warnings. In other words, any newer, stable version of Rust you install using these steps should work as expected with the content of this book.

> ### Buyruqlar qatori yozuvi
> 
> In this chapter and throughout the book, we’ll show some commands used in the terminal. Lines that you should enter in a terminal all start with `$`. You don’t need to type the `$` character; it’s the command line prompt shown to indicate the start of each command. Lines that don’t start with `$` typically show the output of the previous command. Additionally, PowerShell-specific examples will use `>` rather than `$`.

### Linux yoki macOS-ga  `rustup` o'rnatish

Agar siz Linux yoki macOS dan foydalansangiz, terminalni oching va quyidagi buyruqni kiriting:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup` tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```text
Rust is installed now. Great!
```

You will also need a *linker*, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

MacOS-da siz C kompilyatorini ishga tushirish orqali olishingiz mumkin:

```console
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the `build-essential` package.

### Windows-ga `rustup` o'rnatish

On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later.

To acquire the build tools, you’ll need to install [Visual Studio 2022][visualstudio]. When asked which workloads to install, include:

* “Desktop Development with C++”
* TWindows 10 yoki 11 SDK
* Ingliz tili to'plami komponenti va siz tanlagan boshqa tillar to'plami

The rest of this book uses commands that work in both *cmd.exe* and PowerShell. If there are specific differences, we’ll explain which to use.

### Muammolarni bartaraf etish

Rust to'g'ri o'rnatilganligini tekshirish uchun shellni oching va quyidagi qatorni kiriting:

```console
$ rustc --version
```

Quyidagi formatda chiqarilgan so‘nggi barqaror versiya uchun versiya raqami, xesh va tasdiqlangan sanani ko‘rishingiz kerak:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t see this information, check that Rust is in your `%PATH%` system variable as follows.

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

If that’s all correct and Rust still isn’t working, there are a number of places you can get help. Find out how to get in touch with other Rustaceans (a silly nickname we call ourselves) on [the community page][community].

### Yangilash va o'chirish

Once Rust is installed via `rustup`, updating to a newly released version is easy. From your shell, run the following update script:

```console
$ rustup update
```

Rust va  `rustup`-ni o'chirish uchun shelldan quyidagi o'chirish skriptini ishga tushiring:

```console
$ rustup self uninstall
```

### Mahalliy texnik hujjatlar

The installation of Rust also includes a local copy of the documentation so that you can read it offline. Run `rustup doc` to open the local documentation in your browser.

Istalgan vaqtda standart kutubxona tomonidan tur yoki funksiya taqdim etilsa va siz u nima qilishini yoki undan qanday foydalanishni bilmasangiz, bilish uchun amaliy dasturlash interfeysi (API) texnik hujjatlaridan foydalaning!

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/downloads/
[community]: https://www.rust-lang.org/community
