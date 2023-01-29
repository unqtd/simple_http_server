extern crate simple_http_server;

use simple_http_server::{Code, Response, SimpleHttpServer};

fn main() {
    let addr = std::env::var("ADDR").expect("Ожидается переменная ADDR!");

    let server = SimpleHttpServer::new(&addr)
        .unwrap()
        .handle_request("/", |request| {
            println!("\n{request:?}\n");

            Response::new(Code::Ok)
                .header("Content-Type", "text/html")
                .body("<h1>Hello World</h1>".into())
        })
        .build();

    server.listen()
}
