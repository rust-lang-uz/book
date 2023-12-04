# Fearless Concurrency

Bir vaqtning o'zida dasturlashni(concurrent programming) xavfsiz va samarali boshqarish Rustning asosiy maqsadlaridan biridir. Dasturning turli qismlari mustaqil ravishda bajariladigan(execute) *concurrent programming* va dasturning turli qismlari bir vaqtning o'zida bajariladigan *parallel dasturlash* ko'proq kompyuterlar o'zlarining bir nechta protsessorlaridan foydalanishlari sababli tobora muhim ahamiyat kasb etmoqda. Tarixiy jihatdan, ushbu kontekstlarda dasturlash qiyin va xatolarga moyil bo'lgan: Rust buni o'zgartirishga umid qilmoqda.

Dastlab, Rust jamoasi xotira xavfsizligini ta'minlash va parallel muammolarning oldini olish turli usullar bilan hal qilinishi kerak bo'lgan ikkita alohida muammo deb o'ylagan. Vaqt o'tishi bilan jamoa egalik(ownership) va turdagi tizimlar(type system) xotira xavfsizligi *va* parallellik muammolarini boshqarishga yordam beradigan kuchli vositalar to'plami ekanligini aniqladi! Ownership(egalik) va turlarni tekshirishdan(type checking) foydalangan holda, ko'plab parallellik xatolar runtimedagi xatolardan ko'ra Rustda kompilyatsiya vaqtidagi xatolardir. Shuning uchun, runtime bilan bir vaqtda xatolik yuzaga kelgan aniq holatlarni takrorlash uchun ko'p vaqt sarflashdan ko'ra, noto'g'ri kod kompilyatsiya qilishni rad etadi va muammoni tushuntiruvchi xatoni taqdim etadi. Natijada, siz kodingizni ishlab chiqarishga(production) yuborilgandan keyin emas, balki uning ustida ishlayotganingizda tuzatishingiz mumkin. Biz Rustning bu jihatini *fearless* *concurrency* deb nomladik. Fearless concurrency sizga nozik xatolarsiz kod yozish imkonini beradi va yangi xatolarni kiritmasdan qayta tiklash oson.

> Note: For simplicity’s sake, we’ll refer to many of the problems as
> *concurrent* rather than being more precise by saying *concurrent and/or
> parallel*. If this book were about concurrency and/or parallelism, we’d be
> more specific. For this chapter, please mentally substitute *concurrent
> and/or parallel* whenever we use *concurrent*.

Many languages are dogmatic about the solutions they offer for handling
concurrent problems. For example, Erlang has elegant functionality for
message-passing concurrency but has only obscure ways to share state between
threads. Supporting only a subset of possible solutions is a reasonable
strategy for higher-level languages, because a higher-level language promises
benefits from giving up some control to gain abstractions. However, lower-level
languages are expected to provide the solution with the best performance in any
given situation and have fewer abstractions over the hardware. Therefore, Rust
offers a variety of tools for modeling problems in whatever way is appropriate
for your situation and requirements.

Here are the topics we’ll cover in this chapter:

* How to create threads to run multiple pieces of code at the same time
* *Message-passing* concurrency, where channels send messages between threads
* *Shared-state* concurrency, where multiple threads have access to some piece
  of data
* The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to
  user-defined types as well as types provided by the standard library
