# simple_http_server

Демонстрационная реализация простого однопоточного синхронного веб-сервера, 
он же HTTP-сервер, на Rust'е.

### Usage

Hello World пример:
```rust
// examples/hello_world.rs
extern crate simple_http_server;

use simple_http_server::{response::Response, SimpleHttpServer};

fn main() {
    let (port, host) = (
        7070,
        std::env::var("HOST").expect("Ожидается переменная HOST!"),
    );

    let startup = move || println!("Server start on {port} 🚀!");

    let server = SimpleHttpServer::new(&format!("{host}:{port}"))
        .unwrap()
        .handle_startup(startup)
        .handle_request("/", |request| {
            println!("{request:?}");

            Response::ok()
                .header("Content-Type", "text/html")
                .body("<h1>Hello World</h1>".into())
        })
        .build();

    server.listen()
}
```

Run:
```bash
HOST="localhost" cargo run --example hello_world
```

### Ссылки

* [Rustbook](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

