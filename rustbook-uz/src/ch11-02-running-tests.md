## Testlar qanday o'tkazilishini nazorat qilish

Just as `cargo run` compiles your code and then runs the resulting binary, `cargo test` compiles your code in test mode and runs the resulting test binary. The default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results. You can, however, specify command line options to change this default behavior.

Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary. Running `cargo test --help` displays the options you can use with `cargo test`, and running `cargo test -- --help` displays the options you can use after the separator.

### Testlarni parallel yoki ketma-ket bajarish

When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster and you get feedback quicker. Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

For example, say each of your tests runs some code that creates a file on disk named *test-output.txt* and writes some data to that file. Then each test reads the data in that file and asserts that the file contains a particular value, which is different in each test. Because the tests run at the same time, one test might overwrite the file in the time between another test writing and reading the file. The second test will then fail, not because the code is incorrect but because the tests have interfered with each other while running in parallel. One solution is to make sure each test writes to a different file; another solution is to run the tests one at a time.

If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the `--test-threads` flag and the number of threads you want to use to the test binary. Take a look at the following example:

```console
$ cargo test -- --test-threads=1
```

We set the number of test threads to `1`, telling the program not to use any parallelism. Running the tests using one thread will take longer than running them in parallel, but the tests won’t interfere with each other if they share state.

### Funktsiya natijalarini ko'rsatish

By default, if a test passes, Rust’s test library captures anything printed to standard output. For example, if we call `println!` in a test and the test passes, we won’t see the `println!` output in the terminal; we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.

Misol tariqasida, 11-10 ro'yxatida o'z parametrining qiymatini chop etadigan va 10 ni qaytaradigan ahmoqona funksiya, shuningdek, o'tgan test va muvaffaqiyatsizlikka uchragan test mavjud.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```


<span class="caption">Ro'yxat 11-10: `println!`ni chaqiruvchi funksiya uchun testlar</span>

Ushbu testlarni `cargo test` bilan bajarganimizda, biz quyidagi natijani ko'ramiz:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Note that nowhere in this output do we see `I got the value 4`, which is what is printed when the test that passes runs. That output has been captured. The output from the test that failed, `I got the value 8`, appears in the section of the test summary output, which also shows the cause of the test failure.

Agar biz testlardan o'tish uchun yozilgan qiymatlarni ham ko'rishni istasak, Rust-ga `--show-output` bilan muvaffaqiyatli testlar natijasini ham ko'rsatishni aytishimiz mumkin.

```console
$ cargo test -- --show-output
```

11-10 ro'yxatdagi testlarni yana `--show-output` buyrug'i bilan o'tkazganimizda, biz quyidagi natijani ko'ramiz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Testlar to'plamini nomi bo'yicha bajarish(ishga tushirish)

Sometimes, running a full test suite can take a long time. If you’re working on code in a particular area, you might want to run only the tests pertaining to that code. You can choose which tests to run by passing `cargo test` the name or names of the test(s) you want to run as an argument.

Testlar kichik to‘plamini qanday bajarishni ko‘rsatish uchun avval 11-11 ro‘yxatda ko‘rsatilganidek, `ikkita_qoshish` funksiyamiz uchun uchta test yaratamiz va qaysi birini bajarishni tanlaymiz.

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```


<span class="caption">Ro'yxat 11-11: Uch xil nomga ega uchta test</span>

Agar biz testlarni hech qanday argumentlarsiz o'tkazsak, avval ko'rganimizdek, barcha testlar parallel ravishda ishlaydi:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Yagona testlarni o'tkazish

Biz har qanday test funksiyasining nomini faqat shu testni oʻtkazish uchun `cargo test`ga oʻtkazishimiz mumkin:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Only the test with the name `one_hundred` ran; the other two tests didn’t match that name. The test output lets us know we had more tests that didn’t run by displaying `2 filtered out` at the end.

We can’t specify the names of multiple tests in this way; only the first value given to `cargo test` will be used. But there is a way to run multiple tests.

#### Bir nechta testlarni o'tkazish uchun filtrlash

We can specify part of a test name, and any test whose name matches that value will be run. For example, because two of our tests’ names contain `add`, we can run those two by running `cargo test add`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

This command ran all tests with `add` in the name and filtered out the test named `one_hundred`. Also note that the module in which a test appears becomes part of the test’s name, so we can run all the tests in a module by filtering on the module’s name.

### Maxsus talab qilinmasa, ba'zi testlarni e'tiborsiz qoldirish

Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`. Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the `ignore` attribute to exclude them, as shown here:

<span class="filename">Fayl nomi: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```

After `#[test]` we add the `#[ignore]` line to the test we want to exclude. Now when we run our tests, `it_works` runs, but `expensive_test` doesn’t:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

The `expensive_test` function is listed as `ignored`. If we want to run only the ignored tests, we can use `cargo test -- --ignored`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

By controlling which tests run, you can make sure your `cargo test` results will be fast. When you’re at a point where it makes sense to check the results of the `ignored` tests and you have time to wait for the results, you can run `cargo test -- --ignored` instead. If you want to run all tests whether they’re ignored or not, you can run `cargo test -- --include-ignored`.
