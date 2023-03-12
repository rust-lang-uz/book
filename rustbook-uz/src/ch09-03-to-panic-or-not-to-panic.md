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

However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, having a check like this in every function would be
tedious (and might impact performance).

Instead, we can make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere. That
way, it’s safe for functions to use the new type in their signatures and
confidently use the values they receive. Listing 9-13 shows one way to define a
`Guess` type that will only create an instance of `Guess` if the `new` function
receives a value between 1 and 100.

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file requires the `rand` crate. We do want to include it for reader
experimentation purposes, but don't want to include it for rustdoc testing
purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-13/src/main.rs:here}}
```

<span class="caption">Listing 9-13: A `Guess` type that will only continue with
values between 1 and 100</span>

First, we define a struct named `Guess` that has a field named `value` that
holds an `i32`. This is where the number will be stored.

Then we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `i32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it’s between 1 and 100.
If `value` doesn’t pass this test, we make a `panic!` call, which will alert
the programmer who is writing the calling code that they have a bug they need
to fix, because creating a `Guess` with a `value` outside this range would
violate the contract that `Guess::new` is relying on. The conditions in which
`Guess::new` might panic should be discussed in its public-facing API
documentation; we’ll cover documentation conventions indicating the possibility
of a `panic!` in the API documentation that you create in Chapter 14. If
`value` does pass the test, we create a new `Guess` with its `value` field set
to the `value` parameter and return the `Guess`.

Next, we implement a method named `value` that borrows `self`, doesn’t have any
other parameters, and returns an `i32`. This kind of method is sometimes called
a *getter*, because its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It’s important that the `value` field be private so code
using the `Guess` struct is not allowed to set `value` directly: code outside
the module *must* use the `Guess::new` function to create an instance of
`Guess`, thereby ensuring there’s no way for a `Guess` to have a `value` that
hasn’t been checked by the conditions in the `Guess::new` function.

A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than an
`i32` and wouldn’t need to do any additional checks in its body.

## Summary

Rust’s error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can’t handle and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust’s type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.

Now that you’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we’ll talk about how generics work and how you
can use them in your code.

[encoding]: ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
