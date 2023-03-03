pub mod errors;
mod request_reader;

use crate::{request::Request, Response};
use std::{
    io::{self, BufWriter, Write},
    net::TcpStream,
};

pub struct HttpConnection(pub TcpStream);

impl HttpConnection {
    pub fn request(&mut self) -> Result<Request, errors::RequestError> {
        request_reader::read(&mut self.0)
    }

    pub fn send(&mut self, response: Response) -> io::Result<()> {
        let mut bufwriter = BufWriter::new(&mut self.0);

        let starting_line = format!(
            "HTTP/1.1 {code_and_reason}\r\n",
            code_and_reason = response.code.to_string()
        );

        // Отправка стартовой строки
        bufwriter.write_all(starting_line.as_bytes())?;
        // Отправка заголовков
        bufwriter.write_all(response.headers.as_bytes())?;
        // Отправка разделителя между заголовком ответа и телом
        bufwriter.write_all(b"\r\n")?;

        // Отправка тела запроса, если оно есть
        if let Some(body) = &response.body {
            bufwriter.write_all(body)?;
        }

        bufwriter.flush()?;
        Ok(())
    }
}
