# simple_http_server

Демонстрационная реализация простого однопоточного синхронного веб-сервера, 
он же HTTP-сервер, на Rust'е.

### Usage

Hello World пример:
```rust
// examples/hello_world.rs
extern crate simple_http_server;

use simple_http_server::{Code, Response, SimpleHttpServer};

fn main() {
    let addr = std::env::var("ADDR").expect("Ожидается переменная ADDR!");

    let server = SimpleHttpServer::new(&addr)
        .unwrap()
        .handle_request("/", |request| {
            println!("{request:?}");

            Response::new(Code::Ok)
                .header("Content-Type", "text/html")
                .body("<h1>Hello World</h1>".into())
        })
        .build();

    server.listen()
}
```

Run:
```bash
ADDR="localhost:7070" cargo run --example hello_world
```

### Ссылки

* [Rustbook](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

<!-- ### Структура проекта  -->

<!-- ``` -->
<!-- . -->
<!-- ├── addr.rs -->
<!-- ├── builder.rs -->
<!-- ├── lib.rs -->
<!-- ├── protocol_impl -->
<!-- │   ├── http_connection.rs -->
<!-- │   ├── http_parser.rs -->
<!-- │   └── mod.rs -->
<!-- └── types -->
<!--     ├── body.rs -->
<!--     ├── headers.rs -->
<!--     ├── mod.rs -->
<!--     ├── request.rs -->
<!--     └── response -->
<!--         ├── builder.rs -->
<!--         └── mod.rs -->
<!-- ``` -->
<!-- * `lib.rs` - содержит в себе высокоуровненную логику обработки запросов, а  -->
<!-- также по совместительству и интерфейс для пользователя.  -->
<!-- * `types` - здесь содержится типовое описание HTTP-протокола, а именно структуру -->
<!-- запроса и ответа.  -->
<!-- * `protocol_impl` - а это часть является "ядром" веб-сервера, где происходит весь -->
<!-- процесс взаимодействие с сетью.  -->

