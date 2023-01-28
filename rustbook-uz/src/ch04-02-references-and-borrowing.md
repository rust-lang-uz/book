## Reference va Borrowing

Ro'yxat 4-5dagi tuple kodi bilan bog'liq muammo shundaki, biz `String` ni chaqiruvchi funksiyaga qaytarishimiz kerak, shunda biz `uzunlikni_hisoblash` ga chaqiruvdan keyin ham `String` dan foydalanishimiz mumkin, chunki `String` `uzunlikni_hisoblash` ga ko'chirildi. Buning o'rniga biz `String` qiymatiga reference(havola) berishimiz mumkin.

*Reference* pointerga o'xshaydi, chunki u biz ushbu manzilda saqlangan ma'lumotlarga kirish uchun amal qilishimiz mumkin bo'lgan manzildir; bu ma'lumotlar boshqa o'zgaruvchilarga tegishli.
Pointerdan farqli o'laroq, reference ma'lumotnomaning amal qilish muddati davomida ma'lum turdagi haqiqiy qiymatni ko'rsatishi kafolatlanadi.

Qiymatga egalik qilish o'rniga parametr sifatida obyektga referencega ega bo'lgan `uzunlikni_hisoblash` funksiyasini qanday aniqlash va ishlatishingiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

Birinchidan, o'zgaruvchilar deklaratsiyasidagi barcha tuple kodi va funksiyani qaytarish qiymati yo'qolganiga e'tibor bering. Ikkinchidan, `&s1` ni `uzunlikni_hisoblash` ga o'tkazamiz va uning definitionida biz `String` emas, `&String`ni olamiz. Ushbu ampersandlar *reference* ni ifodalaydi va ular sizga biron bir qiymatga ownershiplik qilmasdan murojaat qilish imkonini beradi. 4-5-rasmda ushbu tushuncha tasvirlangan.

<img alt="Three tables: the table for s contains only a pointer to the table
for s1. The table for s1 contains the stack data for s1 and points to the
string data on the heap." src="img/trpl04-05.svg" class="center" />

<span class="caption">4-5-rasm: `&String s` chizmasi `String s1`ga ishora qiladi</span>

> Eslatma: `&` yordamida reference qilishning teskarisi *dereferencing* bo`lib,
> u `*` dereference operatori yordamida amalga oshiriladi. Biz 8-bobda dereference
> operatoridan baʼzi foydalanishni koʻrib chiqamiz va 15-bobda dereference tafsilotlarini 
> muhokama qilamiz.

Keling, bu yerda funksiya chaqiruvini batafsil ko'rib chiqaylik:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&s1` sintaksisi bizga `s1` qiymatiga *refers* qiluvchi, lekin unga tegishli bo`lmagan reference yaratish imkonini beradi. Unga egalik qilmaganligi sababli, reference foydalanishni to'xtatganda, u ko'rsatgan qiymat o'chirilmaydi.

Xuddi shunday, funksiya imzosi `s` parametrining turi reference ekanligini ko'rsatish uchun `&` dan foydalanadi. Keling, ba'zi tushuntirish izohlarini qo'shamiz:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

`s` o'zgaruvchisi amal qiladigan doirasi har qanday funksiya parametrining qamrovi bilan bir xil bo'ladi, lekin `s` foydalanishni to'xtatganda reference  ko'rsatilgan qiymat o'chirilmaydi, chunki `s` ownershipga ega emas. Funtsiya referencelarni haqiqiy qiymatlar o'rniga parametr sifatida ko'rsatsa, biz ownershipni qaytarish uchun qiymatlarni qaytarishimiz shart emas, chunki bizda hech qachon ownership bo'lmagan.

Malumot yaratish harakatini *borrowing*(qarz olish) deb ataymiz. Haqiqiy hayotda bo'lgani kabi, agar biror kishi biror narsaga ega bo'lsa, siz undan qarz olishingiz mumkin. Ishingiz tugagach, uni qaytarib berishingiz kerak. Siz unga egalik qilmaysiz.

Xo'sh, agar biz borrowing qilgan narsani o'zgartirishga harakat qilsak nima bo'ladi? 4-6 ro'yxatdagi kodni sinab ko'ring. Spoiler ogohlantirish: bu ishlamaydi!

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">Ro'yxat 4-6: Borrow qilingan qiymatni o'zgartirishga urinish</span>

Mana xato:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

O'zgaruvchilar standart bo'yicha o'zgarmas bo'lganidek, referencelar ham shunday. Bizga reference biror narsani o'zgartirishga ruxsat berilmagan.

### O'zgaruvchan Referencelar

Biz 4-6 ro'yxatdagi kodni tuzatishimiz mumkin, buning o'rniga `o'zgaruvchan reference`dan foydalanadigan bir nechta kichik sozlashlar bilan borrow qilingan qiymatni o'zgartirishimiz mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

Avval `s` ni `mut` qilib o'zgartiramiz. Keyin biz `&mut s` bilan o'zgaruvchan reference yaratamiz, bu yerda biz `change` funksiyasini chaqiramiz va `some_string: &mut String` bilan o'zgaruvchan referencei qabul qilish uchun funksiya signatureni yangilaymiz. Bu `change` funksiyasi olingan qiymatni o'zgartirishini aniq ko'rsatadi.

O'zgaruvchan referencelar bitta katta cheklovga ega: agar sizda qiymatga o'zgaruvchan reference bo'lsa, sizda bu qiymatga boshqa referencelar bo'lishi mumkin emas. `s` ga ikkita o'zgaruvchan reference yaratishga urinayotgan ushbu kod muvaffaqiyatsiz bo'ladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Mana xato:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Bu xatolik bu kodning yaroqsiz ekanligini bildiradi, chunki biz bir vaqtning o'zida bir necha marta o'zgaruvchan `s` ni borrow qila ololmaymiz. Birinchi o'zgaruvchan borrow `r1` da bo'lib, u `println!` da ishlatilgunga qadar davom etishi kerak, lekin bu o'zgaruvchan referenceni yaratish va undan foydalanish o'rtasida, biz `r2` da `r1` bilan bir xil ma'lumotlarni olgan boshqa o`zgaruvchan reference yaratishga harakat qildik.

Bir vaqtning o'zida bir xil ma'lumotlarga bir nechta o'zgaruvchan referencelarni oldini oluvchi cheklov mutatsiyaga imkon beradi, lekin juda nazorat ostida. Bu yangi Rustaceanlar bilan kurashadigan narsa, chunki aksariyat tillar xohlagan vaqtda mutatsiyaga o'tishga imkon beradi. Ushbu cheklovning afzalligi shundaki, Rust kompilyatsiya vaqtida data raceni oldini oladi. *Data race* poyga holatiga o'xshaydi va bu uchta xatti-harakatlar sodir bo'lganda sodir bo'ladi:

* Ikki yoki undan ortiq pointerlar bir vaqtning o'zida bir xil ma'lumotlarga kirishadi.
* Pointerlardan kamida bittasi ma'lumotlarga yozish uchun ishlatiladi.
* Ma'lumotlarga kirishni sinxronlashtirish uchun hech qanday mexanizm ishlatilmaydi.

Data race aniqlanmagan xatti-harakatlarga olib keladi va ularni runtimeda kuzatib borishga harakat qilayotganingizda tashxis qo'yish va tuzatish qiyin bo'lishi mumkin; Rust data racelari bilan kodni kompilyatsiya qilishni rad etish orqali bu muammoni oldini oladi!

Har doimgidek, biz *bir vaqtning o'zida* emas, balki bir nechta o'zgaruvchan referencelarga ruxsat beruvchi yangi scope yaratish uchun jingalak qavslardan foydalanishimiz mumkin:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust o'zgaruvchan va o'zgarmas referencelarni birlashtirish uchun shunga o'xshash qoidani qo'llaydi.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Mana xato:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Voy! Bizda *shuningdek* o'zgaruvchan referencelar bo'lishi mumkin emas, bizda bir xil qiymatga o'zgarmas reference mavjud.

O'zgarmas reference foydalanuvchilari qiymat birdaniga ularning ostidan o'zgarishini kutishmaydi! Biroq, bir nechta o'zgarmas referencelarga ruxsat beriladi, chunki faqat ma'lumotlarni o'qiyotgan hech kim boshqa hech kimning ma'lumotni o'qishiga ta'sir qilish qobiliyatiga ega emas.

Note that a reference’s scope starts from where it is introduced and continues
through the last time that reference is used. For instance, this code will
compile because the last usage of the immutable references, the `println!`,
occurs before the mutable reference is introduced:

```rust,edition2021
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

The scopes of the immutable references `r1` and `r2` end after the `println!`
where they are last used, which is before the mutable reference `r3` is
created. These scopes don’t overlap, so this code is allowed: the compiler can
tell that the reference is no longer being used at a point before the end of
the scope.

Even though borrowing errors may be frustrating at times, remember that it’s
the Rust compiler pointing out a potential bug early (at compile time rather
than at runtime) and showing you exactly where the problem is. Then you don’t
have to track down why your data isn’t what you thought it was.

### Dangling References

In languages with pointers, it’s easy to erroneously create a *dangling
pointer*—a pointer that references a location in memory that may have been
given to someone else—by freeing some memory while preserving a pointer to that
memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.

Let’s try to create a dangling reference to see how Rust prevents them with a
compile-time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

This error message refers to a feature we haven’t covered yet: lifetimes. We’ll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That’s no good! Rust
won’t let us do this.

The solution here is to return the `String` directly:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

This works without any problems. Ownership is moved out, and nothing is
deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

* At any given time, you can have *either* one mutable reference *or* any
  number of immutable references.
* References must always be valid.

Next, we’ll look at a different kind of reference: slices.
