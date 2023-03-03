use crate::request::method::UnknowMethod;
use std::io;

#[derive(Debug)]
pub enum HttpError {
    Io(io::Error),
    Request(RequestError),
}

#[derive(Debug)]
pub enum RequestError {
    Io(io::Error),
    Invalid(InvalidSyntax),
}

#[derive(Debug)]
pub enum InvalidSyntax {
    Method(UnknowMethod),
    StartingLine,
    Header,
}
