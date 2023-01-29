mod body;
mod handler;
mod headers;
mod http_connection;
mod http_parser;
mod http_server_builder;
pub mod request;
pub mod response;

use std::{io, net::TcpListener};

use handler::{Handler, Handlers};
use http_connection::{HttpConnection, HttpError};
use http_server_builder::SimpleHttpServerBuilder;

use crate::response::Response;

pub struct SimpleHttpServer<'a> {
    listener: TcpListener,
    handlers_on_request: Handlers<'a>,
    handler_on_http_error: HttpErrorHandler,
    handler_on_startup: StartupHandler,
    handler_on_not_found: NotFoundHandler,
}

impl<'a> SimpleHttpServer<'a> {
    pub fn new(addr: &str) -> io::Result<SimpleHttpServerBuilder<'a>> {
        Ok(SimpleHttpServerBuilder(Self {
            listener: TcpListener::bind(addr)?,
            handlers_on_request: Handlers::new(),
            handler_on_http_error: |err| eprintln!("Something went wrong: {err:?}!"),
            handler_on_startup: Box::new(|| {}),
            handler_on_not_found: |_| Response::not_found(),
        }))
    }

    pub fn listen(self) -> ! {
        (self.handler_on_startup)();

        for stream in self.listener.incoming() {
            let mut connection = HttpConnection(stream.unwrap());

            match connection.read_request() {
                Ok(request) => {
                    if let Some(handler) = self.handlers_on_request.get(request.url.uri.as_str()) {
                        connection.send_response(&handler(request)).unwrap();
                    } else {
                        connection
                            .send_response(&(self.handler_on_not_found)(request))
                            .unwrap();
                    }
                }
                Err(err) => {
                    (self.handler_on_http_error)(err);
                    connection.send_response(&Response::bad_request()).unwrap();
                }
            }
        }

        panic!()
    }
}

type HttpErrorHandler = fn(HttpError);
type StartupHandler = Box<dyn FnOnce()>;
type NotFoundHandler = Handler;
