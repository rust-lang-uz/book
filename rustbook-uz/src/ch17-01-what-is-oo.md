## Obyektga Yo'naltirilgan Dasturlash Xususiyatlari

Dasturlash jamiyatida bir tilni obyektga yo'naltirilgan deb hisoblash uchun qanday xususiyatlarga ega bo'lishi kerakligi haqida kelishuv yo'q. Rust ko'p
dasturlash paradigmalaridan, jumladan OOPdan ilhomlangan; masalan, 13-bobda funksional dasturlashdan kelgan xususiyatlarni o'rgandik. Ehtimol, OOP
tillari ma'lum umumiy xususiyatlarga ega, ya'ni obyektlar, inkapsulyatsiya va meros. Keling, har bir xususiyatning nimani anglatishini va Rust uni
qo'llab-quvvatlaydimi yoki yo'qligini ko'rib chiqaylik.

### Obyektlar Ma'lumotlar va Xatti-harakatlarni O'z ichiga oladi

Erich Gamma, Richard Helm, Ralph Johnson va John Vlissides tomonidan yozilgan *Design Patterns: Elements of Reusable Object-Oriented Software
(Addison-Wesley Professional, 1994), oddiygina *The Gang of Four* deb ataladigan kitob, obyektga yo'naltirilgan dizayn shablonlarining katalogidir. U
OOPni quyidagicha ta'riflaydi:

> Obyektga yo'naltirilgan dasturlar obyektlardan tashkil topgan. 
> Bir *obyekt* ma'lumotlar va ushbu ma'lumotlar bilan ishlaydigan 
> protseduralarni paketlaydi. Protseduralar odatda *metodlar*     
> yoki *operatsiyalar* deb ataladi.

Ushbu ta'rifga ko'ra, Rust obyektga yo'naltirilgan: structlar va enumlar ma'lumotlarga ega va `impl` bloklari structlar va enumlarda metodlar taqdim
etadi. Garchi structlar va enumlar metodlar bilan *obyekt* deb atalmagan bo'lsa-da, ular The Gang of Four ta'rifiga ko'ra obyektlarning bir xil
funksionalligini ta'minlaydi.

### Amalga oshirish Tafsilotlarini Yashiruvchi Inkapsulyatsiya

OOP bilan odatda bog'liq bo'lgan yana bir jihat *inkapsulyatsiya* tushunchasi bo'lib, bu obyektning amalga oshirish tafsilotlari ushbu obyektni
ishlatadigan kod uchun ochiq bo'lmasligini anglatadi. Shuning uchun, obyekt bilan o'zaro ta'sir qilishning yagona usuli uning ommaviy APIi orqali amalga
oshiriladi; obyektni ishlatadigan kod obyektning ichki qismlariga kirib, ma'lumotlar yoki xatti-harakatlarni bevosita o'zgartira olmaydi. Bu dasturchiga
obyektning ichki qismlarini o'zgartirish va refaktor qilish imkonini beradi, obyektdan foydalanadigan kodni o'zgartirmasdan.

7-bobda inkapsulyatsiyani qanday boshqarish mumkinligini muhokama qildik: biz `pub` kalit so'zidan foydalanib, kodimizdagi qaysi modullar, turlar,
funksiyalar va metodlar ommaviy bo'lishi kerakligini va standart bo'lib hamma narsalar xususiy ekanligini hal qilishimiz mumkin. Masalan,
`AveragedCollection` deb nomlangan structni aniqlashimiz mumkin, u `i32` qiymatlarining vektorini o'z ichiga oladi. Struct shuningdek, vektordagi
qiymatlarning o'rtacha qiymatini o'z ichiga oluvchi maydonga ham ega bo'lishi mumkin, ya'ni o'rtacha qiymatga har safar ehtiyoj tug'ilganda hisoblanishi
shart emas. Boshqacha qilib aytganda, `AveragedCollection` biz uchun hisoblangan o'rtacha qiymatni keshlaydi. 17-1 ro'yxatda `AveragedCollection`
structining ta'rifi keltirilgan:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-01/src/lib.rs}}
```

<span class="caption">Ro'yxat 17-1: `AveragedCollection` structi, integerlar ro'yxatini va yig'indi elementlarning o'rtacha qiymatini saqlaydi</span>

Struct `pub` deb belgilangan, shuning uchun boshqa kodlar uni ishlatishi mumkin, lekin struct ichidagi maydonlar xususiy bo'lib qoladi. Bu holda bu
muhim, chunki biz ro'yxatga qiymat qo'shilgan yoki o'chirilganida o'rtacha qiymat ham yangilanishini ta'minlamoqchimiz. Buni biz `add`, `remove` va
`average` metodlarini structga tatbiq etish orqali amalga oshiramiz, bu 17-2 ro'yxatda ko'rsatilgan:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-02/src/lib.rs:here}}
```

<span class="caption">Ro'yxat 17-2: `AveragedCollection` ustida ommaviy `add`, `remove` va `average` metodlarining amalga oshirilishi</span>

`add`, `remove` va `average` ommaviy metodlar `AveragedCollection` nusxasidagi ma'lumotlarni kirish yoki o'zgartirishning yagona usuli. Element ro'yxatga
`add` metodi orqali qo'shilganda yoki `remove` metodi orqali olib tashlanganda, har biri `average` maydonini yangilash bilan shug'ullanadigan xususiy
`update_average` metodini chaqiradi.

`list` va `average` maydonlarini xususiy qilib qoldiramiz, shunda tashqi kod `list` maydoniga elementlar qo'shish yoki olib tashlash imkoniga ega
bo'lmaydi; aks holda, `average` maydoni `list` o'zgarganda sinxronlashdan chiqib ketishi mumkin. 
`average` metodi `average` maydonidagi qiymatni qaytaradi, tashqi kodga `average`ni o'qish imkonini beradi, lekin uni o'zgartirish imkonini bermaydi.

Biz `AveragedCollection` structining amalga oshirish tafsilotlarini inkapsulyatsiya qilganimiz sababli, kelajakda uning jihatlarini osonlik bilan
o'zgartirishimiz mumkin. Masalan, `list` maydoni uchun `Vec<i32>` o'rniga `HashSet<i32>` dan foydalanishimiz mumkin. `add`, `remove` va `average` ommaviy
metodlarining imzolari o'zgarmagan holda, `AveragedCollection`dan foydalanadigan kodni o'zgartirish kerak bo'lmaydi. Agar `list`ni ommaviy qilsak, bu har
doim ham shunday bo'lmaydi: `HashSet<i32>` va `Vec<i32>` elementlarni qo'shish va olib tashlash uchun turli xil metodlarga ega, shuning uchun `list`ni
to'g'ridan-to'g'ri o'zgartirayotgan tashqi kod o'zgartirilishi kerak bo'ladi.

Agar inkapsulyatsiya tilni obyektga yo'naltirilgan deb hisoblash uchun zaruriy xususiyat bo'lsa, Rust bu talabga javob beradi. Kodning turli qismlari
uchun `pub`dan foydalanish yoki foydalanmaslik imkoniyati amalga oshirish tafsilotlarini inkapsulyatsiya qilish imkonini beradi.

### Merozdan foydalanish Tizimi va Kodni Ulashish Sifatida

*Meros olish* bu mexanizm bo'lib, bunda obyekt boshqa obyektning ta'rifidan elementlarni meros qilib oladi va shunday qilib, ota obyektning ma'lumotlari
va xatti-harakatlarini qayta ta'riflashsiz oladi.

Agar til obyektga yo'naltirilgan til deb hisoblanishi uchun meros olishga ega bo'lishi kerak bo'lsa, Rust bunday til emas. Ota structning maydonlari va
metodlarini makrosiz meros qilib olishning hech qanday yo'li yo'q.

Biroq, agar siz dasturlash vositangizda meros olishga ega bo'lishga odatlangan bo'lsangiz, Rustda boshqa echimlardan foydalanishingiz mumkin, bu meros
olishga erishmoqchi bo'lgan sababingizga qarab o'zgaradi.

Meros olishni tanlashning ikki asosiy sababi bor. Biri kodni qayta ishlatish uchun: siz bir tur uchun ma'lum xatti-harakatni amalga oshirishingiz mumkin
va meros olish bu amalga oshirishni boshqa tur uchun qayta ishlatishga imkon beradi. Rust kodida siz buni cheklangan tarzda trait metodlari uchun 
standart amalga oshirishlar yordamida amalga oshirishingiz mumkin, bu 10-14 ro'yxatda `Summary` traitida `summarize` metodining standart amalga
oshirilishini qo'shganimizda ko'rsatilgan.

 `Summary` traitini amalga oshirgan har qanday tur `summarize` metodiga ega bo'ladi, hech qanday qo'shimcha kod yozmasdan. Bu ota sinfning metodini
 amalga oshirishiga va meros qilib olingan bola sinfining metodni amalga oshirishiga o'xshaydi. `Summary` traitini amalga oshirganimizda `summarize`
 metodining standart amalga oshirilishini ham bekor qilishimiz mumkin, bu meros qilib olingan bola sinfi ota sinfdan meros qilib olingan metodni amalga
 oshirishini bekor qilishga o'xshaydi.

Meros olishdan foydalanishning boshqa sababi turi tizimiga bog'liq: bola turini ota turi bilan bir xil joylarda ishlatishga imkon berish. Bu shuningdek
*polimorfizm* deb ataladi, bu bir-birini almashtirish imkonini beradi, agar ular ma'lum xususiyatlarga ega bo'lsa.

> ### Polimorfizm
>
> Ko'pchilik uchun polimorfizm meros olish bilan sinonimdir. Ammo bu aslida ko'proq umumiy tushuncha bo'lib, u turli turlardagi ma'lumotlar bilan
ishlaydigan kodni anglatadi. Meros olish uchun bu turlar odatda quyi sinflardir.
>
> Rust esa o'rniga turli xil turlar ustida abstraksiya qilish uchun generiklardan va bu turlarning nimani ta'minlashi kerakligini cheklash uchun trait
cheklovlaridan foydalanadi. Bu ba'zan *cheklangan parametrik polimorfizm* deb ataladi.

Meros olish ko'pincha dastur dizayn yechimi sifatida ko'plab dasturlash tillarida sevilmay qoldi, chunki u ko'pincha kerakli koddan ko'proq kodni ulash
xavfini tug'diradi. Quyi sinflar har doim ham ota sinfining barcha xususiyatlarini ulashmasligi kerak, ammo meros olishda shunday bo'ladi. Bu dastur
dizaynini kamroq moslashuvchan qiladi. Shuningdek, bu quyi sinfda metodlarni chaqirish imkoniyatini beradi, bu metodlar quyi sinfga mos kelmasligi yoki
xatolarga olib kelishi mumkin, chunki metodlar quyi sinfga tatbiq etilmaydi. Bundan tashqari, ba'zi tillar faqat yagona meros olishga ruxsat beradi (bu
quyi sinf faqat bitta sinfdan meros olishi mumkinligini anglatadi), bu esa dastur dizaynining moslashuvchanligini yanada cheklaydi.

Ushbu sabablarga ko'ra, Rust meros olish o'rniga trait obyektlaridan foydalanish yo'lini tanlaydi. Keling, Rustda trait obyektlari qanday qilib
polimorfizmni ta'minlashini ko'rib chiqaylik.
