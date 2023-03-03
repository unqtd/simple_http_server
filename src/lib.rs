mod common;
mod connection;
mod request;
mod response;

use connection::{errors::HttpError, HttpConnection};
use std::{io, net::TcpListener};

pub use request::Request;
pub use response::{Code, Response};

pub struct SimpleHttpServer<OnReqHandler, OnErrHandler> {
    listener: TcpListener,
    handlers: Handlers<OnReqHandler, OnErrHandler>,
}

pub struct Handlers<OnReq, OnErr> {
    pub on_request: OnReq,
    pub on_error: OnErr,
}

impl<OnReq, OnErr> SimpleHttpServer<OnReq, OnErr>
where
    OnReq: FnMut(Request) -> Response,
    OnErr: FnMut(HttpError),
{
    pub fn new(addr: &str, handlers: Handlers<OnReq, OnErr>) -> io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            handlers,
        })
    }

    pub fn listen(mut self) -> ! {
        loop {
            let mut connection = self.accept();

            if let Some(request) = self.read(&mut connection) {
                let response = (self.handlers.on_request)(request);
                self.send(&mut connection, response);
            } else {
                self.send(&mut connection, Response::from(Code::BadRequest));
            }
        }
    }

    fn accept(&mut self) -> HttpConnection {
        match self.listener.accept() {
            Ok((stream, _)) => HttpConnection(stream),
            Err(err) => {
                (self.handlers.on_error)(HttpError::Io(err));
                self.accept()
            }
        }
    }

    fn read(&mut self, connection: &mut HttpConnection) -> Option<Request> {
        match connection.request() {
            Ok(request) => Some(request),
            Err(err) => {
                (self.handlers.on_error)(HttpError::Request(err));
                None
            }
        }
    }

    fn send(&mut self, connection: &mut HttpConnection, response: Response) {
        if let Err(error) = connection.send(response) {
            (self.handlers.on_error)(HttpError::Io(error));
        }
    }
}
