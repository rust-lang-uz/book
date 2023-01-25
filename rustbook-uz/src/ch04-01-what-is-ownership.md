## Ownership Nima?

*Ownership*(Egalik) bu Rust dasturi xotirani qanday boshqarishini boshqaradigan qoidalar to'plami.
Barcha dasturlar ishlayotgan vaqtda kompyuter xotirasidan qanday foydalanishini boshqarishi kerak.
Ba'zi tillarda axlat yig'ish mavjud bo'lib, ular dastur ishlayotgan paytda ishlatilmaydigan xotirani muntazam ravishda qidiradi; boshqa tillarda dasturchi xotirani aniq ajratishi va bo'shatishi kerak. Rust uchinchi yondashuvdan foydalanadi: xotira kompilyator tekshiradigan qoidalar to'plamiga ownership tizimi orqali boshqariladi. Agar biron bir qoidalar buzilgan bo'lsa, dastur kompilatsiya qilinmaydi. Ownership xususiyatlarining hech biri dasturingiz ishlayotgan vaqtda sekinlashtirmaydi.

Ownership ko'plab dasturchilar uchun yangi tushuncha bo'lganligi sababli, unga ko'nikish uchun biroz vaqt kerak bo'ladi. Yaxshi xabar shundaki, siz Rust va ownership tizimi qoidalari bilan qanchalik tajribali bo'lsangiz, xavfsiz va samarali kodni tabiiy ravishda ishlab chiqish osonroq bo'ladi. Unda davom etamiz!

Ownershipni tushunganingizda, Rustni noyob qiladigan xususiyatlarni tushunish uchun mustahkam asosga ega bo'lasiz. Ushbu bobda, siz juda keng tarqalgan ma'lumotlar tuzilishiga qaratilgan ba'zi misollarni orqali  ownershipni ishlashini o'rganasiz: string.

> ### Stack va Heap
>
> Ko'pgina dasturlash tillari stek va heap haqida tez-tez o'ylashingizni talab qilmaydi.
> Ammo Rust kabi tizim dasturlash tilida qiymat stekda yoki heapda bo'ladimi,
> til o'zini qanday tutishiga ta'sir qiladi va nima uchun siz ma'lum qarorlar
> qabul qilishingiz kerak. Ownershipning qismlari stek va heapga nisbatan keyinchalik
> ushbu bobda tasvirlanadi, shuning uchun bu yerda tayyorgarlik jarayonida qisqacha 
> tushuntirish berilgan.
>
> Stack ham, heap ham runtimeda foydalanish uchun kodingiz uchun mavjud bo'lgan 
> xotira qismlaridir, lekin ular turli yo'llar bilan tuzilgan. Stack qiymatlarni
> ularni olgan tartibda saqlaydi va qiymatlarni teskari tartibda o'chiradi
> Bu *oxirgi kelgan, birinchi chiqqan* deb ataladi. Plitalar stackini o'ylab
> ko'ring: ko'proq plastinka qo'shsangiz, ularni qoziqning ustiga qo'yasiz va plastinka
> kerak bo'lganda, siz yuqoridan birini olib qo'yasiz. Plitalarni o'rtadan yoki pastdan
> qo'shish yoki olib tashlash ham ishlamaydi! Ma'lumotlarni qo'shish *stekga qo'shish*,
> ma'lumotlarni olib tashlash esa *stekdan o'chirish* deb ataladi. Stackda saqlangan
> barcha ma'lumotlar ma'lum, qat'iy belgilangan hajmga ega bo'lishi kerak. Kompilyatsiya vaqtida
> noma'lum o'lchamli yoki o'zgarishi mumkin bo'lgan o'lchamdagi ma'lumotlar esa heapda
> saqlanishi kerak.
>
> heap kamroq tartibga solingan: ma'lumotlarni heapga qo'yganingizda, ma'lum miqdorda
> bo'sh joy talab qilasiz. Xotira ajratuvchisi heapda etarlicha katta bo'lgan bo'sh joyni
> topadi, uni ishlatilayotgan deb belgilaydi va o'sha joyning manzili bo'lgan
> *pointerni* ni qaytaradi. Bu jarayon *heap allocating* deb ataladi va ba'zan
> faqat *allocating* deb qisqartiriladi (qiymatlarni stekga qo'shish ajratish
> hisoblanmaydi). Heapga pointer ma'lum, qat'iy o'lcham bo'lgani uchun siz
> pointerni stekda saqlashingiz mumkin, lekin haqiqiy ma'lumotlarni
> olishni istasangiz, pointergaga amal qilishingiz kerak. Restoranda o'tirganingizni
> o'ylab ko'ring. Kirish paytida siz guruhingizdagi odamlar sonini bildirasiz
> va uy egasi hammaga mos keladigan bo'sh stol topadi va sizni u yerga olib boradi.
> Agar guruhingizdagi kimdir kechikib kelsa, sizni topish uchun qayerda o'tirganingizni
> so'rashi mumkin.
>
> Stekga qo'shish heapda allocating qilishdan tezroq bo'ladi, chunki allacator hech
> qachon yangi ma'lumotlarni saqlash uchun joy izlamasligi kerak; bu joy har doim
> stackning yuqori qismida joylashgan. Nisbatan, heapda bo'sh joy ajratish ko'proq
> mehnat talab qiladi, chunki allacator avval ma'lumotlarni saqlash uchun yetarlicha
> katta joy topishi va keyingi allocatinga tayyorgarlik ko'rish uchun buxgalteriya
> hisobini amalga oshirishi kerak.
>
> Heapdagi ma'lumotlarga kirish stekdagi ma'lumotlarga kirishdan ko'ra sekinroq, chunki u yerga 
> borish uchun pointerga amal qilishingiz kerak. Zamonaviy protsessorlar xotirada
> kamroq o'tishsa, tezroq ishlaydi. O'xshashlikni davom ettirib, ko'plab jadvallardan
> buyurtmalarni qabul qiladigan restoran serverini ko'rib chiqing. Keyingi stolga o'tishdan oldin
> barcha buyurtmalarni bitta stolda olish eng samarali hisoblanadi. A jadvalidan
> buyurtma olish, keyin B jadvalidan buyurtma olish, keyin yana A dan va yana B dan bitta
> buyurtma olish ancha sekinroq jarayon bo'ladi. Xuddi shu qoidaga ko'ra,
> protsessor uzoqroqda emas (u heapda bo'lishi mumkin) emas, balki boshqa
> ma'lumotlarga yaqin (stekdagi kabi) ma'lumotlarda ishlasa, o'z ishini yaxshiroq
> bajarishi mumkin.
>
> Sizning kodingiz funksiyani chaqirganda, funksiyaga o'tgan qiymatlar (shu jumladan, potentsial,
> heapdagi ma'lumotlarga pointerlar) va funksiyaning mahalliy o'zgaruvchilari
> stekga qo'shiladi. Funktsiya tugagach, bu qiymatlar stekdan chiqariladi.
>
> Kodning qaysi qismlari heapda qaysi ma'lumotlardan foydalanayotganini kuzatib borish,
> heapdagi takroriy ma'lumotlar miqdorini minimallashtirish va bo'sh joy qolmasligi uchun
> heapdagi foydalanilmagan ma'lumotlarni tozalash - bularning barchasi ownership hal qiladigan 
> muammolardir. Ownershipni tushunganingizdan so'ng, stek va heap haqida tez-tez
> o'ylashingiz shart emas, lekin ownership qilishning asosiy maqsadi heap
> ma'lumotlarni boshqarish ekanligini bilish uning nima uchun shunday ishlashini
> tushuntirishga yordam beradi.

### Ownership qoidalari

Birinchidan, ownership qoidalarini ko'rib chiqaylik.Biz ularni ko'rsatadigan misollar bilan ishlashda ushbu qoidalarni yodda tuting:

* Rust-dagi har bir qiymat *owner*ga ega.
* Bir vaqtning o'zida faqat bitta owneri bo'lishi mumkin.
* Owneri amaldan tashqariga chiqsa, qiymat o'chiriladi.

### O'zgaruvchan Scope

Endi biz Rustning asosiy sintaksisidan o‘tganimiz uchun, biz barcha `fn main() {` kodini misollarga kiritmaymiz, shuning uchun agar kuzatib boradigan bo‘lsangiz, quyidagi misollarni `main` funksiyasiga qo‘lda kiritganingizga ishonch hosil qiling. Natijada, bizning misollarimiz biroz ixchamroq bo'ladi, bu bizga qozon kodiga emas, balki haqiqiy tafsilotlarga e'tibor berishga imkon beradi.

Ownershipning birinchi misoli sifatida biz ba'zi o'zgaruvchilarning *scope*ni ko'rib chiqamiz. Scope - dastur doirasidagi element amal qiladigan diapazon. Quyidagi o'zgaruvchini oling:

```rust
let s = "salom";
```

`s` o'zgaruvchisi satr literaliga ishora qiladi, bu yerda satr qiymati dasturimiz matniga qattiq kodlangan. O'zgaruvchi e'lon qilingan paytdan boshlab joriy *scopning* oxirigacha amal qiladi. 4-1 ro'yxatida `s` o'zgaruvchisi qayerda to'g'ri bo'lishini izohlovchi izohlar bilan dastur ko'rsatilgan.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<span class="caption">Ro'yxat 4-1: O'zgaruvchi va uning amal qiladigan doirasi</span>

Boshqacha qilib aytganda, bu yerda ikkita muhim nuqta bor:

* Qachonki `s` *scopega* kirsa, u amal qiladi.
* U scopedan tashqariga *chiqmaguncha* amal qiladi.

Ushbu nuqtada, scopelar va o'zgaruvchilarning haqiqiyligi o'rtasidagi munosabatlar boshqa dasturlash tillaridagiga o'xshaydi. Endi biz `String` turini joriy qilish orqali ushbu tushunchaga asoslanamiz.

### `String` turi

Ownership qoidalarini tasvirlash uchun bizga 3-bobning [”Ma'lumotlar turlari”][data-types]<!-- ignore -->
bo'limida ko'rib chiqilganlarga qaraganda murakkabroq ma'lumotlar turi kerak. Oldin ko'rib chiqilgan turlar ma'lum o'lchamga ega bo'lib, ular stekda saqlanishi va qo'llanilish doirasi tugagach, stekdan o'chirilishi mumkin va agar kodning boshqa qismi foydalanishi kerak bo'lsa yangi, mustaqil misol yaratish uchun tez va ahamiyatsiz nusxa ko'chirilishi mumkin kodning boshqa qismi bir xil qiymatni boshqa doirada ishlatishi kerak. Ammo biz heapda saqlangan ma'lumotlarni ko'rib chiqmoqchimiz va Rust bu ma'lumotlarni qachon tozalashni bilishini o'rganmoqchimiz va `String` turi ajoyib misoldir.

Biz `String` ning ownership bilan bog'liq qismlariga e'tibor qaratamiz. Ushbu jihatlar standart kutubxona tomonidan taqdim etilganmi yoki siz yaratganmi, boshqa murakkab ma'lumotlar turlariga ham tegishli.
Biz [8-bobda][ch8]<!-- ignore --> `String`ni chuqurroq muhokama qilamiz.

Biz allaqachon string literallarini ko'rdik, bu erda string qiymati bizning dasturimizga qattiq kodlangan. String literallari qulay, ammo ular biz matndan foydalanmoqchi bo'lgan har qanday vaziyatga mos kelmaydi. Buning sabablaridan biri shundaki, ular o'zgarmasdir. Yana bir narsa shundaki, biz kodni yozganimizda har bir satr qiymatini bilish mumkin emas: masalan, agar biz foydalanuvchi ma'lumotlarini olib, uni saqlamoqchi bo'lsak-chi? Bunday holatlar uchun Rust ikkinchi string turiga ega, `String`. Bu tur heapda ajratilgan ma'lumotlarni boshqaradi va shuning uchun kompilyatsiya vaqtida bizga noma'lum bo'lgan matn miqdorini saqlashi mumkin. Siz `from` funksiyasidan foydalanib satr literalidan `String` yaratishingiz mumkin, masalan:

```rust
let s = String::from("salom");
```

Ikki nuqtali `::` operatori bizga `string_from` kabi qandaydir nomdan foydalanish o'rniga `String` turi ostida ushbu `from` funksiyasini nom maydoniga qo`yish imkonini beradi.
Biz ushbu sintaksisni 5-bobning [”Method sintaksisi”][method-syntax]<!-- ignore --> bo'limida ko'proq muhokama qilamiz va 7-bobdagi [”Modul treedagi elementga murojaat qilish yo'llari”][paths-module-tree]<!-- ignore --> da modullar bilan nomlar oralig'i haqida gapiramiz.

Ushbu turdagi *string* mutatsiyaga uchragan bo'lishi mumkin:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Xo'sh, bu erda qanday farq bor? Nima uchun `String` ni mutatsiyaga solish mumkin, lekin harflarni o'zgartirish mumkin emas? Farqi bu ikki turning xotira bilan qanday munosabatda bo'lishida.

### Xotira va Taqsimlash

String literalida biz kompilyatsiya vaqtida tarkibni bilamiz, shuning uchun matn to'g'ridan-to'g'ri yakuniy bajariladigan faylga qattiq kodlangan.Shuning uchun string literallari tez va samarali. Ammo bu xususiyatlar faqat satr literalining o'zgarmasligidan kelib chiqadi. Afsuski, kompilyatsiya vaqtida hajmi noma'lum bo'lgan va dasturni ishga tushirishda hajmi o'zgarishi mumkin bo'lgan har bir matn bo'lagi uchun biz binary faylga bir bo'lak xotira qo'ya olmaymiz.

`String` turida o'zgaruvchan, o'sib boradigan matn qismini qo'llab-quvvatlash uchun tarkibni saqlash uchun kompilyatsiya vaqtida noma'lum bo'lgan xotira hajmini yig'ishda ajratishimiz kerak. Buning ma'nosi:

* Xotira runtimeda xotira allactoridan so'ralishi kerak.
* `String` bilan ishlashni tugatgandan so'ng, bizga ushbu xotirani allacatoriga qaytarish usuli kerak.

Bu birinchi qism biz tomonimizdan amalga oshiriladi: biz `String::from` deb chaqirganimizda, uni implementi kerakli xotirani talab qiladi. Bu dasturlash tillarida deyarli universaldir.

Biroq, ikkinchi qism boshqacha. *Garbage Collector (GC)* bo'lgan tillarda GC endi ishlatilmayotgan xotirani kuzatib boradi va tozalaydi va bu haqda o'ylashimiz shart emas. Ko'pgina tillarda GC bo'lmaganda, xotiradan qachon foydalanilmayotganini aniqlash va uni aniq bo'shatish uchun kodni chaqirish, xuddi biz so'raganimizdek, bizning burchimizdir. Buni to'g'ri bajarish tarixan qiyin dasturlash muammosi bo'lgan. Agar unutsak, xotirani behuda sarflaymiz. Agar biz buni juda erta qilsak, bizda noto'g'ri o'zgaruvchi bo'ladi. Agar buni ikki marta qilsak, bu ham xato. Aynan bitta `allocate`ni bitta `bo'sh` bilan birlashtirishimiz kerak.

Rust boshqa yo'lni egallaydi: unga ega bo'lgan o'zgaruvchi amaldan tashqariga chiqqandan so'ng xotira avtomatik ravishda qaytariladi. Bu yerda 4-1 roʻyxatdagi misolimiz satr harfi oʻrniga `String` yordamida berilgan:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

Biz `String` kerak bo'lgan xotirani ajratuvchiga qaytarishimiz mumkin bo'lgan tabiiy nuqta bor: `s` scopedan chiqib ketganda. O'zgaruvchi scopedan chiqib ketganda, Rust biz uchun maxsus funksiyani chaqiradi.Ushbu funktsiya [`drop`][drop]<!-- ignore --> deb ataladi va u erda `String` muallifi xotirani qaytarish uchun kodni qo'yishi mumkin. Rust yopilgan jingalak qavsda avtomatik ravishda `drop` ni chaqiradi.

> Eslatma: C++ da, elementning ishlash muddati oxirida resurslarni taqsimlashning bunday sxemasi ba'zan
> *Resource Acquisition Is Initialization (RAII)*(Resurslarni yig'ish - ishga tushirish (RAII) deb ataladi.
> Agar siz RAII patternlaridan foydalangan bo'lsangiz, Rust-dagi `drop`
> funksiyasi sizga tanish bo'ladi.

Ushbu pattern Rust kodini yozish usuliga chuqur ta'sir qiladi. Bu hozir oddiy bo'lib tuyulishi mumkin, ammo biz bir nechta o'zgaruvchilar biz yig'ilgan ma'lumotlardan foydalanishni xohlayotganimizda, murakkabroq holatlarda kodning harakati kutilmagan bo'lishi mumkin. Keling, ushbu vaziyatlarning ba'zilarini ko'rib chiqaylik.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-move"></a>

#### Move bilan o'zaro ta'sir qiluvchi o'zgaruvchilar va ma'lumotlar

Rustda bir nechta o'zgaruvchilar bir xil ma'lumotlar bilan turli yo'llar bilan o'zaro ta'sir qilishi mumkin.
4-2 ro'yxatda integer sondan foydalanish misolini ko'rib chiqaylik.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<span class="caption">4-2 roʻyxat: `x` oʻzgaruvchisining butun qiymatini `y` ga belgilash</span>

Bu nima qilayotganini taxmin qilishimiz mumkin: `5` qiymatini `x` ga bog‘lang; keyin `x` dagi qiymatdan nusxa oling va uni `y` ga bog'lang. Endi bizda ikkita o'zgaruvchi bor, `x` va `y` va ikkalasi ham `5` ga teng. Bu haqiqatan ham sodir bo'lmoqda, chunki butun sonlar ma'lum, qat'iy o'lchamga ega oddiy qiymatlardir va bu ikkita `5` qiymat stekga qo'shiladi.

Endi `String` versiyasini ko'rib chiqamiz:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Bu juda o'xshash ko'rinadi, shuning uchun biz uning ishlash usuli bir xil bo'ladi deb taxmin qilishimiz mumkin: ya'ni ikkinchi qator `s1` qiymatining nusxasini yaratadi va uni `s2` bilan bog'laydi. Ammo bu sodir bo'ladigan narsa emas.

Qopqoq ostidagi `String` bilan nima sodir bo'layotganini ko'rish uchun 4-1-rasmga qarang. `String` chap tomonda ko'rsatilgan uchta qismdan iborat: satr tarkibini saqlaydigan xotiraga ko'rsatgich, uzunlik(len) va sig'im(capacity).
Ushbu ma'lumotlar guruhi stekda saqlanadi. O'ng tomonda tarkibni saqlaydigan heap xotira joylashgan.

<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">4-1-rasm: `s1` ga bog‘langan `salom` qiymatiga ega `String` xotirasidagi tasvir</span>

Uzunlik - `String` mazmuni hozirda qancha xotira, baytlarda foydalanayotganligi. Sig'im(capacity) - bu `String` allacatordan olgan xotiraning umumiy hajmi, baytlarda. Uzunlik va si'gimlar o'rtasidagi farq muhim, ammo bu kontekstda emas, shuning uchun hozircha si'gimlarni e'tiborsiz qoldirish yaxshi.

`s1` ni `s2` ga belgilaganimizda, `String` ma'lumotlari nusxalanadi, ya'ni biz stekdagi pointer, uzunlik va sig`imdan nusxa olamiz. Biz pointer(ko'rsatkich) ko'rsatgan to'plamdagi ma'lumotlarni ko'chirmaymiz. Boshqacha qilib aytganda, ma'lumotlarning xotirada ko'rinishi 4-2-rasmga o'xshaydi.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">4-2-rasm: `s1` pointeri, uzunligi va sigʻimi nusxasiga ega `s2` oʻzgaruvchisi xotirasida koʻrsatilishi</span>

Tasvir 4-3-rasmga *o'xshamaydi*, agar Rust o'rniga heap ma'lumotlarni ko'chirsa, xotira qanday ko'rinishga ega bo'lardi. Agar Rust buni amalga oshirgan bo'lsa, `s2 = s1` operatsiyasi, agar heapdagi ma'lumotlar katta bo'lsa, runtimening ishlashi nuqtai nazaridan juda qimmat bo'lishi mumkin.

<img alt="Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">4-3-rasm: Rust heap ma'lumotlarni ham nusxalagan bo'lsa, `s2 = s1` nima qilishi mumkin bo'lgan yana bir imkoniyat</span>

Avvalroq biz aytgan edikki, o‘zgaruvchi qo‘llanish doirasidan chiqib ketganda, Rust avtomatik ravishda `drop` funksiyasini chaqiradi va bu o‘zgaruvchi uchun heap xotirani tozalaydi. Ammo 4-2-rasmda ikkala ma'lumot pointeri bir xil joyga ishora qiladi. Bu muammo: `s2` va `s1` scopedan chiqib ketganda, ikkalasi ham bir xil xotirani bo'shatishga harakat qiladi. Bu *double free*(ikki marta bo'sh)xato sifatida tanilgan va biz avval aytib o'tgan xotira xavfsizligi xatolaridan biridir. Xotirani ikki marta bo'shatish xotira buzilishiga olib kelishi mumkin, bu esa xavfsizlik zaifliklariga olib kelishi mumkin.

Xotira xavfsizligini ta'minlash uchun `let s2 = s1;` qatoridan keyin Rust `s1` ni endi yaroqsiz deb hisoblaydi. Shuning uchun, `s1` qo'llanilgandan tashqariga chiqqanda Rust hech narsani bo'shatishi shart emas. `s2` yaratilgandan keyin `s1` dan foydalanmoqchi bo'lganingizda nima sodir bo`lishini tekshiring; u ishlamaydi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Siz shunday xatoga yo'l qo'yasiz, chunki Rust bekor qilingan havoladan foydalanishga to'sqinlik qiladi:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

Agar siz boshqa tillar bilan ishlashda *shallow copy* va *deep copy* so‘zlarini eshitgan bo‘lsangiz, pointerni nusxalash tushunchasi, ma'lumotlardan nusxa ko'chirmasdan uzunligi va sig'imi olish, ehtimol shallow copy kabi eshitiladi. Ammo Rust birinchi o'zgaruvchini ham bekor qilganligi sababli, shallow copy deb nomlanish o'rniga u *move*(ko'chirish) deb nomlanadi. Bu misolda `s1` `s2` ga *ko'chirilgan* deb aytamiz. Shunday qilib, aslida nima sodir bo'lishi 4-4-rasmda ko'rsatilgan.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">4-4-rasm: `s1` dan keyin xotiradagi ko`rinish bekor qilingan</span>

Bu bizning muammomizni hal qiladi! Faqatgina `s2` amal qilganda, u scopedan tashqariga chiqsa, u faqat xotirani bo'shatadi va biz tugatdik.

Bundan tashqari, dizayn tanlovi ham mavjud: Rust hech qachon avtomatik ravishda ma'lumotlaringizning "deep copyni" yaratmaydi. Shuning uchun, har qanday *avtomatik* nusxa ko'chirish runtimening ishlashi nuqtai nazaridan arzon deb taxmin qilish mumkin.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-clone"></a>

#### Clone bilan o'zaro ta'sir qiluvchi o'zgaruvchilar va ma'lumotlar

If we *do* want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We’ll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you’ve probably seen them before.

Here’s an example of the `clone` method in action:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

This works just fine and explicitly produces the behavior shown in Figure 4-3,
where the heap data *does* get copied.

When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It’s a visual indicator that something
different is going on.

#### Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code using
integers—part of which was shown in Listing 4-2—works and is valid:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

But this code seems to contradict what we just learned: we don’t have a call to
`clone`, but `x` is still valid and wasn’t moved into `y`.

The reason is that types such as integers that have a known size at compile
time are stored entirely on the stack, so copies of the actual values are quick
to make. That means there’s no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there’s no difference
between deep and shallow copying here, so calling `clone` wouldn’t do anything
different from the usual shallow copying, and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on
types that are stored on the stack, as integers are (we’ll talk more about
traits in [Chapter 10][traits]<!-- ignore -->). If a type implements the `Copy`
trait, variables that use it do not move, but rather are trivially copied,
making them still valid after assignment to another variable.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts,
has implemented the `Drop` trait. If the type needs something special to happen
when the value goes out of scope and we add the `Copy` annotation to that type,
we’ll get a compile-time error. To learn about how to add the `Copy` annotation
to your type to implement the trait, see [“Derivable
Traits”][derivable-traits]<!-- ignore --> in Appendix C.

So, what types implement the `Copy` trait? You can check the documentation for
the given type to be sure, but as a general rule, any group of simple scalar
values can implement `Copy`, and nothing that requires allocation or is some
form of resource can implement `Copy`. Here are some of the types that
implement `Copy`:

* All the integer types, such as `u32`.
* The Boolean type, `bool`, with values `true` and `false`.
* All the floating-point types, such as `f64`.
* The character type, `char`.
* Tuples, if they only contain types that also implement `Copy`. For example,
  `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

### Ownership and Functions

The mechanics of passing a value to a function are similar to those when
assigning a value to a variable. Passing a variable to a function will move or
copy, just as assignment does. Listing 4-3 has an example with some annotations
showing where variables go into and out of scope.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<span class="caption">Listing 4-3: Functions with ownership and scope
annotated</span>

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile-time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Listing 4-4 shows an example of a
function that returns some value, with similar annotations as those in Listing
4-3.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<span class="caption">Listing 4-4: Transferring ownership of return
values</span>

The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless ownership
of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every
function is a bit tedious. What if we want to let a function use a value but
not take ownership? It’s quite annoying that anything we pass in also needs to
be passed back if we want to use it again, in addition to any data resulting
from the body of the function that we might want to return as well.

Rust does let us return multiple values using a tuple, as shown in Listing 4-5.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<span class="caption">Listing 4-5: Returning ownership of parameters</span>

But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for using a value without
transferring ownership, called *references*.

[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: ../std/ops/trait.Drop.html#tymethod.drop
