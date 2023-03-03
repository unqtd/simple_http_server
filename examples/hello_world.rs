extern crate simple_http_server;

use simple_http_server::{Code, Handlers, Response, SimpleHttpServer};

fn main() {
    let addr = std::env::var("ADDR").expect("Ожидается переменная ADDR!");

    let server = SimpleHttpServer::new(
        &addr,
        Handlers {
            on_request: |req| {
                println!("{req:?}\n");

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
