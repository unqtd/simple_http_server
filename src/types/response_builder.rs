use super::response::Response;

pub struct ResponseBuilder(pub(crate) Response);

impl ResponseBuilder {
    pub fn build(self) -> Response {
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
