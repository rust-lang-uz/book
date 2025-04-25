# Kengaytirilgan xususiyatlar

Hozirgacha Rust dasturlash tilining eng ko'p qo'llaniladigan qismlarini o'rgandingiz. 
20-bobda yana bir loyihani amalga oshirishimizdan oldin, siz vaqti-vaqti bilan duch kelishingiz 
mumkin bo'lgan, lekin har doim ham ishlatilmaydigan Rust xususiyatlari haqida gaplashamiz. 
Qachondir tushunmovchiliklarga duch kelganingizda, ushbu bobdan qo'llanma sifatida foydalanishingiz mumkin. 
Bu yerda ko'rib chiqiladigan xususiyatlar, funksiyalar faqat muayyan vaziyatlardagina foydali bo'ladi.
Garchi bu funksiyalarga tez-tez murojaat qilmasligingiz mumkin bo‘lsa-da, Rust tili taklif qiladigan 
barcha imkoniyatlarni tushunib olishingizni istaymiz.

Ushbu bobda biz quyidagilarni ko'rib chiqamiz:

* Xavfsiz bo‘lmagan (Unsafe) Rust: Rustning ayrim kafolatlaridan qanday qilib voz kechish va bu kafolatlarni qo‘lda, ya’ni 
o‘zingiz mustaqil tarzda boshqarish mas’uliyatini qanday olishni ko‘rib chiqamiz.

* Kengaytirilgan traitlar: bog‘langan turlar (associated types), standart tur parametrlari (default type parameters), 
to‘liq aniqlangan sintaksis (fully qualified syntax), supertraitlar (supertraits) va newtype namunasi (newtype pattern).

* Kengaytirilgan turlar: newtype namunasi (newtype pattern), tur aliaslari (type aliases),  hech qachon yuz bermaydigan tur
(the never type — !) va dinamik o'lchamdagi turlar (dynamically sized types)

* Kengaytirilgan funksiyalar va yopiq funksiyalar (Closures): funksiya ko'rsatkichlari (function pointers) va closure qaytaradigan funksiyalar (returning closures)
* Andozalar (Macros): kodni jamlash (kompilyatsiya) vaqtida boshqa kodni aniqlovchi kodni yozish usullari.

Bu — har bir dasturchi uchun foydali bo'lgan Rust imkoniyatlarining to‘liq to‘plamidir! Keling, to'plamga sho'ng'iymiz!
