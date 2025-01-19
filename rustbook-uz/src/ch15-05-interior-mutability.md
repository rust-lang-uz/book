## `RefCell<T>` va Ichki O'zgaruvchanlik Shakli(pattern)

*Ichki o'zgaruvchanlik* bu Rustda ma'lumotlarga o'zgarmas referenslar mavjud bo'lganda ham ma'lumotni o'zgartirish imkonini beruvchi (dizayn) shakli/patternidir: odatda bu borrowing qoidalari bo'yicha esa taqiqlangan. Ma'lumotni o'zgaruvchan qilish uchun shakl/pattern Rustning o'zgaruvchanlik va borrowingni boshqaruchi oddiy qoidalarini chetlab o'tish uchun ma'lumotlar strukturasi ichiga `unsafe` kod ishlatiladi. Xavfsiz bo'lmagan kod bizning o'rnimizga kompilyatorga qoidalarni kompliyator yordamisiz tekshirayotganimizni ko'rsatadi; xavfsiz bo'lmagan kod haqida 19-bo'limda o'rganib chiqamiz.

Biz ichki o'zgaruvchanlik shakli/patterni ishlatadigan turlardan faqatgina borrowing qoidalari runtimeda amal qilingaligi paytida ishlatishimiz mumkin, kompilyator bunga kafolat bera olmaydi. Keyin `unsafe` kod xavfsiz APIga ulanadi va tashqi tur o'zgarmasligicha qoladi.

Keling ushbu tushunchani ichki o'zgaruvchanlik shakliga amal qiluvchi quyidagi `RefCell<T>` turiga qarab ko'rib chiqaylik.

### Enforcing Borrowing Rules at Runtime with `RefCell<T>` yordamida Borrowing qoidalarini Runtime vaqtida kuch bilan ishlatish

`Rc<T>`lidan faqrli o'laroq, `RefCell<T>` turi o'zi egalik qilib turgan ma'lumotda yagona egalikni namoyish etadi. Xo'sh, `RefCell<T>` turi `Box<T>` turidan nimasi bilan farq qiladi? 4-bo'limda o'tilgan borrowing qoidalarini esga olaylik: 

* Xohlagan belgilangan vaqtda, siz *yoki* (ikkalasini bir vaqtda ega bo'lish mumkin emas)bitta        o'zgaruvchan referens yoki xohlagan sondagi o'zgarmas referenslarga ega bo'lishingiz mumkin. 
* Referenslar har doim yaroqli bo'lishi shart

Referenslar va `Box<T>` bilan, borrowing qoidalarining kompilyatsiya vaqtida o'zgarmaslar kuchga kiradi. `RefCell<T>` bilan esa ushbu o'zgarmaslar *runtime paytida* kuchga kiradi. Referenslar bilan, agar siz ushbu qoidalarni buzsangiz, sizda kompilyator xatoligi yuzaga keladi. `RefCell<T>` bilan suhbu qoidalarni buzganingizda, sizning dasturingizda panic vujudga kelib, dastur chiqib ketadi.

The advantages of checking the borrowing rules at compile time are that errors
will be caught sooner in the development process, and there is no impact on
runtime performance because all the analysis is completed beforehand. For those
reasons, checking the borrowing rules at compile time is the best choice in the
majority of cases, which is why this is Rust’s default.

Borrowing qoidalarini kompilyatsiay vaqtida tekshirishning yaxshi tarafi xatolarni development vaqtida tezroq topishdir, va runtime unumdorligiga ta'sir ko'rsatmaydi chunki hamma analizlar oldindan qilingan bo'ladi. Ko'p hollarda borrowing qoidalarini kompilyatsiya vaqtida tekshirish eng yaxshi tanlovdir, sababi ushbu xususiyat Rustda odatiy  xususiyatidir. 

Borrowing qoidalarini runtime vaqtida tekshrishning afzalligi shundaki, kompilyatsiya vaqtidagi tekshiruvlar tomonidan ruxsat etilmaganda ba'zi xotira uchun xavfsizlik ssenariylarga ruxsat beriladi. Rust kompilyatoriga o'xshagan statik analizlar o'z-o'zidan konservativdir. Kodni tahlil qilayotganda kodning ba'zi bir xususiyatlarini aniqlash qiyindir: bunga  Halting Problem mashxur misol bo'la oladi, bu kitob doirasidan tashqarida bo'lsada lekin izlanib o'rganish uchun qiziq mavzu

Agar Rust kompilyatori egalik (ownership) qoidalari asosida kompilyatsiya qilayotganini aqiqlay olmasa, bu to'g'ri, ya'ni ishlab turgan dasturni rad etishi mumkin, shuning uchun ham konservativ hisoblanadi va bu ba'zi tahlillar uchun qiyindir. Agar Rust xatolikka ega bo'lgan dasturni qabul qilsa, foydalanuvchilar Rust beradigan kafolatlarga ishona olmaydilar. Agarda, Rust ishlab turgan dasturni rad etsa, dasturchi uchun noqulaylik tug'diradi, lekin hech qanday qo'rqinchli narsa bo'lmaydi. `RefCell<T>` turi sizning kodingiz borrowing qoidlariga amal qilayotganiga ishonchingiz komil bo'lganda lekin kompilyator buni tushuna olmayotganda va kafolat bera olmaganda foydalidir.

`RefCell<T>` `Rc<T>`ga o'xshab bitta potokli (oqimli) ssenariylarda ishlatilinadi va agar siz ko'p potokli (oqimli) holatda ishlatsangiz kompilyatsiya vaqtidagi xatolikni yuzaga keltiradi. Biz `RefCell<T>`ni ko'p potokli (oqimli) dasturda qanday qilib funksionalligini olishni 16-bo'limda ko'rib chiqamiz.  

Quyida takrorlash uchun `Box<T>`, `Rc<T>`, yoki `RefCell<T>`ni tanlash sabablari:

* `Rc<T>` bitta ma'lumotga ko'p egalarga ega bo'lish imkonini beradi; `Box<T>` va `RefCell<T>`
  esa yagona egaga egadirlar;
* `Box<T>` kompilyatsiya vaqtida o'zgaruvchan va o'zgarmas borrowlarni tekshrilishini ta'minlaydi;
  `Rc<T>` kompilyatsiya vaqtida faqat o'zgarmas borrowlarni tekshrilishini ta'minlaydi; 
  `RefCell<T>` runtimeda o'zgaruvchan va o'zgarmas borrowlarni tekshrilishini ta'minlaydi.
* Because `RefCell<T>` runtimeda o'zgaruvchan borrowlar tekshirilishi ta'minlaydi, agar
  `RefCell<T>` o'zgarmas bo'lsada `RefCell<T>` ichida qiymatni o'zgaruvchan qilishingiz mumkin.

Qiymatni o'zgarmas qiymat ichida o'zgaruvchan qilish *ichki o'zgaruvchanlik* shaklidir (pattern).
Keling ichki o'zgaruvchanlikni foydali ekanligini va bu qanday sodir bo'lishini misollarda ko'rib
chiqaylik.

### Ichki o'zgaruvchanlik: O'zgaruvchan Borrowdan O'zgarmas Qiymatga

Borrowing qoilari natijasida, o'zgarmas qiyamtga ega bo'lganingizda, siz o'zgaruvchan borrow qila olmaysiz. Masalan, ushbu kod kompilyatsiya qilinmaydi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Agar siz ushbu kodni kompilyatsiya qilishga harakat qilsangiz, quyidagi xatolik kelib chiqadi:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

Shunday vaziyatlar borki qiymat o'zini-o'zi o'zining metodlarida o'zgarivchan qilishi
foydali hisoblanadi, lekin boshqa kodda o'zgarmas shaklda bo'ladi. Kodning doirasidan
tashqaridagi qiymat metodi qiymatni o'zgaruvchan qila olmaydi. `RefCell<T>`ni ishlatish ichki
o'zgaruvchanlikga ega bo'lishning bir yo'li hisoblanadi, lekin `RefCell<T>` borrowing 
qoidalarini to'liq aylanib o'tmaydi: kompilyatorda borrow tekshiruvchisi shu ichki o'zgaruvchanlikka
ruxsat beradi, va borrowing qoidalari runtimes tekshiriladi. Agar qoidalarni rad etsangiz,
kompilyator xatoligini o'rniga `panic!` ko'rasiz. 

`RefCell<T>`ni o'zgarmas qiymatni o'zgaruvchanga aylantirishni, hamda nimaga `RefCell<T>`ni
ishlatish foydali ekanligini amaliy misollarda ko'rib chiqaylik.

#### Ichki o'zgaruvchanlik uchun foydalanish holati/misoli: Soxta Obyektlar

Ayrim hollarda test vaqti dasturchi boshqa turni o'rniga kerakli hatti-harakatni kuzatish
uchun va to'g'ri kompilyatsiya amalga oshirilganligini tasdiqlash uchun boshqa bir turni
ishlatib ko'radi. Ushbu to'ldiruvchi tur *test double* deb ataladi. Qiyin bo'lgan sahna ko'rinishida
aktyorning o'rniga chiqib, sahna ko'rinishi amalga oshirib beruvchi, ya'nikino yaratishda "kaskadyor"
misolida ko'rib chiqaylik. Test doublelari boshqa turlarda test o'tkazayotganimizda xizmat qiladi.
*Soxta obyektlar* test paytida nimalar sodir bo'lishini qayd etuvchi test doublelar o'ziga xos turlardan biri bo'lib, siz to'g'ri amallar amalga oshirilayotganini ko'zdan kechirishingiz mumkin.

Rustda boshqa dasturlash tillari kabi bir xil ma'noli obyektlarga ega emas,
va soxta obyekt funksionalligini olgan standart kutubxonasi yo'q. Aksincha, soxta 
obyektlar kabi ish bajaruvchi struct yaratishingiz mumkin. 

Ushbu ssenariyni ko'rib chiqaylik: qiymatni maksimal qiymatga nisbatan kuzatuvchi
kutubxona yaratamiz va joriy qiymat maksimal qiymatga qanchalik yaqinligiga qarab bizga
xabar jo'natib turadi. Ushbu kutubxona foydalanuvchi uchun ruxsat etilgan API 'call'lar sonini
kuzatib borish uchun ishlatilishi mumkin, bu ishlatish mumkin bo'lgan bir misol.

Bizning kutubxonamiz faqatgina qiymatni maksimal qiymatga qanchalik yaqin ekanligi
va qaysi vaqtda qaysi xabar jo'natilishini kuzatib turish imkonini beredi. Bizning
kutubxonamizdan foydalanadigan ilovalar xabar jo'natish mexanizmini ta'minlashini
talab qiladi, ya'ni ilova xabarni ilova ichida, email orqali, matnli xabar ko'rinishida
yoki boshqa ko'rinishda yuborishi mumkin. Kutubxona ushbu tafsilotlarni bilishi talab etilmaydi.
Kutubxona uchun biz tomonimizdan qo'llaniladigan `Messenger` traitini implementatsiya qiladigan
narsa kerak xolos.

<span class="filename">Faylnomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

<span class="caption">15-20-ro'yxat: Qiymatni qanchalik maksimal qiymatga yaqinligini kuzatish va kerakli darajaga 
yetganda ogohlantiruvchi kutubxona</span>

Ushbu kodning e'tiborli tomoni shundaki `Messenger` traitining `send` nomli metodi 
xabarning matni hamda`self`ga o'zgarmas referensni oladi. Ushbu trait bizning soxta 
obyektimizning implementatisiyasi uchun kerak bo'lgan interfeys hisoblanadi, shu 
holatda soxta obyekt haqiqiy obyektga o'xshab ishlatilishi mumkin. Yana bir muhim 
tomoni shundaki, biz `set_value`ni ko'rinishini `LimitTracker` orqali ko'rishimiz mumkin.  
Biz xohlaganimizcha o'tkazayotganimizni `value` parametri uchun o'zgartirishimiz mumkin, 
lekin `set_value` biz da'vo qilishimiz mumkin bo'lgan narsani return qilmaydi. Agar biz 
`Messenger` traitini implementatisiya qiladigan va ma'lum bir qiymatga ega bo'lgan `LimitTracker` 
yaratsak, `value` uchun turli raqamlar berganimizda, xabar kerakli xabar ko'rinishida jo'natildi 
deya olishni xohlaymiz.

Pochta orqali yoki matn xabar orqali xabar jo'natish o'rniga biz `send` ni ishga tushurib yuborilishi kerak bo'lgan xabarni 
kuzatish uchun soxta obyekt kerak bo'ladi. Obyektning yangi namunasini yaratishimiz mumkin, soxta obyektdan foydalanadigan 
`LimitTracker` yaratib, `LimitTracker`da `set_value` metodini qo'llashimiz va soxta obyekt biz kutgan xabar bor yoki yo'qligin 
tekshirib ko'ramiz. 15-21-ro'yxat soxta obyekt implementatsiya qilishga urinishi, lekin borrow tekshiruvchi ruxsat bermasligi ko'rsatilgan: 

<span class="filename">Faylnomi: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

<span class="caption">15-21-ro'yxat: `MockMessenger`ning implementatsiya qilishga urinishi, ammo borrow checker bunga ruxsat bermayotgaligi ko'rsatilgan</span>

Ushbu test kodimiz `Sent_messages` maydoniga ega boʻlgan `MockMessenger` strukturasini belgilaydi va u `Vec`ga ega bo'lgan `String` qiymatlari bilan joʻnatilishi kerak boʻlgan xabarlarni kuzatib boradi. Shuningdek, biz bo'sh xabarlar ro'yxati bilan boshlanadigan yangi `MockMessenger` qiymatlarini yaratishni qulay qilish uchun `yangi` funksiyasini aniqlaymiz. Biz bo'sh xabarlar ro'yxati bilan boshlanadigan yangi MockMessenger qiymatlarini yaratamiz. Keyin biz `LimitTracker` ga `MockMessenger`ni berishimiz uchun `MockMessenger` uchun “Messenger” xususiyatini amalga oshiramiz. `send` usulining taʼrifida biz uzatilgan xabarni parametr sifatida qabul qilamiz va uni `sent_messages`ning `MockMessenger` roʻyxatida saqlaymiz. 

Sinovda biz `LimitTracker`ga `value`ni `max` qiymatining 75 foizidan ko‘prog‘iga o‘rnatish buyurilganda nima sodir bo‘lishini sinab ko‘ramiz. Birinchidan, biz yangi `MockMessenger` ni yaratamiz, u xabarlarning bo'sh ro'yxati bilan boshlanadi. Keyin biz yangi `LimitTracker` yaratamiz va unga yangi `MockMessenger` va `max` qiymati 100 ga teng reference beramiz. 100dan 75dan katta bo'lgan, 80ga teng bo'lgan qiymatli `LimitTracker`dagi `set_value` metodini ishga tushiramiz. Keyin biz `MockMessenger` kuzatayotgan xabarlar roʻyxatida bitta xabar boʻlishi kerakligini taʼkidlaymiz.

Biroq, bu testda ko'rsatilganidek, bitta muammo bor:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

Biz `MockMessenger`ni xabarlarni kuzatib borish uchun o‘zgartira olmaymiz, chunki `send` metodi `self`ga o'zgarmas refernce oladi. Shuningdek, xato matnidagi `&mut self` dan foydalanish taklifini ham qabul qila olmaymiz, chunki `send` signaturasi `Messenger` traiti taʼrifidagi imzoga toʻgʻri kelmas edi (urunib ko'rganingizda qanaqa xatolik (error message) bo'lishini ko'rishingiz mumkin).

Ushbu holatda bizga ichgi o'zgaruvchanlik yordam berishi mumkin! Biz `RefCell<T>` orqali `sent_messages`ni joylyamiz, va keyin biz ko'rgan xabarlarni joylashtirish uchun `send` metodi `sent_messages`ni o'zgartira oladi. 15-22-ro'yxat bu qanday ko'rinishda bo'lishini ko'rsatadi:

<span class="filename">Faylnomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

<span class="caption">15-22-ro'yxat: tashqi qiymat o'zgarmas bo'lganida `RefCell<T>` yordamida ichki qiymatni o'zgaruvchan qilish</span>

`sent_messages` maydoni endi `Vec<String>` oʻrniga `RefCell<Vec<String>>` turiga ega. 
`new` funksiyada biz yangisini yaratamiz `RefCell<Vec<String>>` bo'sh vektor atrofidagi misol.
`new` funksiyasida biz yangi `RefCell<Vec<String>>` instancesini bo'sh vektor atrofida yaratib olamiz.

`send` metodini ishga tushirishda, traitning ma'no/tarifiga o'xshash bo'lgan birinchi parametr `self`ning o'zgarmas borrowi bo'lib qolaveradi. Vvektor hisoblangan `RefCell<Vec<String>>`ninig ichidagi qiymatga o'zgaruvchan reference olish uchun `self.sent_messages`ning `RefCell<Vec<String>>`dagi `borrow_mut`ni ishga tushuramiz. Keyin test paytida yuborilgan xabarlarni kuzatib borish uchun vektorga o'zgaruvchan referenceda `push` ni ishga tushirishimiz mumkin.

Tasdiqlash uchun biz qilishimiz kerak bo'lgan oxirgi o'zgarish bu: ichki vektor ichida qancha itemlar borligini ko'rish uchun, vektorga o'zgarmas refrence olish uchun`RefCell<Vec<String>>`dagi `borrow`ni ishga tushiramiz.

`RefCell<T>`dan qanday ishlatish mumkiniligi bilan tanishdik, keling qanday ishlashi haqida o'raganib chiqaylik.

#### Runtime vaqtidan `RefCell<T>` yordamida Borrowlarni kuzatish

O'zgarmas va o'zgaruvchan referencelarni yaratishda biz mos ravishda `&` va `&mut` sintaksisidan foydalanamiz. `RefCell<T>` bilan biz `RefCell<T>`ga tegishli xavfsiz API tarkibiga kiruvchi `borrow` va `borrow_mut` usullaridan foydalanamiz. `borrow` metodi `Ref<T>` smart pointer turini, `borrow_mut` esa `RefMut<T>` smart pointer turini qaytaradi. Ikkala tur ham `Deref` ni implementatsiya qiladi, shuning uchun biz ularni/doimiy oddiy reference kabi ko'rib chiqishimiz mumkin.

`RefCell<T>` hozirda qancha `Ref<T>` va `RefMut<T>` smart pointerlari faol ekanligini kuzatib boradi. Har safar biz `borrow` ishga tushirganimizda, `RefCell<T>` qancha o'zgarmas borrowlar faolligini oshiradi. Agar `Ref<T>` qiymati chegarasidan chiqib ketsa, o'zgarmas borrowlar soni bittaga kamayadi. Kompilyatsiya vaqtidagi borrowing qoidalari kabi, `RefCell<T>` bizga istalgan vaqtda koʻp oʻzgarmas yoki bitta oʻzgaruvchan borrowga ega boʻlish imkonini beradi.

Agar biz referencelarda bo'lgani kabi kompilyator xatosini olishdan ko'ra, ushbu qoidalarni buzishga harakat qilsak, `RefCell<T>` amalga oshirilishi runtime vaqtida panic qo'yadi. 15-23 ro'yxatda 15-22 ro'yxatda `send` ning implementatsiyasining modifikatsiyasi ko'rsatilgan. `RefCell<T>` runtimeda buni amalga oshirishga to'sqinlik qilishini ko‘rsatish uchun biz ataylab bir xil qamrov uchun faol ikkita o‘zgaruvchan borrowni yaratishga harakat qilapmiz.

<span class="filename">Faylnomi: src/lib.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

<span class="caption">Listing 15-23: Creating two mutable references in the
same scope to see that `RefCell<T>` will panic</span>

`borrow_mut`dan qaytarilgan `RefMut<T>` smart pointer uchun `one_borrow` o'zgaruvchisini yaratamiz. Keyin `two_borrow` o'zgaruvchisida xuddi shu tarzda boshqa o'zgaruvchan borrow hosil qilamiz. Bu bir scopeda ikkita o'zgaruvchan reference qiladi, bu aslida mumkin emas. Kutubxonamiz uchun testlarni o'tkazganimizda, 15-23 ro'yxatdagi kod hech qanday xatosiz kompilyatsiya qilinadi, ammo test muvaffaqiyatsiz bo'ladi:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

E'tibor bering ushbu kod quyidagi panicni keltirib chiqardi `already borrowed: BorrowMutError`. Bu `RefCell<T>`runtime vaqtida borrowing qoidalari buzilishini boshqariishini ko'rsak bo'ladi

Ko'p marta qilganimizdek, ya'ni kompilyatsiya vaqtida emas, balki ish vaqtida borriwing 
xatolarini ko'rish, ishlab chiqish jarayonida keyinchalik kodingizda 
xatolar topishingiz mumkinligini anglatadi, ya'ni sizning kodingiz productionga 
deploy qilinmagungacha. Bundan tashqari, sizning kodingiz kompilyatsiya vaqtida emas, aksincha runtime 
vaqtida borrowlarni kuzatib borishi natijasida kichik runtime xatolik bo'lishi mumkin. 
Lekin, `RefCell<T>`dan foydalanish faqat o'zgaruvchan qiymatlar mumkin bo'lgan kontekstdan foydalanayotgan
vaqtingizdagi xabarlarni kuzatish uchun o'zini o'zgaritira oladigan soxta obyektni yaratish imkonini beradi. Doimiy referencelardan ko'ra ko'proq funksionallikni olishni xohlasangiz `RefCell<T>`dan foydalanishingiz mumkin, uning farqli tomomnlariga qaramasdan ham.

### `Rc<T>` va `RefCell<T>`ni birlashtirish orqali o`zgaruvchan ma`lumotlarning bir nechta egalariga ega bo`lish

`RefCell<T>` dan foydalanishning keng tarqalgan usuli `Rc<T>` bilan kombinatsiyadir. Eslatib o'tamiz, `Rc<T>` sizga ba'zi ma'lumotlarning bir nechta egalariga ega bo'lish imkonini beradi, lekin u faqat ushbu ma'lumotlarga o'zgarmas ruxsat beradi. Agar sizda `RefCell<T>` ga ega `Rc<T>` boʻlsa, siz bir nechta egalari boʻlishi mumkin boʻlgan *va* siz o'zgartira oladigan qiymatni olishingiz mumkin!

Misol uchun, 15-18-sonli ro'yxatdagi kamchiliklar ro'yxati misolini eslang, bu ro'yxatda bir nechta ro'yxatlarga boshqa ro'yxat egaligini ulashishga ruxsat berish uchun `Rc<T>` dan foydalanganmiz. `Rc<T>` faqat oʻzgarmas qiymatlarga ega boʻlgani uchun biz ularni yaratganimizdan soʻng biz roʻyxatdagi qiymatlarni oʻzgartira olmaymiz. Ro'yxatlardagi qiymatlarni o'zgartirish imkoniyatiga ega bo'lish uchun `RefCell<T>` ni qo'shaylik. 15-24-ro'yxat shuni ko'rsatadiki, `Cons` ta'rifida `RefCell<T>` dan foydalanib, biz barcha ro'yxatlarda saqlangan qiymatni o'zgartirishimiz mumkin:

<span class="filename">src/main.rs nomli fayl:</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

<span class="caption">15-24-ro'yxat: `Rc<RefCell<i32>>` yordamida o'zgaruvchan `List` yaratish</span>

We create a value that is an instance of `Rc<RefCell<i32>>` and store it in a
variable named `value` so we can access it directly later. Then we create a
`List` in `a` with a `Cons` variant that holds `value`. We need to clone
`value` so both `a` and `value` have ownership of the inner `5` value rather
than transferring ownership from `value` to `a` or having `a` borrow from
`value`.

We wrap the list `a` in an `Rc<T>` so when we create lists `b` and `c`, they
can both refer to `a`, which is what we did in Listing 15-18.

After we’ve created the lists in `a`, `b`, and `c`, we want to add 10 to the
value in `value`. We do this by calling `borrow_mut` on `value`, which uses the
automatic dereferencing feature we discussed in Chapter 5 (see the section
[“Where’s the `->` Operator?”][wheres-the---operator]<!-- ignore -->) to
dereference the `Rc<T>` to the inner `RefCell<T>` value. The `borrow_mut`
method returns a `RefMut<T>` smart pointer, and we use the dereference operator
on it and change the inner value.

When we print `a`, `b`, and `c`, we can see that they all have the modified
value of 15 rather than 5:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

This technique is pretty neat! By using `RefCell<T>`, we have an outwardly
immutable `List` value. But we can use the methods on `RefCell<T>` that provide
access to its interior mutability so we can modify our data when we need to.
The runtime checks of the borrowing rules protect us from data races, and it’s
sometimes worth trading a bit of speed for this flexibility in our data
structures. Note that `RefCell<T>` does not work for multithreaded code!
`Mutex<T>` is the thread-safe version of `RefCell<T>` and we’ll discuss
`Mutex<T>` in Chapter 16.

[wheres-the---operator]: ch05-03-method-syntax.html#wheres-the---operator
