# Patternlar va matching

*Pattern*(namuna, andaza) - Rust dasturlash tilida kodning biron qismini ma'lum bir tuzilma yoki qiymatga moslashtirish uchun ishlatiladigan maxsus sintaksis. Patternlarni `match` bilan birga ishlatish orqali siz dasturning ishlash jarayonini yana ham yaxshiroq boshqara olishingiz mumkin. Patternlar quyidagi kombinatsiyalardan iborat bo'lishi mumkin:

* Literallar
* Qayta strukturalangan arraylar, enumlar, structlar yoki tuplelar
* O'zgaruvchilar
* Pastki chiziqcha (yohud Wildcards, `_` ko'rinishida yoziladi)
* To'ldiruvchilar

Ushbu kombinatsiyalarga misol qilib `x`, `(a, 3)` va `Some(Color::Red)` larni ko'rsatishimiz mumkin. Ushbu kombinatsiyalar ma'lumotning tuzilishini ifoda etadi. Dasturimiz biron ma'lumotni ushbu kombinatsiyalarga solishtirib ko'radi va ma'lumotimizning turi aynan biz istagan shaklda ekani yoki yo'qligini tekshiradi. Ma'lumot tekshirilgach, dasturni davom ettirish mumkin bo'ladi.

Patternni ishlatish uchun, biz uni biron ma'lumot bilan solishtiramiz. Agar pattern ma'lumot bilan mos kelsa, biz u ma'lumotni kodning qolgan qismida ishlatishimiz mumkin bo'ladi. 6-bo'limdagi tangalarni tartiblovchi mashina misolini yodga oladigan bo'lsak, u yerda `match` ishlatilganiga guvoh bo'lamiz. Agar qiymat pattern bilan mos kelsa, biz uni ishlatishimiz mumkin. Aks holda, patternning qolgan qismi ishlamaydi

Ushbu bo'lim patternlarga tegishli barcha ma'lumotlar uchun manba hisoblanadi. Ushbu bo'limda biz patternlarni qanday to'g'ri ishlatish, aniq va noaniq patternlar o'rtasidagi farq and patternlarning turli xil sintaks shakldagi ko'rinishlarini o'rganamiz. Ushbu bo'lim yakunida siz patternlarni o'z dasturlaringizdagi turli konseptsiyalarni sodda va oson yo'l bilan ishlatishni boshlaysiz.
