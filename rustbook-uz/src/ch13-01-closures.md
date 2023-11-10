<!-- Old heading. Do not remove or links may break. -->
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Closurelar: Environmentni qamrab oladigan anonim funksiyalar

Rustning closureri - bu o'zgaruvchida saqlashingiz yoki boshqa funksiyalarga argument sifatida o'tishingiz mumkin bo'lgan anonim funktsiyalar. Closureni bir joyda yaratishingiz va keyin uni boshqa kontekstda baholash uchun boshqa joyga murojaat qilishingiz mumkin. Funksiyalardan farqli o'laroq, closurelar ular belgilangan doiradagi qiymatlarni olishlari mumkin.
Ushbu closure xususiyatlari kodni qayta ishlatish va xatti-harakatlarni moslashtirishga(behavior customization) qanday imkon berishini ko'rsatamiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Environmentni closurelar bilan qo'lga olish

Avvalo, keyinchalik foydalanish uchun ular belgilangan muhitdan(environment) qiymatlarni olish uchun closurelardan qanday foydalanishimiz mumkinligini ko'rib chiqamiz.Bu senariy: Ko'pincha bizning futbolka kompaniyamiz reklama ro'yxatidagi kimgadir eksklyuziv, cheklangan nashrdagi futbolkani sovg'a sifatida taqdim etadi. Pochta ro'yxatidagi odamlar ixtiyoriy ravishda o'z profillariga sevimli ranglarini qo'shishlari mumkin. Agar bepul futbolka uchun tanlangan kishi o'zining sevimli ranglar to'plamiga ega bo'lsa, u rangdagi futbolkani oladi. Agar biror kishi sevimli rangni ko'rsatmagan bo'lsa, u kompaniyada eng ko'p bo'lgan rangni oladi.

Buni amalga oshirishning ko'plab usullari mavjud. Ushbu misol uchun biz `Qizil` va `Moviy` variantlariga ega `FutbolkaRangi` nomli enumdan foydalanamiz (oddiylik uchun mavjud ranglar sonini cheklaydi). Biz kompaniya inventarini `Inventarizatsiya` strukturasi bilan ifodalaymiz, unda `futbolkalar` deb nomlangan maydon mavjud bo‘lib, unda hozirda mavjud bo‘lgan futbolka ranglarini ifodalovchi `Vec<FutbolkaRangi>` mavjud.
`Inventarizatsiya` da belgilangan `yutuq` metodi bepul futbolka g‘olibining ixtiyoriy futbolka rangini afzal ko‘radi va odam oladigan futbolka rangini qaytaradi. Ushbu sozlash 13-1 ro'yxatda ko'rsatilgan:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Ro'yxat 13-1: Futbolka kompaniyasining sovg'a holati</span>

`main` boʻlimida belgilangan `dokon` ikkita moviy futbolka va bitta qizil futbolka qolgan. Qizil ko'ylakni afzal ko'rgan foydalanuvchi va hech qanday imtiyozsiz foydalanuvchi uchun `yutuq` metodini chaqiramiz.

Shunga qaramay, ushbu kod ko'p jihatdan amalga oshirilishi mumkin va bu yerda, closurelarga e'tibor qaratish uchun biz siz allaqachon o'rgangan tushunchalarga yopishib oldik, closuredan foydalanadigan `yutuq` metodidan tashqari. `yutuq` metodida biz `Option<FutbolkaRangi>` turidagi parametr sifatida foydalanuvchi imtiyozini olamiz va `foydalanuvchi_afzalligi` da `unwrap_or_else` metodini chaqiramiz. [`Option<T>` da `unwrap_or_else`][unwrap-or-else]<!-- ignore --> metodi standart kutubxona tomonidan aniqlanadi. Buning uchun bitta argument kerak bo‘ladi: `T` qiymatini qaytaruvchi hech qanday argumentsiz closure (`Option<T>` enumning `Some` variantida, bizning holatimizda `FutbolkaRangi`da tugaydigan qiymat turiga aylantiriladi). Agar `Option<T>` `Some` varianti bo'lsa, `unwrap_or_else` qiymatini `Some` ichidan qaytaradi. Agar `Option<T>` `None` varianti bo'lsa, `unwrap_or_else` closureni chaqiradi va closure orqali qaytarilgan qiymatni qaytaradi.

Biz closure ifodasini belgilaymiz `|| self.most_stocked()`ni `unwrap_or_else` argumenti sifatida. Bu hech qanday parametrlarni o'zi qabul qilmaydigan closuredir (agar closure parametrlari bo'lsa, ular ikkita vertikal chiziq orasida paydo bo'ladi). Closurening asosiy qismi `self.most_stocked()` ni chaqiradi. Biz bu yerda closureni aniqlayapmiz va `unwrap_or_else` ni amalga oshirish, agar natija kerak bo‘lsa, keyinroq closureni baholaydi.

Ushbu kodni ishga tushirsak quyidagi natijani chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Qiziqarli tomoni shundaki, biz joriy `Inventarizatsiya` misolida `self.most_stocked()` deb nomlanuvchi closuredan o‘tdik. Standart kutubxona biz belgilagan `Inventarizatsiya` yoki `FutbolkaRangi` turlari yoki biz ushbu senariyda foydalanmoqchi bo'lgan mantiq haqida hech narsa bilishi shart emas edi. Closure `self`  `Inventarizatsiya` misoliga o'zgarmas(immutable) referenceni oladi va uni biz belgilagan kod bilan `unwrap_or_else` metodiga uzatadi. Funksiyalar esa o'z muhitini(environmentini) shu tarzda ushlab tura olmaydi.

### Closure typi Inference va Annotation

Funksiyalar va closurelar o'rtasida ko'proq farqlar mavjud. Closurelar odatda parametrlar turlarini yoki `fn` funksiyalari kabi qaytarish qiymatini(return value) izohlashni talab qilmaydi. Funksiyalar uchun tur annotationlari talab qilinadi, chunki turlar foydalanuvchilarga ochiq interfeysning bir qismidir. Ushbu interfeysni qat'iy belgilash, har bir kishi funksiya qanday turdagi qiymatlardan foydalanishi va qaytarishi(return) haqida kelishib olishini ta'minlash uchun muhimdir. Boshqa tomondan, closurelar bu kabi ochiq interfeysda ishlatilmaydi: ular o'zgaruvchilarda saqlanadi va ularni nomlamasdan va kutubxonamiz foydalanuvchilariga ko'rsatmasdan foydalaniladi.

Closurelar odatda qisqa va har qanday ixtiyoriy senariyda emas, faqat tor kontekstda tegishli. Ushbu cheklangan kontekstlarda kompilyator ko'pgina o'zgaruvchilarning turlarini qanday aniqlashga qodir bo'lganiga o'xshab, parametrlarning turlarini va qaytish turini taxmin qilishi mumkin (kompilyatorga closure turi annotationlari ham kerak bo'lgan kamdan-kam holatlar mavjud).

O'zgaruvchilarda bo'lgani kabi, agar biz aniqlik va ravshanlikni oshirishni xohlasak, zarur bo'lgandan ko'ra batafsilroq bo'lish uchun turdagi annotationlarni qo'shishimiz mumkin. Closure uchun turlarga izoh(annotation) qo'yish 13-2 ro'yxatda ko'rsatilgan definitionga o'xshaydi. Ushbu misolda biz closureni aniqlaymiz va uni 13-1 ro'yxatda bo'lgani kabi argument sifatida topshirgan joyda closureni belgilash o'rniga uni o'zgaruvchida saqlaymiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-2: Ixtiyoriy turdagi annotationlarni qo'shish
closureda parametr va qaytariladigan qiymat turlari</span>

Turga annotationlar qoʻshilishi bilan closure sintaksisi funksiyalar sintaksisiga koʻproq oʻxshaydi. Bu yerda biz taqqoslash uchun parametriga 1 qo'shadigan funksiyani va bir xil xatti-harakatlarga ega bo'lgan closureni aniqlaymiz. Tegishli qismlarni bir qatorga qo'yish uchun bir nechta bo'shliqlar qo'shdik. Bu pipelardan foydalanish va ixtiyoriy bo'lgan sintaksis miqdori bundan mustasno, closure sintaksisi funksiya sintaksisiga qanchalik o'xshashligini ko'rsatadi:

```rust,ignore
fn  bitta_v1_qoshish    (x: u32) -> u32 { x + 1 }
let bitta_v2_qoshish =  |x: u32| -> u32 { x + 1 };
let bitta_v3_qoshish =  |x|             { x + 1 };
let bitta_v4_qoshish =  |x|               x + 1  ;
```

Birinchi qatorda funksiya taʼrifi(definition), ikkinchi qatorda esa toʻliq izohlangan closure definitioni koʻrsatilgan. Uchinchi qatorda biz closure definitiondan turdagi annotationlarni olib tashlaymiz. To'rtinchi qatorda biz qavslarni olib tashlaymiz, ular ixtiyoriy, chunki closure tanas(body) faqat bitta ifodaga(expression) ega. Bularning barchasi to'g'ri definitionlar bo'lib, ular chaqirilganda bir xil xatti-harakatlarni keltirib chiqaradi. `bitta_v3_qoshish` va `bitta_v4_qoshish` qatorlari kompilyatsiya qilish uchun closurelarni baholashni talab qiladi, chunki turlar ulardan foydalanishdan kelib chiqadi. Bu `let v = Vec::new();` ga o'xshash bo'lib, Rust turini aniqlay olishi uchun `Vec` ga turiga izohlar(annotation) yoki ba'zi turdagi qiymatlar kiritilishi kerak.

Closure definitionlari uchun kompilyator ularning har bir parametri va ularning qaytish(return) qiymati uchun bitta aniq turdagi xulosa chiqaradi. Masalan, 13-3 ro'yxatda parametr sifatida qabul qilingan qiymatni qaytaradigan qisqa closure definitioni ko'rsatilgan. Ushbu closure ushbu misol maqsadlaridan tashqari juda foydali emas. E'tibor bering, biz definitionga hech qanday annotation qo'shmaganmiz.
Hech qanday turdagi annotationlar mavjud emasligi sababli, biz bu yerda birinchi marta `String` bilan qilgan har qanday turdagi closureni chaqirishimiz mumkin. Agar biz `namuna_closure` ni butun(integer) son bilan chaqirishga harakat qilsak, xatoga yo'l qo'yamiz.

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Ro'yxat 13-3: Ikki xil turga ega bo'lgan closureni chaqirishga urinish</span>

Kompilyator bizga quyidagi xatoni beradi:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

Birinchi marta `namuna_closure` `String` qiymati bilan chaqirilganda, kompilyator `x` turini va  closurening qaytish turini `String` deb hisoblaydi. Keyin bu turlar(type) `namuna_closure` bo'limida yopiladi va biz bir xil closure(yopilish) bilan boshqa turdan foydalanishga uringanimizda xatoga duch kelamiz.

### Malumot olish yoki Egalik(Ownership) huquqini ko'chirish

Closurelar o'z muhitidan qiymatlarni uchta usulda olishlari mumkin, ular to'g'ridan-to'g'ri funksiya parametr olishi mumkin bo'lgan uchta usulga mos keladi: immutably borrowing (o'zgarmas borrowing(qarz olish)), mutably borrowing (o'zgaruvchan borrowing(qarz olish)) va egalik qilish(ownership). Closure funksiya tanasi(body) olingan qiymatlar bilan nima qilishiga qarab ulardan qaysi birini ishlatishni hal qiladi.

13-4 ro'yxatda biz `list` deb nomlangan vectorga immutable(o'zgarmas) referencei qamrab oluvchi closureni aniqlaymiz, chunki u qiymatni chop etish uchun faqat immutable referencega muhtoj:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">Ro'yxat 13-4: Buni closureni aniqlash va chaqirish
immutable referenceni ushlaydi</span>

Ushbu misol, shuningdek, o'zgaruvchining closure definitioniga bog'lanishi mumkinligini ko'rsatadi va biz keyinchalik o'zgaruvchi nomi va qavslar yordamida o'zgaruvchi nomi funksiya nomiga o'xshab yopishni chaqirishimiz mumkin.

Biz bir vaqtning o'zida bir nechta immutable(o'zgarmas) referencelarga ega bo'lishimiz mumkin bo'lgan `list` uchun, `list` closure definitionidan oldin, closure definitionidan keyin, lekin closure chaqirilishidan oldin va closure chaqirilgandan keyin hali ham koddan foydalanish mumkin. Ushbu kod kompilyatsiya bo'ladi, ishlaydi va chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Keyinchalik, 13-5 ro'yxatda biz closure bodysini `list` vectoriga element qo'shishi uchun o'zgartiramiz. Closure endi mutable(o'zgaruvchan) referenceni oladi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">Ro'yxat 13-5: Mutable(o'zgaruvchan) referenceni ushlaydigan closureni aniqlash va chaqirish</span>

Ushbu kod kompilyatsiya bo'ladi, ishlaydi va chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

E'tibor bering, `ozgaruvchan_borrow` closurening ta'rifi(definition) va chaqiruvi o'rtasida endi `println!` belgisi yo'q: `ozgaruvchan_borrow` aniqlanganda, u `list`ga o'zgaruvchan(mutable) referenceni oladi. Closure chaqirilgandan keyin biz closureni qayta ishlatmaymiz, shuning uchun mutable borrow(o'zgaruvchan qarz) tugaydi. Closure definationi va closure chaqiruvi o'rtasida chop etish uchun immutable(o'zgarmas) borrowga ruxsat berilmaydi, chunki mutable borrow mavjud bo'lganda boshqa borrowlarga ruxsat berilmaydi. Qaysi xato xabari borligini bilish uchun u yerga `println!` qo'shib ko'ring!

Agar closurening asosiy qismi ownershipga(egalik) muhtoj bo'lmasa ham, uni environmentda foydalanadigan qiymatlarga ownershiplik qilishga harakat qilmoqchi bo'lsangiz, parametrlar ro'yxatidan oldin `move` kalit so'zidan foydalanishingiz mumkin.

Ushbu uslub asosan ma'lumotlarni yangi threadga tegishli bo'lishi uchun ko'chirish uchun yangi threadga closureni o'tkazishda foydalidir. Biz 16-bobda parallellik(concurrency) haqida gapirganda, thereadlarni va nima uchun ulardan foydalanishni xohlashingizni batafsil muhokama qilamiz, ammo hozircha `move` kalit so'ziga muhtoj bo'lgan closure yordamida yangi threadni yaratishni qisqacha ko'rib chiqamiz. 13-6 ro'yxat vektorni asosiy thredda emas, balki yangi threadda chop etish uchun o'zgartirilgan 13-4 ro'yxatini ko'rsatadi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<span class="caption">Roʻyxat 13-6: `list`ga ownershiplik  qilish uchun threadni yopishni majburlash uchun `move` dan foydalanish</span>

Biz argument sifatida ishlash uchun threadni yopish(closure) imkonini berib, yangi threadni yaratamiz. Closure tanasi(body) listni chop etadi. Roʻyxat 13-4, closure faqat oʻzgarmas(immutable) reference yordamida `list`ni yozib oldi, chunki bu uni chop etish uchun zarur boʻlgan `list`ga kirishning eng kam miqdori. Ushbu misolda, closure tanasi(body) hali ham faqat o'zgarmas(immutable) referencega muhtoj bo'lsa ham, biz closure definationing boshiga `move` kalit so'zini qo'yish orqali `list` closurega ko'chirilishi kerakligini ko'rsatishimiz kerak. Yangi thread asosiy threadning qolgan qismi tugashidan oldin tugashi yoki asosiy thread birinchi bo'lib tugashi mumkin. Agar asosiy thread `list`ga ownershiplikni saqlab qolgan boʻlsa-da, lekin yangi thread paydo boʻlishidan oldin tugasa va `list`ni tashlab qoʻysa, threaddagi immutable(oʻzgarmas) reference yaroqsiz boʻladi. Shuning uchun, kompilyator `list`ni yangi threadga berilgan closurega ko'chirishni talab qiladi, shuning uchun reference haqiqiy bo'ladi. Kompilyatorda qanday xatolarga yo'l qo'yganingizni ko'rish uchun closure aniqlangandan so'ng, `move` kalit so'zini olib tashlang yoki asosiy threaddagi `list` dan foydalaning!

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Qabul qilingan qiymatlarni closuredan va `Fn` traitlaridan ko'chirish

Closure ma'lumotnomani qo'lga kiritgandan so'ng(shunday qilib, agar biror narsa bo'lsa, closurega ko'chirilgan narsaga ta'sir qiladi) yoki closure aniqlangan environmentdan qiymatga ownershiplikni qo'lga kiritgandan so'ng,(agar biror narsa bo'lsa, closuredan ko'chirilgan narsaga ta'sir qiladi) closurening asosiy qismidagi kod closure keyinroq baholanganda referencelar yoki qiymatlar bilan nima sodir bo'lishini belgilaydi. 

Closure tanasi(body) quyidagilardan birini amalga oshirishi mumkin: olingan qiymatni closuredan tashqariga ko'chirish(move), olingan qiymatni mutatsiyalash, qiymatni ko'chirish yoki mutatsiyalash yoki boshlash uchun environmentdan hech narsa olmaslik.

Yopishning environmentdan handlelarni ushlash(capture) va boshqarish usuli closure implementlarining qaysi traitlariga ta'sir qiladi va traitlar funksiyalar va structlar qanday closure turlaridan foydalanishi mumkinligini ko'rsatishi mumkin. Closurelar ushbu `Fn` belgilarining bittasi, ikkitasi yoki uchtasini avtomatik ravishda qo'shimcha usulda, closure tanasi qiymatlarni(value) qanday boshqarishiga qarab implement qilinadi:

1. `FnOnce` bir marta chaqirilishi mumkin bo'lgan closurelar uchun amal qiladi. Barcha closurelar hech bo'lmaganda ushbu traitni amalga oshiradi(implement qiladi), chunki barcha closurelar chaqirilishi mumkin. Qabul qilingan qiymatlarni(value) tanasidan tashqariga ko'chiradigan closure faqat `FnOnce` ni implement qiladi va boshqa `Fn` traitlarining hech birini implement qilmaydi, chunki uni faqat bir marta chaqirish mumkin.
2. `FnMut` qo'lga kiritilgan qiymatlarni(value) tanasidan tashqariga olib chiqmaydigan, lekin olingan qiymatlarni o'zgartirishi mumkin bo'lgan closurelarga nisbatan qo'llaniladi.Ushbu closurelarni bir necha marta chaqirish mumkin. 
3. `Fn` qo'lga kiritilgan qiymatlarni tanasidan tashqariga chiqarmaydigan va olingan qiymatlarni o'zgartirmaydigan closurelar, shuningdek, environmentdan hech narsani ushlab(capture) turmaydigan closurelar uchun amal qiladi. Ushbu closurelar environmentni o'zgartirmasdan bir necha marta chaqirilishi mumkin, bu bir vaqtning o'zida bir necha marta closureni chaqirish kabi holatlarda muhimdir.

Keling, 13-1 ro'yxatda biz qo'llagan `Option<T>` bo'yicha `unwrap_or_else` metodining definitionini ko'rib chiqaylik:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Eslatib oʻtamiz, `T` `Option`ning `Some` variantidagi qiymat turini ifodalovchi umumiy turdir(generic type). Bu `T` turi, shuningdek, `unwrap_or_else` funksiyasining qaytish(return) turidir: masalan, `Option<String>`da `unwrap_or_else` ni chaqiruvchi kod, `String` oladi.

Keyin, `unwrap_or_else` funksiyasi qo'shimcha `F` umumiy turdagi parametrga ega ekanligiga e'tibor bering. `F` turi `f` nomli parametrning turi(type) bo'lib, biz `unwrap_or_else` ga chaqiruv(call) qilganimizda ta`minlovchi closuredir.

Generic `F` turida belgilangan belgi `FnOnce() -> T` bo'lib, bu `F` bir marta chaqirilishi, hech qanday argumentga ega bo'lmasligi va `T` qaytarilishini bildiradi. Trait bound-da `FnOnce` dan foydalanish `unwrap_or_else` faqat bir marta `f` ni chaqirishi mumkin bo'lgan cheklovni ifodalaydi. `unwrap_or_else` matnida biz `Option` `Some` bo‘lsa, `f` chaqirilmasligini ko‘rishimiz mumkin. Agar `Option` `None` bo'lsa, `f` bir marta chaqiriladi. Barcha closurelar `FnOnce` ni implement qilganligi sababli, `unwrap_or_else` eng har xil turdagi closurelarni qabul qiladi va imkon qadar moslashuvchan.

> Eslatma: Funksiyalar uchta `Fn` traitlarini ham implement qilishi mumkin. Agar biz
> qilmoqchi bo'lgan narsa environmentdan qiymat olishni(*capture value) talab qilmasa,
> biz `Fn` traitlaridan birini implement qiladigan narsa kerak bo'lganda closure o'rniga
> funksiya nomidan foydalanishimiz mumkin. Masalan, `Option<Vec<T>>` qiymatida,
> agar qiymat `None` bo'lsa, yangi, bo'sh vektorni olish uchun `unwrap_or_else(Vec::new)` ni
> chaqirishimiz mumkin.

Endi keling, slicelarda aniqlangan standart kutubxona metodini ko‘rib chiqamiz, bu `unwrap_or_else`dan qanday farq qilishini va nima uchun  `sort_by_key` trait bound uchun `FnOnce` o‘rniga `FnMut` dan foydalanishini ko‘raylik. Closure ko'rib chiqilayotgan qismdagi joriy elementga reference ko'rinishida bitta argument oladi va order qilinishi mumkin bo'lgan `K` turidagi qiymatni qaytaradi. Ushbu funksiya har bir elementning ma'lum bir atributi bo'yicha sliceni saralashni xohlaganingizda foydalidir. 13-7 ro'yxatda bizda `Kvadrat` misollar listi mavjud va biz ularni `kenglik` atributi bo'yicha pastdan yuqoriga tartiblash uchun `sort_by_key` dan foydalanamiz:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">Ro'yxat 13-7: Kvadratlarlarni kengligi bo'yicha tartiblash uchun `sort_by_key` dan foydalaning</span>

Ushbu kod quyidagi natijani chop etadi:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key` `FnMut` closureni olish uchun aniqlanganining sababi shundaki, u closureni bir necha marta chaqiradi: slicedagi har bir element uchun bir marta. `|r| r.kengligi` o'z environmentidan hech narsani ushlamaydi(capture), mutatsiyaga uchramaydi yoki boshqa joyga ko'chirmaydi, shuning uchun u trait bound bo'lgan talablarga javob beradi.

Bundan farqli o'laroq, 13-8 ro'yxat faqat `FnOnce` traitini amalga oshiradigan closure misolini ko'rsatadi, chunki u qiymatni environmentdan tashqariga ko'chiradi. Kompilyator bu closureni `sort_by_key` bilan ishlatishimizga ruxsat bermaydi:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">Ro'yxat 13-8: `sort_by_key` yordamida `FnOnce` closuredan foydalanishga urinish</span>

Bu `list`ni saralashda `sort_by_key` necha marta chaqirilishini hisoblashning oʻylab topilgan (bu ishlamaydi) usulidir. Ushbu kod closure environmentidan  `qiymat`—a `String` ni `saralash_operatsiyalari` vektoriga surish(push) orqali hisoblashni amalga oshirishga harakat qiladi. Closure `qiymat`ni ushlaydi, so‘ngra `qiymat` ownershipligini `saralash_operatsiyalari` vektoriga o‘tkazish orqali `qiymat`ni closuredan chiqaradi. Ushbu closureni bir marta chaqirish mumkin; uni ikkinchi marta chaqirishga urinish ishlamaydi, chunki `qiymat` endi `saralash_operatsiyalari` ga push qilinadigan environmentda(muhitda) bo'lmaydi! Shuning uchun, bu closure faqat `FnOnce` ni amalga oshiradi(implement qiladi). Ushbu kodni kompilyatsiya qilmoqchi bo'lganimizda, biz `qiymat` ni closuredan chiqarib bo'lmaydigan xatoni olamiz, chunki closure `FnMut` ni implement qilishi kerak:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

Xato `qiymat`ni environmentdan tashqariga olib chiqadigan closure tanasidagi(body) chiziqqa(line) ishora qiladi. Buni tuzatish uchun biz closure tanasini qiymatlarni environmentdan ko'chirmasligi uchun o'zgartirishimiz kerak. `sort_by_key` necha marta chaqirilishini hisoblash uchun hisoblagichni(counter) environment saqlash va uning qiymatini closure tanasida oshirish buni hisoblashning yanada sodda usuli hisoblanadi. 13-9 ro'yxatdagi closure `sort_by_key` bilan ishlaydi, chunki u faqat `raqam_saralash_operatsiyalari` counteriga mutable(o'zgaruvchan) referenceni oladi va shuning uchun uni bir necha marta chaqirish mumkin:

<span class="filename">Fayl nomi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">Roʻyxat 13-9: `sort_by_key` bilan `FnMut` closuredan foydalanishga ruxsat berilgan</span>

`Fn` traitlari closurelardan foydalanadigan funksiyalar yoki turlarni belgilash yoki ishlatishda muhim ahamiyatga ega. Keyingi bo'limda biz iteratorlarni muhokama qilamiz. Ko'pgina iterator metodlari closure argumentlarini oladi, shuning uchun biz davom etayotganda ushbu closure tafsilotlarini(details) yodda tuting!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
