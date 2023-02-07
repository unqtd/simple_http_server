use super::body::Body;

pub mod responder;

#[derive(Debug)]
pub struct Response {
    pub(crate) code: Code,
    pub(crate) headers: String,
    pub(crate) body: Option<Body>,
}

#[derive(Debug)]
pub enum Code {
    Ok,
    BadRequest,
    NotFound,
}

impl Code {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
        }
    }
}
