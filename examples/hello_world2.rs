extern crate simple_http_server;

use simple_http_server::{request::Request, response::Response, SimpleHttpServer};

fn main() {
    let port = 7070;
    let addr = format!("{host}:{port}", host = std::env::var("HOST").unwrap());

    let startup = move || println!("Server start on {port} 🚀!");

    let server = SimpleHttpServer::new(&addr)
        .unwrap()
        .handle_startup(startup)
        .handle_not_found(not_found_handler)
        .handle_request("/", main_handler)
        .build();

    server.listen()
}

fn main_handler(request: Request) -> Response {
    println!("{request:?}");

    Response::ok()
        .header("Content-Type", "text/html")
        .body("<h1>Hello World</h1>".into())
}

fn not_found_handler(_: Request) -> Response {
    Response::not_found()
        .header("Content-Type", "text/html")
        .body("<h4> >_< </h4>".into())
}
