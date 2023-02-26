# Umumiy to'plamlar

Rustning standart kutubxonasi *kolleksiyalar* deb nomlangan juda foydali ma'lumotlar tuzilmalarini o'z ichiga oladi. Ko'pgina boshqa ma'lumotlar turlari bitta ma'lum qiymatni ifodalaydi, lekin to'plamlar bir nechta qiymatlarni o'z ichiga olishi mumkin. O'rnatilgan array va tuple turlaridan farqli o'laroq, ushbu to'plamlar ko'rsatadigan ma'lumotlar heapda saqlanadi, ya'ni ma'lumotlar miqdori kompilyatsiya vaqtida ma'lum bo'lishi shart emas va dastur ishga tushganda o'sishi yoki qisqarishi mumkin. To'plamning har bir turi o'z imkoniyatlariga ega va ishlash jihatidan farq qiladi, shuning uchun ma'lum bir to'plamni tanlash vaziyatga bog'liq va vaqt o'tishi bilan ishlab chiquvchining mahoratidir. Ushbu bobda biz Rust dasturlarida tez-tez ishlatiladigan uchta to'plamni muhokama qilamiz:

* *vector* o'zgaruvchan sonli qiymatlarni bir-birining yonida saqlashga imkon beradi.
* *string* - bu belgilar to'plami. Biz `String` turini avval aytib o'tgan edik, ammo bu bobda biz bu haqda chuqurroq gaplashamiz.
* *hash map* ma'lum bir kalit bilan qiymatni bog'lash imkonini beradi.Bu *map* deb nomlangan umumiy ma'lumotlar strukturasining o'ziga xos tatbiqidir.

Standart kutubxona tomonidan taqdim etilgan boshqa turdagi to'plamlar haqida bilish uchun [texnik hujjatlarga][collections] qarang.

Biz vectorlarni, stringlarni va hash-maplarni qanday yaratish va yangilashni, shuningdek, har birining o'ziga xosligini muhokama qilamiz.

[collections]: ../std/collections/index.html
