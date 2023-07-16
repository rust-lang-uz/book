## Faylni o'qish

Endi biz  `fayl_yoli` argumentida koʻrsatilgan faylni oʻqish funksiyasini qoʻshamiz. Birinchidan, uni sinab ko'rish uchun bizga namuna fayli kerak: biz bir nechta takroriy so'zlar bilan bir nechta satrlarda kichik hajmdagi matnli fayldan foydalanamiz. 12-3 ro'yxatda Olma haqida she'r bor, u yaxshi ishlaydi! Loyihangizning root darajasida *olma.txt* nomli fayl yarating va “Olma” she’rini kiriting.

<span class="filename">Fayl nomi: olma.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/olma.txt}}
```

<span class="caption">Ro'yxat 12-3: Olma haqidagi she'r yaxshi sinov ishini yaratadi</span>

Matn joyida bo'lgan holda *src/main.rs* ni tahrirlang va 12-4 ro'yxatda ko'rsatilganidek, faylni o'qish uchun kod qo'shing.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-4: Ikkinchi argument tomonidan ko'rsatilgan fayl mazmunini o'qish</span>

Birinchidan, biz standart kutubxonaning tegishli qismini `use` statementi bilan keltiramiz: fayllar bilan ishlash uchun bizga `std::fs` kerak.

`main` da yangi `fs::read_to_string` statementi `fayl_yoli`ni oladi, bu faylni ochadi va fayl mazmunining `std::io::Result<String>` ni qaytaradi.

Shundan so'ng, fayl o'qilgandan keyin `tarkib` qiymatini chop etadigan vaqtinchalik `println!` statementini yana qo'shamiz, shuning uchun dasturning hozirgacha ishlayotganligini tekshirishimiz mumkin.

Keling, ushbu kodni birinchi buyruq qatori argumenti sifatida istalgan qator bilan ishga tushiramiz (chunki biz hali qidiruv qismini amalga oshirmaganmiz) va ikkinchi argument sifatida *olma.txt* fayli:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Ajoyib! Ushbu kod fayl mazmunini o'qiydi va fayl mazmunini chop etdi. Ammo kodda bir nechta kamchiliklar mavjud. Ayni paytda `main` funksiya bir nechta mas'uliyatga ega: umuman olganda, har bir funksiya faqat bitta vazifa uchun javobgar bo'lsa, funksiyalar aniqroq va ularni saqlash osonroq bo'ladi. Boshqa muammo shundaki, biz xatolarni imkon qadar yaxshi hal qilmayapmiz. Dastur hali ham kichik, shuning uchun bu kamchiliklar katta muammo emas, lekin dastur o'sib ulg'aygan sayin ularni toza tuzatish qiyinroq bo'ladi. Dasturni ishlab chiqishda refaktorlashni erta boshlash yaxshi amaliyotdir, chunki kichikroq hajmdagi kodlarni qayta ishlash ancha oson. Biz buni keyin qilamiz.
