# O'sib borayotgan loyihalarni paketlar, cratelar va modullar bilan boshqarish

Katta dasturlarni yozganingizda, kodingizni tartibga solish tobora muhim ahamiyat kasb etadi. Tegishli funksiyalarni guruhlash va kodni alohida xususiyatlar bilan ajratish orqali siz ma'lum bir xususiyatni amalga oshiradigan kodni qayerdan topish va funksiya qanday ishlashini o'zgartirish uchun qayerga borish kerakligini aniqlaysiz.

Biz hozirgacha yozgan dasturlar bitta faylda bitta modulda bo'lgan. Loyiha o'sib borishi bilan siz kodni bir nechta modullarga va keyin bir nechta fayllarga bo'lish orqali tartibga solishingiz kerak. Paketda bir nechta binary cratelar va ixtiyoriy ravishda bitta kutubxona cratesi bo'lishi mumkin. Paket o'sishi bilan siz qismlarni alohida cratelarga ajratib olishingiz mumkin, ular tashqi bog'liqlikka aylanadi. Ushbu bo'lim ushbu texnikaning barchasini o'z ichiga oladi. Birgalikda rivojlanadigan o'zaro bog'liq paketlar to'plamini o'z ichiga olgan juda katta loyihalar uchun Cargo *workspacelarni* taqdim etadi, biz ularni 14-bobdagi ["Cargo Workspacelari"][workspaces]<!-- ignore --> bo'limida ko'rib chiqamiz.

Shuningdek, biz kodni yuqoriroq darajada qayta ishlatish imkonini beruvchi implement tafsilotlarini muhokama qilamiz: operatsiyani amalga oshirganingizdan so'ng, boshqa kod dastur qanday ishlashini bilmasdan kodingizni umumiy interfeysi orqali chaqirishi mumkin. The way you write code defines which parts are public for
other code to use and which parts are private implementation details that you
reserve the right to change. This is another way to limit the amount of detail
you have to keep in your head.

A related concept is scope: the nested context in which code is written has a
set of names that are defined as “in scope.” When reading, writing, and
compiling code, programmers and compilers need to know whether a particular
name at a particular spot refers to a variable, function, struct, enum, module,
constant, or other item and what that item means. You can create scopes and
change which names are in or out of scope. You can’t have two items with the
same name in the same scope; tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code’s
organization, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the *module system*, include:

* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and
  privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module

In this chapter, we’ll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!

[workspaces]: ch14-03-cargo-workspaces.html
