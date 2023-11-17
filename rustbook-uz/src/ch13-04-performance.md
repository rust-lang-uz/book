## Ishlash samaradorligini(Performance) solishtirish: Looplar va iteratorlar

Looplar yoki iteratorlardan foydalanishni aniqlash uchun siz qaysi implement qilish tezroq ekanligini bilishingiz kerak: `qidiruv` funksiyasining aniq `for` lop sikliga ega versiyasi yoki iteratorli versiya.

Ser Arthur Conan Doylening *Sherlok Xolmsning sarguzashtlari(The Adventures of Sherlock Holmes)* asarining to‘liq mazmunini `String`ga yuklash va mazmundan *the* so‘zini izlash orqali sinovdan o‘tkizdik. Mana, `for` loop siklidan foydalangan holda `qidiruv` versiyasi va iteratorlardan foydalangan holda sinov natijalari:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

Iterator versiyasi biroz tezroq edi! Biz bu yerda benchmark kodini tushuntirmaymiz, chunki bu ikkita versiyaning ekvivalentligini isbotlash emas, balki ushbu ikki dasturning ishlash jihatidan qanday taqqoslanishi haqida umumiy tushunchaga ega bo'lishdir.

Batafsilroq tahlil qilish uchun siz `tarkib` sifatida har xil oʻlchamdagi turli matnlarni, `sorov` sifatida turli uzunlikdagi turli soʻz va soʻzlarni va boshqa har xil oʻzgarishlardan foydalanishni tekshirishingiz kerak. Gap shundaki: iteratorlar, garchi yuqori darajadagi(high-level) abstraksiya bo'lsa ham, xuddi quyi darajadagi(lower-level) kodni o'zingiz yozganingizdek, taxminan bir xil kodga kompilyatsiya qilinadi. Iteratorlar Rustning *zero-cost(nol xarajatli) abstraksiyalaridan* biri bo‘lib, bu orqali biz abstraktsiyadan foydalanish qo‘shimcha runtime yukini talab qilmaydi. Bu C++ tilining asl dizayneri va amalga oshiruvchisi(implementori) Bjarne Stroustrupning “C++ asoslari (Foundations of C++)” (2012) asarida *zero-overhead* belgilashiga o‘xshaydi:

> Umuman olganda, C++ ilovalari zero overhead(nol qo'shimcha xarajatlar) printsipiga bo'ysunadi:
> Siz foydalanmayotgan narsangiz uchun pul to'lamaysiz. Va yana: Siz foydalanadigan narsadan yaxshiroq
> kodlash mumkin emas.

As another example, the following code is taken from an audio decoder. The
decoding algorithm uses the linear prediction mathematical operation to
estimate future values based on a linear function of the previous samples. This
code uses an iterator chain to do some math on three variables in scope: a
`buffer` slice of data, an array of 12 `coefficients`, and an amount by which
to shift data in `qlp_shift`. We’ve declared the variables within this example
but not given them any values; although this code doesn’t have much meaning
outside of its context, it’s still a concise, real-world example of how Rust
translates high-level ideas to low-level code.

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

To calculate the value of `prediction`, this code iterates through each of the
12 values in `coefficients` and uses the `zip` method to pair the coefficient
values with the previous 12 values in `buffer`. Then, for each pair, we
multiply the values together, sum all the results, and shift the bits in the
sum `qlp_shift` bits to the right.

Calculations in applications like audio decoders often prioritize performance
most highly. Here, we’re creating an iterator, using two adaptors, and then
consuming the value. What assembly code would this Rust code compile to? Well,
as of this writing, it compiles down to the same assembly you’d write by hand.
There’s no loop at all corresponding to the iteration over the values in
`coefficients`: Rust knows that there are 12 iterations, so it “unrolls” the
loop. *Unrolling* is an optimization that removes the overhead of the loop
controlling code and instead generates repetitive code for each iteration of
the loop.

All of the coefficients get stored in registers, which means accessing the
values is very fast. There are no bounds checks on the array access at runtime.
All these optimizations that Rust is able to apply make the resulting code
extremely efficient. Now that you know this, you can use iterators and closures
without fear! They make code seem like it’s higher level but don’t impose a
runtime performance penalty for doing so.

## Summary

Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust’s capability to clearly express
high-level ideas at low-level performance. The implementations of closures and
iterators are such that runtime performance is not affected. This is part of
Rust’s goal to strive to provide zero-cost abstractions.

Now that we’ve improved the expressiveness of our I/O project, let’s look at
some more features of `cargo` that will help us share the project with the
world.
