## `panic!` yoki `panic!` qo'ymaslik

Xo'sh, qachon `panic!` deb murojat qilish va qachon `Result`ni qaytarish kerakligini qanday hal qilasiz? Kodda panic paydo bo'lganda, uni tiklashning iloji yo'q. Qayta tiklashning mumkin bo'lgan yo'li bormi yoki yo'qmi, har qanday xatolik uchun `panic!` deb chaiqruv qilishingiz mumkin, lekin siz chaqiruv kodi nomidan vaziyatni tuzatib bo'lmaydi degan qarorga kelasiz. `Result` qiymatini qaytarishni tanlaganingizda, siz chaqiruv kodini tanlash imkoniyatini berasiz. Chaqiruv kodi vaziyatga mos keladigan tarzda tiklashga urinishi mumkin yoki `Err`dagi xatoni qayta tiklab bo'lmaydi, deb qaror qilishi va `panic!` qo'yishi mumkin, bu sizning tiklanadigan xatongizni tuzatib bo'lmaydiganga aylantiradi. Shuning uchun, muvaffaqiyatsiz bo'lishi mumkin bo'lgan funksiyani belgilashda `Result` ni qaytarish yaxshi standart tanlovdir.

Misollar, prototip kodi va testlar kabi holatlarda `Result`ni qaytarish o'rniga panic qo'yadigan kodni yozish maqsadga muvofiqdir. Keling, nima uchun ekanligini ko'rib chiqaylik, keyin kompilyator muvaffaqiyatsizlik mumkin emasligini ayta olmaydigan vaziyatlarni muhokama qilaylik, lekin siz inson sifatida buni qila olasiz. Bob kutubxona kodida panic qo'yish yoki yo'qligini hal qilish bo'yicha ba'zi umumiy ko'rsatmalar bilan yakunlanadi.

### Misollar, Prototip Kodi va Testlar

Ba'zi bir kontseptsiyani tasvirlash uchun misol yozayotganingizda, shuningdek, xatolarni qayta ishlash kodini o'z ichiga olgan holda, misolni kamroq tushunarli qilish mumkin. Misollarda, panic qo'zg'atishi mumkin bo'lgan `unwrap` kabi metodga murojaat qilish sizning ilovangiz xatoliklarni qanday hal qilishini xohlayotganingiz uchun to'ldiruvchi sifatida tushuniladi, bu sizning kodingizning qolgan qismi nima qilayotganiga qarab farq qilishi mumkin.

Xuddi shunday, prototiplashda xatolarni qanday hal qilishni hal qilishdan oldin `unwrap` va `expect` metodllari juda qulaydir.
Dasturingizni yanada mustahkamroq qilishga tayyor bo'lganingizda ular kodingizda aniq belgilar qoldiradilar.

Agar testda metod chaqiruvi muvaffaqiyatsiz bo'lsa, bu metod sinovdan o'tkazilayotgan funksiya bo'lmasa ham, butun test muvaffaqiyatsiz bo'lishini xohlaysiz. Chunki `panic!` – bu sinovning muvaffaqiyatsiz deb belgilanishi, `unwrap` yoki `expect` deb atalgan narsa aynan shunday bo'lishi kerak.

### Siz kompilyatordan ko'ra ko'proq ma'lumotga ega bo'lgan holatlar

Agar sizda `Result` `Ok` qiymatiga ega bo'lishini ta'minlaydigan boshqa mantiqqa ega bo'lsangiz, `unwrap` yoki `expect` ni chaqirish ham o'rinli bo'lardi, ammo mantiq kompilyator tushunadigan narsa emas. Siz hali ham `Result` qiymatiga ega bo'lasiz, uni hal qilishingiz kerak: siz murojaat qilayotgan har qanday operatsiya sizning vaziyatingizda mantiqan imkonsiz bo'lsa ham, umuman muvaffaqiyatsiz bo'lish ehtimoli bor. Agar siz kodni qo‘lda tekshirish orqali sizda hech qachon `Err` varianti bo‘lmasligiga ishonch hosil qilsangiz, `unwrap` deb nomlash juda maqbuldir, va `expect` matnida hech qachon `Err` varianti bo'lmaydi deb o'ylagan sababni hujjatlash yaxshiroqdir. Mana bir misol:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Qattiq kodlangan stringni tahlil qilish orqali `IpAddr` misolini yaratmoqdamiz. Biz `127.0.0.1` to‘g‘ri IP manzil ekanligini ko‘ramiz, shuning uchun bu yerda `expect` dan foydalanish mumkin. Biroq, qattiq kodlangan, yaroqli satrga ega bo'lish `parse` metodining qaytish turini o'zgartirmaydi: biz hali ham `Result` qiymatini olamiz va kompilyator bizni `Result` bilan ishlashga majbur qiladi, go‘yo `Err` varianti mumkin, chunki kompilyator bu satr har doim haqiqiy IP manzil ekanligini ko‘rish uchun yetarlicha aqlli emas. Agar IP-manzillar qatori dasturga qattiq kodlanganidan ko'ra foydalanuvchidan kelgan bo'lsa va shuning uchun muvaffaqiyatsizlikka uchragan bo'lsa, biz, albatta, `Result` ni yanada ishonchli tarzda boshqarishni xohlaymiz.
Ushbu IP-manzil qattiq kodlangan degan taxminni eslatib o'tsak, agar kelajakda IP-manzilni boshqa manbadan olishimiz kerak bo'lsa, bizni `expect` ni xatolarni boshqarish kodini yaxshiroq o'zgartirishga undaydi.

### Xatolarni bartaraf etish bo'yicha ko'rsatmalar

Agar kodingiz yomon holatda bo'lishi mumkin bo'lsa, kodingiz panic qo'yishi tavsiya etiladi. Shu nuqtai nazardan, *yomon holat* deganda baʼzi taxminlar(assumption), kafolatlar(guarantee), shartnomalar(contract,) yoki oʻzgarmasliklar buzilganda, masalan, notoʻgʻri qiymatlar, qarama-qarshi qiymatlar yoki yetishmayotgan qiymatlar kodingizga oʻtkazilganda, shuningdek quyidagilardan biri yoki bir nechtasi:

* Yomon holat - foydalanuvchi noto'g'ri formatda ma'lumotlarni kiritishi kabi vaqti-vaqti bilan sodir bo'lishi mumkin bo'lgan narsadan farqli o'laroq, kutilmagan narsa.
* Ushbu nuqtadan keyin sizning kodingiz har qadamda muammoni tekshirishdan ko'ra, bu yomon holatda bo'lmaslikka tayanishi kerak.
* Siz foydalanadigan turlarda ushbu ma'lumotni kodlashning yaxshi usuli yo'q. Biz 17-bobning ["Turlar sifatida kodlash holatlari va behaviorlari"][encoding]<!-- ignore --> bo'limida nimani nazarda tutayotganimizni misol qilib ko'rib chiqamiz.

Agar kimdir sizning kodingizga chaqiruv qilsa va mantiqiy bo'lmagan qiymatlarni o'tkazsa, kutubxona foydalanuvchisi bu holatda nima qilishni xohlashini hal qilishi uchun xatolikni qaytarish yaxshidir. Biroq, davom etish xavfli yoki zararli bo'lishi mumkin bo'lgan hollarda, eng yaxshi tanlov `panic!` deb chaqiruv qilish va kutubxonangizdan foydalanuvchini kodidagi xatolik haqida ogohlantirish bo'lishi mumkin, shunda ular ishlab chiqish jarayonida uni tuzatishi mumkin. Xuddi shunday, `panic!`ko'pincha sizning nazoratingizdan tashqarida bo'lgan tashqi kodga chaqiruv qilsangiz va uni tuzatishning imkoni bo'lmagan yaroqsiz holatni qaytarsangiz mos keladi.

Biroq, muvaffaqiyatsizlik kutilganda, `panic!` chaqiruv qilishdan ko'ra, `Result`ni qaytarish maqsadga muvofiqdir. Misollar, tahlilchiga noto'g'ri tuzilgan ma'lumotlar yoki tarif chegarasiga yetganingizni bildiruvchi holatni qaytaruvchi HTTP so'rovini o'z ichiga oladi. Bunday hollarda, `Result` ni qaytarish, chaqiruv kodi qanday ishlov berishni hal qilishi kerak bo'lgan muvaffaqiyatsizlik kutilgan imkoniyat ekanligini ko'rsatadi.

Agar kodingiz noto'g'ri qiymatlar yordamida chaqirilgan bo'lsa, foydalanuvchini xavf ostiga qo'yishi mumkin bo'lgan operatsiyani bajarganda, kodingiz avval qiymatlarning haqiqiyligini tekshirishi va qiymatlar noto'g'ri bo'lsa panic qo'yishi kerak.Bu asosan xavfsizlik nuqtai nazaridan: noto'g'ri ma'lumotlar bilan ishlashga urinish kodingizni zaifliklarga olib kelishi mumkin.
Agar siz chegaradan tashqari xotiraga kirishga harakat qilsangiz, standart kutubxona `panic!` deb chaqirishining asosiy sababi shu: joriy ma'lumotlar tuzilishiga tegishli bo'lmagan xotiraga kirishga urinish umumiy xavfsizlik muammosidir. Funksiyalarda ko'pincha *shartnomalar(contracts)* mavjud: agar kirish ma'lum talablarga javob bersa, ularning xatti-harakati kafolatlanadi. Shartnoma buzilganda panic qo'yish mantiqan to'g'ri keladi, chunki shartnoma buzilishi har doim chaqiruv qiluvchi tomonidagi xatolikni ko'rsatadi va bu siz chaqiruv kodini aniq ko'rib chiqishni xohlagan xatolik emas. Aslida, chaqiruv kodini tiklashning oqilona usuli yo'q; kodni chaqiruvchi *dasturchilar* kodni tuzatishi kerak. Funksiya uchun shartnomalar, ayniqsa buzilish panic keltirib chiqaradigan bo'lsa, funksiya uchun API texnik hujjatlarida tushuntirilishi kerak.

Biroq, barcha funksiyalaringizda ko'plab xatolarni tekshirish batafsil va zerikarli bo'ladi. Yaxshiyamki, siz Rustning turdagi tizimidan (va shunday qilib, kompilyator tomonidan amalga oshiriladigan turdagi tekshirish) siz uchun ko'plab tekshiruvlarni amalga oshiradi. Agar funksiyangiz parametr sifatida ma'lum bir turga ega bo'lsa, kompilyator sizda haqiqiy qiymatga ega ekanligiga ishonch hosil qilgan holda kodingiz mantig'ini davom ettirishingiz mumkin. Misol uchun, agar sizda `Option` emas, balki turingiz bo'lsa, dasturingiz *nothing(hech narsa)* emas, balki *something(nimadir)* bo'lishini kutadi. Sizning kodingiz `Some` va `None` variantlari uchun ikkita holatni ko'rib chiqishi shart emas: aniq qiymatga ega bo'lish uchun faqat bitta holat bo'ladi. Funksiyangizga hech narsa o'tkazmaslikka harakat qiladigan kodni kompilyatsiya qilinmaydi, shuning uchun funksiyangiz runtimeda bu holatni tekshirishi shart emas.
Yana bir misol, parametr hech qachon manfiy bo'lmasligini ta'minlaydigan `u32` kabi belgisiz butun son turidan foydalanishdir.

### Tasdiqlash uchun maxsus turlarni yaratish

Keling, bir qadam oldin haqiqiy qiymatga ega ekanligimizga ishonch hosil qilish uchun Rust turi tizimidan foydalanish g'oyasini olaylik va tekshirish uchun maxsus turni yaratishni ko'rib chiqaylik. 2-bobdagi taxmin qilish o'yinini eslang, unda bizning kodimiz foydalanuvchidan 1 dan 100 gacha bo'lgan raqamni taxmin qilishni so'radi. Biz hech qachon foydalanuvchining taxmini o'sha raqamlar o'rtasida ekanligini tasdiqlaganimiz yo'q, uni bizning maxfiy raqamimizga nisbatan tekshirishdan oldin; biz faqat taxmin ijobiy ekanligini tasdiqladik. Bunday holda, natijalar unchalik dahshatli emas edi: bizning "Raqam katta!" yoki "Raqam Kichik!" chiqishimiz hali ham to'g'ri bo'lar edi. Lekin foydalanuvchini to'g'ri taxmin qilishga va foydalanuvchi diapazondan tashqaridagi raqamni taklif qilganda va foydalanuvchi, masalan, raqamlar o'rniga harflarni kiritganda, boshqacha xatti-harakatlarga ega bo'lishga undash yaxshi bo'lardi.

Buning usullaridan biri potentsial manfiy raqamlarga ruxsat berish uchun taxminni faqat `u32` o‘rniga `i32` sifatida tahlil qilish va keyin diapazondagi raqamni tekshirishni qo‘shishdir, masalan:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

`If` ifodasi bizning qiymatimiz diapazondan tashqarida yoki yo‘qligini tekshiradi, foydalanuvchiga muammo haqida xabar beradi va siklning keyingi iteratsiyasini boshlash uchun `continue` ni chaqiradi va yana bir taxminni so‘raydi. `if` ifodasidan keyin `taxmin` 1 dan 100 gacha ekanligini bilgan holda `taxmin` va maxfiy raqam o‘rtasidagi taqqoslashni davom ettirishimiz mumkin.

Biroq, bu ideal echim emas: agar dastur faqat 1 dan 100 gacha bo'lgan qiymatlarda ishlaganligi juda muhim bo'lsa va bu talab bilan ko'plab funksiyalarga ega bo'lsa, har bir funksiyada bunday tekshiruvga ega bo'lish zerikarli bo'ladi (va ishlashga ta'sir qilishi mumkin).

Buning o'rniga, biz yangi turni yaratishimiz va tekshirishlarni hamma joyda takrorlashdan ko'ra, turdagi namunani yaratish uchun funksiyaga qo'yishimiz mumkin. Shunday qilib, funksiyalar o'zlarining imzolarida yangi turdan foydalanishlari va ular olgan qiymatlardan ishonchli foydalanishlari xavfsiz bo'ladi. 9-13 roʻyxatda `Taxmin` turini aniqlashning bir usuli koʻrsatilgan, bu `new` funksiya 1 dan 100 gacha boʻlgan qiymatni qabul qilsagina `Taxmin` misolini yaratadi.

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file requires the `rand` crate. We do want to include it for reader
experimentation purposes, but don't want to include it for rustdoc testing
purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-13/src/main.rs:here}}
```

<span class="caption">Roʻyxat 9-13: `Taxmin` turi, u faqat 1 dan 100 gacha qiymatlar bilan davom etadi</span>

Birinchidan, biz `i32` ga ega `qiymat` nomli maydonga ega `Taxmin` nomli structni aniqlaymiz. Bu yerda raqam saqlanadi.

Keyin biz `Taxmin` da `new` nomli bog'langan funktsiyani amalga oshiramiz, u `Taxmin` qiymatlari misollarini yaratadi. `new` funksiya `i32` turidagi `qiymat` nomli bitta parametrga ega bo‘lishi va `Taxmin`ni qaytarishi uchun belgilangan. `new` funksiyaning asosiy qismidagi kod `qiymat`ni 1 dan 100 gacha ekanligiga ishonch hosil qilish uchun tekshiradi.
Agar `qiymat` bu sinovdan o‘tmasa, biz `panic!` chaqiruvini qilamiz, bu chaqiruv kodini yozayotgan dasturchini tuzatishi kerak bo‘lgan xatolik haqida ogohlantiradi, chunki bu diapazondan tashqarida `qiymat` bilan `Taxmin` yaratish `Taxmin::new` tayanadigan qoidani buzadi. `Taxmin::new` panic qo'zg'atishi mumkin bo'lgan shartlar uning API texnik hujjatlarida muhokama qilinishi kerak; biz 14-bobda yaratgan API texnik hujjatlarida `panic!` ehtimolini ko‘rsatuvchi hujjatlar konventsiyalarini qamrab olamiz. Agar `qiymat` testdan o'tgan bo'lsa, biz uning `qiymat` maydoni `qiymat` parametriga o'rnatilgan yangi `Taxmin` yaratamiz va `Taxmin`ni qaytaramiz.

Keyinchalik, biz `self` ni oladigan, boshqa parametrlarga ega bo'lmagan va `i32` ni qaytaradigan `qiymat` nomli metodni qo'llaymiz. Bunday usul ba'zan *getter(oluvchi)* deb ataladi, chunki uning maqsadi o'z maydonlaridan ba'zi ma'lumotlarni olish va uni qaytarishdir. Ushbu umumiy metod zarur, chunki `Taxmin` strukturasining `qiymat` maydoni shaxsiydir(private). `qiymat` maydoni shaxsiy(private) bo'lishi juda muhim, shuning uchun `Taxmin` strukturasi yordamida kod to'g'ridan-to'g'ri `qiymat` ni o'rnatishga ruxsat berilmaydi: moduldan tashqaridagi kod `Taxmin::new` funksiyasidan `Taxmin` misolini yaratish uchun foydalanishi kerak, shunday qilib, `Taxmin` ning `Taxmin::new` funksiyasidagi shartlar bo‘yicha tekshirilmagan `qiymat`ga ega bo‘lishining imkoni yo‘qligini ta’minlaydi.

Parametrga ega bo'lgan yoki faqat 1 dan 100 gacha bo'lgan raqamlarni qaytaradigan funksiya o'z imzosida `i32` emas, `Taxmin` ni olishi yoki qaytarishi va uning tanasida qo'shimcha tekshiruvlar o'tkazishga hojat qolmasligini e'lon qilishi mumkin.

## Xulosa

Rust-ning xatolarni boshqarish xususiyatlari sizga yanada mustahkam kod yozishga yordam berish uchun mo'ljallangan.
`panic!` makrosi dasturingiz u bardosh bera olmaydigan holatda ekanligini bildiradi va noto‘g‘ri yoki noto‘g‘ri qiymatlar bilan davom etish o‘rniga jarayonni to‘xtatishni aytish imkonini beradi. `Result` enumi operatsiyalar muvaffaqiyatsiz bo'lishi va kodingiz tiklanishi mumkinligini bildirish uchun Rust turdagi tizimdan foydalanadi. Kodingizga chaqiruv qiladigan kod potentsial muvaffaqiyat yoki muvaffaqiyatsizlikni hal qilishi kerakligini aytish uchun `Result` dan foydalanishingiz mumkin. Tegishli vaziyatlarda `panic!` va `Result` dan foydalanish muqarrar muammolar oldida kodingizni yanada ishonchli qiladi.

Endi siz standart kutubxonada `Option` va `Result` enumlari bilan generiklardan foydalanishning foydali usullarini ko'rganingizdan so'ng, biz generiklar qanday ishlashi va ularni kodingizda qanday ishlatishingiz haqida gaplashamiz.

[encoding]: ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
