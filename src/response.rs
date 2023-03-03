use crate::common::Body;

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

impl Response {
    #[must_use]
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push_str(&format!("{name}: {value}\r\n"));
        self
    }

    #[must_use]
    pub fn body(mut self, content: Vec<u8>) -> Self {
        self.body = Some(content);
        self
    }
}

impl From<Code> for Response {
    fn from(value: Code) -> Self {
        Self {
            code: value,
            headers: String::new(),
            body: None,
        }
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => "200 OK",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
        }
        .to_string()
    }
}
