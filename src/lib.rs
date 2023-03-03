mod http_connection;
mod types;

use http_connection::HttpConnection;
use std::{io, net::TcpListener};

pub use types::{request::Request, response::responder::Responder, response::Code};

pub struct SimpleHttpServer<'a, Handler> {
    addr: &'a str,
    listener: TcpListener,
    /// Callback-реакция на приходящий запрос.
    handler: Handler,
}

impl<'a, Handler> SimpleHttpServer<'a, Handler>
where
    Handler: FnMut(Request) -> Responder,
{
    /// # Errors
    ///
    /// `Err` будет возвращён в случае провальной попытки создать объект
    /// `TcpListener` по заданному адресу.
    ///
    /// # Examples
    ///
    /// ```
    /// let server = SimpleHttpServer::new(
    ///     "localhost:7070",
    ///     |_| Responder::new(Code::Ok)
    /// ).unwrap();
    /// ```
    pub fn new(addr: &'a str, handler: Handler) -> io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            handler,
            addr,
        })
    }

    /// Устанавливает `SimpleHttpServer` в состояние прослушивания.
    ///
    /// # Panics
    ///
    /// Панику вызывают ошибки связанные с IO: сеть, ...
    pub fn listen(mut self) -> ! {
        println!("[INFO]: Сервер запущен на {} 🚀!", self.addr);

        for stream in self.listener.incoming() {
            let mut connection = HttpConnection(stream.unwrap());

            match connection.read_request() {
                Ok(request) => {
                    println!("[TRACE]: {request:?}");

                    let response = (self.handler)(request).response();
                    connection.send_response(&response).unwrap();
                }
                Err(err) => {
                    eprintln!("[WARN]: {err}");

                    connection
                        .send_response(&Responder::new(Code::BadRequest).response())
                        .unwrap();
                }
            }
        }

        unreachable!()
    }
}
