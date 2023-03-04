## Hash Maplarda bog'langan qiymatlarga ega kalitlarni saqlash

Umumiy to'plamlarimizning oxirgisi *hash map*. `HashMap<K, V>` turi `K` turidagi kalitlarning `V` turidagi qiymatlarga xaritasini *xeshlash funksiyasi* yordamida saqlaydi, bu kalit va qiymatlarni xotiraga qanday joylashtirishini belgilaydi. Ko'pgina dasturlash tillari bunday ma'lumotlar strukturasini qo'llab-quvvatlaydi, lekin ular ko'pincha bir nechtasini nomlash uchun hash, map, object, hash table, dictionary, yoki associative array kabi boshqa nomlardan foydalanadilar.

Hash maplar ma'lumotlarni vektorlar bilan bo'lgani kabi indeks yordamida emas, balki har qanday turdagi kalit yordamida qidirmoqchi bo'lsangiz foydali bo'ladi. Misol uchun, o'yinda siz har bir jamoaning balini hesh-mapda kuzatib borishingiz mumkin, unda har bir kalit jamoaning nomi va qiymatlar har bir jamoaning ballidir. Jamoa nomi berilgan bo'lsa, siz uning ballini olishingiz mumkin.

Ushbu bo'limda biz hesh-mapllarining asosiy API-ni ko'rib chiqamiz, ammo yana ko'plab foydali funksiyalar standart kutubxona tomonidan `HashMap<K, V>` da belgilangan funksiyalarda yashiringan.
Har doimgidek, qo'shimcha ma'lumot olish uchun standart kutubxona texnik hujjatlarini tekshiring.

### Yangi Hash Map yaratish

Bo'sh hesh mapni yaratishning bir usuli - `new` dan foydalanish va `insert` bilan elementlarni qo'shish. 8-20 ro'yxatda biz nomlari *Yashil* va *Sariq* bo'lgan ikkita jamoaning ballarini kuzatib boramiz. Yashil jamoa 10 ball bilan, Sariq jamoa esa 50 ball bilan boshlanadi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-20: Yangi hesh mapni yaratish va ba'zi kalitlar va qiymatlarni kiritish</span>

E'tibor bering, biz birinchi navbatda standart kutubxonaning to'plamlar qismidagi `HashMap` dan foydalanishimiz kerak. Bu quyidagicha bo'ladi `use std::collections::HashMap;`. Bizning uchta keng tarqalgan to'plamlarimiz orasida bu eng kam qo'llaniladi, shuning uchun u muqaddimada avtomatik ravishda kiritilgan funtsiyalarga kiritilmagan. Hash Maplar standart kutubxonadan ham kamroq qo'llab-quvvatlanadi; masalan, ularni yaratish uchun o'rnatilgan makros mavjud emas.

Vectorlar singari, hash maplar ham o'z ma'lumotlarini heapda saqlaydi. Ushbu `HashMap`da `String` turidagi kalitlar va `i32` turidagi qiymatlar mavjud. Vectorlar singari, hash maplar ham bir xildir: barcha kalitlar bir-biri bilan bir xil turdagi va barcha qiymatlar bir xil turga ega bo'lishi kerak.

### HashMap-dagi ma'lumotlarga kirish

Biz 8-21 ro'yxatda ko'rsatilganidek, `get` metodiga kalitni taqdim etish orqali hash mapdan qiymat olishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-21: Hesh-Mapda saqlangan Yashil jamoa hisobiga kirish</span>

Bu yerda `ball` Yashil jamoa bilan bog'liq qiymatga ega bo'ladi va natija `10` bo'ladi. `get` metodi `Option<&V>`ni qaytaradi; agar hesh-mapda ushbu kalit uchun qiymat bo'lmasa, `get` `None` ni qaytaradi. Bu dastur `Option<&i32>` emas, `Option<i32>` olish uchun `copied` ga murojaat qilib `Option`ni boshqaradi, so'ngra `unwrap_or` `ballar` da ushbu kalit uchun ma'lumotlar bo'lmasa, ballni nolga o'rnatish uchun.

Biz hesh-mapdagi har bir kalit/qiymat juftligini vectorlar bilan bo'lgani kabi, `for` siklidan foydalangan holda takrorlashimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Ushbu kod har bir juftlikni tasodifiy tartibda chop etadi:

```text
Sariq: 50
Yashil: 10
```

### Hash Maplar va Ownership(Egalik)

`Copy` traitini amalga oshiradigan turlar uchun, masalan, `i32`, qiymatlar hesh-mapiga ko'chiriladi. `String` kabi tegishli qiymatlar uchun qiymatlar boshqa joyga koʻchiriladi va 8-22 roʻyxatda koʻrsatilganidek, hesh-map ushbu qiymatlarning egasi boʻladi.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-22: kalitlar va qiymatlar kiritilgandan so'ng ular hesh-mapda tegishli ekanligini ko'rsatish</span>

`maydon_nomi` va `maydon_qiymati` o'zgaruvchilari qiymatlari `insert` metodini chaqirish orqali HashMap-ga ko'chirilgandan keyin foydalana olmaymiz.

Agar biz HashMap-ga qiymatlarga referencelar kiritsak, ular HashMap-ga ko'chirilmaydi. Murojaatlar ko'rsatadigan qiymatlar hech bo'lmaganda hesh-mapda amal qiladigan vaqt davomida amal qilishi kerak. Biz ushbu muammolar haqida 10-bobning ["Lifetime bilan referencelarni tekshirish"][validating-references-with-lifetimes]<!-- ignore --> bo'limida ko'proq gaplashamiz.

### Hash Mapni yangilash

Kalit va qiymat juftlarining soni o'sishi mumkin bo'lsa-da, har bir noyob kalit bir vaqtning o'zida u bilan bog'langan faqat bitta qiymatga ega bo'lishi mumkin (lekin aksincha emas: masalan, Yashil jamoa ham, Sariq jamoa ham `ballar` hash-mapida saqlangan 10 qiymatiga ega bo'lishi mumkin).

Hash-mapdagi ma'lumotlarni o'zgartirmoqchi bo'lganingizda, kalit allaqachon tayinlangan qiymatga ega bo'lgan holatda qanday ishlashni hal qilishingiz kerak. Eski qiymatni butunlay e'tiborsiz qoldirib, eski qiymatni yangi qiymat bilan almashtirishingiz mumkin. Eski qiymatni saqlab qolishingiz va yangi qiymatni e'tiborsiz qoldirishingiz mumkin, faqat kalitda *qiymat bo'lmasa*, yangi qiymat qo'shishingiz mumkin. Yoki eski qiymat va yangi qiymatni birlashtira olasiz. Keling, bularning har birini qanday qilishni ko'rib chiqaylik!

#### Qiymatni qayta yozish

Agar biz kalit va qiymatni hash-mapga kiritsak va keyin boshqa qiymat bilan bir xil kalitni kiritsak, bu kalit bilan bog'langan qiymat almashtiriladi. Ro'yxat 8-23dagi kod ikki marta `insert` ni chaqirsa ham, hash-mapda faqat bitta kalit/qiymat juftligi bo'ladi, chunki biz har ikki marta Yashil jamoa kaliti qiymatini kiritamiz.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Ro'yxat 8-23: Saqlangan qiymatni ma'lum bir kalit bilan almashtirish</span>

Bu kod `{"Yashil": 25}`ni chop etadi. `10`ning asl qiymati ustiga yozildi.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Kalit va qiymatni faqat kalit mavjud bo'lmaganda qo'shish

Hash Mapda ma'lum bir kalit allaqachon qiymatga ega yoki yo'qligini tekshirish odatiy holdir, keyin quyidagi amallarni bajaring: agar kalit hash-mapda mavjud bo'lsa, mavjud qiymat avvalgidek qolishi kerak. Agar kalit mavjud bo'lmasa, insert va uning qiymatini kiriting.

Hash Mapda buning uchun `entry` deb nomlangan maxsus API mavjud bo'lib, siz tekshirmoqchi bo'lgan kalitni parametr sifatida qabul qiladi. The return value of the `entry` method is an enum
called `Entry` that represents a value that might or might not exist. Let’s say
we want to check whether the key for the Yellow team has a value associated
with it. If it doesn’t, we want to insert the value 50, and the same for the
Blue team. Using the `entry` API, the code looks like Listing 8-24.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Listing 8-24: Using the `entry` method to only insert if
the key does not already have a value</span>

The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-24 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.

#### Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-25 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Listing 8-25: Counting occurrences of words using a hash
map that stores words and counts</span>

This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. You might see
the same key/value pairs printed in a different order: recall from the
[“Accessing Values in a Hash Map”][access]<!-- ignore --> section that
iterating over a hash map happens in an arbitrary order.

The `split_whitespace` method returns an iterator over sub-slices, separated by
whitespace, of the value in `text`. The `or_insert` method returns a mutable
reference (`&mut V`) to the value for the specified key. Here we store that
mutable reference in the `count` variable, so in order to assign to that value,
we must first dereference `count` using the asterisk (`*`). The mutable
reference goes out of scope at the end of the `for` loop, so all of these
changes are safe and allowed by the borrowing rules.

### Hashing Functions

By default, `HashMap` uses a hashing function called *SipHash* that can provide
resistance to Denial of Service (DoS) attacks involving hash
tables[^siphash]<!-- ignore -->. This is not the fastest hashing algorithm
available, but the trade-off for better security that comes with the drop in
performance is worth it. If you profile your code and find that the default
hash function is too slow for your purposes, you can switch to another function
by specifying a different hasher. A *hasher* is a type that implements the
`BuildHasher` trait. We’ll talk about traits and how to implement them in
Chapter 10. You don’t necessarily have to implement your own hasher from
scratch; [crates.io](https://crates.io/)<!-- ignore --> has libraries shared by
other Rust users that provide hashers implementing many common hashing
algorithms.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Summary

Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:

* Given a list of integers, use a vector and return the median (when sorted,
  the value in the middle position) and mode (the value that occurs most often;
  a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to
  the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
  that start with a vowel have “hay” added to the end instead (“apple” becomes
  “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in a company. For example, “Add Sally to
  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.

The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
