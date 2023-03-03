use super::errors::{InvalidSyntax, RequestError};

use crate::{
    common::{headers::Headers, Body},
    request::Request,
};

use std::{
    io::{self, BufRead, BufReader, Read},
    net::TcpStream,
};

pub fn read(stream: &mut TcpStream) -> Result<Request, RequestError> {
    let mut bufreader = BufReader::new(stream);

    let (method, uri) =
        parser::parse_starting_line(&read_line(&mut bufreader).map_err(RequestError::Io)?)
            .map_err(RequestError::Invalid)?;

    let headers = read_headers(&mut bufreader).map_err(RequestError::Invalid)?;

    let body = if let Some(content_length) = headers.get("Content-Length") {
        let content_length = content_length
            .parse::<u64>()
            .map_err(|_| RequestError::Invalid(InvalidSyntax::Header))?;

        Some(read_body(&mut bufreader, content_length).map_err(RequestError::Io)?)
    } else {
        None
    };

    Ok(Request {
        uri,
        method,
        headers,
        body,
    })
}

fn read_body(bufreader: &mut BufReader<&mut TcpStream>, length: u64) -> io::Result<Body> {
    let mut chunk = bufreader.take(length);

    let mut body = Body::new();
    chunk.read_to_end(&mut body)?;

    Ok(body)
}

fn read_headers(bufreader: &mut BufReader<&mut TcpStream>) -> Result<Headers, InvalidSyntax> {
    bufreader
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .map(|line| parser::parse_header(&line))
        .collect()
}

fn read_line(bufreader: &mut BufReader<&mut TcpStream>) -> io::Result<String> {
    let mut line = String::new();
    bufreader.read_line(&mut line)?;
    Ok(line)
}

mod parser {
    use crate::{
        connection::errors::InvalidSyntax,
        request::{method::Method, Uri},
    };

    pub fn parse_starting_line(line: &str) -> Result<(Method, Uri), InvalidSyntax> {
        let (method, line) = line.split_once(' ').ok_or(InvalidSyntax::StartingLine)?;
        let (uri, _) = line.split_once(' ').ok_or(InvalidSyntax::StartingLine)?;

        Ok((
            method.try_into().map_err(InvalidSyntax::Method)?,
            parse_uri(uri),
        ))
    }

    fn parse_uri(input: &str) -> Uri {
        if let Some((path, query_string)) = input.split_once('?') {
            Uri {
                path: path.to_string(),
                query_string: Some(query_string.to_string()),
            }
        } else {
            Uri {
                path: input.to_string(),
                query_string: None,
            }
        }
    }

    pub fn parse_header(line: &str) -> Result<(String, String), InvalidSyntax> {
        let (key, value) = line.split_once(':').ok_or(InvalidSyntax::Header)?;
        Ok((key.to_string(), value.trim().to_string()))
    }
}
