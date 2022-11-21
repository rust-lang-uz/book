# Kirish

> Eslatma: Kitobning ushbu nashri [ Rust dasturlash 
> tili][nsprust] bosma va elektron kitob formatida mavjud [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

Rust haqidagi kirish kitobi *Rust Dasturlash Tili*ga xush kelibsiz. 
Rust dasturlash tili tezroq va ishonchli dasturlarni yozishga yordam beradi.
Yuqori darajadagi samaradorlik va low-leveldagi boshqaruv ko'pincha dasturlash tilini loyihalashda bir-biriga zid keladi; Rust bu ziddiyatga qarshi turadi. Kuchli texnik imkoniyatlar va ishlab chiquvchilarning ajoyib tajribasini muvozanatlash orqali Rust sizga an'anaviy ravishda bunday nazorat bilan bog'liq bo'lgan barcha qiyinchiliklarsiz low-leveldagi tafsilotlarni (masalan, xotiradan foydalanish) boshqarish imkoniyatini beradi.

## Rust kim uchun

Rust turli sabablarga ko'ra ko'p odamlar uchun idealdir. Keling, bir nechtasini ko'rib chiqaylik
eng muhim guruhlar.

### Dasturchilar jamoalari

Rust turli darajadagi tizimlarni dasturlash bo'yicha bilimga ega bo'lgan yirik ishlab chiquvchilar guruhlari o'rtasida hamkorlik qilish uchun samarali vosita ekanligini isbotlamoqda. Low-leveldagi kod turli xil nozik xatolarga moyil bo'lib, ko'pchilik boshqa tillarda ularni faqat keng ko'lamli sinov va tajribali ishlab chiquvchilar tomonidan sinchkovlik bilan tekshirish orqali aniqlash mumkin.Rust-da kompilyator ushbu qiyin xatolar, jumladan, parallellik xatolari bilan kodni kompilyatsiya qilishni rad etib, darvozabon rolini o'ynaydi. Kompilyator bilan birga ishlash orqali jamoa xatolarni ta'qib qilishdan ko'ra, vaqtini dastur mantig'iga qaratishga sarflashi mumkin.

Rust shuningdek, tizim dasturlash dunyosiga zamonaviy ishlab chiquvchilar vositalarini olib keladi:

* Cargo  dependency menejeri va qurish vositasini o'z ichiga oladi, Rust ekotizimida bog'liqliklarni qo'shish, kompilyatsiya qilish va boshqarishni qiyinchiliksiz va davomli qiladi.
* Rustfmt formatlash vositasi ishlab chiquvchilar orasida barqaror kodlash uslubini ta'minlaydi.
* Rust Language Server kodni toʻldirish va inline xato xabarlari uchun Integrated Development Environment (IDE) integratsiyasini quvvatlaydi.

Rust ekotizimidagi ushbu va boshqa vositalardan foydalangan holda, ishlab chiquvchilar tizim darajasidagi kodni yozishda samarali bo'lishi mumkin.

### Talabalar

Rust talabalar va tizim tushunchalarini o'rganishga qiziquvchilar uchun. Rust-dan foydalanib, ko'p odamlar operatsion tizimlarni ishlab chiqish kabi mavzular haqida bilib oldilar. Jamiyat juda mehmondo'st va talabalar savollariga javob berishdan xursand. Ushbu kitob kabi sa'y-harakatlar orqali Rust guruhlari tizim tushunchalarini ko'proq odamlar, ayniqsa dasturlash uchun yangi bo'lganlar uchun qulayroq qilishni xohlashadi.

### Kompaniyalar

Yuzlab yirik va kichik kompaniyalar ishlab chiqarishda Rust-dan buyruq qatori vositalari, veb-xizmatlar, DevOps asboblari, o'rnatilgan qurilmalar, audio va video tahlillari va transkodlar, kriptovalyutalar, bioinformatika, qidiruv tizimlari, Internet of Things ilovalari kabi turli vazifalar uchun foydalanadilar. , machine learning va hatto Firefox veb-brauzerining asosiy qismlari.

### Ochiq kodli dasturchilar

Rust Rust dasturlash tilini, hamjamiyatini, ishlab chiquvchilar vositalarini va kutubxonalarini yaratmoqchi bo'lgan odamlar uchundir. Rust tiliga o'z hissangizni qo'shishingizni istardik.

### Tezlik va barqarorlikni qadrlaydigan odamlar

Rust is for people who crave speed and stability in a language. By speed, we
mean both how quickly Rust code can run and the speed at which Rust lets you
write programs. The Rust compiler’s checks ensure stability through feature
additions and refactoring. This is in contrast to the brittle legacy code in
languages without these checks, which developers are often afraid to modify. By
striving for zero-cost abstractions, higher-level features that compile to
lower-level code as fast as code written manually, Rust endeavors to make safe
code be fast code as well.

The Rust language hopes to support many other users as well; those mentioned
here are merely some of the biggest stakeholders. Overall, Rust’s greatest
ambition is to eliminate the trade-offs that programmers have accepted for
decades by providing safety *and* productivity, speed *and* ergonomics. Give
Rust a try and see if its choices work for you.

## Who This Book Is For

This book assumes that you’ve written code in another programming language but
doesn’t make any assumptions about which one. We’ve tried to make the material
broadly accessible to those from a wide variety of programming backgrounds. We
don’t spend a lot of time talking about what programming *is* or how to think
about it. If you’re entirely new to programming, you would be better served by
reading a book that specifically provides an introduction to programming.

## How to Use This Book

In general, this book assumes that you’re reading it in sequence from front to
back. Later chapters build on concepts in earlier chapters, and earlier
chapters might not delve into details on a particular topic but will revisit
the topic in a later chapter.

You’ll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of Rust. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. Chapters 2, 12, and 20 are project chapters; the rest are concept chapters.

Chapter 1 explains how to install Rust, how to write a “Hello, world!” program,
and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a
hands-on introduction to writing a program in Rust, having you build up a
number guessing game. Here we cover concepts at a high level, and later
chapters will provide additional detail. If you want to get your hands dirty
right away, Chapter 2 is the place for that. Chapter 3 covers Rust features
that are similar to those of other programming languages, and in Chapter 4
you’ll learn about Rust’s ownership system. If you’re a particularly meticulous
learner who prefers to learn every detail before moving on to the next, you
might want to skip Chapter 2 and go straight to Chapter 3, returning to Chapter
2 when you’d like to work on a project applying the details you’ve learned.

Chapter 5 discusses structs and methods, and Chapter 6 covers enums, `match`
expressions, and the `if let` control flow construct. You’ll use structs and
enums to make custom types in Rust.

In Chapter 7, you’ll learn about Rust’s module system and about privacy rules
for organizing your code and its public Application Programming Interface
(API). Chapter 8 discusses some common collection data structures that the
standard library provides, such as vectors, strings, and hash maps. Chapter 9
explores Rust’s error-handling philosophy and techniques.

Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.

Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.

In Chapter 16, we’ll walk through different models of concurrent programming
and talk about how Rust helps you to program in multiple threads fearlessly.
Chapter 17 looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with.

Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.

In Chapter 20, we’ll complete a project in which we’ll implement a low-level
multithreaded web server!

Finally, some appendices contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions. In Appendix F, you can find
translations of the book, and in Appendix G we’ll cover how Rust is made and
what nightly Rust is.

There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.

<span id="ferris"></span>

An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:

| Ferris                                                                                                           | Meaning                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | This code panics!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | This code does not produce the desired behavior. |

In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.

## Source Code

The source files from which this book is generated can be found on
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
