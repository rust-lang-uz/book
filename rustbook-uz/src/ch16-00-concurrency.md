# Fearless Concurrency

Bir vaqtning o'zida dasturlashni(concurrent programming) xavfsiz va samarali boshqarish Rustning asosiy maqsadlaridan biridir. Dasturning turli qismlari mustaqil ravishda bajariladigan(execute) *concurrent programming* va dasturning turli qismlari bir vaqtning o'zida bajariladigan *parallel dasturlash* ko'proq kompyuterlar o'zlarining bir nechta protsessorlaridan foydalanishlari sababli tobora muhim ahamiyat kasb etmoqda. Tarixiy jihatdan, ushbu kontekstlarda dasturlash qiyin va xatolarga moyil bo'lgan: Rust buni o'zgartirishga umid qilmoqda.

Dastlab, Rust jamoasi xotira xavfsizligini ta'minlash va parallel muammolarning oldini olish turli usullar bilan hal qilinishi kerak bo'lgan ikkita alohida muammo deb o'ylagan. Vaqt o'tishi bilan jamoa egalik(ownership) va turdagi tizimlar(type system) xotira xavfsizligi *va* parallellik muammolarini boshqarishga yordam beradigan kuchli vositalar to'plami ekanligini aniqladi! Ownership(egalik) va turlarni tekshirishdan(type checking) foydalangan holda, ko'plab parallellik xatolar runtimedagi xatolardan ko'ra Rustda kompilyatsiya vaqtidagi xatolardir. Shuning uchun, runtime bilan bir vaqtda xatolik yuzaga kelgan aniq holatlarni takrorlash uchun ko'p vaqt sarflashdan ko'ra, noto'g'ri kod kompilyatsiya qilishni rad etadi va muammoni tushuntiruvchi xatoni taqdim etadi. Natijada, siz kodingizni ishlab chiqarishga(production) yuborilgandan keyin emas, balki uning ustida ishlayotganingizda tuzatishingiz mumkin. Biz Rustning bu jihatini *fearless* *concurrency* deb nomladik. Fearless concurrency sizga nozik xatolarsiz kod yozish imkonini beradi va yangi xatolarni kiritmasdan qayta tiklash oson.

> Eslatma: Oddiylik uchun biz ko'p muammolarni *concurrent* va yoki parallel
> deb aniqroq bo'lishdan ko'ra *concurrent* deb ataymiz.
> Agar bu kitob concurrency va yoki  parallellik haqida bo'lsa, biz aniqroq bo'lardik.
> Ushbu bo'lim uchun, iltimos, biz concurrent ishlatganimizda,
> parallel va/yoki concurrentni aqliy ravishda almashtiring.

Ko'pgina tillar bir vaqtda muammolarni hal qilish(concurrent problem) uchun taklif qiladigan yechimlar haqida dogmatikdir. Misol uchun, Erlang xabarlarni bir vaqtda uzatish uchun oqlangan funksiyaga ega, ammo threadlar orasidagi holatni(state) almashishning noaniq usullariga ega. Mumkin bo'lgan yechimlarning faqat bir qismini qo'llab-quvvatlash high-leveldagi tillar uchun oqilona strategiyadir, chunki high-leveldagi til mavhumlikni qo'lga kiritish uchun ba'zi nazoratdan voz kechishdan foyda va'da qiladi. Biroq, low-leveldagi tillar har qanday vaziyatda eng yaxshi samaradorlik bilan yechimni ta'minlashi va hardwarega(qurilma) nisbatan kamroq abstraktsiyalarga ega bo'lishi kutilmoqda. Shu sababli, Rust sizning vaziyatingiz va talablaringizga mos keladigan tarzda muammolarni modellashtirish uchun turli xil vositalarni taklif qiladi.

Mana biz ushbu bobda muhokama qiladigan mavzular:

* Bir vaqtning o'zida bir nechta kod qismlarini ishlatish uchun threadlarni qanday yaratish kerak
* *Message-passing* Xabarlarni uzatish concurrency, bu yerda kanallar threadlar o'rtasida xabarlar yuboradi
* *Shared-state* bir vaqtning o'zida bir nechta thereadlar(multiple thread) ma'lumotlarning bir qismiga kirish huquqiga ega
* `Sync` va `Send` traitlari, Rustning parallellik kafolatlarini foydalanuvchi tomonidan belgilangan turlarga hamda standart kutubxona tomonidan taqdim etilgan turlarga kengaytiradi.
