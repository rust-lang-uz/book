## Appendix G - Rust va "Nightly Rust" qanday yaratiladi

Ushbu ilova Rust qanday yaratilishi va bu sizga Rust dasturchi sifatida
qanday taʼsir qilishi haqida.

### Uzluksiz barqarorlik

Til sifatida Rust kodingizning barqarorligiga *katta* e’tibor qaratadi.
Rustʼni mustahkam poydevor sifatida qurishingiz mumkin boʻlgan mustahkam asos boʻlishini xohlaymiz, 
va agar narsalar doimiy ravishda oʻzgarib tursa, bu mumkin boʻlmas edi. Shu bilan birga, agar buni qila olmasak
yangi funksiyalar bilan tajriba oʻtkazganimizdan keyingina muhim muammolarni aniqlashimiz mumkin.
Ularning chiqarilish paytida, biz hech narsani o‘zgartira olmagan bo‘lar edik.

Bu muammoning yechimini "uzluksiz barqarorlik" deb ataymiz.
Asosiy tamoyilimiz quyidagicha: siz hech qachon Rustʼning yangi versiyasiga
oʻtishdan qoʻrqmasligingiz kerak.
Har bir yangilanish og‘riqsiz bo‘lishi kerak, lekin shu bilan yana birga
sizga yangi funksiyalar, kamroq xatolar va tezroq kompilyatsiya vaqti
berishi kerak.

### Chux-chux! Reliz kanallar va poyezdda sayr qilish

Rustʼni ishlab chiqish *grafik* bo‘yicha amalga oshiriladi. Ya’ni, butun ishlanma
Rust repozitoriysining `master` branchʼida o‘tkaziladi (hozirgi kunlarga
`main` branchʼiga o‘tdi).
Rustʼdagi relizlar, reliz poyezdi modeliga qatʼiy amal qiladi. Shu model Cisco IOS va boshqa loyihalarda ishlatadi.
Rustʼda uchta reliz kanallar bor:
* Nightly
* Beta
* Stable

Koʻp Rust dasturchilar asosan `stable` kanaldan foydalanadilar, lekin xohlovchilar
tajribaviy yangi funksiyalarni sinash uchun, `nightly` yoki `beta` versiyalardan foydalanishi mumkin.

Ishlab chiqish va reliz jarayoni qanday ishlashiga misol:
faraz qilaylik, Rust dasturchilari Rust 1.5 versiyasi ustida ishlashmoqda. Bu reliz
2015-yil dekabr oyida boʻlib o‘tdi, ammo u bizga haqiqiy versiya raqamlarini beradi.
Rustʼga yangi funksiya qoʻshildi: yangi kommit main branchʼiga tushadi.
Har kecha Rustning yangi `nightly` versiyasi yaratiladi. Har kuni - reliz
kuni va bu relizlar bizning reliz infratuzilmamiz tomonidan avtomatik ravishda yaratiladi.
Shunday qilib, vaqt oʻtgan sari, bizning relizlarimiz har tun quyidagi koʻrinishga ega boʻladi:

```text
nightly: * - - * - - *
```

Har olti hafta yangi reliz tayyorlash vaqti keladi! Rust repozitoriyasining `beta` branchʼi,
nightly tomonidan ishlatiladigan `main` branchʼidan ajralib chiqadi. Endi
ikkita reliz bor:

```text
nightly: * - - * - - *
                     |
beta:                *
```

Rust foydalanuvchilarining aksariyati beta-versiyalarni kundalik ishda ishlatmaydi, 
ammo mumkin bo‘lgan regressiyalarni aniqlashga yordam berish uchun ularni CI tizimlarida sinovdan o‘tkazadi. 
Vaholanki, har kecha avvalgidek `nightly` build chiqariladi:

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

Birinchi beta reliz yaratilganidan olti hafta o‘tgach, `stable`ni chiqarilish vaqti keldi!
`stable` branch `beta` branchdan yaratiladi:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Ura! Rust 1.5 tayyor! Biroq biz bir narsani unutib qoʻydik: 6 hafta oʻtib
ketgani sababli, bizga Rustʼning *keyingi* 1.6 versiyasining yangi beta
relizi kerak.
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

Rust har olti haftada soat kabi chiqariladi. Agar siz Rustning bitta relizni
sanasini bilsangiz, keyingi relizni sanasini bilib olishingiz mumkin: u olti
haftadan keyin boʻlib oʻtadi.  Relizlar har olti haftada chiqishini yaxshi
jihati shundaki, keyingi poyezd tez orada keladi.  Agar biror funksiya maʼlum
bir relizga kiritilmagan boʻlsa, tashvishlanmang: tez orada boshqasi keladi!
Bu reliz chiqish muddati yaqinlashganda, ehtimol, xomroq funksiyalarni
shoshilinch ravishda qoʻshishga boʻlgan bosimni kamaytirishga yordam beradi.

Ushbu jarayon tufayli siz har doim Rust’ning navbatdagi versiyasini tekshirib
koʻrishingiz va yangilanish qanchalik osonligiga shaxsan amin boʻlishingiz
mumkin: agar beta reliz kutilganidek ishlamasa, bu haqida jamoaga xabar
berishingiz va muammo keyingi barqaror reliz chiqqunga qadar tuzatilishiga
yordam berishingiz mumkin! Beta relizda nosozliklar yuz berishi nisbatan
kam uchraydigan holat, biroq `rustc` ham dasturiy taʼminot mahsulidir va
unda xatoliklar uchrab turadi.

### Beqaror funksiyalar

Ushbu reliz modelining yana bir oʻziga xos jihati bor: bular
beqaror funksiyalardir. Rust maʼlum bir relizda qaysi funksiyalar
faollashtirilganligini aniqlash uchun “feature flags” (funksiya
bayroqchalari) deb nomlangan usuldan foydalanadi. Agar yangi funksiya faol
ishlab chiqish jarayonida boʻlsa, u master branchʼiga, binobarin, nightly
reliziga joylanadi, biroq maxsus *feature flag* ostida. Agar siz foydalanuvchi
sifatida hali yakunlanmagan funksiyani sinab koʻrmoqchi boʻlsangiz, buni
amalga oshirishingiz mumkin. Buning uchun sizdan Rustʼning nightly relizidan
foydalanish va ushbu funksiyani faollashtirish uchun kodga tegishli bayroqchani
(flag) qoʻshish talab etiladi.

Agar Rustʼning `beta` yoki `stable` relizidan foydalansangiz, hech qanday funksiya bayroqlardan foydalana olmaysiz.
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

Koʻrib turibsizki, `stable` toolchain birlamchi (default) sifatida turibdi.
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
`rustup` siz `nightly` da ishlatayotganingizga ishonch komil qiladi (boshqa loyihalarga taʼsir qilmaydi, globalda `stable` turadi).
Sizda Rustʼda loyihalaringiz koʻp boʻlsa, bu juda qoʻl keladi!

### RFC Jarayonlar va Jamoalar

Yangi funksiyalar haqida qanday bilish mumkin? Rust ishlab chiqish modeli quyidagicha
*Fikr soʻrovi (RFC - request for comments) jarayoni* deb etiladi. Agar Rustni yaxshilamoqchi boʻlsangiz,
RFC deb nomlangan ariza yozishingiz mumkin.

Istalgan kishi Rustni yaxshilash uchun RFC yozishi mumkin va takliflar koʻrib chiqiladi va
turli mavzulardagi koʻplab kichik jamoalardan tashkil topgan Rust jamoasi tomonidan muhokama qilinadi. Toʻliq jamoalar roʻyxatini
Rust [veb-saytida](https://www.rust-lang.org/governance) topishingiz mumkin, bu yerda
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
