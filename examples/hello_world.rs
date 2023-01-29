extern crate simple_http_server;

use simple_http_server::{
    response::{Code, Response},
    SimpleHttpServer,
};

fn main() {
    let addr = std::env::var("ADDR").expect("Ожидается переменная ADDR!");

    let server = SimpleHttpServer::bind(&addr)
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
