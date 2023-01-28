use std::collections::HashMap;

use crate::{request::Request, response::Response};

pub type Handler = fn(Request) -> Response;
pub type Handlers<'a> = HashMap<&'a str, Handler>;
