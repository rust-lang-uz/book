## Hash Maplarda bog'langan qiymatlarga ega kalitlarni saqlash

The last of our common collections is the *hash map*. The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a *hashing function*, which determines how it places these keys and values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type. For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score. Given a team name, you can retrieve its score.

We’ll go over the basic API of hash maps in this section, but many more goodies are hiding in the functions defined on `HashMap<K, V>` by the standard library. As always, check the standard library documentation for more information.

### Yangi Hash Map yaratish

One way to create an empty hash map is using `new` and adding elements with `insert`. In Listing 8-20, we’re keeping track of the scores of two teams whose names are *Blue* and *Yellow*. The Blue team starts with 10 points, and the Yellow team starts with 50.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-20: Yangi hesh mapni yaratish va ba'zi kalitlar va qiymatlarni kiritish</span>

Note that we need to first `use` the `HashMap` from the collections portion of the standard library. Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude. Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This `HashMap` has keys of type `String` and values of type `i32`. Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

### HashMap-dagi ma'lumotlarga kirish

Biz 8-21 ro'yxatda ko'rsatilganidek, `get` metodiga kalitni taqdim etish orqali hash mapdan qiymat olishimiz mumkin.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-21: Hesh-Mapda saqlangan Yashil jamoa hisobiga kirish</span>

Here, `score` will have the value that’s associated with the Blue team, and the result will be `10`. The `get` method returns an `Option<&V>`; if there’s no value for that key in the hash map, `get` will return `None`. This program handles the `Option` by calling `copied` to get an `Option<i32>` rather than an `Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` doesn't have an entry for the key.

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

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-22: kalitlar va qiymatlar kiritilgandan so'ng ular hesh-mapda tegishli ekanligini ko'rsatish</span>

`maydon_nomi` va `maydon_qiymati` o'zgaruvchilari qiymatlari `insert` metodini chaqirish orqali HashMap-ga ko'chirilgandan keyin foydalana olmaymiz.

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid. We’ll talk more about these issues in the [“Validating References with Lifetimes”]()<!-- ignore --> section in Chapter 10.

### Hash Mapni yangilash

Kalit va qiymat juftlarining soni o'sishi mumkin bo'lsa-da, har bir noyob kalit bir vaqtning o'zida u bilan bog'langan faqat bitta qiymatga ega bo'lishi mumkin (lekin aksincha emas: masalan, Yashil jamoa ham, Sariq jamoa ham `ballar` hash-mapida saqlangan 10 qiymatiga ega bo'lishi mumkin).

When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key *doesn’t* already have a value. Or you could combine the old value and the new value. Let’s look at how to do each of these!

#### Qiymatni qayta yozish

If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced. Even though the code in Listing 8-23 calls `insert` twice, the hash map will only contain one key/value pair because we’re inserting the value for the Blue team’s key both times.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-23: Saqlangan qiymatni ma'lum bir kalit bilan almashtirish</span>

This code will print `{"Blue": 25}`. The original value of `10` has been overwritten.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Kalit va qiymatni faqat kalit mavjud bo'lmaganda qo'shish

It’s common to check whether a particular key already exists in the hash map with a value then take the following actions: if the key does exist in the hash map, the existing value should remain the way it is. If the key doesn’t exist, insert it and a value for it.

Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter. The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist. Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team. Using the `entry` API, the code looks like Listing 8-24.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-24: `entry` metodidan faqat kalitda qiymat mavjud bo'lmasa, kiritish uchun foydalanish</span>

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-24 will print `{"Yellow": 50, "Blue": 10}`. The first call to `entry` will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already. The second call to `entry` will not change the hash map because the Blue team already has the value 10.

#### Eski qiymat asosida yangi qiymatni yangilash

Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, Listing 8-25 shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```


<span class="caption">Ro'yxat 8-25: So'zlar va hisoblarni saqlaydigan hash-mapi yordamida so'zlarning necha marta takrorlanganini hisoblash</span>

This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. You might see the same key/value pairs printed in a different order: recall from the [“Accessing Values in a Hash Map”][access]<!-- ignore --> section that iterating over a hash map happens in an arbitrary order.

The `split_whitespace` method returns an iterator over sub-slices, separated by whitespace, of the value in `text`. The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. Here we store that mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using the asterisk (`*`). The mutable reference goes out of scope at the end of the `for` loop, so all of these changes are safe and allowed by the borrowing rules.

### Hashing funksiyalari

By default, `HashMap` uses a hashing function called *SipHash* that can provide resistance to Denial of Service (DoS) attacks involving hash tables[^siphash]<!-- ignore -->. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A *hasher* is a type that implements the `BuildHasher` trait. We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; [crates.io](https://crates.io/)<!-- ignore --> has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

## Xulosa

Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:

* Butun sonlar roʻyxati berilgan boʻlsa, vectordan foydalaning va roʻyxatning medianasini (tartiblanganda, oʻrtadagi qiymat) va  rejimni (koʻpincha sodir boʻladigan qiymat; bu yerda hash-map foydali boʻladi) qaytaring.
* Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

Standart kutubxona API texnik hujjatlari vectorlar, stringlar va hash-maplarda ushbu mashqlar uchun foydali bo'lgan usullarni tavsiflaydi!

We’re getting into more complex programs in which operations can fail, so, it’s a perfect time to discuss error handling. We’ll do that next!
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

[access]: #accessing-values-in-a-hash-map
