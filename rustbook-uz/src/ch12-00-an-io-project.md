# I/O loyihasi: Buyruqlar qatori dasturini yaratish(command line)

Ushbu bob siz hozirgacha o'rgangan ko'plab ko'nikmalarning qisqacha mazmuni va yana bir nechta standart kutubxona xususiyatlarining o'rganilishidir. Biz hozirda mavjud bo'lgan Rust tushunchalarini mashq qilish uchun fayl va buyruq qatori kiritish/chiqarish(input/output) bilan o'zaro ta'sir qiluvchi buyruq qatori vositasini(command line tool) yaratamiz.

Rust-ning tezligi, xavfsizligi, bitta ikkilik chiqishi(single binary output) va platformalararo9cross-platform qo'llab-quvvatlashi uni buyruqlar qatori vositalarini(command line tools) yaratish uchun ideal tilga aylantiradi, shuning uchun loyihamiz uchun biz klassik buyruq qatori qidiruv vositasi `grep` ning o'z versiyasini yaratamiz (**g**lobally search a **r**egular **e**xpression and **p**rint) qidirish. Foydalanishning eng oddiy holatida `grep` belgilangan faylni belgilangan qator uchun qidiradi. Buning uchun `grep` o'z argumenti sifatida fayl yo'li(file path) va satrni oladi. Keyin u faylni o'qiydi, o'sha faylda string argumentini o'z ichiga olgan qatorlarni topadi va bu satrlarni chop(print qiladi) etadi.

Yo'l davomida biz buyruq qatori vositasini boshqa ko'plab buyruq qatori vositalari ishlatadigan terminal xususiyatlaridan qanday foydalanishni ko'rsatamiz. Biz foydalanuvchiga vositamizning harakatini sozlash imkonini berish uchun atrof-muhit o'zgaruvchisining qiymatini(environment variable) o'qiymiz.
Shuningdek, biz xato xabarlarini standart chiqish (`stdout`) o'rniga standart xato konsoli oqimiga (`stderr`) chop qilamiz, shuning uchun, masalan, foydalanuvchi ekranda xato xabarlarini ko'rayotganda muvaffaqiyatli chiqishni faylga yo'naltirishi mumkin.

Rust hamjamiyatining bir a'zosi Andrew Gallant allaqachon `grep` ning `ripgrep` deb nomlangan to'liq xususiyatli, juda tez versiyasini yaratgan. Taqqoslash uchun, bizning versiyamiz ancha sodda bo'ladi, ammo bu bob sizga `ripgrep` kabi real loyihani tushunish uchun zarur bo'lgan asosiy bilimlarni beradi.

Bizning `grep` loyihamiz siz hozirgacha o'rgangan bir qator tushunchalarni birlashtiradi:

* Kodni tashkil qilish ([7-bobda][ch7]<!--
  ignore --> modullar haqida bilib olganlaringizdan foydalangan holda)
* Vectorlar va stringlardan foydalanish (to'plamlar(collection), [8-bob][ch8]<!-- ignore -->)
* Xatolarni qayta ishlash(handling error) ([9-bob][ch9]<!-- ignore -->)
* Kerakli hollarda traitlar va lifetimelardan foydalanish ([10-bob][ch10]<!-- ignore
  -->)
* Testlar yozish ([11-bob][ch11]<!-- ignore -->)

Shuningdek, biz [13][ch13]<!-- ignore --> va [17][ch17]<!-- ignore -->-boblarda batafsil yoritilgan closurelar, iteratorlar va trait obyektlarini qisqacha tanishtiramiz.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html
