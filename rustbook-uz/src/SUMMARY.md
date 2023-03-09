# Rust Dasturlash Tili

[Rust dasturlash tili](title-page.md)
[Muqaddima](foreword.md)
[Kirish](ch00-00-introduction.md)

## Ishni boshlash

- [Ishni boshlash](ch01-00-getting-started.md)
    - [O'rnatish](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Taxmin qilish o'yinini dasturlash](ch02-00-guessing-game-tutorial.md)

- [Umumiy dasturlash tushunchalari](ch03-00-common-programming-concepts.md)
    - [O'zgaruvchilar va o'zgaruvchanlik](ch03-01-variables-and-mutability.md)
    - [Ma'lumotlar turlari](ch03-02-data-types.md)
    - [Funksiyalar](ch03-03-how-functions-work.md)
    - [Izohlar](ch03-04-comments.md)
    - [Control Flow](ch03-05-control-flow.md)

- [Ownershipni tushunish](ch04-00-understanding-ownership.md)
    - [Ownership nima?](ch04-01-what-is-ownership.md)
    - [Referencelar va  Borrowing](ch04-02-references-and-borrowing.md)
    - [Slice turi](ch04-03-slices.md)

- [Tegishli ma'lumotlarni tuzish uchun Structdan foydalanish](ch05-00-structs.md)
    - [Structlarni aniqlash va yaratish](ch05-01-defining-structs.md)
    - [Structs yordamida namunaviy dastur](ch05-02-example-structs.md)
    - [Metod sintaksisi](ch05-03-method-syntax.md)

- [Enumlar va patternlarni moslashtirish](ch06-00-enums.md)
    - [Enumni aniqlash](ch06-01-defining-an-enum.md)
    - [`match` Control Flow konstruksiyasi](ch06-02-match.md)
    - [`if let` bilan qisqacha Control Flow](ch06-03-if-let.md)

## Asosiy Rust savodxonligi

- [O'sib borayotgan loyihalarni Paketlar, Cratelar va Modullar bilan boshqarish](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [Paketlar va Cratelar](ch07-01-packages-and-crates.md)
    - [Qo'llanish doirasi va maxfiylikni nazorat qilish uchun modullarni aniqlash](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Modul daraxtidagi elementga murojaat qilish yo'llari](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [`use` kalit so'zi bilan yo'llarni qamrab olish](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Modullarni turli fayllarga ajratish](ch07-05-separating-modules-into-different-files.md)

- [Umumiy to'plamlar](ch08-00-common-collections.md)
    - [Vectorlar bilan qiymatlar ro'yxatini saqlash](ch08-01-vectors.md)
    - [UTF-8 kodlangan matnni Stringlar bilan saqlash](ch08-02-strings.md)
    - [Hash Mapda bog'langan qiymatlarga ega kalitlarni saqlash](ch08-03-hash-maps.md)

- [Xatolar bilan ishlash](ch09-00-error-handling.md)
    - [`panic!` bilan tuzatib bo'lmaydigan xatolar](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` bilan tiklanadigan xatolar](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` yoki `panic!` emas](ch09-03-to-panic-or-not-to-panic.md)

- [Umumiy turlar, traitlar va lifetime](ch10-00-generics.md)
    - [Generik ma'lumotlar turlari](ch10-01-syntax.md)
    - [Traitlar: umumiy xulq-atvorni aniqlash](ch10-02-traits.md)
    - [Referencelarni lifetime bilan tekshirish](ch10-03-lifetime-syntax.md)

- [Avtomatlashtirilgan testlarni yozish](ch11-00-testing.md)
    - [Testlarni qanday yozish kerak](ch11-01-writing-tests.md)
    - [Sinovlar qanday o'tkazilishini nazorat qilish](ch11-02-running-tests.md)
    - [Test tashkil etish](ch11-03-test-organization.md)

- [I/O loyihasi: Buyruqlar qatori dasturini yaratish](ch12-00-an-io-project.md)
    - [Buyruqlar qatori argumentini qabul qilishs](ch12-01-accepting-command-line-arguments.md)
    - [Faylni o'qish](ch12-02-reading-a-file.md)
    - [Modullikni yaxshilash va xatolarni qayta ishlash uchun refactoring](ch12-03-improving-error-handling-and-modularity.md)
    - [Sinovga asoslangan ishlab chiqish bilan kutubxonaning funksionalligini rivojlantirish](ch12-04-testing-the-librarys-functionality.md)
    - [Environment o'zgaruvchilari bilan ishlash](ch12-05-working-with-environment-variables.md)
    - [Xato xabarlarini standart chiqish o'rniga standart xatoga yozish](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Rustda o'ylash

- [Funksional til xususiyatlari: iteratorlar va closurelar](ch13-00-functional-features.md)
    - [Closurelar: Environmentni qamrab oladigan anonim funksiyalar](ch13-01-closures.md)
    - [Iteratorlar bilan bir qator elementlarga ishlov berish](ch13-02-iterators.md)
    - [I/O loyihamizni takomillashtirish](ch13-03-improving-our-io-project.md)
    - [Ishlash samaradorligini solishtirish: Looplar va iteratorlar](ch13-04-performance.md)

- [Cargo va Crates.io haqida ko'proq](ch14-00-more-about-cargo.md)
    - [Reliz profillari bilan structlarni moslashtirish](ch14-01-release-profiles.md)
    - [Crateni Crates.io-ga nashr qilish](ch14-02-publishing-to-crates-io.md)
    - [Cargo Workspacelar](ch14-03-cargo-workspaces.md)
    - [Binary fayllarni Crates.io'dan `cargo install` bilan o'rnatish](ch14-04-installing-binaries.md)
    - [Maxsus buyruqlar bilan Cargoni kengaytirish](ch14-05-extending-cargo.md)

- [Smart Pointerlar](ch15-00-smart-pointers.md)
    - [Heapdagi ma'lumotlarni ko'rsatish uchun `Box<T>` dan foydalanish](ch15-01-box.md)
    - [`Deref` Traitidan foydalangan holda oddiy referencelar kabi Smart Pointerlar bilan ishlash](ch15-02-deref.md)
    - [`Drop` Traiti bilan tozalashda kodni ishga tushirish](ch15-03-drop.md)
    - [`Rc<T>`, reference hisoblangan Smart Pointer](ch15-04-rc.md)
    - [`RefCell<T>` va ichki o'zgaruvchanlik namunasi](ch15-05-interior-mutability.md)
    - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Qo'rqmas parallellik](ch16-00-concurrency.md)
    - [Kodni bir vaqtning o'zida ishga tushirish uchun threadlardan foydalanish](ch16-01-threads.md)
    - [Threadlar orasidagi ma'lumotlarni uzatish uchun xabar uzatishdan(Message Passing) foydalanish](ch16-02-message-passing.md)
    - [Shared-State Concurrency](ch16-03-shared-state.md)
    - [`Sync` va `Send` traitlari bilan kengaytiriladigan parallellik](ch16-04-extensible-concurrency-sync-and-send.md)

- [Rustning ob'ektga yo'naltirilgan dasturlash xususiyatlari- OOP](ch17-00-oop.md)
    - [Ob'ektga yo'naltirilgan tillarning xususiyatlari](ch17-01-what-is-oo.md)
    - [Har xil turdagi qiymatlarga ruxsat beruvchi trait ob'ektlaridan foydalanish](ch17-02-trait-objects.md)
    - [Ob'ektga yo'naltirilgan dizayn patternini implement qilish](ch17-03-oo-design-patterns.md)

## Kengaytirilgan mavzular

- [Patternlar va Matching](ch18-00-patterns.md)
    - [All the Places Patterns Can Be Used](ch18-01-all-the-places-for-patterns.md)
    - [Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.md)
    - [Pattern Sintaksisi](ch18-03-pattern-syntax.md)

- [Kengaytirilgan Xususiyatlar](ch19-00-advanced-features.md)
    - [Xavfsiz Rust](ch19-01-unsafe-rust.md)
    - [Murakkab Traitlar](ch19-03-advanced-traits.md)
    - [Kengaytirilgan Turlar](ch19-04-advanced-types.md)
    - [Kengaytirilgan funksiyalar va closurelar](ch19-05-advanced-functions-and-closures.md)
    - [Makrolar](ch19-06-macros.md)

- [Yakuniy loyiha: Ko'p tarmoqli veb-serverni qurish](ch20-00-final-project-a-web-server.md)
    - [Yagona tarmoqli veb-serverni qurish](ch20-01-single-threaded.md)
    - [Bizning yagona tarmoqli serverimizni ko'p tarmoqli serverga aylantirish](ch20-02-multithreaded.md)
    - [Ajoyib o'chirish va tozalash](ch20-03-graceful-shutdown-and-cleanup.md)

- [Ilova](appendix-00.md)
    - [A - Kalit so'zlar](appendix-01-keywords.md)
    - [B - Operatorlar va Symbollar](appendix-02-operators.md)
    - [C - Hosilaviy Traitlar](appendix-03-derivable-traits.md)
    - [D - Foydali Development Toollar](appendix-04-useful-development-tools.md)
    - [E - Nashrlar](appendix-05-editions.md)
    - [F - Kitobning tarjimalari](appendix-06-translation.md)
    - [G - Rust qanday yasaladi va “Nightly Rust”](appendix-07-nightly-rust.md)
