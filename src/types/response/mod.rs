use self::builder::ResponseBuilder;
use super::body::Body;

pub mod builder;

#[derive(Debug)]
pub struct Response {
    pub(crate) code: Code,
    pub(crate) headers: String,
    pub(crate) body: Option<Body>,
}

impl Response {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(code: Code) -> ResponseBuilder {
        ResponseBuilder(Self {
            code,
            headers: "Server: SimpleHttpServer\r\n".to_string(),
            body: None,
        })
    }
}

#[derive(Debug)]
pub enum Code {
    Ok,
    BadRequest,
    NotFound,
}

impl Code {
    pub fn as_str(&self) -> &'static str {
        match self {
            Code::Ok => "200 OK",
            Code::BadRequest => "400 Bad Request",
            Code::NotFound => "404 Not Found",
        }
    }
}
