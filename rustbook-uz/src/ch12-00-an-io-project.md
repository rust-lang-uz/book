# I/O loyihasi: Buyruqlar qatori dasturini yaratish(command line)

Ushbu bob siz hozirgacha o'rgangan ko'plab ko'nikmalarning qisqacha mazmuni va yana bir nechta standart kutubxona xususiyatlarining o'rganilishidir. Biz hozirda mavjud bo'lgan Rust tushunchalarini mashq qilish uchun fayl va buyruq qatori kiritish/chiqarish(input/output) bilan o'zaro ta'sir qiluvchi buyruq qatori vositasini(command line tool) yaratamiz.

Rust-ning tezligi, xavfsizligi, bitta ikkilik chiqishi(single binary output) va platformalararo9cross-platform qo'llab-quvvatlashi uni buyruqlar qatori vositalarini(command line tools) yaratish uchun ideal tilga aylantiradi, shuning uchun loyihamiz uchun biz klassik buyruq qatori qidiruv vositasi `grep` ning o'z versiyasini yaratamiz (**g**lobally search a **r**egular **e**xpression and **p**rint) qidirish. Foydalanishning eng oddiy holatida `grep` belgilangan faylni belgilangan qator uchun qidiradi. Buning uchun `grep` o'z argumenti sifatida fayl yo'li(file path) va satrni oladi. Keyin u faylni o'qiydi, o'sha faylda string argumentini o'z ichiga olgan qatorlarni topadi va bu satrlarni chop(print qiladi) etadi.

Along the way, we’ll show how to make our command line tool use the terminal
features that many other command line tools use. We’ll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We’ll also print error messages to the standard error console stream (`stderr`)
instead of standard output (`stdout`), so, for example, the user can redirect
successful output to a file while still seeing error messages onscreen.

One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version will be fairly simple, but this chapter will give you some of the
background knowledge you need to understand a real-world project such as
`ripgrep`.

Our `grep` project will combine a number of concepts you’ve learned so far:

* Organizing code (using what you learned about modules in [Chapter 7][ch7]<!--
  ignore -->)
* Using vectors and strings (collections, [Chapter 8][ch8]<!-- ignore -->)
* Handling errors ([Chapter 9][ch9]<!-- ignore -->)
* Using traits and lifetimes where appropriate ([Chapter 10][ch10]<!-- ignore
  -->)
* Writing tests ([Chapter 11][ch11]<!-- ignore -->)

We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters [13][ch13]<!-- ignore --> and [17][ch17]<!-- ignore --> will cover in
detail.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html
