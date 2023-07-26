## Modullilikni va xatolarni boshqarishni yaxshilash uchun refaktoring

Dasturimizni yaxshilash uchun dastur tuzilishi va uning yuzaga kelishi mumkin bo'lgan xatolarni qanday hal qilishi bilan bog'liq bo'lgan to'rtta muammoni tuzatamiz. Birinchidan, bizning `main` funksiyamiz endi ikkita vazifani bajaradi: u argumentlarni tahlil qiladi va fayllarni o'qiydi. Dasturimiz o'sib borishi bilan `main` funksiya boshqaradigan alohida vazifalar soni ortadi. Funksiyaga mas'uliyat yuklagan sari, uning qismlaridan birini buzmasdan fikr yuritish, sinab ko'rish va o'zgartirish qiyinroq bo'ladi. Har bir funksiya bitta vazifa uchun javobgar bo'lishi uchun funksionallikni ajratish yaxshiroqdir.

Bu muammo ikkinchi muammo bilan ham bog'liq: `sorov` va `fayl_yoli` bizning dasturimiz uchun konfiguratsiya o'zgaruvchilari bo'lsa-da, dastur mantig'ini bajarish uchun `tarkib` kabi o'zgaruvchilardan foydalaniladi. `main` qancha uzun bo'lsa, biz ko'proq o'zgaruvchilarni qamrab olishimiz kerak bo'ladi; bizda qancha ko'p o'zgaruvchilar mavjud bo'lsa, ularning har birining maqsadini kuzatib borish shunchalik qiyin bo'ladi. Maqsadlari aniq bo'lishi uchun konfiguratsiya o'zgaruvchilarini bitta tuzilishga guruhlash yaxshidir.

Uchinchi muammo shundaki, biz faylni o‘qib chiqmaganda xato xabarini chop etish uchun `expect` tugmasidan foydalanganmiz, biroq xato xabari “Faylni o‘qishi kerak edi” degan yozuvni chiqaradi. Faylni o'qish bir necha usul bilan muvaffaqiyatsiz bo'lishi mumkin: masalan, fayl yetishmayotgan bo'lishi mumkin yoki bizda uni ochishga ruxsat yo'q.
Hozirda, vaziyatdan qat'i nazar, biz hamma narsa uchun bir xil xato xabarini chop qilamiz, bu esa foydalanuvchiga hech qanday ma'lumot bermaydi!

To‘rtinchidan, biz turli xil xatolarni qayta ishlash uchun `expect` dan qayta-qayta foydalanamiz va agar foydalanuvchi dasturimizni yetarlicha argumentlarni ko'rsatmasdan ishga tushirsa, Rustdan `index out of bounds`("chegaradan tashqari indeks") xatosini oladi va bu muammoni aniq tushuntirmaydi. Xatolarni qayta ishlash mantig'ini o'zgartirish kerak bo'lsa, kelajakdagi saqlovchilar(maintainerlar) kod bilan maslahatlashish uchun faqat bitta joyga ega bo'lishlari uchun barcha xatolarni qayta ishlash kodi bir joyda bo'lsa yaxshi bo'lar edi. Xatolarni qayta ishlash uchun barcha kodlar bir joyda bo'lsa, biz oxirgi foydalanuvchilarimiz uchun mazmunli bo'lgan xabarlarni chop etishimizni ta'minlaydi.

Keling, loyihamizni qayta tiklash orqali ushbu to'rtta muammoni hal qilaylik.

### Binary loyihalar uchun vazifalarni ajratish

Bir nechta vazifalar uchun javobgarlikni `main` funksiyaga taqsimlashning tashkiliy muammosi ko'plab ikkilik(binary) loyihalar uchun umumiydir. Natijada, Rust hamjamiyati `main` kattalasha boshlaganda ikkilik dasturning alohida muammolarini ajratish bo'yicha ko'rsatmalar ishlab chiqdi. Bu jarayon quyidagi bosqichlardan iborat:

* Dasturingizni *main.rs* va *lib.rs* ga bo'ling va dasturingiz mantig'ini *lib.rs* ga o'tkazing.

* Agar buyruq satrini tahlil qilish mantig'i kichik bo'lsa, u *main.rs* da qolishi mumkin.

* Buyruqlar qatorini tahlil qilish mantig'i murakkablasha boshlagach, uni *main.rs* dan chiqarib, *lib.rs* ga o'tkazing.

Ushbu jarayondan keyin `main` funksiyada qoladigan mas'uliyatlar quyidagilar bilan cheklanishi kerak:

* Argument qiymatlari bilan buyruq satrini tahlil qilish mantig'ini chaqirish
* Boshqa har qanday konfiguratsiyani sozlash
* *lib.rs* da `run` funksiyasini chaqirish
* `run` xatoni qaytarsa, xatoni hal qilish

Ushbu pattern vazifalarni ajratish bilan bog'liq: *main.rs* dasturni ishga tushirishni boshqaradi va *lib.rs* topshirilgan vazifaning barcha mantig'ini boshqaradi. `main` funksiyani toʻgʻridan-toʻgʻri test qilib koʻra olmasligingiz sababli, ushbu structura dasturingizning barcha mantig'ini *lib.rs* funksiyalariga koʻchirish orqali test qilib koʻrish imkonini beradi. *main.rs* da qolgan kod uni o'qish orqali uning to'g'riligini tekshirish uchun yetarlicha kichik bo'ladi. Keling, ushbu jarayonni kuzatib, dasturimizni qayta ishlaymiz.

#### Argument tahlilchisini(parser) chiqarish

Argumentlarni tahlil qilish(parsing qilish) funksiyasini `main` buyruq satrini tahlil qilish mantig'ini *src/lib.rs* ga ko'chirishga tayyorlash uchun chaqiradigan funksiyaga ajratamiz. Ro'yxat 12-5 `main` ning yangi boshlanishini ko'rsatadi, u `parse_config` yangi funksiyasini chaqiradi, biz buni hozircha *src/main.rs* da aniqlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-5: `main` dan `parse_config` funksiyasini chiqarish</span>

Biz hali ham buyruq qatori argumentlarini vectorga yig‘moqdamiz, lekin 1-indeksdagi argument qiymatini `sorov` o‘zgaruvchisiga va 2 indeksidagi argument qiymatini `main` funksiyasi ichidagi `fayl_yoli` o‘zgaruvchisiga belgilash o‘rniga, butun vectorni `parse_config` funksiyasiga o‘tkazamiz. Keyin `parse_config` funksiyasi qaysi argument qaysi o'zgaruvchiga kirishini aniqlaydigan mantiqni ushlab turadi va qiymatlarni `main`ga qaytaradi. Biz hali ham `sorov` va `fayl_yoli` o'zgaruvchilarini `main`da yaratamiz, lekin `main` endi buyruq qatori argumentlari va o'zgaruvchilari qanday mos kelishini aniqlash vazifasiga ega emas.

Ushbu qayta ishlash bizning kichik dasturimiz uchun ortiqcha bo'lib tuyulishi mumkin, ammo biz kichik, bosqichma-bosqich refactoring qilmoqdamiz. Ushbu o'zgartirishni amalga oshirgandan so'ng, argumentni tahlil qilish hali ham ishlayotganligini tekshirish uchun dasturni qayta ishga tushiring. Muammolar yuzaga kelganda sabablarini aniqlashga yordam berish uchun taraqqiyotingizni tez-tez tekshirib turish yaxshidir.

#### Konfiguratsiya qiymatlarini guruhlash

`parse_config` funksiyasini yanada yaxshilash uchun yana bir kichik qadam tashlashimiz mumkin.
Ayni paytda biz tupleni qaytarmoqdamiz, lekin keyin darhol bu tupleni yana alohida qismlarga ajratamiz. Bu, ehtimol, bizda hali to'g'ri mavhumlik yo'qligining belgisidir.

Yaxshilash uchun joy borligini ko'rsatadigan yana bir ko'rsatkich `parse_config` ning `config` qismidir, bu biz qaytaradigan ikkita qiymat bir-biriga bog'liqligini va ikkalasi ham bitta konfiguratsiya qiymatining bir qismi ekanligini anglatadi. Biz hozirda bu mantiqni ma'lumotlar strukturasida yetkazmayapmiz, bundan tashqari ikkita qiymatni tuplega guruhlash; Buning o'rniga biz ikkita qiymatni bitta strukturaga joylashtiramiz va har bir struktura maydoniga mazmunli nom beramiz. Buni qilish ushbu kodning kelajakdagi saqlovchilariga(maintainerlarga) turli qadriyatlar bir-biriga qanday bog'liqligini va ularning maqsadi nima ekanligini tushunishni osonlashtiradi.

12-6 ro'yxatda `parse_config` funksiyasining yaxshilanishi ko'rsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-6: `Config` strukturasining namunasini qaytarish uchun `parse_config` ni qayta tahrirlash</span>

Biz `sorov` va `fayl_yoli` nomli maydonlarga ega bo'lishi uchun aniqlangan `Config` nomli structi qo'shdik. Endi `parse_config` signaturesi `Config` qiymatini qaytarishini bildiradi. Biz `args`dagi `String` qiymatlariga reference qilingan satr bo‘laklarini qaytargan `parse_config` korpusida endi `Config` ga tegishli `String` qiymatlarini o‘z ichiga olgan holda belgilaymiz. `main`dagi `args` oʻzgaruvchisi argument qiymatlarining owneri(ega) boʻlib, faqat `parse_config` funksiyasiga ularni borrowga(qarz olish) ruxsat beradi, yaʼni `Config` `args` qiymatlariga ownership(egalik) qilmoqchi boʻlsa, Rustning borrowing(qarz olish) qoidalarini buzgan boʻlamiz.

`String` ma'lumotlarini boshqarishning bir qancha usullari mavjud; Eng oson, garchi unchalik samarasiz bo'lsa ham, route qiymatlar bo'yicha `clone` metodini chaqirishdir.
Bu `Config` nusxasi uchun ma'lumotlarning to'liq nusxasini oladi, bu esa satr(string) ma'lumotlariga referenceni saqlashdan ko'ra ko'proq vaqt va xotirani oladi. Biroq, ma'lumotlarni klonlash bizning kodimizni juda sodda qiladi, chunki biz referencelarning lifetimeni(ishlash muddati) boshqarishimiz shart emas; bu holatda, soddalikka erishish uchun ozgina ishlashdan voz kechish foydali savdodir.

> ### `clone` dan foydalanishning o'zaro kelishuvlari
>
> Ko'pgina Rustaceanlar orasida `clone` dan foydalanish vaqti xarajati tufayli ownership
> muammolarini hal qilish uchun foydalanmaslik tendentsiyasi mavjud.
> [13-bobda][ch13]<!-- ignore --> siz ushbu turdagi vaziyatda samaraliroq
> usullardan qanday foydalanishni o'rganasiz. Ammo hozircha rivojlanishni
> davom ettirish uchun bir nechta satrlarni nusxalash ma'qul, chunki siz bu nusxalarni
> faqat bir marta qilasiz va fayl yo'li va so'rovlar qatori juda kichik. Birinchi o'tishda
> kodni giperoptimallashtirishga urinishdan ko'ra, biroz samarasiz ishlaydigan dasturga
> ega bo'lish yaxshiroqdir. Rust bilan tajribangiz ortgan sayin, eng samarali
> yechimdan boshlash osonroq bo'ladi, ammo hozircha `clone` deb
> nomlash juda maqbuldir.

Biz `main`ni yangiladik, shuning uchun u `parse_config` tomonidan qaytarilgan `Config` namunasini `config` nomli o‘zgaruvchiga joylashtiradi va biz avval alohida `sorov` va `fayl_yoli` o‘zgaruvchilaridan foydalangan kodni yangiladik, shuning uchun u endi `Config` strukturasidagi maydonlardan foydalanadi.

Endi bizning kodimiz `sorov` va `fayl_yoli` bir-biriga bog'liqligini va ularning maqsadi dastur qanday ishlashini sozlash ekanligini aniqroq bildiradi. Ushbu qiymatlardan foydalanadigan har qanday kod ularni maqsadlari uchun nomlangan maydonlardagi `config` misolida topishni biladi.

#### `Config` uchun konstruktor yaratish

Hozircha biz `main` dan buyruq qatori argumentlarini tahlil qilish uchun javob beradigan mantiqni chiqarib oldik va uni `parse_config` funksiyasiga joylashtirdik. Bu bizga `sorov` va `fayl_yoli` qiymatlari o'zaro bog'liqligini va bu munosabatlar bizning kodimizda ko'rsatilishi kerakligini ko'rishga yordam berdi. Keyin biz `sorov` va `fayl_yoli` ning tegishli maqsadini nomlash va `parse_config` funksiyasidan qiymatlar nomlarini stuct maydoni nomi sifatida qaytarish uchun `Config` structini qo'shdik.

Endi `parse_config` funksiyasining maqsadi `Config` misolini yaratish bo‘lganligi sababli, biz `parse_config` ni oddiy funksiyadan `Config` structi bilan bog'langan `new` funksiyaga o‘zgartirishimiz mumkin. Ushbu o'zgarish kodni yanada idiomatik qiladi. Biz standart kutubxonada `String` kabi turlarning namunalarini `String::new` ni chaqirish orqali yaratishimiz mumkin. Xuddi shunday, `parse_config`ni `Config` bilan bog‘langan `new` funksiyaga o‘zgartirib, `Config::new` ni chaqirish orqali `Config` misollarini yaratishimiz mumkin bo‘ladi. 12-7 ro'yxat biz qilishimiz kerak bo'lgan o'zgarishlarni ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-7: `parse_config` ni `Config::new` ga o'zgartirish</span>

Biz `parse_config` deb chaqirgan `main`ni yangilab, `Config::new` deb chaqirdik. Biz `parse_config` nomini `new` ga o‘zgartirdik va uni `new` funksiyani `Config` bilan bog‘laydigan `impl` blokiga o‘tkazdik. Ishlayotganiga ishonch hosil qilish uchun ushbu kodni qayta kompilyatsiya qilib ko'ring.

### Qayta ishlash xatolarini tuzatish

Endi biz xatolarimizni tuzatish ustida ishlaymiz. Eslatib o'tamiz, `args` vectoridagi qiymatlarga 1 yoki indeks 2 da kirishga urinish vector uchtadan kam elementni o'z ichiga olgan bo'lsa, dastur panic paydo bo'ladi. Dasturni hech qanday argumentlarsiz ishga tushirishga harakat qiling; u shunday ko'rinadi:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1`(indeks chegaradan tashqarida: len 1, lekin indeks 1) qatori dasturchilar uchun moʻljallangan xato xabaridir. Bu bizning oxirgi foydalanuvchilarga nima qilish kerakligini tushunishga yordam bermaydi. Keling, buni hozir tuzatamiz.

#### Xato xabarini yaxshilash

Ro'yxat 12-8da biz `new` funksiyasiga chek qo'shamiz, bu 1 va 2 indekslarga kirishdan oldin bo'lakning yetarlicha uzunligini tasdiqlaydi. Agar bo'lak yetarlicha uzun bo'lmasa, dastur panic chiqaradi va yaxshiroq xato xabarini ko'rsatadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-8: Argumentlar soni uchun chek qo'shish</span>

Bu kod biz 9-13 roʻyxatda yozgan [`Taxmin::new` funksiyasiga oʻxshaydi,][ch9-custom-types]<!-- ignore --> bu yerda `qiymat` argumenti amaldagi qiymatlar oraligʻidan tashqarida boʻlganida `panic!` deb chaqirdik. Bu yerda bir qator qiymatlar mavjudligini tekshirish o‘rniga, biz `args` uzunligi kamida 3 ekanligini va funksiyaning qolgan qismi ushbu shart bajarilgan deb taxmin qilingan holda ishlashini tekshiramiz. Agar `args` uchta elementdan kam boʻlsa, bu shart toʻgʻri boʻladi va dasturni darhol tugatish uchun `panic!` makrosini chaqiramiz.

`new` da qoʻshimcha bir necha qator kodlar mavjud boʻlsa, keling, xatolik qanday koʻrinishini koʻrish uchun dasturni argumentlarsiz yana ishga tushiramiz:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Bu chiqish yaxshiroq: endi bizda oqilona xato xabari bor. Biroq, bizda foydalanuvchilarga berishni istamaydigan begona ma'lumotlar ham bor. Ehtimol, biz 9-13 roʻyxatda qoʻllagan texnikamizdan foydalanish bu yerda eng yaxshisi emas: `panic!` chaqiruvi [9-bobda muhokama qilinganidek][ch9-error-guidelines]<!-- ignore -->, foydalanish muammosidan koʻra dasturlash muammosiga koʻproq mos keladi. Buning o'rniga biz 9-bobda o'rgangan boshqa texnikadan foydalanamiz - muvaffaqiyat yoki xatoni ko'rsatadigan [`Result`ni][ch9-result]<!-- ignore -->  qaytarish.

<!-- Old headings. Do not remove or links may break. -->
<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### `panic!` o‘rniga `Result`ni qaytarish

Buning o'rniga, muvaffaqiyatli holatda `Config` misolini o'z ichiga olgan va xatolik holatida muammoni tasvirlaydigan `Result` qiymatini qaytarishimiz mumkin. Shuningdek, biz funksiya nomini `new`dan `build`ga o'zgartiramiz, chunki ko'plab dasturchilar `new` funksiyalar hech qachon ishlamay qolmasligini kutishadi. `Config::build` `main` bilan bog'langanda, muammo borligini bildirish uchun `Result` turidan foydalanishimiz mumkin.Keyin biz `main` ni `Err` variantini `panic!` chaqiruvi keltirib chiqaradigan `thread 'main'` va `RUST_BACKTRACE` haqidagi matnsiz foydalanuvchilarimiz uchun amaliyroq xatoga aylantirishimiz mumkin.

12-9 ro'yxatda biz hozir `Config::build` deb nomlanayotgan funksiyaning qaytish(result) qiymatiga va `Result`ni qaytarish uchun zarur bo'lgan funksiyaning tanasiga qilishimiz kerak bo'lgan o'zgarishlar ko'rsatilgan. E'tibor bering, biz `main`ni ham yangilamagunimizcha, bu kompilyatsiya qilinmaydi, biz buni keyingi ro'yxatda qilamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-9: `Config::build` dan `Result`ni qaytarish</span>

Bizning `build` funksiyamiz muvaffaqiyatli holatda `Config` misoli va xato holatida `&'static str` bilan `Result`ni qaytaradi. Bizning xato qiymatlarimiz har doim `'static` lifetimega ega bo'lgan satr harflari(string literal) bo'ladi. Biz funksiyaning asosiy qismiga ikkita o'zgartirish kiritdik: agar foydalanuvchi yetarli argumentlarni o'tkazmasa, `panic!` deb chaqirish o'rniga, biz endi `Err` qiymatini qaytaramiz va `Config` qaytish(return) qiymatini `OK` bilan o'rab oldik. Ushbu o'zgarishlar funksiyani yangi turdagi signaturega moslashtiradi.

`Config::build` dan `Err` qiymatini qaytarish `main` funksiyaga `build` funksiyasidan qaytarilgan `Result` qiymatini boshqarish imkonini beradi va xato holatida jarayondan tozaroq chiqish imkonini beradi.

<!-- Old headings. Do not remove or links may break. -->
<a id="calling-confignew-and-handling-errors"></a>

#### `Config::build` ga murojaat qilish va xatolarni qayta ishlash

Xato holatini hal qilish va foydalanuvchi uchun qulay xabarni chop etish uchun biz 12-10 roʻyxatda koʻrsatilganidek, `Config::build` tomonidan qaytariladigan `Result`ni qayta ishlash uchun `main`ni yangilashimiz kerak. Shuningdek, biz `panic!` dan nolga teng bo‘lmagan xato kodi bilan buyruq qatori dasturidan chiqish va uning o‘rniga uni qo‘lda amalga oshirish mas’uliyatini o‘z zimmamizga olamiz. Nolga teng bo'lmagan chiqish holati - bu bizning dasturimizni chaqirgan jarayonga dastur xato holati bilan chiqqanligi haqida signal berish uchun konventsiya.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-10: Agar `Config` build bo'lmasa, xato kodi bilan chiqish</span>

Ushbu ro'yxatda biz hali batafsil ko'rib chiqmagan metoddan foydalandik: standart kutubxona tomonidan `Result<T, E>` da aniqlangan `unwrap_or_else`.
`unwrap_or_else` dan foydalanish bizga `panic!` qo'ymaydigan xatoliklarni aniqlash imkonini beradi. Agar `Result` `Ok` qiymati bo'lsa, bu metodning harakati `unwrap` ga o'xshaydi: u `Ok` o'ralayotgan(wrap) ichki qiymatni qaytaradi. Biroq, agar qiymat `Err` qiymati bo'lsa, bu metod kodni *closure*(yopish) ga chaqiradi, bu biz belgilab beradigan anonim funksiya bo'lib, `unwrap_or_else` ga argument sifatida o'tkazamiz. Biz [13-bobda][ch13]<!-- ignore --> closure(yopilish)larni batafsil ko'rib chiqamiz.  Hozircha siz shuni bilishingiz kerakki, `unwrap_or_else` `Err` ning ichki qiymatidan o‘tadi, bu holda biz 12-9-listga qo‘shgan `"argumentlar yetarli emas"` statik qatori bo‘lib, bizning yopishimiz uchun Vertikal quvurlar(pipe) o'rtasida paydo bo'ladigan `Err` argumenti. Yopishdagi(closure) kod ishlayotganida `err` qiymatidan foydalanishi mumkin.

Biz standart kutubxonadan `process`ni qamrab olish uchun yangi `use` qatorini qo‘shdik. Xato holatida ishga tushiriladigan yopishdagi kod faqat ikkita qatordan iborat: biz `err` qiymatini chop qilamiz va keyin `process::exit`ni chaqiramiz. `process::exit` funksiyasi dasturni darhol to'xtatadi va chiqish holati kodi sifatida berilgan raqamni qaytaradi. Bu biz 12-8 roʻyxatda qoʻllagan `panic!` asosidagi ishlovga oʻxshaydi, ammo biz endi barcha qoʻshimcha natijalarni olmaymiz. Keling, sinab ko'raylik:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Ajoyib! Ushbu chiqish bizning foydalanuvchilarimiz uchun juda qulay.

### `main` dan mantiqni ajratib olish

Endi biz konfiguratsiyani tahlil qilishni qayta tiklashni tugatdik, keling, dastur mantig'iga murojaat qilaylik. ["Binary loyihalar uchun vazifalarni ajratish"](#separation-of-concerns-for-binary-projects)<!-- ignore --> da aytib o'tganimizdek, biz konfiguratsiyani o'rnatish yoki xatolarni qayta ishlash bilan bog'liq bo'lmagan `main` funksiyadagi barcha mantiqni ushlab turadigan `run` nomli funksiyani chiqaramiz. Ishimiz tugagach, `main` qisqa va tekshirish orqali tekshirish oson bo'ladi va biz boshqa barcha mantiqlar uchun testlarni yozishimiz mumkin bo'ladi.

12-11 ro'yxatda ajratilgan `run` funksiyasi ko'rsatilgan. Hozircha biz funksiyani chiqarishni kichik, bosqichma-bosqich yaxshilashni amalga oshirmoqdamiz. Biz hali ham *src/main.rs* da funksiyani aniqlayapmiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-11: Dastur mantig'ining qolgan qismini o'z ichiga olgan `run` funksiyasini chiqarish</span>

`run` funksiyasi endi faylni o‘qishdan boshlab `main` dan qolgan barcha mantiqni o‘z ichiga oladi. `run` funksiyasi argument sifatida `Config` misolini oladi.

#### `run` funksiyasidan xatolarni qaytarish(return)

Qolgan dastur mantigʻi `run` funksiyasiga ajratilgan boʻlsa, biz 12 9-ro'yxatdagi `Config::build` bilan qilganimiz kabi, xatolarni boshqarishni yaxshilashimiz mumkin. Dasturni `expect` deb chaqirish orqali panic qo‘yish o‘rniga, `run` funksiyasi biror narsa noto‘g‘ri ketganda `Result<T, E>`ni qaytaradi. Bu bizga foydalanuvchilarga qulay tarzda xatolarni `main`ga qayta ishlash mantig'ini yanada mustahkamlash imkonini beradi. 12-12 roʻyxatda `run` signaturesi va asosiy qismiga qilishimiz kerak boʻlgan oʻzgarishlar koʻrsatilgan.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-12: `run` funksiyasini `Result`ni qaytarish uchun o'zgartirish</span>

Biz bu yerda uchta muhim o'zgarishlarni amalga oshirdik. Birinchidan, biz `run` funksiyasining qaytish turini `Result<(), Box<dyn Error>>`ga o'zgartirdik. Bu funksiya avval birlik(binary) turini qaytardi, `()` va biz buni `Ok` holatida qaytarilgan qiymat sifatida saqlaymiz.

Xato turi uchun biz *trait obyekti* `Box<dyn Error>`dan foydalandik (va biz `std::error::Error` ni yuqori qismida `use` statementi bilan qamrab oldik). Biz [17-bobda][ch17]<!-- ignore --> trait objectlarni ko'rib chiqamiz. Hozircha shuni bilingki, `Box<dyn Error>` funksiya `Error` traitini amalga oshiradigan turni qaytarishini bildiradi, lekin qaytariladigan qiymatning qaysi turini belgilashimiz shart emas. Bu bizga turli xil xato holatlarida har xil turdagi xato qiymatlarini qaytarish uchun moslashuvchanlikni beradi. `dyn` kalit so'zi(keywordi) "dynamic(dinamik)" so'zining qisqartmasi.

Ikkinchidan, biz [9-bobda][ch9-question-mark]<!-- ignore --> aytib o'tganimizdek, `?` operatori foydasiga `expect` chaqiruvini olib tashladik. Xatoda `panic!` o‘rniga, `?` murojat qiluvchiga ishlov berish uchun joriy funksiyadan xato qiymatini qaytaradi.

Uchinchidan, `run` funksiyasi endi muvaffaqiyatli holatda `Ok` qiymatini qaytaradi.
Biz signatureda `run` funksiyasining muvaffaqiyat turini `()` deb e’lon qildik, ya’ni birlik turi qiymatini `Ok` qiymatiga o‘rashimiz(wrap) kerak. Bu `Ok(())` sintaksisi dastlab biroz g‘alati ko‘rinishi mumkin, ammo `()` dan foydalanish biz `run`ni faqat uning yon ta’siri uchun chaqirayotganimizni bildirishning idiomatik usulidir; u bizga kerakli qiymatni qaytarmaydi.

Ushbu kodni ishga tushirganingizda, u kompilyatsiya qilinadi, lekin ogohlantirishni ko'rsatadi:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust bizga kodimiz `Result` qiymatini e'tiborsiz qoldirganligini va `Result` qiymati xatolik yuz berganligini ko'rsatishi mumkinligini aytadi. Ammo biz xatolik bor yoki yo'qligini tekshirmayapmiz va kompilyator bu yerda xatoliklarni hal qilish uchun kodga ega bo'lishimiz kerakligini eslatadi! Keling, bu muammoni hozir tuzatamiz.

#### `main`dagi `run` dan qaytarilgan xatolarni qayta ishlash

Biz xatolarni tekshirib ko'ramiz va ularni 12-10-sonli ro'yxatdagi `Config::build` bilan ishlatganimizga o'xshash metod yordamida hal qilamiz, lekin bir oz farq bilan:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

`run` `Err` qiymatini qaytaradimi yoki yo‘qligini tekshirish uchun `unwrap_or_else` o‘rniga `if let` dan foydalanamiz va agar qaytarsa `process::exit(1)`ni chaqiramiz. `run` funksiyasi `Config::build` `Config` misolini qaytarganidek, biz `unwrap`ni xohlagan qiymatni qaytarmaydi. Muvaffaqiyatli holatda `run`  `()` ni qaytargani uchun biz faqat xatoni aniqlash haqida qayg'uramiz, shuning uchun o'ralgan(wrap) qiymatni qaytarish uchun `unwrap_or_else` shart emas, bu faqat `()` bo`ladi.

`if let` va `unwrap_or_else` funksiyalarining tanasi ikkala holatda ham bir xil: biz xatoni chop qilamiz va chiqamiz.

### Kodni kutubxona(library) cratesiga bo'lish

Bizning `minigrep` loyihamiz hozircha yaxshi ko'rinmoqda! Endi biz *src/main.rs* faylini ajratamiz va *src/lib.rs* fayliga bir nechta kodni joylashtiramiz. Shunday qilib, biz kodni sinab ko'rishimiz va kamroq mas'uliyatli *src/main.rs* fayliga ega bo'lishimiz mumkin.

Keling, `main` funksiya bo'lmagan barcha kodlarni *src/main.rs* dan *src/lib.rs* ga o'tkazamiz:

* `run` funksiyasi definitioni
* Tegishli `use` statementlari
* `Config` ning definitioni
* `Config::build` definitioni

*src/lib.rs* ning mazmuni 12-13 roʻyxatda koʻrsatilgan signaturelarga ega boʻlishi kerak (qisqalik uchun funksiyalarning qismlarini olib tashladik). E'tibor bering, biz 12-14 ro'yxatdagi *src/main.rs* ni o'zgartirmagunimizcha, bu kompilyatsiya qilinmaydi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 12-13: `Config` va `run` ni *src/lib.rs* ichiga ko'chirish</span>

Biz `pub` kalit so‘zidan erkin foydalandik: `Config` da, uning maydonlari va `build` metodida va `run` funksiyasida. Endi bizda testdan o'tkazishimiz mumkin bo'lgan ommaviy(public) API mavjud kutubxona cratesi bor!

Endi biz *src/lib.rs* ga ko'chirilgan kodni 12-14 ro'yxatda ko'rsatilganidek *src/main.rs* dagi binary crate doirasiga olib kirishimiz kerak.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<span class="caption">Ro'yxat 12-14: *src/main.rs* da `minigrep` kutubxona cratesidan foydalanish</span>

Kutubxona cratesidan `Config` turini binary crate ko'lamiga olib kirish uchun `use minigrep::Config` qatorini qo'shamiz va `run` funksiyasiga crate nomimiz bilan prefix qo'shamiz. Endi barcha funksiyalar ulanishi va ishlashi kerak. Dasturni `cargo run` bilan ishga tushiring va hamma narsa to'g'ri ishlashiga ishonch hosil qiling.

Vouv! Bu juda ko'p ish edi, lekin biz kelajakda muvaffaqiyatga erishdik. Endi xatolarni hal qilish ancha oson va biz kodni modulliroq qildik. Deyarli barcha ishlarimiz bundan buyon *src/lib.rs* da amalga oshiriladi.

Keling, eski kod bilan qiyin bo'lgan, ammo yangi kod bilan oson bo'lgan narsani qilish orqali ushbu yangi modullikdan foydalanaylik: biz bir nechta testlarni yozamiz!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
