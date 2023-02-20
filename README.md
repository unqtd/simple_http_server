# simple_http_server

Демонстрационная реализация простого однопоточного синхронного веб-сервера, 
он же HTTP/1.1-сервер, на Rust'е.

### Usage

Hello World пример:
```rust
extern crate simple_http_server;

use simple_http_server::{Code, Responder, SimpleHttpServer};

fn main() {
    let addr = std::env::var("ADDR").expect("Ожидается переменная ADDR!");

    let server = SimpleHttpServer::new(&addr, |_| {
        Responder::new(Code::Ok)
            .header("Content-Type", "text/html")
            .body("<h1>Hello World</h1>".into())
    })
    .unwrap();

    server.listen()
}
```

Run:
```bash
ADDR="localhost:7070" cargo run --example hello_world
```

### Структура проекта 

Основной исходный код сервера содержит директория `src`, внутри которой:
- `lib.rs` - точка-входа сервера.  
- `http_connection` - реализация HTTP-соединения. 
- `types` - содержит типовое описание объектов Request, Response и т.д., а 
также методы для их обработки. 

### Ссылки

* [Rustbook](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
