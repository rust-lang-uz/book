## Concurrencyda Shared-State

Message passing(Xabarni uzatish) - bu concurrencyni boshqarishning yaxshi usuli, ammo bu yagona emas. Yana bir usul bir nechta(multiple) threadlar bir xil umumiy ma'lumotlarga(shared data) kirishlari mumkin. Go tilidagi texnik hujjatlardagi shiorning ushbu qismini yana bir bor ko'rib chiqing: "xotirani almashish(sharing memory) orqali muloqot(comminicate) qilmang."

Xotirani almashish(sharing memory) orqali muloqot(comminication) qanday ko'rinishga ega bo'lar edi? Bundan tashqari, nima uchun message-passing enthusiastlar memory sharingdan foydalanmaslik haqida ogohlantiradilar?

Qaysidir ma'noda, har qanday dasturlash tilidagi kanallar bitta ownershiplik huquqiga o'xshaydi, chunki qiymatni kanalga o'tkazganingizdan so'ng, siz boshqa qiymatdan foydalanmasligingiz kerak. Shared memory concurrencyda bir nechta ownershiplik huquqiga o'xshaydi: concurrencyda bir nechta threadlar bir xil xotira joyiga(memory location) kirishi mumkin. 15-bobda ko'rganingizdek, smart pointerlar bir nechta ownershiplik qilish imkoniyatini yaratdi, bir nechta(multiple) ownershiplik murakkablikni oshirishi mumkin, chunki bu turli ownerlarni boshqarish kerak. Rust type tizimi va ownershiplik qoidalari ushbu boshqaruvni to'g'ri bajarishga katta yordam beradi. Misol uchun, shared memory uchun eng keng tarqalgan concurrency primitivlaridan biri bo'lgan mutexlarni ko'rib chiqaylik.

### Bir vaqtning o'zida bitta threaddan ma'lumotlarga kirishga ruxsat berish uchun mutexlardan foydalanish

*Mutex* bu *mutual exclusion* ning qisqartmasi boʻlib, mutex istalgan vaqtda baʼzi maʼlumotlarga faqat bitta threadga kirish imkonini beradi. Mutexdagi ma'lumotlarga kirish uchun thread birinchi navbatda mutexning *lock(qulf)*ni olishni so'rab kirishni xohlashini bildirishi kerak. Lock(qulf) - bu mutexning bir qismi bo'lgan ma'lumotlar tuzilmasi bo'lib, u hozirda ma'lumotlarga kimning eksklyuziv kirish huquqiga ega ekanligini kuzatib boradi. Shuning uchun, mutex qulflash tizimi(locking system) orqali o'zida mavjud bo'lgan ma'lumotlarni *himoya qilish(guarding)* sifatida tavsiflanadi.

Mutexlardan foydalanish qiyinligi bilan mashhur, chunki siz ikkita qoidani eslab qolishingiz kerak:

* Ma'lumotlardan foydalanishdan oldin siz qulfni olishga harakat qilishingiz kerak.
* Mutex himoya qiladigan ma'lumotlar bilan ishlashni tugatgandan so'ng, boshqa threadlar qulfni(lock) olishi uchun ma'lumotlarni qulfdan chiqarishingiz(unlock) kerak.

Mutexni tushunish uchun bitta mikrofon bilan konferensiyada guruh muhokamasining haqiqiy hayotiy misolini tasavvur qiling. Panel ishtirokchisi gapirishdan oldin mikrofondan foydalanishni xohlashini so'rashi yoki signal berishi kerak. Mikrofonni olishganda, ular xohlagancha gaplashishi mumkin va keyin mikrofonni gapirishni so'ragan keyingi ishtirokchiga beradi. Agar panel ishtirokchisi mikrofon bilan ishlashni tugatgandan so'ng uni o'chirishni unutib qo'ysa, boshqa hech kim gapira olmaydi. Agar umumiy mikrofonni boshqarish noto'g'ri bo'lsa, panel rejalashtirilganidek ishlamaydi!

Mutexlarni boshqarish juda qiyin bo'lishi mumkin, shuning uchun ko'p odamlar kanallarga(channel) ishtiyoq bilan qarashadi. Biroq, Rust type tizimi va ownershiplik qoidalari tufayli siz qulflash(locking) va qulfni noto'g'ri ochishingiz(unlocking) mumkin emas.

#### `Mutex<T>` API

Mutexdan qanday foydalanishga misol sifatida, keling, 16-12 ro'yxatda ko'rsatilganidek, bitta threadli kontekstda mutexdan foydalanishdan boshlaylik:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

<span class="caption">Ro'yxat 16-12: `Mutex<T>` API-ni soddaligi uchun single-threadli kontekstda oʻrganish</span>

Ko'pgina turlarda(type) bo'lgani kabi, biz bog'langan `new` funksiyasidan foydalangan holda `Mutex<T>` ni yaratamiz.
Mutex ichidagi ma'lumotlarga kirish uchun biz qulfni olish uchun `lock` metodidan foydalanamiz. Bu chaqiruv joriy threadni bloklaydi, shuning uchun u bizni qulflash navbati kelmaguncha hech qanday ishni bajara olmaydi.

Qulfni ushlab turgan boshqa thread panic qo'zg'atsa, `lock` chaqiruvi muvaffaqiyatsiz bo'ladi. Bunday holda, hech kim qulfni qo'lga kirita olmaydi, shuning uchun biz  `unwrap`ni tanladik va agar shunday vaziyatda bo'lsak, bu threadni panic qo'yishni tanladik.

Qulfni qo'lga kiritganimizdan so'ng, biz bu holatda `num` deb nomlangan return qiymatini ichidagi ma'lumotlarga o'zgaruvchan reference sifatida ko'rib chiqishimiz mumkin. Tur(type) tizimi `m` dagi qiymatni ishlatishdan oldin qulfni olishimizni ta'minlaydi. `m` turi `i32` emas, `Mutex<i32>`, shuning uchun biz `i32` qiymatidan foydalanish uchun `lock`ni chaqirishimiz kerak. Biz unuta olmaymiz; aks holda turdagi tizim bizga ichki `i32` ga kirishga ruxsat bermaydi.

As you might suspect, `Mutex<T>` is a smart pointer. More accurately, the call
to `lock` *returns* a smart pointer called `MutexGuard`, wrapped in a
`LockResult` that we handled with the call to `unwrap`. The `MutexGuard` smart
pointer implements `Deref` to point at our inner data; the smart pointer also
has a `Drop` implementation that releases the lock automatically when a
`MutexGuard` goes out of scope, which happens at the end of the inner scope. As
a result, we don’t risk forgetting to release the lock and blocking the mutex
from being used by other threads, because the lock release happens
automatically.

After dropping the lock, we can print the mutex value and see that we were able
to change the inner `i32` to 6.

#### Sharing a `Mutex<T>` Between Multiple Threads

Now, let’s try to share a value between multiple threads using `Mutex<T>`.
We’ll spin up 10 threads and have them each increment a counter value by 1, so
the counter goes from 0 to 10. The next example in Listing 16-13 will have
a compiler error, and we’ll use that error to learn more about using
`Mutex<T>` and how Rust helps us use it correctly.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

<span class="caption">Listing 16-13: Ten threads each increment a counter
guarded by a `Mutex<T>`</span>

We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we did
in Listing 16-12. Next, we create 10 threads by iterating over a range of
numbers. We use `thread::spawn` and give all the threads the same closure: one
that moves the counter into the thread, acquires a lock on the `Mutex<T>` by
calling the `lock` method, and then adds 1 to the value in the mutex. When a
thread finishes running its closure, `num` will go out of scope and release the
lock so another thread can acquire it.

In the main thread, we collect all the join handles. Then, as we did in Listing
16-2, we call `join` on each handle to make sure all the threads finish. At
that point, the main thread will acquire the lock and print the result of this
program.

We hinted that this example wouldn’t compile. Now let’s find out why!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

The error message states that the `counter` value was moved in the previous
iteration of the loop. Rust is telling us that we can’t move the ownership
of lock `counter` into multiple threads. Let’s fix the compiler error with a
multiple-ownership method we discussed in Chapter 15.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we gave a value multiple owners by using the smart pointer
`Rc<T>` to create a reference counted value. Let’s do the same here and see
what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing 16-14 and clone
the `Rc<T>` before moving ownership to the thread.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

<span class="caption">Listing 16-14: Attempting to use `Rc<T>` to allow
multiple threads to own the `Mutex<T>`</span>

Once again, we compile and get... different errors! The compiler is teaching us
a lot.

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Wow, that error message is very wordy! Here’s the important part to focus on:
`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. The compiler is
also telling us the reason why: ``the trait `Send` is not implemented for
`Rc<Mutex<i32>>` ``. We’ll talk about `Send` in the next section: it’s one of
the traits that ensures the types we use with threads are meant for use in
concurrent situations.

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped. But it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts—subtle bugs that
could in turn lead to memory leaks or a value being dropped before we’re done
with it. What we need is a type exactly like `Rc<T>` but one that makes changes
to the reference count in a thread-safe way.

#### Atomic Reference Counting with `Arc<T>`

Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in
concurrent situations. The *a* stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here: see the standard library
documentation for [`std::sync::atomic`][atomic]<!-- ignore --> for more
details. At this point, you just need to know that atomics work like primitive
types but are safe to share across threads.

You might then wonder why all primitive types aren’t atomic and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re just performing operations on values within a
single thread, your code can run faster if it doesn’t have to enforce the
guarantees atomics provide.

Let’s return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code in Listing 16-15 will finally compile and run:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

<span class="caption">Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>`
to be able to share ownership across multiple threads</span>

This code will print the following:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety. You could also use this
program’s structure to do more complicated operations than just incrementing a
counter. Using this strategy, you can divide a calculation into independent
parts, split those parts across threads, and then use a `Mutex<T>` to have each
thread update the final result with its part.

Note that if you are doing simple numerical operations, there are types simpler
than `Mutex<T>` types provided by the [`std::sync::atomic` module of the
standard library][atomic]<!-- ignore -->. These types provide safe, concurrent,
atomic access to primitive types. We chose to use `Mutex<T>` with a primitive
type for this example so we could concentrate on how `Mutex<T>` works.

### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

You might have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in
Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>`
to mutate contents inside an `Arc<T>`.

Another detail to note is that Rust can’t protect you from all kinds of logic
errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of
creating *deadlocks*. These occur when an operation needs to lock two resources
and two threads have each acquired one of the locks, causing them to wait for
each other forever. If you’re interested in deadlocks, try creating a Rust
program that has a deadlock; then research deadlock mitigation strategies for
mutexes in any language and have a go at implementing them in Rust. The
standard library API documentation for `Mutex<T>` and `MutexGuard` offers
useful information.

We’ll round out this chapter by talking about the `Send` and `Sync` traits and
how we can use them with custom types.

[atomic]: ../std/sync/atomic/index.html
