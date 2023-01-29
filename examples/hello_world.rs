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
