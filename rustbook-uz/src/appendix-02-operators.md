## B-ilova: Operatorlar va Belgilar

Ushbu qo'shimchada Rust sintaksisining lug'ati, shu jumladan o'z-o'zidan yoki yo'llar (paths), umumlashmalar (generiklar), turlar, makroslar, atributlar, sharhlar, katakchalar (tuples) va qavslar kontekstida paydo bo'ladigan operatorlar va boshqa belgilar mavjud.

### Operatorlar

B-1 jadvalida Rust tili operatorlari, operator qanday kontekstda ko'rinishi, qisqa tushuntirish va ushbu operator yuklanishi mumkinmi (overload) yoki yo'qmi ko'rsatilgan. Agar operator yuklanishi mumkin bo'lsa, bu operatorni yuklash uchun foydalanish kerak bo'lgan tegishli trait keltirilgan.

<span class="caption">Jadval B-1: Operatorlar</span>

| Operator | Misol | Tushuntirish | Yuklanishi mumkinmi? |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Makros chaqiruvi | |
| `!` | `!expr` | Bit yoki mantiqiy inkor | `Not` |
| `!=` | `expr != expr` | Tengsizlik taqqoslash | `PartialEq` |
| `%` | `expr % expr` | Bo'linishning qolg'i | `Rem` |
| `%=` | `var %= expr` | Bo'linishning qolg'i va tayinlash | `RemAssign` |
| `&` | `&expr`, `&mut expr` | Qarz olish | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Ushbu tur qarzga olinganligini ko'rsatadi | |
| `&` | `expr & expr` | Bitlik VA | `BitAnd` |
| `&=` | `var &= expr` | Bitlik VA va tayinlash | `BitAndAssign` |
| `&&` | `expr && expr` | Mantiqiy VA | |
| `*` | `expr * expr` | Arifmetik ko'paytirish | `Mul` |
| `*=` | `var *= expr` | Arifmetik ko'paytirish va tayinlash | `MulAssign` |
| `*` | `*expr` | Havolani bekor qilish | `Deref` |
| `*` | `*const type`, `*mut type` | Ushbu tur xom ko'rsatkich ekanligini ko'rsatadi | |
| `+` | `trait + trait`, `'a + trait` | Murakkab turdagi cheklov | |
| `+` | `expr + expr` | Arifmetik qo'shish | `Add` |
| `+=` | `var += expr` | Arifmetik qo'shish va tayinlash | `AddAssign` |
| `,` | `expr, expr` | Argument va element ajratuvchisi | |
| `-` | `- expr` | Arifmetik inkor | `Neg` |
| `-` | `expr - expr` | Arifmetik ayirish | `Sub` |
| `-=` | `var -= expr` | Arifmetik ayirish va tayinlash | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Funktsiya va yopilishning qaytish turi | |
| `.` | `expr.ident` | A'zoga (elementga) kirish | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | O'ngdan tashqari raqamlar oralig'ini bildiradi | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | Raqamlar oralig'ini, shu jumladan o'ng tomonni bildiradi | `PartialOrd` |
| `..` | `..expr` | Strukturani yangilash sintaksisi | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | "Va boshqa hamma narsa"ni bog'lash | |
| `...` | `expr...expr` | (Eskirgan, yangi sintaksisdan foydalaning ..= ) Inklyuziv diapazonni aniqlashda ishlatiladi | |
| `/` | `expr / expr` | Arifmetik bo'lish | `Div` |
| `/=` | `var /= expr` | Arifmetik bo'lish va tayinlash | `DivAssign` |
| `:` | `pat: type`, `ident: type` | Turlarning cheklovlari | |
| `:` | `ident: expr` | Tuzilish maydonini ishga tushirish | |
| `:` | `'a: loop {...}` | Tsikl yorlig'i | |
| `;` | `expr;` | Ko'rsatma va elementning oxiri belgisi | |
| `;` | `[...; len]` | Qat'iy o'lchamdagi massiv sintaksisining qismi | |
| `<<` | `expr << expr` | Bitlik chapga surish | `Shl` |
| `<<=` | `var <<= expr` | Bitlik chapga surish va tayinlash | `ShlAssign` |
| `<` | `expr < expr` | "Kamroq" taqqoslash | `PartialOrd` |
| `<=` | `expr <= expr` | "Kamroq yoki teng" taqqoslash | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | Tayinlash/ekvivalentlik | |
| `==` | `expr == expr` | Tenglik taqqoslash | `PartialEq` |
| `=>` | `pat => expr` | Moslama qolipining qismi | |
| `>` | `expr > expr` | "Kattaroq" taqqoslash | `PartialOrd` |
| `>=` | `expr >= expr` | "Kattaroq yoki teng" taqqoslash | `PartialOrd` |
| `>>` | `expr >> expr` | Bitlik o'ngga surish | `Shr` |
| `>>=` | `var >>= expr` | Bitlik o'ngga surish va tayinlash | `ShrAssign` |
| `@` | `ident @ pat` | Naqshni bog'lash (Pattern binding) | |
| `^` | `expr ^ expr` | Bitlik istisno YOKI | `BitXor` |
| `^=` | `var ^= expr` | Bitlik istisno YOKI va tayinlash | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | Muqobil naqshlar | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | Bitlik YOKI | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | Bitlik YOKI va tayinlash | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | Qisqa mantiqiy YOKI | |
| `?` | `expr?` | Xato qaytarish | |

### Operator bo'lmagan Belgilar

Quyidagi ro'yxatda operator sifatida ishlamaydigan barcha belgilar mavjud; ya'ni ular funktsiya yoki usul chaqiruvi kabi harakat qilmaydi.

B-2 jadvali o'z-o'zidan paydo bo'ladigan va turli joylarda qabul qilinadigan belgilarni ko'rsatadi.

<span class="caption">Jadval B-2: Mustaqil Sintaksis</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `'ident` | Nomlangan umrbod yoki tsikl yorlig'i |
| `...u8`, `...i32`, `...f64`, `...usize`, va h.k. | Ma'lum turdagi sonli literal |
| `"..."` | Qator (String) literal |
| `r"..."`, `r#"..."#`, `r##"..."##`, va h.k. | Qochish belgilarini qayta ishlamaydigan xom satrli literal |
| `b"..."` | Bayt string literal; string o'rniga bayt massivini hosil qiladi |
| `br"..."`, `br#"..."#`, `br##"..."##`, va h.k. | Xom satr baytli harf, xom va baytli harflarning kombinatsiyasi |
| `'...'` | Belgilar literal |
| `b'...'` | ASCII bayt literal |
| <code>&vert;...&vert; expr</code> | Yopilish |
| `!` | Har doim bo'sh pastki tur divergiruvchi funksiyalar uchun |
| `_` | “E'tiborsiz” pattern binding; shuningdek, butun sonli literalni o'qilishi uchun ishlatiladi |

Jadval B-3 modul ierarxiyasi orqali elementga yo'l kontekstida ko'rinadigan belgilarni ko'rsatadi.

<span class="caption">Jadval B-3: Yo'lga Tegishli Sintaksis</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `ident::ident` | Nomlar maydoni yo'li |
| `::path` | Crate ildiziga nisbatan yo'l (ya'ni, aniq absolyut yo'l) |
| `self::path` | Joriy modulga nisbatan yo'l (ya'ni, aniq nisbiy yo'l) |
| `super::path` | Joriy modulning ota moduliga nisbatan yo'l |
| `type::ident`, `<type as trait>::ident` | Tegishli konstantalar, funksiyalar va turlar |
| `<type>::...` | To'g'ridan-to'g'ri nomlanishi mumkin bo'lmagan turga tegishli element (masalan, `<&T>::...`, `<[T]>::...`, va hokazo.) |
| `trait::method(...)` | Usul chaqiruvini aniqlashtirish uchun usulni aniqlagan traitni nomlash |
| `type::method(...)` | Usul chaqiruvini aniqlashtirish uchun usul aniqlangan tur nomini ko'rsatish |
| `<type as trait>::method(...)` | Usul chaqiruvini aniqlashtirish uchun trait va tur nomini ko'rsatish |

Jadval B-4 generik turdagi parametrlarni ishlatish kontekstida ko'rinadigan belgilarni ko'rsatadi.

<span class="caption">Jadval B-4: Generiklar</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `path<...>` | Turdagi umumlashtirilgan parametrlar uchun parametrlarni belgilaydi (masalan, `Vec<u8>`) |
| `path::<...>`, `method::<...>` | Ifodada generik tur, funksiya yoki usul parametrlari; ko'pincha turbofish deb ataladi (masalan, `"42".parse::<i32>()`) |
| `fn ident<...> ...` | Umumlashtirilgan funksiyani aniqlash |
| `struct ident<...> ...` | Umumlashtirilgan tuzilmani aniqlash |
| `enum ident<...> ...` | Umumlashtirilgan enum aniqlash |
| `impl<...> ...` | Umumlashtirilgan amalga oshirishning ta'rifi |
| `for<...> type` | Yuqori darajadagi umrbod cheklovlar |
| `type<ident=type>` | Bir yoki bir nechta tegishli turlarga aniq tayinlangan generik tur (masalan, `Iterator<Item=T>`) |

Jadval b-5 ko'rsatadi belgilar turi cheklangan umumlashtirilgan parametr turlaridan foydalanish kontekstida paydo bo'ladi

<span class="caption">Jadval B-5: Trait Cheklovlari</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `T: U` | `T` Umumlashtirilgan parametri `U`ni amalga oshiruvchi turlar bilan cheklangan |
| `T: 'a` | `T` Umumlashtirilgan turi `a` umrbodidan uzoqroq bo'lishi kerak (ya'ni, tur hech qanday `a`dan qisqaroq bo'lgan ko'rsatkichlarga ega bo'lmasligi kerak) |
| `T: 'static` | `T` Umumlashtirilgan turi faqat `static` umrbod ko'rsatkichlarini o'z ichiga oladi |
| `'b: 'a` | `'b` umrbodi `a` umrbodidan uzoqroq bo'lishi kerak |
| `T: ?Sized` | Umumlashtirilgan tur parametri dinamik o'lchamli tur bo'lishiga ruxsat beradi |
| `'a + trait`, `trait + trait` | Murakkab tur cheklovi |

Jadval B-6 makrolarni chaqirish yoki aniqlash va elementga atributlarni belgilash kontekstida ko'rinadigan belgilarni ko'rsatadi.

<span class="caption">Jadval B-6: Makrolar va Atributlar</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `#[meta]` | Tashqi atribut |
| `#![meta]` | Ichki atribut |
| `$ident` | Makro almashtirish |
| `$ident:kind` | Makro qo'lga olish |
| `$(…)…` | Makro takrorlash |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Makro chaqiruvi |

B-7 jadvalida sharhlar yaratadigan belgilar ko'rsatilgan.

<span class="caption">Jadval B-7: Kommentariyalar</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `//` | Ichki bir qatorli hujjat sharhi |
| `//!` | Tashqi bir qatorli hujjat sharhi |
| `///` | Tashqi chiziqli doc kommentariya |
| `/*...*/` | Ko'p qatorli sharh |
| `/*!...*/` | Ichki ko'p qatorli hujjat sharhi |
| `/**...*/` | Tashqi ko'p qatorli hujjat sharhi |

B-8 jadvalida katakchalardan (tuple-lardan) foydalanish kontekstida paydo bo'ladigan belgilar ko'rsatilgan.

<span class="caption">Jadval B-8: Katakchalar (tuples)</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `()` | Bo'sh tupl (ya'ni, birlik), literal va tur |
| `(expr)` | Qavs ichidagi ifoda |
| `(expr,)` | Yagona elementli tupl ifodasi |
| `(type,)` | Yagona elementli tupl turi |
| `(expr, ...)` | Tupl ifodasi |
| `(type, ...)` | Tupl turi |
| `expr(expr, ...)` | Funksiya chaqiruvi ifodasi; shuningdek, tupl strukturasi va tupl enum variantlarini boshlash uchun ishlatiladi |
| `expr.0`, `expr.1`, va h.k. | Tupl indekslash |

B-9 jadvali jingalak qavslardan ("{}") foydalanadigan kontekstlarni ko'rsatadi.

<span class="caption">Jadval B-9: Jingalak Qavslar</span>

| Kontekst | Tushuntirish |
|---------|-------------|
| `{...}` | Blok ifodasi |
| `Type {...}` | `struct` literal |

Jadval B-10 to'rtburchak qavslar ishlatiladigan kontekstlarni ko'rsatadi.

<span class="caption">Jadval B-10: To'rtburchak Qavslar</span>

| Kontekst | Tushuntirish |
|---------|-------------|
| `[...]` | Array (yig'ilma) literal |
| `[expr; len]` | `len` nusxalarini o'z ichiga olgan array literal |
| `[type; len]` | `len` nusxalarini o'z ichiga olgan array turi |
| `expr[expr]` | To'plamni indekslash. Yuklanishi mumkin (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | To'plamni kesishga o'xshash qilib indekslash, `Range`, `RangeFrom`, `RangeTo` yoki `RangeFull` ni "indeks" sifatida ishlatish |