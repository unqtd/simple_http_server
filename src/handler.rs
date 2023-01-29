use std::collections::HashMap;

use crate::{request::Request, response_builder::ResponseBuilder};

pub type Handler = fn(Request) -> ResponseBuilder;
pub type Handlers<'a> = HashMap<&'a str, Handler>;
