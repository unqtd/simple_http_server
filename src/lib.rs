mod http_connection;
mod types;

use http_connection::HttpConnection;
use std::{io, net::TcpListener};

pub use types::{request::Request, response::responder::Responder, response::Code};

/// Callback-реакция на приходящий запрос.
type Handler = fn(Request) -> Responder;

pub struct SimpleHttpServer<'a> {
    addr: &'a str,
    listener: TcpListener,
    request_handler: Handler,
}

impl<'a> SimpleHttpServer<'a> {
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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(addr: &'a str, handler: Handler) -> io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            request_handler: handler,
            addr,
        })
    }

    /// Устанавливает `SimpleHttpServer` в состояние прослушивания.
    ///
    /// # Panics
    ///
    /// Панику вызывают ошибки связанные с работой сетью, IO:
    /// * Не удалось установить соединение.
    /// * Не удалось отправить ответ.
    pub fn listen(self) {
        println!("[INFO]: Сервер запущен на {} 🚀!", self.addr);

        for stream in self.listener.incoming() {
            let mut connection = HttpConnection(stream.unwrap());

            match connection.read_request() {
                Ok(request) => {
                    println!("[TRACE]: {request:?}");

                    let response = (self.request_handler)(request).response();
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
    }
}
