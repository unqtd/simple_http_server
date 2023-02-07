use super::{Code, Response};

#[must_use]
pub struct Responder(pub(crate) Response);
impl Responder {
    pub fn new(code: Code) -> Self {
        Self(Response {
            code,
            headers: String::from("Server: SimpleHttpServer\r\n"),
            body: None,
        })
    }

    pub(crate) fn response(self) -> Response {
        self.0
    }

    pub fn body(mut self, content: Vec<u8>) -> Self {
        let length = content.len();
        self.0.body = Some(content);

        self.header("Content-Length", length.to_string().as_str())
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.0.headers.push_str(key);
        self.0.headers.push(':');
        self.0.headers.push_str(value);
        self.0.headers.push_str("\r\n");

        self
    }
}
