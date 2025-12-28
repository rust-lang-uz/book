## Appendix G - Rust va "Nightly Rust" qanday yaratiladi

Ushbu ilova Rust qanday yaratilishi va bu sizga Rust dasturchi sifatida
qanday taʼsir qilishi haqida.

### Uzluksiz barqarorlik

Til sifatida Rust kodingizning barqarorligiga *katta* e’tibor qaratadi. Biz xohlaymizki
Rustʼni mustahkam poydevor sifatida qurishingiz mumkin boʻlgan mustahkam asos boʻlishini xohlaymiz, 
va agar narsalar doimiy ravishda oʻzgarib tursa, bu mumkin boʻlmas edi. Shu bilan birga, agar buni qila olmasak
yangi funksiyalar bilan tajriba oʻtkazganimizdan keyingina muhim muammolarni aniqlashimiz mumkin.
Ularning chiqarilish paytida, biz hech narsani o‘zgartira olmagan bo‘lar edik.

Bu muammoning yechimini "uzluksiz barqarorlik" deb ataymiz.
Asosiy tamoyilimiz quyidagicha: siz hech qachon Rustʼning yangi versiyasiga
oʻtishdan qoʻrqmasligingiz kerak.
Har bir yangilanish og‘riqsiz bo‘lishi kerak, lekin shu bilan yana birga
sizga yangi funksiyalar, kamroq xatolar va tezroq compile time (kompilyatsiya vaqti) berishi kerak.

### Chux-chux! Chiqarish kanallar va poyezdda sayr qilmoq

Rustni ishlab chiqish *grafik* bo‘yicha amalga oshiriladi. Ya’ni, butun ishlanma
Rust repozitoriysining `master` branchda o‘tkaziladi (hozirgi kunlarga `main` branchga o‘tdi).
Rustʼdagi relizlar, reliz poyezdi modeliga qatʼiy amal qiladi. Shu model Cisco IOS va boshqa loyihalarda ishlatadi.
Rustʼda uchta reliz kanallar bor:
* Nightly
* Beta
* Stable

Ko'p Rust dasturchilar asosan `stable` kanaldan foydalanadilar, lekin xohlovchilar
tajribaviy yangi funksiyalarni sinash, `nightly` yoki `beta` versiyalardan foydalanishi mumkin.

Ishlab chiqish va reliz jarayoni qanday ishlashiga misol:
faraz qilaylik, Rust dasturchilari Rust 1.5 versiyasi ustida ishlashmoqda. Bu reliz
2015-yil dekabr oyida boʻlib o‘tdi, ammo u bizga haqiqiy versiya raqamlarini beradi.
Rustʼga yangi funksiya qoʻshildi: yangi kommit main branchʼiga tushadi.
Har kecha Rustning yangi `nightly` versiyasi yaratiladi. Har kuni - bu
kuni va bu relizlar bizning reliz infratuzilmamiz tomonidan avtomatik ravishda yaratiladi.
Shunday qilib, vaqt o‘tishi bilan bizning relizlarimiz quyidagicha ko‘rinadi, bir kecha-kunduzda bir marta:

```text
nightly: * - - * - - *
```

Har olti haftada yangi reliz tayyorlash vaqti keladi! Rust repositoriy `beta` branch,
nightly tomonidan ishlatiladigan `main` branchidan ajralib turadi. Endi
ikkita reliz bor:

```text
nightly: * - - * - - *
                     |
beta:                *
```

K'op Rust foydalanuvchilari beta-versiyalardan faol foydalanmaydi, lekin ularni da sinovdan o‘tkazadi
Rustga mumkin bo‘lgan regressiyalarni aniqlashga yordam berish uchun o‘zlarining CI tizimlarida ishlatadi. Vaholanki,
har kecha avvalgidek `nightly` build chiqariladi:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Aytaylik, regressiya topildi. Yaxshiyam beta-versiyani sinab ko‘rishga vaqtimiz boʻldi,
Regressiya barqaror versiyaga kirishdan oldin! Tuzatish tatbiq
`main` branch ga qilinadi, shuning uchun `nightly` tuzatildi va keyin tuzatish ga koʻchiriladi
`beta` branchga va yangi beta versiyasi chiqariladi:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Birinchi beta-versiya yaratilganidan olti hafta o‘tgach, `stable`ni chiqarilish vaqti keldi!
`stable` branch `beta` branchdan yaratiladi:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Ura! Rust 1.5 tayyor! Biroq biz bir narsani unutib qoʻydik: chunki 6 haft o'tgandan deb, 
bizga Rustning *keyingi* versiyasining yangi beta-versiyasi ham kerak, 1.6.
Shuning uchun `stable` `beta` dan ajralgandan so‘ng, `beta` ning keyingi versiyasi `nightly` dan yana ajralib chiqadi:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Bu "poyezd modeli" deb ataladi, chunki har olti haftada reliz "bekatdan joʻnatiladi",
lekin hali ham beta-kanal orqali oʻtishi kerak, `stable` relizga aylanish uchun.

Rust har olti haftada soat kabi chiqariladi. Agar siz Rustning bitta relizni sanasini bilsangiz,
keyingi relizni sanasini bilib olishingiz mumkin: u olti haftadan keyin boʻlib oʻtadi. Yoqimli
har olti haftada chiqarishning bir jihati shundaki, keyingi poyezd
tez orada keladi. Agar biror funksiya maʼlum bir relizga kiritilmagan boʻlsa, buning keragi yoʻq
tashvishlanmoq: qisqa vaqt ichida boshqasi boʻladi! Bu tazyiqni kamaytirishga yordam beradi,
chiqarishdan oldingi soʻnggi lahzada, ehtimol, toʻliq ishlab chiqilmagan funksiyalarni kiritish zarurati bilan bogʻliq.

Ushbu jarayon tufayli siz har doim quyidagi Rust buildni tekshirishingiz mumkin va
uni yangilash osonligiga ishonch hosil qiling: agar beta-versiya
kutilganidek ishlamasa, siz bu haqda jamoaga xabar berishingiz va xatosini tuzatishingiz mumkin
keyingi stable relizdan oldin! Beta-versiyada nosozliklar muammolar kam uchraydi, lekin
`rustc` baribir dasturiy ta’minot va unda xatolar mavjud.

### Beqaror funksiyalar

Ushbu chiqarish modeli bilan yana bir muammo bor: beqaror funksiyalar. Rust ishlatiladi
funksiyada qanday funksiyalar yoqilganini aniqlash uchun "feature bayroqchalar" deb nomlangan narsa bor
. Agar yangi funksiya faol ishlab chiqilayotgan bo‘lsa, u `main`ga tushadi
, va shuning uchun, `nightly` da b‘oladi, lekin *feature bayrogʻni* ortida. Agar siz foydalanuvchi sifatida 
ishlab chiqilayotgan funksiyani sinab koʻrmoqchi boʻlsangiz, buni qilishingiz mumkin, 
lekin buning uchun siz Rust `nightly` builddan foydalanishingiz va ushbu funksiyani yoqish uchun tegishli
bayroq yordamida manba kodingizni annotate qilishiz kerak.

Agar Rustning `beta` yoki `stable` versiyasidan foydalansangiz, hech qanday feature bayroqlardan foydalana olmaysiz.
Bu kalit bizga yangi funksiyalarni amalda qoʻllash imkonini beradi,
Biz ularni abadiy barqaror (stable) deb eʼlon qilishimizdan oldin. 
Eng soʻnggi versiyaga oʻtishni xohlovchilar buni amalga oshirishlari mumkin,
ishonchli tajribaga ega boʻlishni dasturchilar esa `stable` versiyada qolishlari
va kodlari buzilmasligiga ishonch hosil qilishlari mumkin. Uzluksiz barqarorlik.

Ushbu kitobda faqat `stable` funksiyalar haqida maʼlumot berilgan, chunki ishlab chiqilayotgan funksiyalar
ular hali ham oʻzgarib bormoqda va, albatta, ushbu kitob yozilgan paytdan va ular barqaror toʻplamlarga 
kiritilgan paytdan farq qiladi.
Internetda `nightly` jamlanmalarda mavjud funksiyalar hujjatlarini topishingiz mumkin

### Rustup va Rust Nightly ni oʻrni

Rustup Rustni reliz kanallarini oʻrtasida almashtirishni osonlashtiradi, 
global yoki per-project basisda. Standart holatda sizda `stable` Rust oʻrnatiladi.
`nightly`ni oʻrnatish uchun, masalan:

```console
$ rustup toolchain install nightly
```

Shuningdek, siz barcha `rustup` bilan oʻrnatilgan *toolchainlar* (Rustni relizlar va uni aloqador komponentlar) koʻrishingiz mumkin.
Bizni mualliflarimizdan birning Windows kompyuteri dan:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Koʻrib turibsizki, `stable` toolchain odatiy (default) sifatida turibdi.
Kʻop Rust foydalanuvchilar, ko‘pincha vaqtida `stable` ni ishlatadi.
Masalan, siz `stable` ni koʻpincha vaqt ishlatmoqchisiz, lekin bitta aniq proektda
`nightly` ni ishlatmoqchisiz, chunki sizga yangi, yana stable ga chiqmagan funksiya
kerak boʻlib qoladi. Shuni qilish uchun, `rustup override` komanda ni proektizni direktoriyasi ichida,
`nightly` toolchain qoyib, rustup uni oʻsha direktoriyada ishlatish uchun, qilishiz mumkin:

```console
$ cd ~/proektlarim/nightly-kerak-narsa
$ rustup override set nightly
```

Endi, har doim, `rustc` yoki `cargo` shu direktoriya (*~/proektlarim/nightly-kerak-narsa*) ni ichida chaqirganizda
`rustup` siz `nightly` da ishlayotganingizni aniqlantiradi (boshqa proektlarga tasir qilmaydi, global da `stable` turadi).
Sizda koʻp Rust dagi proektlar holatida, bu juda foydalik bʻoladi!

### RFC Jarayonlar va Jamoalar

Yangi funksiyalar haqida qanday bilish mumkin? Rust ishlab chiqish modeli quyidagicha
*Fikr soʻrovi (RFC - request for comments) jarayoni* deb etiladi. Agar Rustni yaxshilamoqchi boʻlsangiz,
RFC deb nomlangan jumla yozishingiz mumkin.

Istalgan kishi Rustni yaxshilash uchun RFC yozishi mumkin va takliflar koʻrib chiqiladi va
turli mavzulardagi koʻplab kichik jamoalardan tashkil topgan Rust jamoasi tomonidan muhokama qilinadi. Toʻliq buyruqlar ro‘yxati
Rust [web-saytida](https://www.rust-lang.org/governance) topishingiz mumkin, bu yerda
loyihaning har bir sohasi: tilni loyihlantirish, compiler ni amalga oshirish,
infratuzilma, documentatsiya va boshqalar. Tegishli jamoa oʻqiydi
taklif va sharhlar, oʻz sharhlarini yozadi va oxir-oqibat,
funksiyani qabul qilish yoki rad etish toʻgʻrisida konsensusga keladi.

Agar funksiya qabul qilinsa, Rust repozitoriysida issue ochiladi va uni kimdir amalga oshirishi mumkin.
Uni amalga oshiradigan odam bu funksiyani birinchi boʻlib taklif qilgan odam boʻlmasligi mumkin!
Amalga oshirish tugallangach, u funksiya shlyuzi ortidagi `main` branchga tushadi,
buni biz ["Beqaror funksiyalar"](#beqaror-funksiyalar) boʻlimida muhokama qilgan edik <!-- ignore -->.

Biroz vaqt oʻtgach, `nightly` relizlardan foydalanadigan Rust dasturchilari yangi funksiyani sinab koʻrishlari mumkin boʻlganidan soʻng, 
jamoa aʼzolari uni qanday ishlatishini muhokama qiladilar.
`nightly` versiyalarda ishlaydi va uni Rustning `stable` versiyasiga kiritish yoki kiritmaslik toʻgʻrisida qaror qabul qiladi.
Agar targʻibot haqida qaror qabul qilinsa, funksiya `feature gate` olinadi va endi u `stable` hisoblanadi! 
U yangi Rust `stable` relizga kiradi.
