## Control Flow

Shartning `true` yoki yo'qligiga qarab ba'zi kodlarni ishga tushirish va shart `true` bo'lganda ba'zi kodlarni qayta-qayta ishga tushirish qobiliyati ko'pchilik dasturlash tillarida asosiy building bloklari hisoblanadi. Rust kodining bajarilishini nazorat qilish imkonini beruvchi eng keng tarqalgan konstruksiyalar `if` expressionlari va looplaridir.

### `if` ifodalari

`if` ifodasi shartlarga qarab kodingizni branchga ajratish imkonini beradi. Siz shartni taqdim etasiz va keyin shunday deb aytasiz: “Agar bu shart bajarilsa, ushbu kod blokini ishga tushiring. Agar shart bajarilmasa, ushbu kod blokini ishga tushirmang."

`If` ifodasini oʻrganish uchun *loyihalar* jildingizda *branchlar* nomli yangi loyiha yarating. *src/main.rs* faylida quyidagilarni kiriting:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Barcha `if` expressionlari `if` kalit so‘zidan boshlanadi, undan keyin shart keladi. Bunday holda, shart `raqam` o'zgaruvchisi 5 dan kichik qiymatga ega yoki yo'qligini tekshiradi. Agar shart `true` bo'lsa, biz kod blokini shartdan keyin darhol jingalak qavslar ichiga joylashtiramiz.
`if` expressionlaridagi shartlar bilan bog‘langan kod bloklari ba’zan *arms* deb ataladi, xuddi biz 2-bobning [“Tahminni maxfiy raqam bilan solishtirish”][comparing-the-guess-to-the-secret-number]<!--ignore --> bo‘limida muhokama qilgan `match` expressionlaridagi qurollar kabi.

Ixtiyoriy ravishda, agar shart `false` deb baholansa, dasturga bajarilishi uchun muqobil kod blokini berish uchun biz tanlagan `else` expressionini ham kiritishimiz mumkin. Agar `else` ifodasini bermasangiz va shart `false` bo‘lsa, dastur shunchaki `if` blokini o‘tkazib yuboradi va kodning keyingi bitiga o‘tadi.

Ushbu kodni ishga tushirishga harakat qiling; quyidagi chiqishni ko'rishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Keling, nima sodir bo'lishini ko'rish uchun `raqam` qiymatini shartni `false` qiladigan qiymatga o'zgartirib ko'raylik:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Dasturni qayta ishga tushiring va natijaga qarang:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Shuni ham ta'kidlash kerakki, ushbu koddagi shart `bool` bo'lishi kerak. Agar shart `bool` bo'lmasa, biz xatoga yo'l qo'yamiz. Masalan, quyidagi kodni ishga tushirishga harakat qiling:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

`if` sharti bu safar `3` qiymatiga teng bo'ladi va Rust xato qiladi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Xato shuni ko'rsatadiki, Rust `bool` kutgan, lekin integer(butun) son olgan. Ruby va JavaScript kabi tillardan farqli o'laroq, Rust boolean bo'lmagan turlarni boolean tilga o'zgartirishga avtomatik ravishda urinmaydi. Siz aniq bo'lishingiz va har doim `if` ni mantiqiy shart sifatida ko'rsatishingiz kerak. Agar biz `if` kod bloki faqat raqam `0` ga teng bo‘lmaganda ishlashini istasak, masalan, `if` ifodasini quyidagiga o‘zgartirishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Ushbu kodni ishga tushirish `raqam noldan boshqa narsa edi` chop etiladi.

#### `else if` bilan bir nechta shartlarni boshqarish

`if` va `else` ni `else if` ifodasida birlashtirib, bir nechta shartlardan foydalanishingiz mumkin.Misol uchun:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Ushbu dasturda to'rtta yo'l bor. Uni ishga tushirgandan so'ng siz quyidagi chiqishni ko'rishingiz kerak:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Ushbu dastur bajarilganda, u har bir `if` expressionni navbatma-navbat tekshiradi va shart `true` deb baholanadigan birinchi tanani bajaradi. E'tibor bering 6, 2 ga bo'linsa ham, biz `son 2 ga bo'linmaydi` chiqishini ko'rmayapmiz va `else` blokidagi `raqam 4, 3 yoki 2 ga bo'linmaydi` matnini ko'rmaymiz.Buning sababi, Rust faqat birinchi `true` shart uchun blokni bajaradi va bir marta topilsa, qolganlarini ham tekshirmaydi.
Juda ko'p `else if` expressionlaridan foydalanish kodingizni buzishi mumkin, shuning uchun sizda bir nechta bo'lsa, kodingizni qayta tahrirlashni xohlashingiz mumkin. 6-bobda bu holatlar uchun `match` deb nomlangan kuchli Rust tarmoqli konstruksiyasi tasvirlangan.

#### `let` statementida `if` dan foydalanish

`if` expression bo‘lganligi sababli, biz 3-2-listdagi kabi natijani o‘zgaruvchiga belgilash uchun `let` statementining o‘ng tomonida foydalanishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Ro'yxat 3-2: `if` expressioni natijasini o‘zgaruvchiga tayinlash</span>

`raqam` o'zgaruvchisi `if` expressioni natijasiga asoslangan qiymatga bog'lanadi. Nima sodir bo'lishini ko'rish uchun ushbu kodni ishga tushiring:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Esda tutingki, kod bloklari ulardagi oxirgi expressiongacha evaluate qilianadi va raqamlar o'zlari ham expressionlardir. Bu holda butun `if` expressionning qiymati qaysi kod bloki bajarilishiga bog'liq. Bu `if` ning har bir armidan result bo'lish potentsialiga ega bo'lgan qiymatlar bir xil turdagi bo'lishi kerakligini anglatadi; 3-2 ro'yxatda `if` va `else` armllarining natijalari `i32` butun sonlari edi. Agar turlar mos kelmasa(mismatched), quyidagi misolda bo'lgani kabi, biz xatoga duch kelamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Ushbu kodni kompilyatsiya qilmoqchi bo'lganimizda, biz xatoga duch kelamiz. `if` va `else` armllari mos kelmaydigan qiymat turlariga ega va Rust muammoni dasturda qayerdan topish mumkinligini aniq ko'rsatadi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` blokidagi expression butun songa, `else` blokidagi expression esa satrga baholanadi. Bu ishlamaydi, chunki oʻzgaruvchilar bitta turga ega boʻlishi kerak va Rust kompilyatsiya vaqtida `raqam` oʻzgaruvchisi qaysi turini aniq bilishi kerak. `raqam` turini bilish kompilyatorga ushbu tur biz `raqam` ishlatadigan hamma joyda yaroqliligini tekshirish imkonini beradi. Agar `raqam` turi faqat runtimeda aniqlangan bo'lsa, Rust buni qila olmaydi; kompilyator murakkabroq bo'lar edi va agar u har qanday o'zgaruvchi uchun bir nechta gipotetik turlarni kuzatib borishi kerak bo'lsa, kod haqida kamroq kafolatlar beradi.

### Looplar bilan takrorlash

Ko'pincha kod blokini bir necha marta bajarish foydali bo'ladi. Ushbu vazifani bajarish uchun Rust bir nechta *looplarni* taqdim etadi, ular sikl tanasi ichidagi kod orqali oxirigacha ishlaydi va keyin darhol boshida boshlanadi. Looplar bilan tajriba o'tkazish uchun keling, *looplar* deb nomlangan yangi loyiha yarataylik.

Rustda uch xil looplar mavjud: `loop`, `while` va `for`. Keling, har birini sinab ko'raylik.

#### Kodni `loop` bilan takrorlash

`loop` kalit so'zi Rustga kod blokini abadiy qayta-qayta bajarishni yoki uni to'xtatishni aniq aytmaguningizcha bajarishni aytadi.

Misol tariqasida, *looplar* jildingizdagi *src/main.rs* faylini quyidagicha o'zgartiring:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Ushbu dasturni ishga tushirganimizda, dasturni qo'lda to'xtatmagunimizcha, `yana!` so'zi doimiy ravishda chop etilishini ko'ramiz.Aksariyat terminallar uzluksiz siklda ishlab qolgan dasturni to'xtatish uchun <span class="keystroke">ctrl-c</span>  klaviatura yorliqlarini qo'llab-quvvatlaydi.
Sinab ko'ring:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/looplar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/looplar`
yana!
yana!
yana!
yana!
^Cyana!
```

`^C` belgisi <span class="keystroke">ctrl-c</span> tugmalarini bosgan joyni bildiradi. Kod uzilish signalini qabul qilganda siklning qayerda bo'lganiga qarab, `^C` dan keyin chop etilgan `yana!` so'zini ko'rishingiz yoki ko'rmasligingiz mumkin.

Yaxshiyamki, Rust kod yordamida loopdan chiqish yo'lini ham taqdim etadi. Siz dasturga siklni bajarishni qachon to'xtatish kerakligini aytish uchun `break` kalit so'zini siklga qo'yishingiz mumkin. 
Eslatib o'tamiz, biz buni 2-bobning [”To'g'ri taxmindan keyin chiqish”][quitting-after-a-correct-guess]<!-- ignore --> bo'limidagi taxminiy o'yinda, foydalanuvchi to'g'ri raqamni taxmin qilish orqali o'yinda g'alaba qozonganida dasturdan chiqish uchun qilganmiz.

Shuningdek, biz taxmin qilish o'yinida `continue` dan foydalandik, bu siklda dasturga siklning ushbu iteratsiyasida qolgan har qanday kodni o'tkazib yuborish va keyingi iteratsiyaga o'tishni aytadi.

#### Looplardan qiymatlarni qaytarish(return)

`loop` dan foydalanishdan biri bu ish bajarilmasligi mumkin bo'lgan operatsiyani qaytadan urinish, masalan, thread o'z ishini tugatganligini tekshirish. Bundan tashqari, ushbu operatsiya natijasini kodingizning qolgan qismiga sikldan o'tkazishingiz kerak bo'lishi mumkin. Buning uchun siklni toʻxtatish uchun foydalanadigan `break` ifodasidan keyin return qilinishi kerak boʻlgan qiymatni qoʻshishingiz mumkin; bu qiymat loopdan qaytariladi, shuning uchun uni bu yerda ko'rsatilganidek ishlatishingiz mumkin:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Loopdan oldin biz `hisoblagich` nomli o‘zgaruvchini e’lon qilamiz va uni `0` ga ishga tushiramiz. Keyin sikldan qaytarilgan qiymatni ushlab turish uchun `natija` nomli o'zgaruvchini e'lon qilamiz. Loopning har bir iteratsiyasida biz `hisoblagich` o‘zgaruvchisiga `1` qo‘shamiz va keyin `hisoblagich` 10 ga teng yoki yo‘qligini tekshiramiz. Bu bo'lganda, biz `hisoblagich * 2` qiymati bilan `break` kalit so'zidan foydalanamiz. Loopdan so'ng biz `natija` qiymatini belgilaydigan statementni tugatish uchun nuqta-verguldan foydalanamiz. Nihoyat, biz qiymatni `natija`da chop qilamiz, bu holda `20`.

#### Bir nechta looplar orasidagi farqni ajratish uchun loop labellari

Agar sizda looplar ichida looplaringiz bo'lsa, o'sha nuqtada eng ichki loopga `break` va `continue` amallari qo'llaniladi. Siz ixtiyoriy ravishda siklda *loop label* belgilashingiz mumkin, undan so‘ng `break` yoki  `continue` bilan o‘sha kalit so‘zlar eng ichki loop o‘rniga belgilangan loopga qo‘llanilishini belgilashingiz mumkin. Loop labellari bitta tirnoqcha bilan boshlanishi kerak. Mana ikkita ichki loop bilan bir misol:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Tashqi loopda `'hisoblash` labeli bor va u 0 dan 2 gacha hisoblanadi.
Labelsiz ichki loop 10 dan 9 gacha hisoblanadi. Label ko'rsatilmagan birinchi `break` faqat ichki sikldan chiqadi. `break 'hisoblash;` statementi tashqi sikldan chiqadi. Keling kodni run qilib ko'ramiz:  

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### `while` bilan shartli looplar

Dastur ko'pincha loop ichidagi shartni evaluate qilishi kerak bo'ladi. Shart `true` bo'lsa-da, loop ishlaydi. Shart `true` bo'lishni to'xtatganda, dastur loopni to'xtatib, `break` ni chaqiradi. Bu kabi xatti-harakatlarni `loop`, `if`, `else` va `break` kombinatsiyasidan foydalanib amalga oshirish mumkin; Agar xohlasangiz, buni hozir dasturda sinab ko'rishingiz mumkin. Biroq, bu pattern shunchalik keng tarqalganki, Rustda buning uchun `while` sikli deb ataladigan o'rnatilgan til konstruktsiyasi mavjud. 3-3 ro'yxatda biz dasturni uch marta aylanish uchun `while` dan foydalanamiz, har safar sanab chiqamiz, so'ngra sikldan so'ng xabarni chop etamiz va chiqamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Ro'yxat 3-3: Shart to'g'ri bo'lganda kodni ishga tushirish uchun `while` siklidan foydalanish</span>

Bu konstruksiya `loop`, `if`, `else` va `break` dan foydalansangiz, zarur bo'ladigan ko'plab joylashtirishlarni yo'q qiladi va bu aniqroq bo'ladi. Shart `true` deb baholansa, kod ishlaydi; aks holda, u loopdan chiqadi.

#### `for` bilan to'plam bo'ylab aylanish

Siz `while` konstruksiyasidan array kabi to‘plam elementlari ustidan aylanishni tanlashingiz mumkin. Masalan, 3-4 ro'yxatdagi sikl `a` arrayidagi har bir elementni chop etadi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Ro'yxat 3-4: `while` sikli yordamida to‘plamning har bir elementi bo‘ylab aylanish</span>

Bu erda kod arraydagi elementlar orqali hisoblanadi. U `0` indeksidan boshlanadi va keyin arraydagi yakuniy indeksga yetguncha (ya'ni, `index < 5` endi `true` bo`lmaganda) sikl davom etadi. Ushbu kodni ishga tushirish arraydagi har bir elementni chop etadi:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Barcha besh array qiymatlari kutilganidek terminalda paydo bo'ladi. Garchi `index` bir nuqtada `5` qiymatiga yetsa ham, arraydan oltinchi qiymatni olishga urinishdan oldin sikl ishlashni to‘xtatadi.

Biroq, bu yondashuv xatoga moyil; Agar indeks qiymati yoki test holati noto'g'ri bo'lsa, biz dasturni panic qo'yishimiz mumkin. Misol uchun, agar siz `a` arrayining ta'rifini to'rtta elementga o'zgartirsangiz, lekin shartni `while index < 4` bo'lganda yangilashni unutgan bo'lsangiz, kod panic qo'zg'atadi. Bu ham sekin, chunki kompilyator sikl orqali har bir iteratsiyada indeks array chegaralarida ekanligini shartli tekshirish uchun runtime kodini qo‘shadi.

Aniqroq variant sifatida, siz `for` siklidan foydalanishingiz va to'plamdagi har bir element uchun ba'zi kodlarni bajarishingiz mumkin. `for` sikli 3-5-ro'yxatdagi kodga o'xshaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Ro'yxat 3-5: `for` `sikli yordamida to'plamning har bir elementi bo'ylab aylanish</span>

Ushbu kodni ishga tushirganimizda, biz 3-4 ro'yxatdagi kabi natijani ko'ramiz. Eng muhimi, biz kodning xavfsizligini oshirdik va arrayning oxiridan tashqariga chiqish yoki yetarlicha uzoqqa bormaslik va ba'zi elementlarni yetishmayotganligi sababli paydo bo'lishi mumkin bo'lgan xatolar ehtimolini yo'q qildik.

`for` siklidan foydalanib, agar siz 3-4 roʻyxatda qoʻllanilgan metodda boʻlgani kabi arraydagi qiymatlar sonini oʻzgartirsangiz, boshqa kodni oʻzgartirishni eslab qolishingiz shart emas.

`for` looplarining xavfsizligi va ixchamligi ularni Rustda eng ko‘p ishlatiladigan loop konstruksiyasiga aylantiradi. 3-3 ro'yxatdagi `while` siklidan foydalanilgan ortga hisoblash misolida bo'lgani kabi, ma'lum bir necha marta kodni ishlatmoqchi bo'lgan vaziyatlarda ham ko'pchilik Rustaceanlar `for` siklidan foydalanadilar. Buning yo'li standart kutubxona tomonidan taqdim etilgan `Range` dan foydalanish bo'lib, bir raqamdan boshlanib, boshqa raqamdan oldin tugaydigan barcha raqamlarni ketma-ketlikda hosil qiladi.

Ortga hisoblash `for` sikli va biz hali u to‘g‘risida gapirmagan boshqa metod – `rev` yordamida diapazonni teskari tomonga o‘zgartirishga o‘xshaydi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Bu kod biroz chiroyliroq, shunday emasmi?

## Xulosa

Siz erishdingiz! Bu juda katta bob bo'ldi: siz o'zgaruvchilar, skalyar va compound ma'lumotlar turlari, funksiyalar, izohlar, `if` expressionlari va sikllar haqida bilib oldingiz! Ushbu bobda muhokama qilingan tushunchalar bilan mashq qilish uchun quyidagilarni amalga oshirish uchun dasturlar yaratishga harakat qiling:

* Haroratni Farengeyt va Selsiy o'rtasida o'zgartiring.
* *n*ta Fibonachchi raqamini yarating.
* Qo'shiqning takrorlanishidan foydalanib, “Rojdestvoning o'n ikki kuni“ Rojdestvo qo'shig'ining so'zlarini chop eting.

Davom etishga tayyor bo'lganingizda, Rustda boshqa dasturlash tillarida odatda mavjud bo'lmagan ownership(egalik) tushunchasi haqida gaplashamiz.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
