## `panic!` bilan tuzatib bo'lmaydigan xatolar

Ba'zida sizning kodingizda yomon narsalar sodir bo'ladi va siz bu haqda hech narsa qila olmaysiz. Bunday hollarda Rustda `panic!` makrosi mavjud. Amalda panic chaqirishning ikki yo'li mavjud: kodimizni panic chiqaradigan harakatni amalga oshirish (masalan, arrayning oxiridan o'tib kirish) yoki aniq `panic!` makrosini chaqirish.
Ikkala holatda ham biz dasturimizda panicni keltirib chiqaramiz. Odatiy bo'lib, bu panic muvaffaqiyatsizlik xabarini chop etadi, dasturni bajarish paytida ajratilgan resurslarni tozalaydi, stackni tozalaydi va ishdan chiqadi. Atrof-muhit o'zgaruvchisi orqali siz panic paydo bo'lganda panic manbasini kuzatishni osonlashtirish uchun Rust chaqiruvlar srackini ko'rsatishi ham mumkin.

> ### panicga javoban stackni bo'shatish yoki bekor qilish
>
> Odatiy bo'lib, panic paydo bo'lganda, dastur o'chiriladi, ya'ni Rust
> stekni zaxiraga olib chiqadi va duch kelgan har bir funksiyadan ma'lumotlarni
> tozalaydi. Biroq, bu orqaga qaytish va tozalash juda ko'p ishdir. Rust,
> shuning uchun dasturni tozalamasdan tugatadigan darhol bekor
> qilishning muqobilini tanlashga imkon beradi.
>
> Dastur ishlatgan xotira keyinchalik operatsion tizim tomonidan
> tozalanishi kerak bo'ladi. Agar loyihangizda natijada olingan binary faylni
> iloji boricha kichikroq qilishingiz kerak bo'lsa, *Cargo.toml* faylingizdagi
> tegishli `[profile]` bo'limlariga `panic = 'abort'` qo'shish orqali panic
> bo'yicha bo'shatishdan bekor qilishga o'tishingiz mumkin. Misol uchun, agar siz
> bo'shatish rejimida panic holatini to'xtatmoqchi bo'lsangiz, quyidagilarni qo'shing:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Keling, oddiy dasturda `panic!` deb chaqirib ko'raylik:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

When you run the program, you’ll see something like this:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where
the panic occurred: *src/main.rs:2:5* indicates that it’s the second line,
fifth character of our *src/main.rs* file.

In this case, the line indicated is part of our code, and if we go to that
line, we see the `panic!` macro call. In other cases, the `panic!` call might
be in code that our code calls, and the filename and line number reported by
the error message will be someone else’s code where the `panic!` macro is
called, not the line of our code that eventually led to the `panic!` call. We
can use the backtrace of the functions the `panic!` call came from to figure
out the part of our code that is causing the problem. We’ll discuss backtraces
in more detail next.

### Using a `panic!` Backtrace

Let’s look at another example to see what it’s like when a `panic!` call comes
from a library because of a bug in our code instead of from our code calling
the macro directly. Listing 9-1 has some code that attempts to access an
index in a vector beyond the range of valid indexes.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Listing 9-1: Attempting to access an element beyond the
end of a vector, which will cause a call to `panic!`</span>

Here, we’re attempting to access the 100th element of our vector (which is at
index 99 because indexing starts at zero), but the vector has only 3 elements.
In this situation, Rust will panic. Using `[]` is supposed to return an
element, but if you pass an invalid index, there’s no element that Rust could
return here that would be correct.

In C, attempting to read beyond the end of a data structure is undefined
behavior. You might get whatever is at the location in memory that would
correspond to that element in the data structure, even though the memory
doesn’t belong to that structure. This is called a *buffer overread* and can
lead to security vulnerabilities if an attacker is able to manipulate the index
in such a way as to read data they shouldn’t be allowed to that is stored after
the data structure.

To protect your program from this sort of vulnerability, if you try to read an
element at an index that doesn’t exist, Rust will stop execution and refuse to
continue. Let’s try it and see:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

This error points at line 4 of our `main.rs` where we attempt to access index
99. The next note line tells us that we can set the `RUST_BACKTRACE`
environment variable to get a backtrace of exactly what happened to cause the
error. A *backtrace* is a list of all the functions that have been called to
get to this point. Backtraces in Rust work as they do in other languages: the
key to reading the backtrace is to start from the top and read until you see
files you wrote. That’s the spot where the problem originated. The lines above
that spot are code that your code has called; the lines below are code that
called your code. These before-and-after lines might include core Rust code,
standard library code, or crates that you’re using. Let’s try getting a
backtrace by setting the `RUST_BACKTRACE` environment variable to any value
except 0. Listing 9-2 shows output similar to what you’ll see.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">Listing 9-2: The backtrace generated by a call to
`panic!` displayed when the environment variable `RUST_BACKTRACE` is set</span>

That’s a lot of output! The exact output you see might be different depending
on your operating system and Rust version. In order to get backtraces with this
information, debug symbols must be enabled. Debug symbols are enabled by
default when using `cargo build` or `cargo run` without the `--release` flag,
as we have here.

In the output in Listing 9-2, line 6 of the backtrace points to the line in our
project that’s causing the problem: line 4 of *src/main.rs*. If we don’t want
our program to panic, we should start our investigation at the location pointed
to by the first line mentioning a file we wrote. In Listing 9-1, where we
deliberately wrote code that would panic, the way to fix the panic is to not
request an element beyond the range of the vector indexes. When your code
panics in the future, you’ll need to figure out what action the code is taking
with what values to cause the panic and what the code should do instead.

We’ll come back to `panic!` and when we should and should not use `panic!` to
handle error conditions in the [“To `panic!` or Not to
`panic!`”][to-panic-or-not-to-panic]<!-- ignore --> section later in this
chapter. Next, we’ll look at how to recover from an error using `Result`.

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
