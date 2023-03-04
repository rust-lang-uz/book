# Xatolar bilan ishlash

Xatolar dasturiy ta'minotdagi hayot haqiqatidir, shuning uchun Rust nimadir noto'g'ri bo'lgan vaziyatlarni hal qilish uchun bir qator xususiyatlarga ega. Ko'p hollarda Rust sizdan xatolik ehtimolini tan olishingizni va kodingizni kompilyatsiya qilishdan oldin ba'zi choralarni ko'rishingizni talab qiladi. Ushbu talab sizning kodingizni ishlab chiqarishga joylashtirishdan oldin xatolarni aniqlashingiz va ularni to'g'ri hal qilishingizni ta'minlash orqali dasturingizni yanada mustahkam qiladi!

Rust xatolarni ikkita asosiy toifaga ajratadi: *tiklash mumkin* va *tiklab bo‘lmaydigan* xatolar. *file not found (fayl topilmadi)* xatosi kabi tiklanadigan xatolik uchun biz muammo haqida foydalanuvchiga xabar berishni va operatsiyani qaytadan urinib ko'rishni xohlaymiz.
Qayta tiklanmaydigan xatolar har doim xato belgilaridir, masalan, array oxiridan tashqaridagi joyga kirishga urinish va shuning uchun biz dasturni darhol to'xtatmoqchimiz.

Aksariyat tillar bu ikki turdagi xatolarni farqlamaydi va istisnolar(exceptions) kabi mexanizmlardan foydalangan holda ikkalasini ham xuddi shunday hal qiladi. Rustda istisnolar yo'q. Buning o'rniga, u tiklanadigan xatolar uchun `Result<T, E>` turiga va dastur tuzatib bo'lmaydigan xatolikka duch kelganda jarayonni to'xtatuvchi `panic!` makrosiga ega. Bu bob avval `panic!` chaqirish haqida so'z boradi, soʻngra `Result<T, E>` qiymatlarini qaytarish haqida so'z boradi. Bundan tashqari, biz xatolikdan xalos bo'lish yoki bajarishni to'xtatish haqida qaror qabul qilishda fikrlarni o'rganamiz.
