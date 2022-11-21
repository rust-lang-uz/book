## Ilova B: Operatorlar va belgilar

Ushbu ilova Rust sintaksisining lug'atini o'z ichiga oladi, jumladan operatorlar va o'z-o'zidan yoki yo'llar, umumiy belgilar, belgilar chegaralari, makroslar, atributlar, izohlar, kortejlar va qavslar kontekstida paydo bo'ladigan boshqa belgilar.

### Operatorlar

B-1-jadval Rust-dagi operatorlarni o'z ichiga oladi, operator kontekstda qanday paydo bo'lishiga misol, qisqa tushuntirish va bu operatorning haddan tashqari yuklanishi mumkinmi. Agar operator haddan tashqari yuklanishi mumkin bo'lsa, ushbu operatorni ortiqcha yuklash uchun foydalaniladigan tegishli xususiyat ro'yxatga olinadi.

<span class="caption">Jadval B-1: Operatorlar</span>

| Operator | Misol | Tushuntirish | Ko'p yuklanganmi ? |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Makro kengaytirish| |
| `!` | `!expr` | Bit yoki mantiqiy to'ldiruvchi| `Not` |
| `!=` | `expr != expr` | Tengsizlik bilan taqqoslash | `PartialEq` |
| `%` | `expr % expr` | Arifmetik qoldiq | `Rem` |
| `%=` | `var %= expr` | Arifmetik qoldiq va topshiriq| `RemAssign` |
| `&` | `&expr`, `&mut expr` | Ampersand| |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Ampersand ko'rsatkich turi | |
| `&` | `expr & expr` | Bit boʻyicha AND | `BitAnd` |
| `&=` | `var &= expr` | Bit bo'yicha AND va tayinlash | `BitAndAssign` |
| `&&` | `expr && expr` | Qisqa tutashuv mantiqiy AND | |
| `*` | `expr * expr` | Arifmetik ko‘paytirish | `Mul` |
| `*=` | `var *= expr` | Arifmetik ko‘paytirish va topshiriq | `MulAssign` |
| `*` | `*expr` | Dereference | `Deref` |
| `*` | `*const type`, `*mut type` | Raw ko'rsatkich | |
| `+` | `trait + trait`, `'a + trait` | Murakkab turdagi cheklov| |
| `+` | `expr + expr` | Arifmetik qo'shish | `Add` |
| `+=` | `var += expr` | Arifmetik qo‘shish va topshiriq | `AddAssign` |
| `,` | `expr, expr` | Argument va elementlarni ajratuvchi | |
| `-` | `- expr` | Arifmetik inkor | `Neg` |
| `-` | `expr - expr` | Arifmetik ayirish | `Sub` |
| `-=` | `var -= expr` | Arifmetik ayirish va topshiriq | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Funktsiya va yopilish qaytish turi | |
| `.` | `expr.ident` | A'zoga kirish | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | To'g'ridan-to'g'ri eksklyuziv diapazon | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | Toʻgʻri chiziqli diapazon| `PartialOrd` |
| `..` | `..expr` | Strukturani yangilash sintaksisi | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | “And the rest” pattern binding | |
| `...` | `expr...expr` | (Eskirgan, o‘rniga `..=` dan foydalaning) Shaklda: inklyuziv diapazon namunasi | |
| `/` | `expr / expr` | Arifmetik bo'lish | `Div` |
| `/=` | `var /= expr` | Arifmetik bo‘linish va topshiriq | `DivAssign` |
| `:` | `pat: type`, `ident: type` | Cheklovlar | |
| `:` | `ident: expr` | Struktura maydonini ishga tushirish | |
| `:` | `'a: loop {...}` | Loop label | |
| `;` | `expr;` | Bayonot va element terminatori | |
| `;` | `[...; len]` | Ruxsat etilgan o'lchamli massiv sintaksisining bir qismi | |
| `<<` | `expr << expr` | Chapga siljish | `Shl` |
| `<<=` | `var <<= expr` | Chapga siljish va tayinlash | `ShlAssign` |
| `<` | `expr < expr` | Taqqoslashdan kamroq | `PartialOrd` |
| `<=` | `expr <= expr` | Taqqoslashdan kamroq yoki teng| `PartialOrd` |
| `=` | `var = expr`, `ident = type` | Topshiriq/ekvivalentlik | |
| `==` | `expr == expr` | Tenglikni taqqoslash | `PartialEq` |
| `=>` | `pat => expr` | Mos qoʻl sintaksisining bir qismi | |
| `>` | `expr > expr` | Taqqoslashdan kattaroq | `PartialOrd` |
| `>=` | `expr >= expr` | Taqqoslashdan katta yoki teng | `PartialOrd` |
| `>>` | `expr >> expr` | O'ngga siljish | `Shr` |
| `>>=` | `var >>= expr` | O'ngga siljish va tayinlash | `ShrAssign` |
| `@` | `ident @ pat` | Shaklni bog'lash | |
| `^` | `expr ^ expr` | Bit boʻyicha eksklyuziv OR | `BitXor` |
| `^=` | `var ^= expr` | Bit bo'yicha eksklyuziv OR va tayinlash | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | Shaklning alternativlari | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | Bitwise OR | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | Bitwise OR va topshiriq | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | Qisqa tutashuvli mantiqiy OR | |
| `?` | `expr?` | Xatoning tarqalishi | |

### Non-operator Symbols

Quyidagi ro'yxatda operator sifatida ishlamaydigan barcha belgilar mavjud; ya'ni ular funktsiya yoki usul chaqiruvi kabi harakat qilmaydi.

B-2-jadvalda o'z-o'zidan paydo bo'ladigan va turli joylarda amal qiladigan belgilar ko'rsatilgan.

<span class="caption">B-2-jadval: Mustaqil sintaksis</span>

| Belgi | Tushuntirish |
|--------|-------------|
| `'ident` | Nomlangan davr yoki tsikl yorlig'i |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | Muayyan turdagi raqamli harflar |
| `"..."` | String literal |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | Raw string literal, escape belgilar qayta ishlanmadi |
| `b"..."` | Byte string literal; satr o'rniga baytlar massivini tuzadi |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | Raw byte string literal, raw va byte string litereal birikmasi |
| `'...'` | Character literal |
| `b'...'` | ASCII byte literal |
| <code>&vert;...&vert; expr</code> | Yopish |
| `!` | Funktsiyalarni ajratish uchun har doim bo'sh pastki turi |
| `_` | “Ignored” pattern binding; shuningdek, butun sonli harflarni o'qilishi mumkin qilish uchun ishlatiladi |

B-3-jadvalda modul ierarxiyasi orqali elementga boradigan yo'l kontekstida paydo bo'ladigan belgilar ko'rsatilgan.

<span class="caption">B-3-jadval: Yo'l bilan bog'liq sintaksis</span>

| Symbol | Explanation |
|--------|-------------|
| `ident::ident` | Namespace path |
| `::path` | Path relative to the crate root (i.e., an explicitly absolute path) |
| `self::path` | Path relative to the current module (i.e., an explicitly relative path).
| `super::path` | Path relative to the parent of the current module |
| `type::ident`, `<type as trait>::ident` | Associated constants, functions, and types |
| `<type>::...` | Associated item for a type that cannot be directly named (e.g., `<&T>::...`, `<[T]>::...`, etc.) |
| `trait::method(...)` | Disambiguating a method call by naming the trait that defines it |
| `type::method(...)` | Disambiguating a method call by naming the type for which it’s defined |
| `<type as trait>::method(...)` | Disambiguating a method call by naming the trait and type |

Table B-4 shows symbols that appear in the context of using generic type
parameters.

<span class="caption">Table B-4: Generics</span>

| Symbol | Explanation |
|--------|-------------|
| `path<...>` | Specifies parameters to generic type in a type (e.g., `Vec<u8>`) |
| `path::<...>`, `method::<...>` | Specifies parameters to generic type, function, or method in an expression; often referred to as turbofish (e.g., `"42".parse::<i32>()`) |
| `fn ident<...> ...` | Define generic function |
| `struct ident<...> ...` | Define generic structure |
| `enum ident<...> ...` | Define generic enumeration |
| `impl<...> ...` | Define generic implementation |
| `for<...> type` | Higher-ranked lifetime bounds |
| `type<ident=type>` | A generic type where one or more associated types have specific assignments (e.g., `Iterator<Item=T>`) |

Table B-5 shows symbols that appear in the context of constraining generic type
parameters with trait bounds.

<span class="caption">Table B-5: Trait Bound Constraints</span>

| Symbol | Explanation |
|--------|-------------|
| `T: U` | Generic parameter `T` constrained to types that implement `U` |
| `T: 'a` | Generic type `T` must outlive lifetime `'a` (meaning the type cannot transitively contain any references with lifetimes shorter than `'a`) |
| `T: 'static` | Generic type `T` contains no borrowed references other than `'static` ones |
| `'b: 'a` | Generic lifetime `'b` must outlive lifetime `'a` |
| `T: ?Sized` | Allow generic type parameter to be a dynamically sized type |
| `'a + trait`, `trait + trait` | Compound type constraint |

Table B-6 shows symbols that appear in the context of calling or defining
macros and specifying attributes on an item.

<span class="caption">Table B-6: Macros and Attributes</span>

| Symbol | Explanation |
|--------|-------------|
| `#[meta]` | Outer attribute |
| `#![meta]` | Inner attribute |
| `$ident` | Macro substitution |
| `$ident:kind` | Macro capture |
| `$(…)…` | Macro repetition |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Macro invocation |

Table B-7 shows symbols that create comments.

<span class="caption">Table B-7: Comments</span>

| Symbol | Explanation |
|--------|-------------|
| `//` | Line comment |
| `//!` | Inner line doc comment |
| `///` | Outer line doc comment |
| `/*...*/` | Block comment |
| `/*!...*/` | Inner block doc comment |
| `/**...*/` | Outer block doc comment |

Table B-8 shows symbols that appear in the context of using tuples.

<span class="caption">Table B-8: Tuples</span>

| Symbol | Explanation |
|--------|-------------|
| `()` | Empty tuple (aka unit), both literal and type |
| `(expr)` | Parenthesized expression |
| `(expr,)` | Single-element tuple expression |
| `(type,)` | Single-element tuple type |
| `(expr, ...)` | Tuple expression |
| `(type, ...)` | Tuple type |
| `expr(expr, ...)` | Function call expression; also used to initialize tuple `struct`s and tuple `enum` variants |
| `expr.0`, `expr.1`, etc. | Tuple indexing |

Table B-9 shows the contexts in which curly braces are used.

<span class="caption">Table B-9: Curly Brackets</span>

| Context | Explanation |
|---------|-------------|
| `{...}` | Block expression |
| `Type {...}` | `struct` literal |

Table B-10 shows the contexts in which square brackets are used.

<span class="caption">Table B-10: Square Brackets</span>

| Context | Explanation |
|---------|-------------|
| `[...]` | Array literal |
| `[expr; len]` | Array literal containing `len` copies of `expr` |
| `[type; len]` | Array type containing `len` instances of `type` |
| `expr[expr]` | Collection indexing. Overloadable (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, or `RangeFull` as the “index” |
