use crate::{addr::Addr, handler::Handler, HttpErrorHandler, NotFoundHandler, SimpleHttpServer};

pub struct SimpleHttpServerBuilder<'a>(pub(crate) SimpleHttpServer<'a>);

impl<'a> SimpleHttpServerBuilder<'a> {
    pub fn build(self) -> SimpleHttpServer<'a> {
        self.0
    }

    pub fn handle_request(mut self, path: &'a str, handler: Handler) -> Self {
        self.0.handlers_on_request.insert(path, handler);
        self
    }

    pub fn handle_http_error(mut self, handler: HttpErrorHandler) -> Self {
        self.0.handler_on_http_error = handler;
        self
    }

    pub fn handle_startup<F: Fn(&Addr) + 'static>(mut self, handler: F) -> Self {
        self.0.handler_on_startup = Box::new(handler);
        self
    }

    pub fn handle_not_found(mut self, handler: NotFoundHandler) -> Self {
        self.0.handler_on_not_found = handler;
        self
    }
}
