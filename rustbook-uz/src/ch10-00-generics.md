# Generik turlar, Traitlar va Lifetimelar

Har bir dasturlash tilida kontseptsiyalarning takrorlanishini samarali boshqarish vositalari mavjud. Rustda bunday vositalardan biri *generiklar*: concrete  turlari yoki boshqa xususiyatlar uchun mavhum stendlar. Kodni kompilyatsiya qilish va ishga tushirishda ularning o'rnida nima bo'lishini bilmasdan, biz generiklarning xatti-harakatlarini yoki ularning boshqa generiklar bilan qanday bog'liqligini ifodalashimiz mumkin.

Funktsiyalar `i32` yoki `String` kabi aniq turdagi o'rniga ba'zi umumiy turdagi parametrlarni olishi mumkin, xuddi shu tarzda funksiya bir xil kodni bir nechta aniq qiymatlarda ishlatish uchun noma'lum qiymatlarga ega parametrlarni oladi. Aslida, biz 6-bobda `Option<T>`, 8-bobda `Vec<T>` va `HashMap<K, V>` va 9-bobda `Result<T, E>` bilan generiklardan allaqachon foydalanganmiz. Ushbu bobda siz o'zingizning turlaringizni, funksiyalaringizni va metodlaringizni generiklar bilan qanday aniqlashni o'rganasiz!

Birinchidan, kodning takrorlanishini kamaytirish uchun funksiyani qanday chiqarishni ko'rib chiqamiz. Keyin biz bir xil texnikadan faqat parametrlari turida farq qiladigan ikkita funksiyadan umumiy funksiyani yaratamiz. Shuningdek, biz struct va enum ta'riflarida generik turlardan qanday foydalanishni tushuntiramiz.

Keyin xulq-atvorni umumiy tarzda aniqlash uchun *traitlar* dan qanday foydalanishni o'rganasiz. Har qanday turdan farqli o'laroq, faqat ma'lum bir xatti-harakatga ega bo'lgan turlarni qabul qilish uchun umumiy turni cheklash uchun traitlarni umumiy turlar bilan birlashtira olasiz.

Va nihoyat, biz *lifetimelar* haqida gaplashamiz: kompilyatorga referencelar bir-biriga qanday bog'liqligi haqida ma'lumot beradigan turli xil generiklar. Lifetimelar kompilyatorga olingan qiymatlar haqida yetarli ma'lumot berishga imkon beradi, shunda u murojaatlar bizning yordamimizsiz ko'proq holatlarda haqiqiy bo'lishini ta'minlaydi.

## Funksiyani ajratib olish orqali takrorlanishni olib tashlash

Generiklar bizga kodning takrorlanishini olib tashlash uchun bir nechta turlarni ifodalovchi maxsus turlarni to'ldiruvchi bilan almashtirishga imkon beradi. Generik sintaksisga kirishdan oldin, keling, birinchi navbatda, ma'lum qiymatlarni bir nechta qiymatlarni ifodalovchi to'ldiruvchi bilan almashtiradigan funksiyani chiqarib, generik turlarni o'z ichiga olmaydigan tarzda takrorlashni qanday olib tashlashni ko'rib chiqaylik. Keyin generik funksiyani chiqarish uchun xuddi shu texnikani qo'llaymiz! Funksiyaga chiqarishingiz mumkin bo'lgan takrorlangan kodni qanday tanib olishni ko'rib chiqsangiz, generiklardan foydalanishi mumkin bo'lgan takrorlangan kodni taniy boshlaysiz.

Biz ro'yxatdagi eng katta raqamni topadigan 10-1 ro'yxatidagi qisqa dasturdan boshlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 10-1: Raqamlar ro'yxatidagi eng katta raqamni topish</span>

Biz butun sonlar roʻyxatini `raqamlar_listi` oʻzgaruvchisida saqlaymiz va roʻyxatdagi birinchi raqamga referenceni `eng_katta` nomli oʻzgaruvchiga joylashtiramiz. Keyin biz roʻyxatdagi barcha raqamlarni takrorlaymiz va agar joriy raqam `eng_katta`da saqlangan raqamdan katta boʻlsa, ushbu oʻzgaruvchidagi referenceni almashtiramiz.
Biroq, agar joriy raqam hozirgacha ko'rilgan eng katta raqamdan kichik yoki unga teng bo'lsa, o'zgaruvchi o'zgarmaydi va kod ro'yxatdagi keyingi raqamga o'tadi. Ro'yxatdagi barcha raqamlarni ko'rib chiqqandan so'ng, `eng_katta` eng katta raqamga ishora qilishi kerak, bu holda bu 100 ga teng.

Bizga endi ikki xil raqamlar ro‘yxatidagi eng katta raqamni topish vazifasi qo‘yildi. Buning uchun biz 10-1 roʻyxatdagi kodni takrorlashni tanlashimiz va 10-2 roʻyxatda koʻrsatilganidek, dasturning ikki xil joyida bir xil mantiqdan foydalanishimiz mumkin.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

<span class="caption">Listing 10-2: Code to find the largest number in *two*
lists of numbers</span>

Although this code works, duplicating code is tedious and error prone. We also
have to remember to update the code in multiple places when we want to change
it.

To eliminate this duplication, we’ll create an abstraction by defining a
function that operates on any list of integers passed in a parameter. This
solution makes our code clearer and lets us express the concept of finding the
largest number in a list abstractly.

In Listing 10-3, we extract the code that finds the largest number into a
function named `largest`. Then we call the function to find the largest number
in the two lists from Listing 10-2. We could also use the function on any other
list of `i32` values we might have in the future.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>

The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values we might pass into the function. As a result,
when we call the function, the code runs on the specific values that we pass
in.

In summary, here are the steps we took to change the code from Listing 10-2 to
Listing 10-3:

1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the
   inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.

Next, we’ll use these same steps with generics to reduce code duplication. In
the same way that the function body can operate on an abstract `list` instead
of specific values, generics allow code to operate on abstract types.

For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let’s find out!
