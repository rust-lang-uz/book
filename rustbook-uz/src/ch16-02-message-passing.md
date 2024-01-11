## Threadlar orasidagi ma'lumotlarni uzatish uchun Message Passing(xabar uzatish)dan foydalanish

Xavfsiz concurrencyni ta'minlashning tobora ommalashib borayotgan yondashuvlaridan biri bu *message passing* bo'lib, bu yerda threadlar yoki actorlar bir-biriga ma'lumotlarni o'z ichiga olgan xabarlarni yuborish orqali muloqot qilishadi. [Go tili texnik hujjatlaridagi](https://golang.org/doc/effective_go.html#concurrency) shiordagi g‘oya: “Xotirani almashish(share) orqali muloqot qilmang; Buning o'rniga, muloqot(communication) orqali xotirani share qiling."

Message-sending(xabar yuborish) concurrencyni amalga oshirish uchun Rustning standart kutubxonasi *channels* amalga oshirishni ta'minlaydi. Channel(kanal) - bu umumiy dasturlash tushunchasi bo'lib, uning yordamida ma'lumotlar bir threaddan ikkinchisiga yuboriladi.

Dasturlashdagi kanalni(channel) oqim yoki daryo kabi suvning yo'naltirilgan kanali kabi tasavvur qilishingiz mumkin. Agar siz daryoga rezina o'rdak kabi narsalarni qo'ysangiz, u suv yo'lining oxirigacha pastga tushadi.

Kanalning ikkita yarmi bor: uzatuvchi(transmitte) va qabul qiluvchi(receiver). Transmitterning yarmi daryoga rezina o'rdak qo'yadigan yuqori oqim joyidir va qabul qiluvchining yarmi rezina o'rdak quyi oqimga tushadi. Kodingizning bir qismi siz yubormoqchi bo'lgan ma'lumotlarga ega bo'lgan transmitterdagi metodlarni chaqiradi, boshqa qismi esa kelgan xabarlarni qabul qiluvchi tomonni tekshiradi. Agar transmitter yoki receiverning yarmi tushib qolsa, kanal *closed(yopiq)* deyiladi.

Bu yerda biz qiymatlarni yaratish va ularni kanalga yuborish uchun bitta threadga ega bo'lgan dasturni va qiymatlarni qabul qiladigan va ularni chop etadigan boshqa threadni ishlab chiqamiz. Featureni tasvirlash uchun kanal yordamida  threadlar orasidagi oddiy qiymatlarni yuboramiz. Texnika bilan tanishganingizdan so'ng, siz bir-biringiz bilan aloqa o'rnatishingiz kerak bo'lgan har qanday threadlar uchun kanallardan foydalanishingiz mumkin, masalan, chat tizimi yoki ko'p threadlar hisoblash qismlarini bajaradigan va qismlarni natijalarni jamlaydigan bitta threadga yuboradigan tizim.

Birinchidan, 16-6 ro'yxatda biz channel(kanal) yaratamiz, lekin u bilan hech narsa qilmaymiz.
E'tibor bering, bu hali kompilyatsiya qilinmaydi, chunki Rust kanal orqali qanday turdagi qiymatlarni yuborishimizni ayta olmaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">Ro'yxat 16-6: Kanal yaratish va ikkita yarmini `tx` va `rx` ga belgilash</span>

Biz `mpsc::channel` funksiyasidan foydalanib yangi kanal yaratamiz; `mpsc` *multiple producer, single consumer* degan maʼnoni anglatadi. Qisqa qilib aytganda, Rustning standart kutubxonasi kanallarni implement qilish usuli kanalda qiymatlarni ishlab chiqaradigan bir nechta *sending(jo'natish)* uchlari bo'lishi mumkin, ammo bu qiymatlarni qabul qiladigan consumer faqat bitta *receiving(qabul qiluvchi)* end bo'lishi mumkinligini anglatadi. Tasavvur qiling-a, bir nechta daryolar birlashib, bitta katta daryoga quyiladi: har qanday oqim oxirida bitta daryoga tushadi. Hozircha biz bitta ishlab chiqaruvchidan(single producer) boshlaymiz, lekin biz ushbu misol ishlaganda bir nechta producerlarni(multiple producer) qo'shamiz.

`mpsc::channel` funksiyasi tupleni qaytaradi, uning birinchi elementi jo'natuvchi end - transmitter va ikkinchi element - receiver end - qabul qiluvchidir. `tx` va `rx` qisqartmalari an'anaviy ravishda ko'plab sohalarda mos ravishda *transmitter* va *receiver* uchun ishlatiladi, shuning uchun biz har bir endni ko'rsatish uchun o'zgaruvchilarimizni shunday nomlaymiz. Biz tuplelarni destruksiya pattern `let` statementdan foydalanmoqdamiz; Biz 18-bobda `let` statementlarida patternlardan foydalanish va destruksiyani muhokama qilamiz. Hozircha shuni bilingki, `let` statementdan shu tarzda foydalanish `mpsc::channel` tomonidan qaytarilgan tuple qismlarini ajratib olishning qulay usuli hisoblanadi.

16-7 ro'yxatda ko'rsatilganidek, transmitter uchini ochilgan threadga o'tkazamiz va u bitta threadni yuborsin, shunda hosil qilingan thread main thread bilan bog'lanadi. Bu daryoning yuqori oqimiga rezina o'rdak qo'yish yoki bir threaddan ikkinchisiga chat xabarini yuborishga o'xshaydi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<span class="caption">Roʻyxat 16-7: `tx` ni ochilgan threadga koʻchirish va `salom` yuborish</span>

Shunga qaramay, biz yangi thread yaratish uchun `thread::spawn` dan foydalanamiz va keyin `move` yordamida `tx` ni yopishga(close) o'tkazamiz, shunda hosil qilingan thread `tx`ga ega bo'ladi. Kanal orqali xabarlarni jo'natish uchun ochilgan thread transmitterga ega bo'lishi kerak. Transmitterda biz jo'natmoqchi bo'lgan qiymatni qabul qiluvchi `send` metodi mavjud. `send` metodi `Result<T, E>` typeni qaytaradi, shuning uchun agar qabul qiluvchi(receiver) allaqachon drop qilingan bo'lsa va qiymatni yuborish uchun joy bo'lmasa, yuborish operatsiyasi xatolikni qaytaradi. Ushbu misolda biz xatolik yuz berganda panic qo'yish uchun `unwrap` ni chaqiramiz. Ammo haqiqiy dasturda biz uni to'g'ri hal qilamiz: xatolarni to'g'ri hal qilish strategiyalarini ko'rib chiqish uchun 9-bobga qayting.

16-8 ro'yxatda biz main threaddagi qabul qiluvchidan(receive) qiymatni olamiz. Bu daryoning oxiridagi suvdan rezina o'rdakni olish yoki suhbat xabarini olish kabi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<span class="caption">Ro'yxat 16-8: main threadda `salom` qiymatini olish va uni chop etish</span>

Receiverda ikkita foydali metod mavjud: `recv` va `try_recv`. Biz `recv` dan foydalanmoqdamiz, bu *receive(qabul qilish)* ning qisqartmasi bo'lib, u main threadning bajarilishini bloklaydi va kanalga qiymat yuborilguncha kutadi. Qiymat yuborilgach, `recv` uni `Result<T, E>` shaklida qaytaradi. Transmitter yopilganda, `recv` boshqa qiymatlar kelmasligini bildirish uchun xatolikni qaytaradi.

`try_recv` metodi bloklanmaydi, aksincha darhol `Result<T, E>`ni qaytaradi: `Ok` qiymati, agar mavjud bo‘lsa, xabarni ushlab turadi va bu safar hech qanday xabar bo‘lmasa, `Err` qiymati. `try_recv` dan foydalanish, agar ushbu thread xabarlarni kutayotganda boshqa ishi boʻlsa foydali boʻladi: biz tez-tez `try_recv` ni chaqiradigan, agar mavjud bo'lsa, xabarni ko'rib chiqadigan va boshqasi qayta tekshirilgunga qadar biroz vaqt ishlaydigan sikl yozishimiz mumkin.

Biz ushbu misolda soddalik uchun `recv` dan foydalandik; bizda main thread uchun xabarlarni kutishdan boshqa ishimiz yo'q, shuning uchun main threadi bloklash maqsadga muvofiqdir.

Kodni 16-8 ro'yxatda ishga tushirganimizda, biz main threaddan chop etilgan qiymatni ko'ramiz:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Tushundim: salom
```

Mukammal! Perfect!

### Channels and Ownership Transference

The ownership rules play a vital role in message sending because they help you
write safe, concurrent code. Preventing errors in concurrent programming is the
advantage of thinking about ownership throughout your Rust programs. Let’s do
an experiment to show how channels and ownership work together to prevent
problems: we’ll try to use a `val` value in the spawned thread *after* we’ve
sent it down the channel. Try compiling the code in Listing 16-9 to see why
this code isn’t allowed:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<span class="caption">Listing 16-9: Attempting to use `val` after we’ve sent it
down the channel</span>

Here, we try to print `val` after we’ve sent it down the channel via `tx.send`.
Allowing this would be a bad idea: once the value has been sent to another
thread, that thread could modify or drop it before we try to use the value
again. Potentially, the other thread’s modifications could cause errors or
unexpected results due to inconsistent or nonexistent data. However, Rust gives
us an error if we try to compile the code in Listing 16-9:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Our concurrency mistake has caused a compile time error. The `send` function
takes ownership of its parameter, and when the value is moved, the receiver
takes ownership of it. This stops us from accidentally using the value again
after sending it; the ownership system checks that everything is okay.

### Sending Multiple Values and Seeing the Receiver Waiting

The code in Listing 16-8 compiled and ran, but it didn’t clearly show us that
two separate threads were talking to each other over the channel. In Listing
16-10 we’ve made some modifications that will prove the code in Listing 16-8 is
running concurrently: the spawned thread will now send multiple messages and
pause for a second between each message.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<span class="caption">Listing 16-10: Sending multiple messages and pausing
between each</span>

This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the `thread::sleep` function with a `Duration` value of
1 second.

In the main thread, we’re not calling the `recv` function explicitly anymore:
instead, we’re treating `rx` as an iterator. For each value received, we’re
printing it. When the channel is closed, iteration will end.

When running the code in Listing 16-10, you should see the following output
with a 1-second pause in between each line:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

Because we don’t have any code that pauses or delays in the `for` loop in the
main thread, we can tell that the main thread is waiting to receive values from
the spawned thread.

### Creating Multiple Producers by Cloning the Transmitter

Earlier we mentioned that `mpsc` was an acronym for *multiple producer,
single consumer*. Let’s put `mpsc` to use and expand the code in Listing 16-10
to create multiple threads that all send values to the same receiver. We can do
so by cloning the transmitter, as shown in Listing 16-11:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<span class="caption">Listing 16-11: Sending multiple messages from multiple
producers</span>

This time, before we create the first spawned thread, we call `clone` on the
transmitter. This will give us a new transmitter we can pass to the first
spawned thread. We pass the original transmitter to a second spawned thread.
This gives us two threads, each sending different messages to the one receiver.

When you run the code, your output should look something like this:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

You might see the values in another order, depending on your system. This is
what makes concurrency interesting as well as difficult. If you experiment with
`thread::sleep`, giving it various values in the different threads, each run
will be more nondeterministic and create different output each time.

Now that we’ve looked at how channels work, let’s look at a different method of
concurrency.
