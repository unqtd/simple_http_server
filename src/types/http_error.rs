use std::{fmt::Display, io};

#[derive(Debug)]
pub enum HttpError {
    Io(io::Error),
    NotFoundHost,
    InvalidSyntaxRequest(&'static str),
    InvalidMethod(String),
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io) => write!(f, "{io}"),
            Self::NotFoundHost => write!(f, "Не найден заголовок Host!"),
            Self::InvalidSyntaxRequest(res) => {
                write!(f, "Некорректный синтаксис запроса: {res}")
            }
            Self::InvalidMethod(verb) => write!(f, "Неизвестный метод \"{verb}\"!"),
        }
    }
}
