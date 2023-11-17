## Ishlash samaradorligini(Performance) solishtirish: Looplar va iteratorlar

Looplar yoki iteratorlardan foydalanishni aniqlash uchun siz qaysi implement qilish tezroq ekanligini bilishingiz kerak: `qidiruv` funksiyasining aniq `for` lop sikliga ega versiyasi yoki iteratorli versiya.

Ser Arthur Conan Doylening *Sherlok Xolmsning sarguzashtlari(The Adventures of Sherlock Holmes)* asarining to‘liq mazmunini `String`ga yuklash va mazmundan *the* so‘zini izlash orqali sinovdan o‘tkizdik. Mana, `for` loop siklidan foydalangan holda `qidiruv` versiyasi va iteratorlardan foydalangan holda sinov natijalari:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

Iterator versiyasi biroz tezroq edi! Biz bu yerda benchmark kodini tushuntirmaymiz, chunki bu ikkita versiyaning ekvivalentligini isbotlash emas, balki ushbu ikki dasturning ishlash jihatidan qanday taqqoslanishi haqida umumiy tushunchaga ega bo'lishdir.

Batafsilroq tahlil qilish uchun siz `tarkib` sifatida har xil oʻlchamdagi turli matnlarni, `sorov` sifatida turli uzunlikdagi turli soʻz va soʻzlarni va boshqa har xil oʻzgarishlardan foydalanishni tekshirishingiz kerak. Gap shundaki: iteratorlar, garchi yuqori darajadagi(high-level) abstraksiya bo'lsa ham, xuddi quyi darajadagi(lower-level) kodni o'zingiz yozganingizdek, taxminan bir xil kodga kompilyatsiya qilinadi. Iteratorlar Rustning *zero-cost(nol xarajatli) abstraksiyalaridan* biri bo‘lib, bu orqali biz abstraktsiyadan foydalanish qo‘shimcha runtime yukini talab qilmaydi. Bu C++ tilining asl dizayneri va amalga oshiruvchisi(implementori) Bjarne Stroustrupning “C++ asoslari (Foundations of C++)” (2012) asarida *zero-overhead* belgilashiga o‘xshaydi:

> Umuman olganda, C++ ilovalari zero overhead(nol qo'shimcha xarajatlar) printsipiga bo'ysunadi:
> Siz foydalanmayotgan narsangiz uchun pul to'lamaysiz. Va yana: Siz foydalanadigan narsadan yaxshiroq
> kodlash mumkin emas.

Yana bir misol sifatida, quyidagi kod audio dekoderdan olingan. Dekodlash algoritmi oldingi namunalarning chiziqli(linear) funksiyasi asosida kelajakdagi qiymatlarni baholash uchun linear prediction(taxmin qilish) qilish matematik operatsiyasidan foydalanadi. Ushbu kod uchta o'zgaruvchi bo'yicha matematikani amalga oshirish uchun iterator zanjiridan foydalanadi: ma'lumotlarning `bufer` qismi, 12 `koeffitsient` massivi va `qlp_shift` da ma'lumotlarni o'zgartirish uchun miqdor. Biz ushbu misolda o'zgaruvchilarni e'lon qildik, lekin ularga hech qanday qiymat bermadik; Garchi bu kod o'z kontekstidan tashqarida unchalik katta ma'noga ega bo'lmasa-da, bu Rust yuqori darajadagi g'oyalarni low-leveldagi kodga qanday tarjima qilishining qisqacha, haqiqiy misolidir.

```rust,ignore
let buffer: &mut [i32];
let koeffitsient: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let taxmin_qilish = koeffitsient.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = taxmin_qilish as i32 + delta;
}
```

`taxmin_qilish` qiymatini hisoblash uchun ushbu kod `koeffitsient` dagi 12 ta qiymatning har birini iteratsiya(takrorlaydi) qiladi va koeffitsient qiymatlarini `bufer`dagi oldingi 12 ta qiymat bilan bog‘lash uchun `zip` metodidan foydalanadi. Keyin, har bir juftlik uchun biz qiymatlarni ko'paytiramiz, barcha natijalarni yig'amiz va `qlp_shift` bitlari yig'indisidagi bitlarni o'ngga siljitamiz.

Audio dekoderlar kabi ilovalardagi hisob-kitoblar ko'pincha performenceni(ish samaradorligi) birinchi o'ringa qo'yadi. Bu yerda biz iterator yaratamiz, ikkita adapterdan foydalanamiz va keyin qiymatni consume(iste'mol) qilamiz. Ushbu Rust kodi qaysi assembly kodini kompilyatsiya qiladi? Xo'sh, ushbu yozuvdan boshlab, u siz qo'lda yozadigan assemblyga kopilatsiya qiladi. `koeffitsient` dagi qiymatlar ustidagi iteratsiyaga mos keladigan loop sikl umuman yo‘q: Rust 12 ta iteratsiya(takrorlash) borligini biladi, shuning uchun u loop siklni `unrolls` qiladi. *Unrolling* - bu optimallashtirish bo'lib, u loop siklni boshqarish kodining qo'shimcha yukini olib tashlaydi va buning o'rniga loop siklning har bir iteratsiyasi uchun takroriy kod hosil qiladi.

Barcha koeffitsientlar registrlarda saqlanadi, ya'ni qiymatlarga kirish juda tez. Runtimeda massivga kirishda chegaralar yo'q.
Rust qo'llashi mumkin bo'lgan barcha optimallashtirishlar natijada olingan kodni juda samarali qiladi. Endi siz buni bilganingizdan so'ng, siz iteratorlar va closurelardan qo'rqmasdan foydalanishingiz mumkin! Ular kodni yuqori darajadagi(high-level) ko'rinishga olib keladi, lekin buning uchun runtime ishlashi uchun jazo qo'llamaydi.

## Xulosa

Closure va iteratorlar - bu funksional dasturlash tili g'oyalaridan ilhomlangan Rust xususiyatlari(feature). Ular Rustning low-leveldagi ishlashda high-leveldagi g'oyalarni aniq ifodalash qobiliyatiga hissa qo'shadilar. Closure va iteratorlarni implement qilish runtimening ishlashiga ta'sir qilmaydi. Bu Rustning zero-cost(nol xarajatli) abstraksiyalarni taqdim etishga intilish maqsadining bir qismidir.

Endi biz I/O loyihamizning ifodaliligini yaxshilaganimizdan so‘ng, keling, loyihani dunyo bilan baham ko‘rishimizga yordam beradigan `cargo`ning yana bir qancha xususiyatlarini ko‘rib chiqaylik.
