## Kodni bir vaqtning o'zida ishga tushirish uchun threadlardan foydalanish

Kodni bir vaqtning o'zida ishga tushirish(simultaneously) uchun threadlardan foydalanish hozirgi operatsion tizimlarning ko'pchiligida bajarilgan(execute) dastur kodi *process* ishga tushiriladi va operatsion tizim bir vaqtning o'zida bir nechta processlarni boshqaradi.Dastur doirasida siz bir vaqtning o'zida ishlaydigan(simultaneously) mustaqil qismlarga(independent part) ham ega bo'lishingiz mumkin. Ushbu mustaqil qismlarni boshqaradigan xususiyatlar *threadlar* deb ataladi. Masalan, veb-server bir vaqtning o'zida bir nechta so'rovlarga(requestlar) javob berishi uchun bir nechta(multiple) threadlarga ega bo'lishi mumkin.

Bir vaqtning o'zida bir nechta vazifalarni(multiple task) bajarish uchun dasturingizdagi hisoblashni bir nechta(multiple) threadlarga bo'lish unumdorlikni oshirishi mumkin, ammo bu murakkablikni ham oshiradi.
Theredlar bir vaqtning o'zida(simultaneously) ishlashi mumkinligi sababli, kodingizning turli xil ish threadlaridagi qismlari qaysi tartibda ishlashi haqida hech qanday kafolat yo'q. Bu muammolarga olib kelishi mumkin, masalan:

* Race conditionlari, bu yerda threadlar ma'lumotlar yoki resurslarga mos kelmaydigan tartibda kirishadi
* Deadlock, bu yerda ikkita thread bir-birini kutib, ikkala threadning davom etishiga to'sqinlik qiladi
* Faqat ma'lum holatlarda yuzaga keladigan va qayta ishlab chiqarish va ishonchli tarzda tuzatish qiyin bo'lgan xatolar

Rust threadlardan foydalanishning salbiy ta'sirini yumshatishga harakat qiladi, lekin multithreadli kontekstda dasturlash hali ham ehtiyotkorlik bilan o'ylashni talab qiladi va bitta threadda ishlaydigan dasturlardan farq qiladigan kod tuzilishini talab qiladi.

Dasturlash tillari threadlarni turli yo'llar bilan amalga oshiradi(impelement qiladi) va ko'pgina operatsion tizimlar yangi threadlarni yaratish uchun til chaqirishi mumkin bo'lgan API-ni taqdim etadi. Rust standart kutubxonasi(standard library) *1:1* threadni amalga oshirish modelidan foydalanadi, bunda dastur har bir til uchun bitta operatsion tizim threadidan foydalanadi. 1:1 modeliga turli xil o'zgarishlarni keltirib chiqaradigan boshqa theredlar modellarini amalga oshiradigan(implement qiladigan) cratelar mavjud.

### `spawn` yordamida yangi thread yaratish

Yangi thread yaratish uchun biz `thread::spawn` funksiyasini chaqiramiz va unga yangi threadda ishga tushirmoqchi bo'lgan kodni o'z ichiga olgan closureni (biz 13-bobda closurelar haqida gapirgan edik) o'tkazamiz. 16-1 ro'yxatdagi misol asosiy(main) threaddagi ba'zi matnni va yangi threaddagi boshqa matnni chop etadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

<span class="caption">Ro'yxat 16-1: Bir narsani chop etish uchun yangi thread yaratish, main thread esa boshqa narsalarni chop etish</span>

Esda tutingki, Rust dasturining asosiy ishi tugagach, barcha ochilgan threadlar ishlashni tugatgan yoki tugatmaganidan qat'i nazar, o'chiriladi. Ushbu dasturning chiqishi har safar bir oz boshqacha bo'lishi mumkin, ammo u quyidagilarga o'xshaydi:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
salom, main threaddan 1-raqam!
salom, ochilgan threaddan 1-raqam!
salom, main threaddan 2-raqam!
salom, ochilgan threaddan 2-raqam!
salom, main threaddan 3-raqam!
salom, ochilgan threaddan 3-raqam!
salom, main threaddan 4-raqam!
salom, ochilgan threaddan 4-raqam!
salom, ochilgan threaddan 5-raqam!
```

`thread::sleep`ga chaqiruvlar threadni qisqa muddatga uning bajarilishini to'xtatishga majbur qiladi, bu esa boshqa threadning ishlashiga imkon beradi. Ehtimol, threadlar navbatma-navbat bo'ladi, lekin bu kafolatlanmaydi: bu sizning operatsion tizimingiz threadlarni qanday rejalashtirishiga(schedule) bog'liq. Bu ishga tushirishda birinchi bo'lib main thread chop etiladi, garchi ishlab chiqarilgan threadning chop etish bayonoti kodda birinchi bo'lib paydo bo'lsa ham. Va biz paydo bo'lgan thredga `i` 9 bo'lguncha chop etishni aytgan bo'lsak ham, asosiy thread yopilishidan oldin u 5 ga yetdi.

Agar siz ushbu kodni ishga tushirsangiz va faqat main threaddan olingan ma'lumotlarni ko'rsangiz yoki hech qanday o'xshashlikni ko'rmasangiz, operatsion tizimning threadlar o'rtasida almashishi uchun ko'proq imkoniyatlar yaratish uchun diapazonlardagi raqamlarni oshirib ko'ring.

### `join` handlerlari yordamida barcha threadlar tugashini kutilmoqda

16-1 ro'yxatidagi kod ko'pincha main thread tugashi tufayli paydo bo'lgan threadni muddatidan oldin to'xtatibgina qolmay, balki threadlarning ishlash tartibiga kafolat yo'qligi sababli, biz ham yangi ochilgangan threadning umuman ishga tushishiga kafolat bera olmaymiz!

O'zgaruvchida `thread::spawn` ning qaytish(return) qiymatini saqlash orqali ochilgangan threadning ishlamasligi yoki muddatidan oldin tugashi muammosini hal qilishimiz mumkin. `thread::spawn` ning qaytish turi(return type) `JoinHandle` dir. `JoinHandle` - bu tegishli qiymat bo'lib, biz `join` metodini chaqirganimizda, uning threadi tugashini kutamiz. 16-2 ro'yxatda biz 16-1 ro'yxatda yaratgan `JoinHandle` dan qanday foydalanish va `join`ni chaqirish orqali yaratilgan thread `main` chiqishdan oldin tugashini ko'rsatadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

<span class="caption">Roʻyxat 16-2: `JoinHandle` ni `thread::spawn` dan saqlash, threadning oxirigacha ishga tushishini kafolatlash</span>

Handleda `join`ni chaqirish, handle bilan ifodalangan thread tugaguncha ishlayotgan threadni bloklaydi. Threadni *bloklash* uning ish bajarishi yoki chiqishining oldini olish degani. Biz chaqiruvni(call) main threadning `foor` loop siklidan keyin qo'yganimiz sababli, 16-2 ro'yxatini ishga tushirish shunga o'xshash natijani berishi kerak:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
salom, main threaddan 1-raqam!
salom, main threaddan 2-raqam!
salom, ochilgan threaddan 1-raqam!
salom, main threaddan 3-raqam!
salom, ochilgan threaddan 2-raqam!
salom, main threaddan 4-raqam!
salom, ochilgan threaddan 3-raqam!
salom, ochilgan threaddan 4-raqam!
salom, ochilgan threaddan 5-raqam!
salom, ochilgan threaddan 6-raqam!
salom, ochilgan threaddan 7-raqam!
salom, ochilgan threaddan 8-raqam!
salom, ochilgan threaddan 9-raqam!
```

Ikki thread almashishda davom etadi, lekin main thread `handle.join()` chaqiruvi tufayli kutadi va hosil qilingan thread tugamaguncha tugamaydi.

Ammo keling, `main` da `for` loop siklidan oldin `handle.join()` ni ko‘chirsak nima bo‘lishini ko‘rib chiqamiz, masalan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

Main thread ochilgan thread tugashini kutadi va keyin `for` loop siklini ishga tushiradi, shuning uchun bu yerda ko'rsatilganidek, chiqish boshqa qo'shilmaydi:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
salom, ochilgan threaddan 1-raqam!
salom, ochilgan threaddan 2-raqam!
salom, ochilgan threaddan 3-raqam!
salom, ochilgan threaddan 4-raqam!
salom, ochilgan threaddan 5-raqam!
salom, ochilgan threaddan 6-raqam!
salom, ochilgan threaddan 7-raqam!
salom, ochilgan threaddan 8-raqam!
salom, ochilgan threaddan 9-raqam!
salom, main threaddan 1-raqam!
salom, main threaddan 2-raqam!
salom, main threaddan 3-raqam!
salom, main threaddan 4-raqam!
```

Kichik tafsilotlar(detail), masalan, `join` deb ataladigan joy, sizning threadlaringiz bir vaqtning o'zida ishlashi yoki ishlamasligiga ta'sir qilishi mumkin.

### Using `move` Closures with Threads

We'll often use the `move` keyword with closures passed to `thread::spawn`
because the closure will then take ownership of the values it uses from the
environment, thus transferring ownership of those values from one thread to
another. In the [“Capturing References or Moving Ownership”][capture]<!-- ignore
--> section of Chapter 13, we discussed `move` in the context of closures. Now,
we’ll concentrate more on the interaction between `move` and `thread::spawn`.

Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. To use data from the main thread in the spawned thread, the
spawned thread’s closure must capture the values it needs. Listing 16-3 shows
an attempt to create a vector in the main thread and use it in the spawned
thread. However, this won’t yet work, as you’ll see in a moment.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

<span class="caption">Listing 16-3: Attempting to use a vector created by the
main thread in another thread</span>

The closure uses `v`, so it will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread. But when we compile this
example, we get the following error:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust *infers* how to capture `v`, and because `println!` only needs a reference
to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t
tell how long the spawned thread will run, so it doesn’t know if the reference
to `v` will always be valid.

Listing 16-4 provides a scenario that’s more likely to have a reference to `v`
that won’t be valid:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

<span class="caption">Listing 16-4: A thread with a closure that attempts to
capture a reference to `v` from a main thread that drops `v`</span>

If Rust allowed us to run this code, there’s a possibility the spawned thread
would be immediately put in the background without running at all. The spawned
thread has a reference to `v` inside, but the main thread immediately drops
`v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!

To fix the compiler error in Listing 16-3, we can use the error message’s
advice:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using rather than allowing Rust to infer that it
should borrow the values. The modification to Listing 16-3 shown in Listing
16-5 will compile and run as we intend:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

<span class="caption">Listing 16-5: Using the `move` keyword to force a closure
to take ownership of the values it uses</span>

We might be tempted to try the same thing to fix the code in Listing 16-4 where
the main thread called `drop` by using a `move` closure. However, this fix will
not work because what Listing 16-4 is trying to do is disallowed for a
different reason. If we added `move` to the closure, we would move `v` into the
closure’s environment, and we could no longer call `drop` on it in the main
thread. We would get this compiler error instead:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust’s ownership rules have saved us again! We got an error from the code in
Listing 16-3 because Rust was being conservative and only borrowing `v` for the
thread, which meant the main thread could theoretically invalidate the spawned
thread’s reference. By telling Rust to move ownership of `v` to the spawned
thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If
we change Listing 16-4 in the same way, we’re then violating the ownership
rules when we try to use `v` in the main thread. The `move` keyword overrides
Rust’s conservative default of borrowing; it doesn’t let us violate the
ownership rules.

With a basic understanding of threads and the thread API, let’s look at what we
can *do* with threads.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
