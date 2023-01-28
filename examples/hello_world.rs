extern crate simple_http_server;

use simple_http_server::{response::Response, SimpleHttpServer};

fn main() {
    let port = 7070;
    let addr = format!("{host}:{port}", host = std::env::var("HOST").unwrap());

    let startup = move || println!("Server start on {port} 🚀!");

    let server = SimpleHttpServer::new(&addr)
        .unwrap()
        .handle_startup(startup)
        .handle_request("/", |request| {
            println!("{request:?}");

            Response::ok()
                .header("Content-Type", "text/html")
                .body("<h1>Hello World</h1>".into())
        });

    server.listen()
}
