# simple_http_server

Демонстрационная реализация игрушечного однопоточного синхронного веб-сервера. 

### Usage

Hello World пример:
```rust
extern crate simple_http_server;

use simple_http_server::{Code, Handlers, Response, SimpleHttpServer};

fn main() {
    let addr = std::env::var("ADDR").expect("Ожидается переменная ADDR!");

    let server = SimpleHttpServer::new(
        &addr,
        Handlers {
            on_request: |req| {
                println!("{req:?}");

                Response::from(Code::Ok)
                    .header("Content-Type", "text/html")
                    .body("<h1>Hello World</h1>".into())
            },
            on_error: |err| eprintln!("{err:?}"),
        },
    )
    .unwrap();

    server.listen()
}
```

Run:
```bash
ADDR="localhost:7070" cargo run --example hello_world
```

### Ссылки

* [Rustbook](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
