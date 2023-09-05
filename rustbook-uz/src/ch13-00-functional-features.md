# Funksional til xususiyatlari: iteratorlar va closurelar

Rust dizayni ko'plab mavjud tillar va texnikalardan ilhomlangan va muhim ta'sirlardan biri *funksional dasturlash*.
Funksional uslubda dasturlash ko'pincha funksiyalarni argumentlar orqali uzatish, ularni boshqa funksiyalardan qaytarish, keyinchalik bajarish uchun o'zgaruvchilarga tayinlash va hokazolar orqali qiymat sifatida foydalanishni o'z ichiga oladi.

Ushbu bobda biz funksional dasturlash nima yoki yo'qligi masalasini muhokama qilmaymiz, aksincha, Rustning ko'p tillardagi funksiyalarga o'xshash ba'zi xususiyatlarini muhokama qilamiz.

Aniqroq aytganda, biz quyidagilarni ko'rib chiqamiz:

* *Closurelar*, oʻzgaruvchida saqlashingiz mumkin boʻlgan funksiyaga oʻxshash konstruksiya
* *Iteratorlar*, bir qator elementlarni qayta ishlash usuli
* 12-bobdagi I/O loyihasini yaxshilash uchun closure va iteratorlardan qanday foydalanish kerak
* Closure va iteratorlarning ishlashi (Spoiler ogohlantirishi: ular siz o'ylagandan ham tezroq!)

Biz allaqachon Rustning boshqa ba'zi xususiyatlarini ko'rib chiqdik, masalan, pattern matching va enumlar, ular ham funksional uslubga ta'sir qiladi. Closure va iteratorlarni o'zlashtirish idiomatik, tezkor Rust kodini yozishning muhim qismi bo'lganligi sababli, biz ushbu bobni ularga bag'ishlaymiz.
