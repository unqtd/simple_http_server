use crate::body::Body;

#[derive(Debug)]
pub struct Response {
    pub(crate) code: Code,
    pub(crate) headers: String,
    pub(crate) body: Option<Body>,
}

impl Response {
    pub fn new(code: Code) -> Self {
        Self {
            code,
            headers: "Server: SimpleHttpServer\r\n".to_string(),
            body: None,
        }
    }

    pub fn body(mut self, content: Vec<u8>) -> Self {
        let length = content.len();
        self.body = Some(content);

        self.header("Content-Length", length.to_string().as_str())
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push_str(key);
        self.headers.push(':');
        self.headers.push_str(value);
        self.headers.push_str("\r\n");

        self
    }

    pub fn ok() -> Self {
        Self::new(Code::Ok)
    }

    pub fn bad_request() -> Self {
        Self::new(Code::BadRequest)
    }

    pub fn not_found() -> Self {
        Self::new(Code::NotFound)
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
