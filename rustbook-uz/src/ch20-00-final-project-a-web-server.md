# So'ngi Loyiha: Ko'p oqimli (Multithreaded) Veb Server qurish

Bu uzoq safar bo‘ldi, lekin nihoyat kitobning oxiriga yetdik. Ushbu bobda biz yakuniy boblarda o‘rganilgan ba’zi tushunchalarni amalda qo‘llash uchun yana bir loyihani birga quramiz, shuningdek, avvalgi darslarni ham eslab o‘tamiz.

Yakuniy loyihamizda biz “salom” deb javob beradigan va veb-brauzerda 20-1-rasmga o‘xshash ko‘rinadigan veb-server yaratamiz.

![hello from rust](img/trpl20-01.png)

<span class="caption">20-rasm: Bizning yakuniy umumiy loyihamiz</span>

Veb-serverni yaratish bo‘yicha rejamiz quyidagicha:

1. TCP va HTTP haqida biroz ma’lumot olish.
2. Soket orqali TCP ulanishlarini tinglash.
3. Bir nechta HTTP so‘rovlarini tahlil qilish.
4. To‘g‘ri HTTP javobini yaratish.
5. Serverimiz unumdorligini oqimlar to‘plami (thread pool) yordamida oshirish.

Boshlashdan oldin bir narsani aytib o‘tishimiz kerak: biz foydalanadigan usul Rust tilida veb-server qurishning eng yaxshi usuli bo‘lmaydi. Rust hamjamiyati 
[crates.io](https://crates.io/) saytida chop etilgan, to‘liq funksiyali veb-server va oqimlar to‘plami (thread pool) kutubxonalarini yaratgan. Bu kutubxonalar biz quradigan versiyadan ko‘ra ancha mukammal. Biroq, ushbu bobdagi maqsadimiz – sizni o‘rgatish, eng oson yo‘lni tanlash emas. Rust tizim dasturlash tili bo‘lganligi sababli, biz o‘zimiz ishlamoqchi bo‘lgan abstraktsiya darajasini tanlashimiz mumkin va boshqa tillarda amalga oshirish qiyin yoki imkonsiz bo‘lgan chuqur darajadagi ishlarni bajarish imkoniyatiga egamiz. Shuning uchun biz HTTP server va oqimlar to‘plamini qo‘lda yozamiz, shunda siz kelajakda foydalanishingiz mumkin bo‘lgan kutubxonalar ortida qanday g‘oya va texnikalar yotganini tushunib olasiz.
