## Izohlar

Barcha dasturchilar o'z kodlarini tushunishni osonlashtirishga harakat qilishadi, lekin ba'zida qo'shimcha tushuntirish kerak. Bunday hollarda dasturchilar o'zlarining manba kodlarida *izohlar* qoldiradilar, ularni kompilyator e'tiborsiz qoldiradi, ammo manba kodini o'qiyotgan odamlar uchun foydali bo'lishi mumkin.

Mana oddiy izoh:

```rust
// hello, world
```

Rustda idiomatik izoh uslubi izohni ikki qiyshiq chiziq bilan boshlaydi va izoh satr oxirigacha davom etadi. Bitta satrdan tashqariga chiqadigan izohlar uchun har bir satrga `//` qo'shishingiz kerak bo'ladi, masalan:

```rust
// Shunday qilib, biz bu erda murakkab ish qilyapmiz,
// bizga bir nechta izohlar kerak bo'ladi! Vou! Umid qilamanki,
// bu izoh nima bo'layotganini tushuntiradi.
```

Izohlar, shuningdek, kodni o'z ichiga olgan qatorlar oxirida joylashtirilishi mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ammo siz ularni ushbu formatda ko'proq ko'rasiz, izohli kod ustidagi alohida satrda izoh bilan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```
Rustda yana bir turdagi izohlar, hujjatlar izohlari mavjud, biz ularni 14-bobning [“Crates.io-ga crateni nashr qilish“][publishing]<!-- ignore --> bo'limida muhokama qilamiz.

[publishing]: ch14-02-publishing-to-crates-io.html
