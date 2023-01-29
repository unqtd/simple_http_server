use addr::Addr;
use builder::SimpleHttpServerBuilder;
use protocol_impl::http_connection::{HttpConnection, HttpError};
use std::{collections::HashMap, io, net::TcpListener};
use types::response::builder::ResponseBuilder;

pub use types::{
    request::Request,
    response::{Code, Response},
};

mod addr;
mod builder;
mod protocol_impl;
mod types;

/// Интерфейс инкапсулирующий в себе работу с сетью и обработку запросов.
pub struct SimpleHttpServer<'a> {
    /// Адрес на котором весит сервер
    addr: Addr<'a>,
    /// Интерфейс из stdlib для работы с TCP
    listener: TcpListener,
    /// Обработчик корректных HTTP-запросов
    handlers_on_request: Handlers<'a>,
    /// Обработчик некорректных HTTP-запросов
    handler_on_http_error: HttpErrorHandler,
    /// Обработчик действия при старте сервера
    handler_on_startup: StartupHandler,
    /// Обработчик для HTTP-запросов, для которых не был найден обработчик
    handler_on_not_found: NotFoundHandler,
}

impl<'a> SimpleHttpServer<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(addr: &'a str) -> io::Result<SimpleHttpServerBuilder<'a>> {
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
