# O'sib borayotgan loyihalarni paketlar, cratelar va modullar bilan boshqarish

Katta dasturlarni yozganingizda, kodingizni tartibga solish tobora muhim ahamiyat kasb etadi. Tegishli funksiyalarni guruhlash va kodni alohida xususiyatlar bilan ajratish orqali siz ma'lum bir xususiyatni amalga oshiradigan kodni qayerdan topish va funksiya qanday ishlashini o'zgartirish uchun qayerga borish kerakligini aniqlaysiz.

Biz hozirgacha yozgan dasturlar bitta faylda bitta modulda bo'lgan. Loyiha o'sib borishi bilan siz kodni bir nechta modullarga va keyin bir nechta fayllarga bo'lish orqali tartibga solishingiz kerak. Paketda bir nechta binary cratelar va ixtiyoriy ravishda bitta kutubxona cratesi bo'lishi mumkin. Paket o'sishi bilan siz qismlarni alohida cratelarga ajratib olishingiz mumkin, ular tashqi bog'liqlikka aylanadi. Ushbu bo'lim ushbu texnikaning barchasini o'z ichiga oladi. Birgalikda rivojlanadigan o'zaro bog'liq paketlar to'plamini o'z ichiga olgan juda katta loyihalar uchun Cargo *workspacelarni* taqdim etadi, biz ularni 14-bobdagi ["Cargo Workspacelari"][workspaces]<!-- ignore --> bo'limida ko'rib chiqamiz.

Shuningdek, biz kodni yuqoriroq darajada qayta ishlatish imkonini beruvchi implement tafsilotlarini muhokama qilamiz: operatsiyani amalga oshirganingizdan so'ng, boshqa kod dastur qanday ishlashini bilmasdan kodingizni umumiy interfeysi orqali chaqirishi mumkin. Kodni yozish usuli qaysi qismlar boshqa koddan foydalanishi uchun public ekanligini va qaysi qismlar o'zgartirish huquqiga ega bo'lgan private implement tafsilotlarini belgilaydi. Bu yodda tutish kerak bo'lgan tafsilotlar miqdorini cheklashning yana bir usuli.

Tegishli tushuncha - bu scope: kod yoziladigan ichki kontekstda "scope" deb belgilangan nomlar to'plami mavjud. Kodni o'qish, yozish va kompilyatsiya qilishda dasturchilar va kompilyatorlar ma'lum bir nuqtadagi ma'lum bir nom o'zgaruvchiga, funksiyaga, structga, enumga, modulga, constantaga yoki boshqa elementga tegishli ekanligini va bu element nimani anglatishini bilishlari kerak. Siz scopelarni yaratishingiz va qaysi nomlar doirasida yoki tashqarida ekanligini o'zgartirishingiz mumkin. Bir xil nomdagi ikkita elementni bir xil scopeda bo'lishi mumkin emas; nom ziddiyatlarini hal qilish uchun tollar mavjud.

Rust sizga kodingizning tuzilishini boshqarish imkonini beruvchi bir qator xususiyatlarga ega, jumladan, qaysi tafsilotlar oshkor bo'lishi, qaysi tafsilotlar shaxsiy va dasturlaringizdagi har bir sohada qanday nomlar mavjudligi. Ba'zan birgalikda *modul tizimi* deb ataladigan bu xususiyatlar quyidagilarni o'z ichiga oladi:

* **Paketlar:** Cratelarni qurish, sinab ko'rish va almashish imkonini beruvchi cargo xususiyati
* **Cratelar:** Kutubxona yoki bajariladigan faylni yaratuvchi modullar daraxti
* **Modullar** va **use:** Fayl yoʻllarning tashkil etilishi, koʻlami va maxfiyligini nazorat qilish imkonini beradi
* **Pathlar:** Struct, funksiya yoki modul kabi elementni nomlash usuli

Ushbu bobda biz ushbu xususiyatlarning barchasini ko'rib chiqamiz, ularning qanday o'zaro ta'sirini muhokama qilamiz va qamrovni boshqarish uchun ulardan qanday foydalanishni tushuntiramiz. Oxir-oqibat, siz modul tizimi haqida yaxshi tushunchaga ega bo'lishingiz va professional kabi sohalar bilan ishlashingiz kerak!

[workspaces]: ch14-03-cargo-workspaces.html
