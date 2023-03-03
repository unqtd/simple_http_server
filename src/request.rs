use self::method::Method;
use crate::common::{headers::Headers, Body};

#[derive(Debug)]
pub struct Request {
    pub uri: Uri,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<Body>,
}

#[derive(Debug)]
pub struct Uri {
    pub path: String,
    pub query_string: Option<String>,
}

pub mod method {
    #[derive(Debug)]
    pub enum Method {
        Get,
        Post,
    }

    #[derive(Debug)]
    pub struct UnknowMethod(pub String);

    impl<'a> TryFrom<&'a str> for Method {
        type Error = UnknowMethod;

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            match value {
                "GET" => Ok(Self::Get),
                "POST" => Ok(Self::Post),
                _ => Err(UnknowMethod(value.to_string())),
            }
        }
    }
}
