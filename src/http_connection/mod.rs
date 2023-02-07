mod parser;

use crate::types::{
    body::Body,
    headers::Headers,
    http_error::HttpError,
    request::{Request, Url},
    response::Response,
};

use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

pub struct HttpConnection(pub(crate) TcpStream);

impl HttpConnection {
    pub fn read_request(&mut self) -> Result<Request, HttpError> {
        let mut bufreader = BufReader::new(&mut self.0);

        let starting_line = Self::read_line(&mut bufreader)?;
        let (method, uri) = parser::parse_starting_line(&starting_line)?;

        let headers = Self::read_headers(&mut bufreader)?;
        let host = headers.get("Host").ok_or(HttpError::NotFoundHost)?.clone();

        let body = if let Some(length) = headers.get("Content-Length") {
            let length = length.parse().map_err(|_| {
                HttpError::InvalidSyntaxRequest("Некорректное значения заголовка Content-Length!")
            })?;

            if length == 0 {
                None
            } else {
                Some(Self::read_body(&mut bufreader, length)?)
            }
        } else {
            None
        };

        Ok(Request {
            url: Url { uri, host },
            method,
            headers,
            body,
        })
    }

    pub fn send_response(&mut self, response: &Response) -> Result<(), HttpError> {
        let mut bufwriter = BufWriter::new(&mut self.0);

        let starting_line = format!(
            "HTTP/1.1 {code_and_reason}\r\n",
            code_and_reason = response.code.as_str()
        );

        // Отправка стартовой строки
        Self::send_bytes(&mut bufwriter, starting_line.as_bytes())?;
        // Отправка заголовков
        Self::send_bytes(&mut bufwriter, response.headers.as_bytes())?;
        // Отправка разделителя между заголовком ответа и телом
        Self::send_bytes(&mut bufwriter, b"\r\n")?;

        // Отправка тела запроса, если оно есть
        if let Some(body) = &response.body {
            Self::send_bytes(&mut bufwriter, body.as_slice())?;
        }

        bufwriter.flush().map_err(HttpError::Io)?;
        Ok(())
    }

    fn read_line(bufreader: &mut BufReader<&mut TcpStream>) -> Result<String, HttpError> {
        let mut buffer = String::new();
        bufreader.read_line(&mut buffer).map_err(HttpError::Io)?;
        Ok(buffer)
    }

    fn send_bytes(
        bufwriter: &mut BufWriter<&mut TcpStream>,
        bytes: &[u8],
    ) -> Result<(), HttpError> {
        bufwriter.write_all(bytes).map_err(HttpError::Io)
    }

    fn read_headers(bufreader: &mut BufReader<&mut TcpStream>) -> Result<Headers, HttpError> {
        bufreader
            .lines()
            .map(Result::unwrap)
            .take_while(|line| !line.is_empty())
            .map(|line| parser::parse_header(&line))
            .collect()
    }

    fn read_body(
        bufreader: &mut BufReader<&mut TcpStream>,
        length: u64,
    ) -> Result<Body, HttpError> {
        let mut chunk = bufreader.take(length);

        let mut body = Body::new();
        chunk.read_to_end(&mut body).map_err(HttpError::Io)?;

        Ok(body)
    }
}
