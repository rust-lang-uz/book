# Patternlar va matching

*Pattern*(namuna, andaza) - Rust dasturlash tilida kodning biron qismini maʼlum bir tuzilma yoki qiymatga moslashtirish
uchun ishlatiladigan maxsus sintaksis. Patternlarni `match` bilan birga ishlatish orqali siz dasturning ishlash jarayonini
yana ham yaxshiroq boshqara olishingiz mumkin. Patternlar quyidagi kombinatsiyalardan iborat boʼlishi mumkin:

* Literallar
* Qayta strukturalangan arrayʼlar, enumʼlar, structlʼar yoki tupleʼlar
* Oʼzgaruvchilar
* Pastki chiziqcha (yohud Wildcards, `_` koʼrinishida yoziladi)
* Toʼldiruvchilar

Ushbu kombinatsiyalarga misol qilib `x`, `(a, 3)` va `Some(Color::Red)` larni koʼrsatishimiz mumkin. Ushbu kombinatsiyalar maʼlumotning tuzilishini ifoda etadi. Dasturimiz biron maʼlumotni ushbu kombinatsiyalarga solishtirib koʼradi va maʼlumotimizning turi aynan biz istagan shaklda ekani yoki yoʼqligini tekshiradi. Maʼlumot tekshirilgach, dasturni davom ettirish mumkin boʼladi.

Patternni ishlatish uchun, biz uni biron maʼlumot bilan solishtiramiz. Agar pattern maʼlumot bilan mos kelsa, biz u maʼlumotni kodning qolgan qismida ishlatishimiz mumkin boʼladi. 6-boʼlimdagi tangalarni tartiblovchi mashina misolini yodga oladigan boʼlsak, u yerda `match` ishlatilganiga guvoh boʼlamiz. Agar qiymat pattern bilan mos kelsa, biz uni ishlatishimiz mumkin. Aks holda, patternning qolgan qismi ishlamaydi

Ushbu boʼlim patternlarga tegishli barcha maʼlumotlar uchun manba hisoblanadi. Ushbu boʼlimda biz patternlarni qanday toʼgʼri ishlatish, aniq va noaniq patternlar oʼrtasidagi farq and patternlarning turli xil sintaks shakldagi koʼrinishlarini oʼrganamiz. Ushbu boʼlim yakunida siz patternlarni oʼz dasturlaringizdagi turli konseptsiyalarni sodda va oson yoʼl bilan ishlatishni boshlaysiz.
