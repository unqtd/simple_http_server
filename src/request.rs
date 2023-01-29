use crate::{
    body::Body,
    headers::Headers,
    http_connection::{HttpError, InvalidBadRequestKind},
};

#[derive(Debug)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<Body>,
}

#[derive(Debug)]
pub struct Url {
    pub uri: Uri,
    pub host: String,
}

#[derive(Debug)]
pub struct Uri {
    pub path: String,
    pub query_string: Option<String>,
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl TryFrom<&str> for Method {
    type Error = HttpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err(HttpError::BadRequest(InvalidBadRequestKind::Method)),
        }
    }
}
