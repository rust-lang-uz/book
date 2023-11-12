## Iteratorlar yordamida elementlar ketma-ketligini qayta ishlash

Iterator pattern sizga navbat bilan elementlarning ketma-ketligi bo'yicha ba'zi vazifalarni(task) bajarishga imkon beradi. Iterator har bir elementni takrorlash va ketma-ketlik qachon tugashini aniqlash mantiqi uchun javobgardir. Iteratorlardan foydalanganda, bu mantiqni(logic) o'zingiz takrorlashingiz shart emas.

Rust-da iteratorlar *dangasa*, ya'ni iteratorni ishlatish uchun ishlatadigan metodlarni chaqirmaguningizcha ular hech qanday ta'sir ko'rsatmaydi. Masalan, 13-10-Ro'yxatdagi kod `Vec<T>` da belgilangan `iter` metodini chaqirish orqali `v1` vektoridagi elementlar ustidan iterator yaratadi. Ushbu kod o'z-o'zidan hech qanday foydali ish qilmaydi.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-10: iterator yaratish</span>

Iterator `v1_iter` o'zgaruvchisida saqlanadi. Biz iteratorni yaratganimizdan so'ng, biz uni turli usullarda ishlatishimiz mumkin. 3-bobdagi 3-5 ro'yxatda biz arrayning har bir elementida ba'zi kodlarni bajarish uchun `for` loop siklidan foydalangan holda uni takrorladik. Korpus ostida bu bilvosita yaratgan va keyin iteratorni ishlatgan, ammo biz hozirgacha uning qanday ishlashini ko'rib chiqdik.

13-11 Ro'yxatdagi misolda biz iteratorni yaratishni `for` loop siklidagi iteratordan foydalanishdan ajratamiz. `for` loop sikli `v1_iter` da iterator yordamida chaqirilganda, iteratordagi har bir element loop siklning bir iteratsiyasida ishlatiladi, bu esa har bir qiymatni chop etadi.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-11: `for` loop siklida iteratordan foydalanish</span>

Standart kutubxonalari tomonidan taqdim etilgan iteratorlarga ega bo'lmagan tillarda siz xuddi shu funksiyani o'zgaruvchini 0 indeksidan boshlab yozishingiz mumkin, qiymat olish uchun vektorga indekslash uchun ushbu o'zgaruvchidan foydalanish va vektordagi elementlarning umumiy soniga yetgunga qadar sikldagi o'zgaruvchi qiymatini oshirish.

Iteratorlar siz uchun barcha mantiqni(logic) boshqaradi, siz chalkashtirib yuborishingiz mumkin bo'lgan takroriy kodni qisqartiradi. Iteratorlar vektorlar kabi indekslash mumkin bo'lgan ma'lumotlar tuzilmalari(data structure) emas, balki turli xil ketma-ketliklar(sequence) bilan bir xil mantiqdan foydalanish uchun ko'proq moslashuvchanlikni beradi. Keling, iteratorlar buni qanday qilishini ko'rib chiqaylik.

### `Iterator` traiti va `next` metodi

Barcha iteratorlar standart kutubxonada(standard library) aniqlangan `Iterator` nomli traitni implement qiladilar. Traitning definitioni quyidagicha ko'rinadi:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // default implement qilingan  metodlar bekor qilindi
}
```

Eʼtibor bering, bu definitionda baʼzi yangi sintaksislar qoʻllangan: `type Item` va `Self::Item` bu trait bilan *bogʻlangan turni(associated type)* belgilaydi. Bog'langan turlar haqida 19-bobda batafsil gaplashamiz. Hozircha siz bilishingiz kerak bo'lgan narsa shuki, ushbu kodda aytilishicha, `Iterator` traitini implement qilish uchun siz `Item` turini ham belgilashingiz kerak bo'ladi va bu `Item` turi `next` metodining qaytarish(return) turida qo'llaniladi. Boshqacha qilib aytganda, `Item` turi iteratordan qaytarilgan tur bo'ladi.

`Iterator` traiti amalga oshiruvchilardan(implementorlar) faqat bitta metodni belgilashni talab qiladi: `next` metod, u bir vaqtning o'zida `Some` ga o'ralgan(wrapped) iteratorning bir elementini qaytaradi va takrorlash(iteratsiya) tugagach, `None`ni qaytaradi.

Biz iteratorlarda `next` metodini to'g'ridan-to'g'ri chaqirishimiz mumkin; Ro'yxat 13-12 vektordan yaratilgan iteratorda `next` ga takroriy chaqiruvlardan qanday qiymatlar qaytarilishini ko'rsatadi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 13-12: iteratorda `next` metodini chaqirish</span>

Esda tutingki, biz `v1_iter` ni o'zgaruvchan(mutable) qilishimiz kerak edi: iteratorda `next` metodini chaqirish iterator ketma-ketlikda(sequence) qayerdaligini kuzatish uchun foydalanadigan ichki holatni(internal state) o'zgartiradi. Boshqacha qilib aytganda, bu kod iteratorni *iste'mol qiladi(consumes)* yoki ishlatadi. `next` ga har bir chaqiruv(call) iteratordan biror elementni olib tashlaydi. Biz `for` loop siklidan foydalanganda `v1_iter`ni o‘zgaruvchan(mutable) qilishimiz shart emas edi, chunki sikl `v1_iter` ga ownership(egalik) qildi va uni sahna ortida o‘zgaruvchan qildi.

Shuni ham yodda tutingki, biz `next` ga chaiqruvlardan oladigan qiymatlar vektordagi qiymatlarga o'zgarmas(immutable) referencelardir. `iter` metodi immutable(o'zgarmas) referencelar ustida iterator hosil qiladi. Agar biz `v1` ga ownershiplik(egalik) qiluvchi va tegishli qiymatlarni qaytaruvchi iterator yaratmoqchi bo'lsak, `iter` o‘rniga `into_iter` ni chaqirishimiz mumkin. Xuddi shunday, agar biz mutable(o'zgaruvchan) referencelarni takrorlashni xohlasak, `iter` o'rniga `iter_mut` ni chaqirishimiz mumkin.

### Iteratorni consume qiladigan metodlar

`Iterator` traiti standart kutubxona(standard library) tomonidan taqdim etilgan default implementationlar bilan bir qator turli metodlarga ega; ushbu metodlar haqida `Iterator` traiti uchun standart kutubxona API texnik hujjatlarini ko'rib chiqish orqali bilib olishingiz mumkin. Ushbu metodlarning ba'zilari o'z definitionlarida `next` metodni chaqiradi, shuning uchun `Iterator` tratini implement qilishda `next` metodni qo'llash talab qilinadi.

`next` ni chaqiruvchi metoflar *consuming adaptorlar* deb ataladi, chunki ularni chaqirish iteratordan foydalanadi. Bitta misol, iteratorga ownership(egalik) qiladigan va `next` deb qayta-qayta chaqirish orqali elementlarni takrorlaydigan, shu bilan iteratorni consume qiladigan `sum` metodidir. U takrorlanayotganda, u har bir elementni ishlayotgan jamiga qo'shadi va takrorlash tugagach, jamini qaytaradi. 13-13 ro'yxatda `sum` metodidan foydalanishni ko'rsatadigan test mavjud:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 13-13: iteratordagi barcha elementlarning umumiy miqdorini olish uchun `sum` metodini chaqirish</span>

Bizga `sum` chaqiruvidan keyin `v1_iter` dan foydalanishga ruxsat berilmagan, chunki `sum` biz chaqiruvchi iteratorga ownershiplik(egalik) qiladi.

### Boshqa iteratorlarni yaratuvchi metodlar

*Iterator adaptorlari* iteratorni consume(iste'mol) qilmaydigan `Iterator` traiti bo'yicha aniqlangan metoddir. Buning o'rniga, ular asl iteratorning ba'zi jihatlarini o'zgartirib, turli iteratorlarni ishlab chiqaradilar.

13-14 ro'yxatda iterator adapter metodini `map` deb chaqirish misoli ko'rsatilgan, bunda elementlar takrorlanganda(iteratsiya) har bir elementga chaqiruv(call) qilish yopiladi.
`map` metodi o'zgartirilgan elementlarni ishlab chiqaradigan yangi iteratorni qaytaradi. Bu yerda closure vektorning har bir elementi 1 ga oshiriladigan yangi iteratorni yaratadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-14: Yangi iterator yaratish uchun iterator adapteriga `map` chaqiruv qilish  qilish</span>

Biroq, bu kod ogohlantirish(warning) ishlab chiqaradi:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

13-14 ro'yxatdagi kod hech narsa qilmaydi; biz belgilagan closure hech qachon chaqirilmaydi. Ogohlantirish(warning) bizga nima uchun eslatib turadi: iterator adapterlari dangasa va biz bu yerda iteratorni consume(ishlatish) qilishimiz kerak.

Ushbu ogohlantirishni tuzatish va iteratorni consume qilish uchun biz 12-bobda `env::args` bilan 12-1 ro'yxatda qo'llagan `collect` metodian foydalanamiz. Ushbu metod iteratorni consume qiladi va natijada olingan qiymatlarni ma'lumotlar to'plamiga(data type) to'playdi.

13-15 ro'yxatda biz vektorga `map`-ga chaqiruvdan qaytgan iterator bo'yicha takrorlash natijalarini yig'amiz. Ushbu vektor 1 ga oshirilgan asl vektorning har bir elementini o'z ichiga oladi.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-15: Yangi iterator yaratish uchun `map` metodini chaqirish va keyin yangi iteratorni consume qilish va vektor yaratish uchun `collect` metodini chaqirish</span>

`map` yopilganligi sababli, biz har bir elementda bajarmoqchi bo'lgan har qanday operatsiyani belgilashimiz mumkin. Bu `Iterator` traiti taʼminlaydigan iteratsiya xatti-harakatlarini(behavior) qayta ishlatishda closurelar sizga qandaydir behaviorlarni sozlash imkonini berishining ajoyib namunasidir.

Murakkab harakatlarni(complex action) o'qilishi mumkin bo'lgan tarzda bajarish uchun iterator adapterlariga bir nechta chaiquvlarni zanjirlashingiz(chain) mumkin. Ammo barcha iteratorlar dangasa bo'lgani uchun, iterator adapterlariga chaqiruvlardan natijalarni olish uchun consuming adapter metodlaridan birini chaqirishingiz kerak.

### Environmentni qamrab oladigan(capture) closurelardan foydalanish

Ko'pgina iterator adapterlari closurelarni argument sifatida qabul qiladilar va odatda biz iterator adapterlariga argument sifatida ko'rsatadigan closurelar ularning environmentini oladigan closurelar bo'ladi.

Ushbu misol uchun biz closureni oladigan `filter` metodidan foydalanamiz. Closure iteratordan element oladi va `bool` ni qaytaradi. Agar closure `true` qiymatini qaytarsa, qiymat `filtr` tomonidan ishlab chiqarilgan iteratsiyaga kiritiladi. Agar closure `false` bo'lsa, qiymat kiritilmaydi.

13-16 roʻyxatda biz `Poyabzal` structi misollari toʻplamini iteratsiya qilish  uchun uning environmentidan `poyabzal_olchami` oʻzgaruvchisini ushlaydigan(capture) closure bilan `filtr`dan foydalanamiz. U faqat belgilangan o'lchamdagi poyabzallarni qaytaradi.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

<span class="caption">Roʻyxat 13-16: `poyabzal_olchami`ni ushlaydigan closure bilan`filter` metodidan foydalanish</span>

`olcham_boyicha_poyabzal` funksiyasi parametr sifatida poyabzal vektori va poyabzal o'lchamiga egalik qiladi. U faqat belgilangan o'lchamdagi poyabzallarni o'z ichiga olgan vektorni qaytaradi.

`olcham_boyicha_poyabzal` bodysida(tanasida) vektorga ownershiplik(egalik) qiluvchi iterator yaratish uchun `into_iter` ni chaqiramiz. Keyin biz ushbu iteratorni faqat closure `true`ni qaytaradigan elementlarni o'z ichiga olgan yangi iteratorga moslashtirish uchun `filter` ni chaqiramiz.

Closure muhitdan `poyabzal_olchami` parametrini oladi va qiymatni har bir poyabzal o'lchami bilan solishtiradi, faqat belgilangan o'lchamdagi poyabzallarni saqlaydi. Nihoyat, `collect` ni chaqirish moslashtirilgan iterator tomonidan qaytarilgan qiymatlarni funksiya tomonidan qaytariladigan vektorga to'playdi.

Test shuni ko'rsatadiki, biz `olcham_boyicha_poyabzal` deb ataganimizda, biz faqat biz ko'rsatgan qiymat bilan bir xil o'lchamdagi poyabzallarni qaytarib olamiz.
