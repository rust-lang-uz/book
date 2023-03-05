## `panic!` bilan tuzatib bo'lmaydigan xatolar

Ba'zida sizning kodingizda yomon narsalar sodir bo'ladi va siz bu haqda hech narsa qila olmaysiz. Bunday hollarda Rustda `panic!` makrosi mavjud. Amalda panic chaqirishning ikki yo'li mavjud: kodimizni panic chiqaradigan harakatni amalga oshirish (masalan, arrayning oxiridan o'tib kirish) yoki aniq `panic!` makrosini chaqirish.
Ikkala holatda ham biz dasturimizda panicni keltirib chiqaramiz. Odatiy bo'lib, bu panic muvaffaqiyatsizlik xabarini chop etadi, dasturni bajarish paytida ajratilgan resurslarni tozalaydi, stackni tozalaydi va ishdan chiqadi. Atrof-muhit o'zgaruvchisi orqali siz panic paydo bo'lganda panic manbasini kuzatishni osonlashtirish uchun Rust chaqiruvlar srackini ko'rsatishi ham mumkin.

> ### panicga javoban stackni bo'shatish yoki bekor qilish
>
> Odatiy bo'lib, panic paydo bo'lganda, dastur o'chiriladi, ya'ni Rust
> stekni zaxiraga olib chiqadi va duch kelgan har bir funksiyadan ma'lumotlarni
> tozalaydi. Biroq, bu orqaga qaytish va tozalash juda ko'p ishdir. Rust,
> shuning uchun dasturni tozalamasdan tugatadigan darhol bekor
> qilishning muqobilini tanlashga imkon beradi.
>
> Dastur ishlatgan xotira keyinchalik operatsion tizim tomonidan
> tozalanishi kerak bo'ladi. Agar loyihangizda natijada olingan binary faylni
> iloji boricha kichikroq qilishingiz kerak bo'lsa, *Cargo.toml* faylingizdagi
> tegishli `[profile]` bo'limlariga `panic = 'abort'` qo'shish orqali panic
> bo'yicha bo'shatishdan bekor qilishga o'tishingiz mumkin. Misol uchun, agar siz
> bo'shatish rejimida panic holatini to'xtatmoqchi bo'lsangiz, quyidagilarni qo'shing:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Keling, oddiy dasturda `panic!` deb chaqirib ko'raylik:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

Dasturni ishga tushirganingizda, siz shunga o'xshash narsani ko'rasiz:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

`panic!` chaqiruvi oxirgi ikki qatordagi xato xabarini keltirib chiqaradi.
Birinchi qatorda panic haqidagi xabarimiz va manba kodimizdagi panic sodir bo'lgan joy ko'rsatilgan: *src/main.rs:2:5* bu bizning *src/main.rs* faylimizning ikkinchi qatori, beshinchi belgisi ekanligini bildiradi.

Bunday holda, ko'rsatilgan qator bizning kodimizning bir qismidir va agar biz ushbu qatorga o'tsak, biz `panic!` makro chaqiruvini ko'ramiz. Boshqa hollarda, `panic!` chaqiruvi bizning kodimiz chaqiradigan kodda bo'lishi mumkin, va xato xabari tomonidan bildirilgan fayl nomi va satr raqami boshqa birovning kodi bo'ladi, bu yerda `panic!` makro chaqiriladi, kodimizning oxir-oqibat `panic!` chaqiruviga olib kelgan qatori emas. Kodimizning muammoga sabab bo'lgan qismini aniqlash uchun `panic!` chaqiruvidan kelgan funksiyalarning backtracedan foydalanishimiz mumkin. Keyinchalik orqaga qaytish(backtraces) haqida batafsilroq gaplashamiz.

### `panic!` Backtracedan foydalanish

Yana bir misolni ko‘rib chiqaylik, `panic!` chaqiruvi to‘g‘ridan-to‘g‘ri makroni chaqiruvchi kodimizdan emas, balki kodimizdagi xato tufayli kutubxonadan kelganida qanday bo‘lishini ko‘raylik. 9-1 ro'yxatida joriy indekslar doirasidan tashqarida vectordagi indeksga kirishga harakat qiladigan ba'zi kodlar mavjud.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Roʻyxat 9-1: Vector oxiridan tashqaridagi elementga kirishga urinish, bu esa `panic!` chaqiruviga sabab boʻladi.</span>

Bu erda biz vectorimizning 100-elementiga kirishga harakat qilyapmiz (bu indeks 99-da, chunki indekslash noldan boshlanadi), lekin vectorda faqat 3 ta element mavjud.
Bunday vaziyatda Rust panic qo'yadi. `[]` dan foydalanish elementni qaytarishi kerak, lekin siz noto'g'ri indeksni o'tkazyapsiz: Rust qaytara oladigan element yo'q.

C tilida ma'lumotlar strukturasining oxiridan tashqarida o'qishga urinish aniqlanmagan xatti-harakatlardir. Siz xotirada ma'lumotlar strukturasidagi ushbu elementga mos keladigan har qanday joyni olishingiz mumkin, garchi xotira ushbu tuzilishga tegishli bo'lmasa ham. Bu *buffer overread* deb ataladi va agar tajovuzkor indeksni ma'lumotlar tuzilmasidan keyin saqlanadigan ma'lumotlarni o'qishga ruxsat etilmasligi kerak bo'lgan tarzda o'zgartira olsa, xavfsizlik zaifliklariga olib kelishi mumkin.

Dasturingizni bunday zaiflikdan himoya qilish uchun, agar siz mavjud bo'lmagan indeksdagi elementni o'qishga harakat qilsangiz, Rust ishlashni to'xtatadi va davom etishni rad etadi. Keling, sinab ko'raylik va ko'ramiz:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Bu xatolik `main.rs` ning 4-qatoriga ishora qiladi, bu yerda biz 99 indeksiga kirishga harakat qilamiz. Keyingi eslatma satrida biz xatoga nima sabab bo'lganligining backtraceni olish uchun `RUST_BACKTRACE` muhit o'zgaruvchisini o'rnatishimiz mumkinligini aytadi. *backtrace* - bu dastur bajarilishining ma'lum bir nuqtasiga qadar chaqirilgan barcha funktsiyalar ro'yxatini. Backtrace boshqa tillarda bo'lgani kabi Rust tilida ham xuddi shunday ishlaydi. Shuning uchun, biz boshqa joylarda bo'lgani kabi, backtrace ma'lumotlarini o'qishni tavsiya qilamiz - siz yozgan fayllar haqidagi ma'lumotlarni ko'rmaguningizcha yuqoridan pastga o'qing. Bu muammo paydo bo'lgan joy. Yuqoridagi satrlar sizning kodingiz chaqirgan koddir; Quyidagi satrlar sizning kodingiz deb ataladigan koddir. Ushbu oldingi va keyingi qatorlar asosiy Rust kodini, standart kutubxona kodi yoki siz foydalanayotgan cratelarni o'z ichiga olishi mumkin. `RUST_BACKTRACE` muhit oʻzgaruvchisini 0 dan tashqari istalgan qiymatga oʻrnatish orqali backtraceni olishga harakat qilaylik. 9-2 ro'yxati siz ko'rgan narsaga o'xshash natijani ko'rsatadi.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">Roʻyxat 9-2: `RUST_BACKTRACE` muhit oʻzgaruvchisi oʻrnatilganda koʻrsatiladigan `panic!` chaqiruvi tomonidan yaratilgan backtrace</span>

Bu juda ko'p natija! Siz ko'rgan aniq chiqish operatsion tizimingiz va Rust versiyasiga qarab farq qilishi mumkin. Ushbu ma'lumotlar bilan backtrace uchun debug symbollari yoqilgan bo'lishi kerak. Nosozliklarni tuzatish belgilari standart boʻyicha `--release` opsiyasiz `cargo build` yoki `cargo run` funksiyalaridan foydalanilganda yoqilgan.

9-2 ro'yxatdagi chiqishda backtracening 6-qatori muammoni keltirib chiqarayotgan loyihamizdagi chiziqqa ishora qiladi: *src/main.rs* ning 4-qatori. Agar dasturimiz panic bo'lishini istamasak, biz yozgan faylni eslatib o'tgan birinchi qatorda ko'rsatilgan joydan tekshirishni boshlashimiz kerak. 9-1 ro'yxatida biz panic qo'yadigan kodni ataylab yozganmiz. panicni tuzatish usuli vector indekslari doirasidan tashqarida elementni so'ramaslikdir. Kelajakda sizning kodingiz panic qo'zg'atganda, siz panic qo'zg'ash uchun kod qanday harakatlarni amalga oshirayotganini va buning o'rniga kod nima qilishi kerakligini aniqlashingiz kerak bo'ladi.

Biz `panic!` makrosi va qachon `panic!` qo'llashimiz kerak va qachon foydalanmaslik kerakligi haqidagi muhokamaga qaytamiz! Ushbu bobning keyingi qismida [“`panic!` qo'yish yoki `panic!` qo'ymaslik”][to-panic-or-not-to-panic]<!-- ignore --> haqida gapalashamiz.

Keyinchalik, `Result` yordamida xatoni qanday tiklashni ko'rib chiqamiz.

[to-panic-or-not-to-panic]:ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
