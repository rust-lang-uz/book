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

In the test, we’re testing what happens when the `LimitTracker` is told to set
`value` to something that is more than 75 percent of the `max` value. First, we
create a new `MockMessenger`, which will start with an empty list of messages.
Then we create a new `LimitTracker` and give it a reference to the new
`MockMessenger` and a `max` value of 100. We call the `set_value` method on the
`LimitTracker` with a value of 80, which is more than 75 percent of 100. Then
we assert that the list of messages that the `MockMessenger` is keeping track
of should now have one message in it.

However, there’s one problem with this test, as shown here:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

We can’t modify the `MockMessenger` to keep track of the messages, because the
`send` method takes an immutable reference to `self`. We also can’t take the
suggestion from the error text to use `&mut self` instead, because then the
signature of `send` wouldn’t match the signature in the `Messenger` trait
definition (feel free to try and see what error message you get).

This is a situation in which interior mutability can help! We’ll store the
`sent_messages` within a `RefCell<T>`, and then the `send` method will be
able to modify `sent_messages` to store the messages we’ve seen. Listing 15-22
shows what that looks like:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

<span class="caption">Listing 15-22: Using `RefCell<T>` to mutate an inner
value while the outer value is considered immutable</span>

The `sent_messages` field is now of type `RefCell<Vec<String>>` instead of
`Vec<String>`. In the `new` function, we create a new `RefCell<Vec<String>>`
instance around the empty vector.

For the implementation of the `send` method, the first parameter is still an
immutable borrow of `self`, which matches the trait definition. We call
`borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a
mutable reference to the value inside the `RefCell<Vec<String>>`, which is the
vector. Then we can call `push` on the mutable reference to the vector to keep
track of the messages sent during the test.

The last change we have to make is in the assertion: to see how many items are
in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an
immutable reference to the vector.

Now that you’ve seen how to use `RefCell<T>`, let’s dig into how it works!

#### Keeping Track of Borrows at Runtime with `RefCell<T>`

When creating immutable and mutable references, we use the `&` and `&mut`
syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut`
methods, which are part of the safe API that belongs to `RefCell<T>`. The
`borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut`
returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we
can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart
pointers are currently active. Every time we call `borrow`, the `RefCell<T>`
increases its count of how many immutable borrows are active. When a `Ref<T>`
value goes out of scope, the count of immutable borrows goes down by one. Just
like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable
borrows or one mutable borrow at any point in time.

If we try to violate these rules, rather than getting a compiler error as we
would with references, the implementation of `RefCell<T>` will panic at
runtime. Listing 15-23 shows a modification of the implementation of `send` in
Listing 15-22. We’re deliberately trying to create two mutable borrows active
for the same scope to illustrate that `RefCell<T>` prevents us from doing this
at runtime.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

<span class="caption">Listing 15-23: Creating two mutable references in the
same scope to see that `RefCell<T>` will panic</span>

We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned
from `borrow_mut`. Then we create another mutable borrow in the same way in the
variable `two_borrow`. This makes two mutable references in the same scope,
which isn’t allowed. When we run the tests for our library, the code in Listing
15-23 will compile without any errors, but the test will fail:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Notice that the code panicked with the message `already borrowed:
BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing
rules at runtime.

Choosing to catch borrowing errors at runtime rather than compile time, as
we’ve done here, means you’d potentially be finding mistakes in your code later
in the development process: possibly not until your code was deployed to
production. Also, your code would incur a small runtime performance penalty as
a result of keeping track of the borrows at runtime rather than compile time.
However, using `RefCell<T>` makes it possible to write a mock object that can
modify itself to keep track of the messages it has seen while you’re using it
in a context where only immutable values are allowed. You can use `RefCell<T>`
despite its trade-offs to get more functionality than regular references
provide.

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that
`Rc<T>` lets you have multiple owners of some data, but it only gives immutable
access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can
get a value that can have multiple owners *and* that you can mutate!

For example, recall the cons list example in Listing 15-18 where we used
`Rc<T>` to allow multiple lists to share ownership of another list. Because
`Rc<T>` holds only immutable values, we can’t change any of the values in the
list once we’ve created them. Let’s add in `RefCell<T>` to gain the ability to
change the values in the lists. Listing 15-24 shows that by using a
`RefCell<T>` in the `Cons` definition, we can modify the value stored in all
the lists:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

<span class="caption">Listing 15-24: Using `Rc<RefCell<i32>>` to create a
`List` that we can mutate</span>

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
