<!-- Old heading. Do not remove or links may break. -->
<a id="the-match-control-flow-operator"></a>
## `match` Control Flow konstruksiyasi

Rust `match` deb nomlangan juda kuchli control flow konstruksiyasiga ega, bu sizga qiymatni bir qator patternlar bilan solishtirish va keyin qaysi pattern mos kelishiga qarab kodni bajarish imkonini beradi. Patternlar literal qiymatlar, o'zgaruvchilar nomlari, wildcardlar va boshqa ko'plab narsalardan iborat bo'lishi mumkin; [18-bobda][ch18-00-patterns]<!-- ignore --> har xil turdagi patternlar va ular bajaradigan ishlar yoritilgan. `match`ning kuchi patternlarning ifodaliligidan va kompilyator barcha mumkin bo'lgan holatlar ko'rib chiqilishini tasdiqlashidan kelib chiqadi.

`match` iborasini tanga saralash mashinasiga o'xshatib tasavvur qiling: tangalar bo'ylab turli o'lchamdagi teshiklari bo'lgan yo'ldan pastga siljiydi va har bir tanga o'zi mos keladigan birinchi teshikdan tushadi. Xuddi shu tarzda, qiymatlar `match` dagi har bir patterndan o'tadi va birinchi patternda qiymat “fits,”, qiymat bajarish paytida ishlatiladigan tegishli kod blokiga tushadi.

Tangalar haqida gap ketganda, keling, ularni `match` yordamida misol qilib olaylik! Biz noma'lum AQSH tangasini oladigan funksiyani yozishimiz mumkin va xuddi sanash mashinasiga o'xshab uning qaysi tanga ekanligini aniqlaydi va 6-3 ro'yxatda ko'rsatilganidek, uning qiymatini sentlarda qaytaradi.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 6-3: Enum va `match` ifodasi, uning namunalari sifatida enumning variantlari mavjud</span>

Keling, `sentdagi_qiymat` funksiyasidagi `match` ni ajratamiz. Avval biz `match` kalit so'zidan keyin ifodani keltiramiz, bu holda bu qiymat `tanga` bo'ladi. Bu `if` bilan ishlatiladigan shartli ifodaga juda o'xshaydi, lekin
katta farq bor: `if` bilan shart mantiqiy qiymatga baholanishi kerak, ammo bu yerda u har qanday turdagi bo'lishi mumkin. Ushbu misoldagi `tanga` turi biz birinchi qatorda belgilagan `Tanga` enumidir.

Keyingi `match` armlari. Arm ikki qismdan iborat: pattern va ba'zi kod. Bu yerdagi birinchi arm `Tanga::Penny` qiymati boʻlgan patternga ega, soʻngra ishlash uchun pattern va kodni ajratuvchi `=>` operatori. Bu holatda kod faqat `1` qiymatidan iborat. Har bir arm keyingisidan vergul bilan ajratiladi.

`match` ifodasi bajarilganda, natijaviy qiymatni har bir armning patterniga solishtiradi. Agar pattern qiymatga mos kelsa, ushbu pattern bilan bog'langan kod bajariladi. Agar bu pattern qiymatga mos kelmasa, ijro tanga saralash mashinasida bo'lgani kabi keyingi armda davom etadi.
Bizda qancha arm kerak bo'lsa, shuncha arm bo'lishi mumkin: 6-3 ro'yxatda bizning `match`imizda to'rtta arm bor.

Har bir arm bilan bog'langan kod ifodadir va mos keladigan qismdagi ifodaning natijaviy qiymati butun `match` ifodasi uchun qaytariladigan qiymatdir.

Agar mos keladigan arm kodi qisqa bo'lsa, biz odatda jingalak qavslardan foydalanmaymiz, chunki bu ro'yxat 6-3da bo'lgani kabi, har bir arm shunchaki qiymat qaytaradi. Agar siz mos keladigan chiziqda bir nechta kod qatorlarini ishlatmoqchi bo'lsangiz, jingalak qavslardan foydalaning va armdan keyingi vergul ixtiyoriy bo'ladi. Masalan, quyidagi kodda `Omadli tanga!` metod har safar `Tanga::Penny` bilan chaqirilganda, lekin baribir blokning oxirgi qiymatini qaytaradi, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Qiymatlarni bog'laydigan patternlar

match armlarining yana bir foydali xususiyati shundaki, ular patternga mos keladigan qiymatlarning qismlarini bog'lashlari mumkin. Enum variantlaridan qiymatlarni shunday chiqarishimiz mumkin.

Misol tariqasida, uning ichida ma'lumotlarni saqlash uchun enum variantlarimizdan birini o'zgartiraylik.
1999 yildan 2008 yilgacha Qo'shma Shtatlar bir tomondan 50 shtatning har biri uchun turli dizayndagi tangalarni bosib chiqardi. Boshqa hech qanday tangalar davlat dizayniga ega emas, shuning uchun faqat quarterlarda bunday qo'shimcha qiymat mavjud. Biz ushbu maʼlumotni `Quarter` variantini uning ichida saqlangan `UsState` qiymatini kiritish uchun oʻzgartirish orqali `enum`ga qoʻshishimiz mumkin, biz buni 6-4 roʻyxatda qilganmiz.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Roʻyxat 6-4: `Quarter` varianti ham `UsState` qiymatiga ega boʻlgan `Tanga` enumi</span>

Tasavvur qiling-a, sizning do'stingiz barcha 50 shtatdan quarter yig'ishga harakat qilmoqda. Biz tangalar turi bo'yicha saralashimiz bilan birga, agar do'stimizda yo'q bo'lsa, ular uni o'z kollektsiyasiga qo'shishlari uchun har quarter bilan bog'liq shtat nomini ham chaqiramiz.

Ushbu kod uchun match ifodasida biz `Tanga::Quarter` varianti qiymatlariga mos keladigan patternga `shtat` deb nomlangan o'zgaruvchini qo‘shamiz. `Tanga::Quarter` mos kelganda, `shtat` o'zgaruvchisi o'sha quarter holati qiymatiga bog'lanadi. Keyin biz ushbu arm uchun kodda `shtat` dan foydalanishimiz mumkin, masalan:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.

### Matching with `Option<T>`

In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match`, as
we did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way the `match` expression works remains the
same.

Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.

This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>

Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

Does `Some(5)` match `Some(i)`? It does! We have the same variant. The `i`
binds to the value contained in `Some`, so `i` takes the value `5`. The code in
the match arm is then executed, so we add 1 to the value of `i` and create a
new `Some` value with our total `6` inside.

Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.

Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.

### Matches Are Exhaustive

There’s one other aspect of `match` we need to discuss: the arms’ patterns must
cover all possibilities. Consider this version of our `plus_one` function,
which has a bug and won’t compile:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust knows that we didn’t cover every possible case, and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.

### Catch-all Patterns and the `_` Placeholder

Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we’re implementing a game
where, if you roll a 3 on a dice roll, your player doesn’t move, but instead
gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all
other values, your player moves that number of spaces on the game board. Here’s
a `match` that implements that logic, with the result of the dice roll
hardcoded rather than a random value, and all other logic represented by
functions without bodies because actually implementing them is out of scope for
this example:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

For the first two arms, the patterns are the literal values `3` and `7`. For
the last arm that covers every other possible value, the pattern is the
variable we’ve chosen to name `other`. The code that runs for the `other` arm
uses the variable by passing it to the `move_player` function.

This code compiles, even though we haven’t listed all the possible values a
`u8` can have, because the last pattern will match all values not specifically
listed. This catch-all pattern meets the requirement that `match` must be
exhaustive. Note that we have to put the catch-all arm last because the
patterns are evaluated in order. If we put the catch-all arm earlier, the other
arms would never run, so Rust will warn us if we add arms after a catch-all!

Rust also has a pattern we can use when we want a catch-all but don’t want to
*use* the value in the catch-all pattern: `_` is a special pattern that matches
any value and does not bind to that value. This tells Rust we aren’t going to
use the value, so Rust won’t warn us about an unused variable.

Let’s change the rules of the game: now, if you roll anything other than a 3 or
a 7, you must roll again. We no longer need to use the catch-all value, so we
can change our code to use `_` instead of the variable named `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

This example also meets the exhaustiveness requirement because we’re explicitly
ignoring all other values in the last arm; we haven’t forgotten anything.

Finally, we’ll change the rules of the game one more time so that nothing else
happens on your turn if you roll anything other than a 3 or a 7. We can express
that by using the unit value (the empty tuple type we mentioned in [“The Tuple
Type”][tuples]<!-- ignore --> section) as the code that goes with the `_` arm:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Here, we’re telling Rust explicitly that we aren’t going to use any other value
that doesn’t match a pattern in an earlier arm, and we don’t want to run any
code in this case.

There’s more about patterns and matching that we’ll cover in [Chapter
18][ch18-00-patterns]<!-- ignore -->. For now, we’re going to move on to the
`if let` syntax, which can be useful in situations where the `match` expression
is a bit wordy.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html
