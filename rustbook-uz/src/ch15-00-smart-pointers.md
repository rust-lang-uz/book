# Smart Pointerlar

*Pointer* bu xotirada biror manzilni o'z ichiga olgan o'zgaruvchi uchun umumiy
tushuncha. Ushbu manzil biror boshqa ma'lumotga ishora qiladi. Rustda eng keng
tarqalgan pointer turi bu siz 4-bobda o'rgangan reference hisoblanadi.
Referencelar `&` belgisi bilan ko'rsatiladi va qaratilgan qiymatni qarzga oladi.
Ularning ma'lumotga murojaat qilishdan boshqa hech qanday maxsus qobiliyati
bo'lmaydi.

*Smart pointerlar* esa pointer kabi ishlaydigan, lekin qo'shimcha
metama'lumotlar va imkoniyatlarga ega ma'lumot tuzilmalaridir. Smart pointerlar
tushunchasi faqatgina Rustda yagona emas: smart pointerlar C++da paydo bo'lgan
va boshqa tillarda ham mavjud. Rust standart kutubxonasi taqdim etadigan turli
xil smart pointerlarga ega bo'lib, ular referencelardan tashqari funksionallikni
ta'minlaydi. Umumiy tushunchani o'rganish uchun biz smart pointerlarning bir
nechta turli misollarini ko'rib chiqamiz, jumladan *reference hisoblash* smart
pointer turi. Ushbu pointer sizga egalar sonini kuzatish orqali ma'lumotning bir
nechta egalari bo'lish imkonini beradi va egalari qolmaganda ma'lumotni
tozalaydi.

Rust o'zining egalik va qarz olish tushunchasi bilan referencelar va smart
pointerlar o'rtasida qo'shimcha farqga ega: referencelar faqat ma'lumotlarni
qarzga olsa, ko'p hollarda smart pointerlar ma'lumotga *egalik* qiladilar.

Garchi biz ularni o‘sha paytda shunday deb atamagan bo‘lsak-da, biz ushbu
kitobda bir nechta smart pointerlarga duch keldik, jumladan, 8-bobdagi `String`
va `Vec<T>`. Bu ikkala tur ham smart pointer hisoblanadi, chunki u biror
xotiraga egalik qiladi va sizga uni manipulyatsiya qilish imkonini beradi.
Shuningdek, ularda metama'lumot va qo'shimcha imkoniyatlar yoki kafolatlar
bo'ladi. Masalan, `String` o'z sig'imini metama'lumot sifatida saqlaydi va uning
ma'lumoti har doim UTF-8 bo'lishini ta'minlash uchun qo'shimcha xususiyatga ega.

Smart pointerlar odatda structlar yordamida amalga oshiriladi. Oddiy structdan
farqli o'laroq, smart pointerlar `Deref` va `Drop` traitlarini amalga oshiradi.
`Deref` traiti smart pointer struct-ning instancesiga reference kabi o'zini
tutish imkonini beradi, shunday qilib siz ikkala referencelar yoki smart
pointerlar bilan ishlash uchun kod yozishingiz mumkin. `Drop` traiti smart
pointerning instancesi scopedan chiqib ketganda ishga tushadigan kodni
moslashtirish imkonini beradi. Ushbu bobda biz ikkala traitni muhokama qilamiz
va nima uchun ular smart pointerlar uchun muhimligini ko'rsatamiz.

Smart pointer patterni Rustda tez-tez ishlatiladigan umumiy dizayn patterni
ekanligini hisobga olsak, bu bobda mavjud bo'lgan barcha smart pointerlar qamrab
olinmaydi. Ko'pgina kutubxonalarda o'zlarining smart pointerlari mavjud va siz
hatto o'zingiznikini yozishingiz mumkin. Biz standart kutubxonadagi eng keng
tarqalgan smart pointerlarni ko'rib chiqamiz:

- `Box<T>` heapga qiymatlarni joylashtirish uchun
- `Rc<T>`, bir nechta egalik qilish imkonini beruvchi referencelar hisoblash
  turi
- `Ref<T>` va `RefMut<T>`, `RefCell<T>` orqali kiriladi, bu qarz olish
  qoidalariga kompilyatsiya vaqti o'rniga runtimeda rioya qilishga majburlovchi
  tur

Bundan tashqari, biz *ichki o'zgaruvchanlik* patternini ko'rib chiqamiz, bunda
o'zgarmas tur ichki qiymatni o'zgartirish uchun APIni ochib beradi. Shuningdek,
biz *reference davrlarini* muhokama qilamiz: ular xotirani oqishiga olib kelishi
va ularni qanday oldini olish mumkin.

Keling sho'ng'iymiz!
