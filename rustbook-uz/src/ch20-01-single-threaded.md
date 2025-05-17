## Bitta oqimli (single-threaded) veb-server yaratish

Biz avval bitta oqimli veb-serverni ishga tushirishdan boshlaymiz. Ishni boshlashdan oldin, veb-serverlarni qurishda ishtirok etadigan protokollar haqida qisqacha ko‘rib chiqamiz. Ushbu protokollarning batafsil tafsilotlari ushbu kitob doirasidan tashqarida, ammo qisqacha tushuntirish sizga zarur bo‘lgan asosiy ma’lumotlarni beradi.

Veb-serverlarda ishtirok etadigan ikkita asosiy protokol bu *Gipermatn uzatish protokoli* *(HTTP)* va *Uzatishni boshqarish protokoli* *(TCP)* hisoblanadi. Har ikkala protokol ham *so‘rov-javob* (request-response) protokollari bo‘lib, bunda *mijoz* (client) so‘rov yuboradi, *server* esa bu so‘rovlarni tinglaydi va mijozga javob qaytaradi. Ushbu so‘rovlar va javoblarning mazmuni protokollar tomonidan aniqlanadi.

TCP bu past darajadagi (lower-level) protokol bo‘lib, ma’lumotlar bir serverdan boshqasiga qanday uzatilishini belgilaydi, lekin bu ma’lumotlarning mazmuni qanday bo‘lishi kerakligini aniqlamaydi. HTTP esa TCP ustida qurilgan bo‘lib, so‘rovlar va javoblarning tarkibini belgilaydi. Texnik jihatdan HTTP’ni boshqa protokollar bilan ham ishlatish mumkin, ammo aksariyat hollarda HTTP o‘z ma’lumotlarini TCP orqali uzatadi. Biz esa TCP va HTTP so‘rov hamda javoblarining xom baytlari (raw bytes) bilan ishlaymiz.

### TCP ulanishni tinglash

Veb-serverimiz TCP ulanishini tinglashi kerak, shuning uchun birinchi navbatda shu qism ustida ishlaymiz. Rust’ning standart kutubxonasi bizga buni amalga oshirish imkonini beruvchi `std::net` modulini taklif qiladi. Keling, odatdagidek yangi loyiha yaratamiz:

```console
$ cargo new salom
     Created binary (application) `salom` project
$ cd salom
```

Endi *src/main.rs* fayliga 20-1 ro‘yxatdagi (Listing 20-1) kodni kiriting. Bu kod `127.0.0.1:7878` lokal manzilida kiruvchi TCP oqimlarini tinglaydi. Har safar yangi oqim kelganda, `Connection established!` (Ulanish o‘rnatildi!) deb chop etadi.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-01/src/main.rs}}
```

<span class="caption">20-1 ro‘yxat: Kiruvchi oqimlarni tinglash va oqim qabul qilinganda xabar chop etish</span>

`TcpListener` yordamida biz `127.0.0.1:7878` manzilida TCP ulanishlarini tinglashimiz mumkin. Manzilda ikki nuqtadan (:) oldingi qism — bu kompyuteringizni ifodalovchi IP manzil (bu barcha kompyuterlarda bir xil bo‘ladi va mualliflarning kompyuteriga xos emas), `7878` esa port raqami. Biz bu portni ikki sababga ko‘ra tanladik: odatda HTTP bu portda ishlatilmaydi, shuning uchun serverimiz kompyuteringizda ishlayotgan boshqa veb-serverlar bilan to‘qnash kelmaydi; ikkinchidan, telefon klaviaturasida *rust* so‘zini terishda 7878 raqamlari ishlatiladi.

Ushbu holatda `bind` funksiyasi `new` funksiyasiga o‘xshab ishlaydi, ya’ni u yangi `TcpListener` obyektini qaytaradi. Funksiya `bind` deb nomlangan, chunki tarmoqlarda portga ulanib tinglash jarayoni “portga bog‘lanish” (binding) deb ataladi.

`bind` funksiyasi `Result<T, E>` turini qaytaradi, bu esa bog‘lanish (binding) muvaffaqiyatsiz bo‘lishi mumkinligini bildiradi. Masalan, 80-portga ulanish uchun administrator huquqlari talab qilinadi (administrator bo‘lmagan foydalanuvchilar faqat 1023 dan yuqori portlarni tinglashi mumkin). Shuning uchun, agar biz administrator bo‘lmasak va 80-portga ulanishga harakat qilsak, bog‘lanish amalga oshmaydi. Bundan tashqari, agar dasturimizning ikkita nusxasini ishga tushirsak va ular bir xil portda tinglashga harakat qilsa, bog‘lanish yana amalga oshmaydi. Biz bu yerda faqat o‘rganish maqsadida oddiy server yozayotganimiz uchun bunday xatoliklarni oldini olish haqida hozircha qayg‘urmaymiz; uning o‘rniga dasturimizda xatolik yuz bersa dasturni to'xtatadigan `unwrap` funksiyasidan foydalanamiz.

`TcpListener` ustidagi `incoming` metodi bizga oqimlar ketma-ketligini (ya’ni, `TcpStream` turidagi oqimlar) taqdim etuvchi iteratorni qaytaradi. Har bir *oqim* (stream) mijoz (client) va server o‘rtasidagi ochiq ulanishni ifodalaydi. *Ulanish* (connection) deganda, mijoz serverga ulanadigan, server javob tayyorlab qaytaradigan va so‘ng ulanishni yopadigan to‘liq so‘rov-javob (request response) jarayoni tushuniladi. Shunday ekan, biz `TcpStream` dan o‘qib, mijoz nimani yuborganini bilamiz va javobimizni aynan shu oqim orqali yozib, mijozga yuboramiz. Umuman olganda, bu `for` sikli har bir ulanishni navbati bilan qayta ishlaydi va bizga boshqarish uchun bir nechta oqimlar beradi.

Hozircha oqimni (stream) qayta ishlashimiz faqat `unwrap` chaqirishdan iborat: agar oqimda xatolik yuz bersa, dastur to‘xtaydi; xatolik bo‘lmasa, dastur xabar chop etadi. Kelasi ro‘yxatda muvaffaqiyatli holatlar uchun ko‘proq funksionallik qo‘shamiz. Mijoz serverga ulanganida `incoming` metodidan xatoliklar chiqishi mumkin, chunki biz aslida ulanishlarning o‘zini emas, *ulanishga urinishlarni* (connection attemps) ko‘rib chiqayapmiz (iterating). Har bir urinish muvaffaqiyatli bo‘lavermasligi mumkin, va buning sabablari ko‘pincha operatsion tizimga bog‘liq bo‘ladi. Masalan, ko‘plab operatsion tizimlarda bir vaqtning o‘zida ochiq bo‘lishi mumkin bo‘lgan ulanishlar soni cheklangan bo‘ladi; bu limitdan oshib ketilganida, yangi ulanishga urinishlar xatolik chiqaradi, toki mavjud ulanishlardan ba’zilari yopilmaguncha.

Keling, ushbu kodni ishga tushirib ko‘ramiz! Terminalda `cargo run` buyrug‘ini invoke qiling (yurg'azing) va so‘ng brauzeringizda *127.0.0.1:7878* manzilini oching. Brauzer “Connection reset” (Ulanish tiklandi) kabi xatolik xabarini ko‘rsatishi mumkin, chunki hozircha server hech qanday ma’lumot qaytarayotgani yo‘q. Biroq terminalingizga qarasangiz, brauzer serverga ulanganida chiqarilgan bir nechta xabarlarni ko‘rishingiz mumkin bo‘ladi!

```text
     Running `target/debug/salom`
Connection established!
Connection established!
Connection established!
```

Ba’zan brauzerning bitta so‘rovi uchun bir nechta xabarlar chiqishini ko‘rishingiz mumkin; buning sababi shundaki, brauzer sahifa uchun so‘rov yuborish bilan birga, brauzer tab da (yorlig‘ida) ko‘rinadigan *favicon.ico* kabi boshqa resurslar uchun ham so‘rov yuboradi.

Brauzer bir necha marta serverga ulanishga harakat qilayotgan bo‘lishi ham mumkin, chunki server hech qanday ma’lumot yubormayapti. `stream` sikl oxirida ko‘lam (scope) dan chiqib ketganda, `drop` implementatsiyasi orqali ulanish yopiladi. Brauzerlar ba’zida yopilgan ulanishlarga qayta urinish qiladi, chunki muammo vaqtincha bo‘lishi mumkin. Muhim jihati shuki — biz TCP ulanishni muvaffaqiyatli qo‘lga kiritdik!

Dasturdan foydalanishni tugatganingizda, uni <span class="keystroke">ctrl-c</span> tugmasi yordamida to‘xtatishni unutmang. Har safar kodga o‘zgartirish kiritganingizdan so‘ng, eng so‘nggi versiyasi ishga tushirilayotganiga ishonch hosil qilish uchun `cargo run` buyrug‘ini qayta ishga tushuring.

### So'rovni o'qish

Brauzerdan yuborilgan so‘rovni o‘qish funksiyasini ishlab chiqamiz! Avval ulanishni (connection) qabul qilish va keyin ular bilan ishlash bosqichlarini bir-biridan ajratish uchun, ulanishni qayta ishlovchi yangi funksiya yozamiz. Bu yangi `handle_connection` funksiyasida TCP oqimidan (stream) ma’lumotlarni o‘qiymiz va brauzer yuborayotgan ma’lumotlarni ko‘rish uchun ularni chop etamiz. Kodni 20-02 ro‘yxatdagidek qilib o‘zgartiring.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-02/src/main.rs}}
```

<span class="caption">20-02 ro‘yxat: TcpStream dan o‘qish va yuborilgan ma’lumotni chop etish</span>

`std::io::prelude` va `std::io::BufReader` ni ko‘lamga (scope) olib kiramiz — bu bizga oqimdan (from stream) o‘qish va yozish imkonini beradigan trait va tiplardan foydalanish imkonini beradi. Endi `main` funksiyasidagi `for` siklida ulanish o‘rnatilgani haqida xabar chiqarish o‘rniga, yangi `handle_connection` funksiyasini chaqiramiz va unga stream `ni` uzatamiz.

`handle_connection` funksiyasida biz `oqimga (stream)` o‘zgaruvchan (mutable) murojaatni o‘rab oladigan yangi `BufReader` obyektini yaratamiz. `BufReader` buferlashni qo‘shadi — ya’ni u `std::io::Read` traitining metodlariga murojaat qilishni boshqaradi.

Brauzer serverimizga yuboradigan so‘rov satrlarini yig‘ish uchun `http_request` nomli o‘zgaruvchi yaratamiz. Bu satrlarni vector (vektor) ko‘rinishida yig‘moqchi ekanligimizni ko‘rsatish uchun `Vec<_>` tip annotatsiyasini qo‘shamiz.

`BufReader` implements the `std::io::BufRead` trait, which provides the `lines`
method. The `lines` method returns an iterator of `Result<String,
std::io::Error>` by splitting the stream of data whenever it sees a newline
byte. To get each `String`, we map and `unwrap` each `Result`. The `Result`
might be an error if the data isn’t valid UTF-8 or if there was a problem
reading from the stream. Again, a production program should handle these errors
more gracefully, but we’re choosing to stop the program in the error case for
simplicity.

`BufReader`  `std::io::BufRead` traitini amalga oshiradi va shu orqali `lines` metodini taqdim etadi. `Lines` metodi har gal yangi qatordan ajratib, oqimdagi ma’lumotlarni `Result<String, std::io::Error>` ko‘rinishidagi iterator sifatida qaytaradi. Har bir String qiymatni olish uchun biz har bir Result ustida map va unwrap chaqiramiz. Agar ma’lumotlar yaroqsiz UTF-8 formatida bo‘lsa yoki oqimdan o‘qishda muammo yuz bersa, Result xatolik (error) bo‘lishi mumkin. Ishlab chiqarish darajasidagi dastur bu xatolarni ancha ehtiyotkorlik bilan boshqarishi kerak, ammo soddalashtirish uchun biz bu yerda xatolik yuz bersa, dasturni to‘xtatamiz.

The browser signals the end of an HTTP request by sending two newline
characters in a row, so to get one request from the stream, we take lines until
we get a line that is the empty string. Once we’ve collected the lines into the
vector, we’re printing them out using pretty debug formatting so we can take a
look at the instructions the web browser is sending to our server.

Let’s try this code! Start the program and make a request in a web browser
again. Note that we’ll still get an error page in the browser, but our
program’s output in the terminal will now look similar to this:

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

Depending on your browser, you might get slightly different output. Now that
we’re printing the request data, we can see why we get multiple connections
from one browser request by looking at the path after `GET` in the first line
of the request. If the repeated connections are all requesting */*, we know the
browser is trying to fetch */* repeatedly because it’s not getting a response
from our program.

Let’s break down this request data to understand what the browser is asking of
our program.

### A Closer Look at an HTTP Request

HTTP is a text-based protocol, and a request takes this format:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is the *request line* that holds information about what the
client is requesting. The first part of the request line indicates the *method*
being used, such as `GET` or `POST`, which describes how the client is making
this request. Our client used a `GET` request, which means it is asking for
information.

The next part of the request line is */*, which indicates the *Uniform Resource
Identifier* *(URI)* the client is requesting: a URI is almost, but not quite,
the same as a *Uniform Resource Locator* *(URL)*. The difference between URIs
and URLs isn’t important for our purposes in this chapter, but the HTTP spec
uses the term URI, so we can just mentally substitute URL for URI here.

The last part is the HTTP version the client uses, and then the request line
ends in a *CRLF sequence*. (CRLF stands for *carriage return* and *line feed*,
which are terms from the typewriter days!) The CRLF sequence can also be
written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed. The
CRLF sequence separates the request line from the rest of the request data.
Note that when the CRLF is printed, we see a new line start rather than `\r\n`.

Looking at the request line data we received from running our program so far,
we see that `GET` is the method, */* is the request URI, and `HTTP/1.1` is the
version.

After the request line, the remaining lines starting from `Host:` onward are
headers. `GET` requests have no body.

Try making a request from a different browser or asking for a different
address, such as *127.0.0.1:7878/test*, to see how the request data changes.

Now that we know what the browser is asking for, let’s send back some data!

### Writing a Response

We’re going to implement sending data in response to a client request.
Responses have the following format:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is a *status line* that contains the HTTP version used in the
response, a numeric status code that summarizes the result of the request, and
a reason phrase that provides a text description of the status code. After the
CRLF sequence are any headers, another CRLF sequence, and the body of the
response.

Here is an example response that uses HTTP version 1.1, has a status code of
200, an OK reason phrase, no headers, and no body:

```text
HTTP/1.1 200 OK\r\n\r\n
```

The status code 200 is the standard success response. The text is a tiny
successful HTTP response. Let’s write this to the stream as our response to a
successful request! From the `handle_connection` function, remove the
`println!` that was printing the request data and replace it with the code in
Listing 20-3.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-03/src/main.rs:here}}
```

<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to
the stream</span>

The first new line defines the `response` variable that holds the success
message’s data. Then we call `as_bytes` on our `response` to convert the string
data to bytes. The `write_all` method on `stream` takes a `&[u8]` and sends
those bytes directly down the connection. Because the `write_all` operation
could fail, we use `unwrap` on any error result as before. Again, in a real
application you would add error handling here.

With these changes, let’s run our code and make a request. We’re no longer
printing any data to the terminal, so we won’t see any output other than the
output from Cargo. When you load *127.0.0.1:7878* in a web browser, you should
get a blank page instead of an error. You’ve just hand-coded receiving an HTTP
request and sending a response!

### Returning Real HTML

Let’s implement the functionality for returning more than a blank page. Create
the new file *hello.html* in the root of your project directory, not in the
*src* directory. You can input any HTML you want; Listing 20-4 shows one
possibility.

<span class="filename">Filename: hello.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-05/hello.html}}
```

<span class="caption">Listing 20-4: A sample HTML file to return in a
response</span>

This is a minimal HTML5 document with a heading and some text. To return this
from the server when a request is received, we’ll modify `handle_connection` as
shown in Listing 20-5 to read the HTML file, add it to the response as a body,
and send it.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-05/src/main.rs:here}}
```

<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>

We’ve added `fs` to the `use` statement to bring the standard library’s
filesystem module into scope. The code for reading the contents of a file to a
string should look familiar; we used it in Chapter 12 when we read the contents
of a file for our I/O project in Listing 12-4.

Next, we use `format!` to add the file’s contents as the body of the success
response. To ensure a valid HTTP response, we add the `Content-Length` header
which is set to the size of our response body, in this case the size of
`hello.html`.

Run this code with `cargo run` and load *127.0.0.1:7878* in your browser; you
should see your HTML rendered!

Currently, we’re ignoring the request data in `http_request` and just sending
back the contents of the HTML file unconditionally. That means if you try
requesting *127.0.0.1:7878/something-else* in your browser, you’ll still get
back this same HTML response. At the moment, our server is very limited and
does not do what most web servers do. We want to customize our responses
depending on the request and only send back the HTML file for a well-formed
request to */*.

### Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the
client requested. Let’s add functionality to check that the browser is
requesting */* before returning the HTML file and return an error if the
browser requests anything else. For this we need to modify `handle_connection`,
as shown in Listing 20-6. This new code checks the content of the request
received against what we know a request for */* looks like and adds `if` and
`else` blocks to treat requests differently.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-06/src/main.rs:here}}
```

<span class="caption">Listing 20-6: Handling requests to */* differently from
other requests</span>

We’re only going to be looking at the first line of the HTTP request, so rather
than reading the entire request into a vector, we’re calling `next` to get the
first item from the iterator. The first `unwrap` takes care of the `Option` and
stops the program if the iterator has no items. The second `unwrap` handles the
`Result` and has the same effect as the `unwrap` that was in the `map` added in
Listing 20-2.

Next, we check the `request_line` to see if it equals the request line of a GET
request to the */* path. If it does, the `if` block returns the contents of our
HTML file.

If the `request_line` does *not* equal the GET request to the */* path, it
means we’ve received some other request. We’ll add code to the `else` block in
a moment to respond to all other requests.

Run this code now and request *127.0.0.1:7878*; you should get the HTML in
*hello.html*. If you make any other request, such as
*127.0.0.1:7878/something-else*, you’ll get a connection error like those you
saw when running the code in Listing 20-1 and Listing 20-2.

Now let’s add the code in Listing 20-7 to the `else` block to return a response
with the status code 404, which signals that the content for the request was
not found. We’ll also return some HTML for a page to render in the browser
indicating the response to the end user.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-07/src/main.rs:here}}
```

<span class="caption">Listing 20-7: Responding with status code 404 and an
error page if anything other than */* was requested</span>

Here, our response has a status line with status code 404 and the reason phrase
`NOT FOUND`. The body of the response will be the HTML in the file *404.html*.
You’ll need to create a *404.html* file next to *hello.html* for the error
page; again feel free to use any HTML you want or use the example HTML in
Listing 20-8.

<span class="filename">Filename: 404.html</span>

```html
{{#include ../listings/ch20-web-server/listing-20-07/404.html}}
```

<span class="caption">Listing 20-8: Sample content for the page to send back
with any 404 response</span>

With these changes, run your server again. Requesting *127.0.0.1:7878* should
return the contents of *hello.html*, and any other request, like
*127.0.0.1:7878/foo*, should return the error HTML from *404.html*.

### A Touch of Refactoring

At the moment the `if` and `else` blocks have a lot of repetition: they’re both
reading files and writing the contents of the files to the stream. The only
differences are the status line and the filename. Let’s make the code more
concise by pulling out those differences into separate `if` and `else` lines
that will assign the values of the status line and the filename to variables;
we can then use those variables unconditionally in the code to read the file
and write the response. Listing 20-9 shows the resulting code after replacing
the large `if` and `else` blocks.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-09/src/main.rs:here}}
```

<span class="caption">Listing 20-9: Refactoring the `if` and `else` blocks to
contain only the code that differs between the two cases</span>

Now the `if` and `else` blocks only return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement, as discussed in Chapter 18.

The previously duplicated code is now outside the `if` and `else` blocks and
uses the `status_line` and `filename` variables. This makes it easier to see
the difference between the two cases, and it means we have only one place to
update the code if we want to change how the file reading and response writing
work. The behavior of the code in Listing 20-9 will be the same as that in
Listing 20-8.

Awesome! We now have a simple web server in approximately 40 lines of Rust code
that responds to one request with a page of content and responds to all other
requests with a 404 response.

Currently, our server runs in a single thread, meaning it can only serve one
request at a time. Let’s examine how that can be a problem by simulating some
slow requests. Then we’ll fix it so our server can handle multiple requests at
once.
