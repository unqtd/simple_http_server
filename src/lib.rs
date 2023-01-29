use addr::Addr;
use http_server_builder::SimpleHttpServerBuilder;
use protocol_impl::http_connection::{HttpConnection, HttpError};
use std::{collections::HashMap, io, net::TcpListener};
pub use types::{
    request::Request,
    response::{Code, Response},
    response_builder::ResponseBuilder,
};

mod addr;
mod http_server_builder;
mod protocol_impl;
mod types;

pub struct SimpleHttpServer<'a> {
    addr: Addr<'a>,
    listener: TcpListener,
    handlers_on_request: Handlers<'a>,
    handler_on_http_error: HttpErrorHandler,
    handler_on_startup: StartupHandler,
    handler_on_not_found: NotFoundHandler,
}

impl<'a> SimpleHttpServer<'a> {
    pub fn bind(addr: &'a str) -> io::Result<SimpleHttpServerBuilder<'a>> {
        Ok(SimpleHttpServerBuilder(Self {
            listener: TcpListener::bind(addr)?,
            addr: Addr::from(addr),
            handlers_on_request: Handlers::new(),
            handler_on_http_error: |err| eprintln!("Something went wrong: {err:?}!"),
            handler_on_startup: Box::new(|Addr { source: addr, .. }| {
                println!("Server start on {addr} 🚀!")
            }),
            handler_on_not_found: |_| Response::new(Code::NotFound),
        }))
    }

    pub fn listen(self) -> ! {
        (self.handler_on_startup)(&self.addr);

        for stream in self.listener.incoming() {
            let mut connection = HttpConnection(stream.unwrap());

            match connection.read_request() {
                Ok(request) => {
                    if let Some(handler) =
                        self.handlers_on_request.get(request.url.uri.path.as_str())
                    {
                        connection.send_response(&handler(request).build()).unwrap();
                    } else {
                        connection
                            .send_response(&(self.handler_on_not_found)(request).build())
                            .unwrap();
                    }
                }
                Err(err) => {
                    (self.handler_on_http_error)(err);
                    connection
                        .send_response(&Response::new(Code::BadRequest).build())
                        .unwrap();
                }
            }
        }

        panic!()
    }
}

type HttpErrorHandler = fn(HttpError);
type StartupHandler = Box<dyn FnOnce(&Addr)>;
type NotFoundHandler = Handler;

type Handler = fn(Request) -> ResponseBuilder;
type Handlers<'a> = HashMap<&'a str, Handler>;
