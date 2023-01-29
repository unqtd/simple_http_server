use crate::types::{
    body::Body,
    headers::Headers,
    request::{Request, Url},
    response::Response,
};
use std::{
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

use super::http_parser;

pub struct HttpConnection(pub TcpStream);

pub type IResult<T> = Result<T, HttpError>;

impl HttpConnection {
    pub fn read_request(&mut self) -> IResult<Request> {
        let mut bufreader = BufReader::new(&mut self.0);

        let starting_line = Self::read_line(&mut bufreader)?;
        let (method, uri) = http_parser::parse_starting_line(&starting_line)?;

        let headers = Self::read_headers(&mut bufreader)?;
        let host = headers.get("Host").ok_or(HttpError::NotFoundHost)?.clone();

        let body = if let Some(length) = headers.get("Content-Length") {
            let length = length
                .parse()
                .map_err(|_| HttpError::BadRequest(InvalidBadRequestKind::ContentLengthValue))?;

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

    pub fn send_response(&mut self, response: &Response) -> IResult<()> {
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
        Self::send_bytes(&mut bufwriter, "\r\n".as_bytes())?;

        // Отправка тела запроса, если оно есть
        if let Some(body) = &response.body {
            Self::send_bytes(&mut bufwriter, body.as_slice())?;
        }

        bufwriter.flush().expect("Ошибка при сбросе буффера.");
        Ok(())
    }

    fn read_line(bufreader: &mut BufReader<&mut TcpStream>) -> IResult<String> {
        let mut buffer = String::new();
        bufreader.read_line(&mut buffer).map_err(HttpError::Io)?;
        Ok(buffer)
    }

    fn send_bytes(bufwriter: &mut BufWriter<&mut TcpStream>, bytes: &[u8]) -> IResult<()> {
        bufwriter.write_all(bytes).map_err(HttpError::Io)
    }

    fn read_headers(bufreader: &mut BufReader<&mut TcpStream>) -> IResult<Headers> {
        let mut headers = Headers::new();

        let lines = bufreader
            .lines()
            .map(Result::unwrap)
            .take_while(|x| !x.is_empty());

        for line in lines {
            let (key, value) = http_parser::parse_header(&line)?;
            headers.insert(key, value);
        }

        Ok(headers)
    }

    fn read_body(bufreader: &mut BufReader<&mut TcpStream>, length: u64) -> IResult<Body> {
        let mut chunk = bufreader.take(length);

        let mut body = Body::new();
        chunk.read_to_end(&mut body).map_err(HttpError::Io)?;

        Ok(body)
    }
}

#[derive(Debug)]
pub enum HttpError {
    Io(io::Error),
    BadRequest(InvalidBadRequestKind),
    NotFoundHost,
}

#[derive(Debug)]
pub enum InvalidBadRequestKind {
    Method,
    ContentLengthValue,
    StaringLine,
    HeaderSyntax,
}
