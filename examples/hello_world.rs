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
