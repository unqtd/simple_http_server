use crate::types::{
    http_error::HttpError,
    request::{Method, Uri},
};

pub fn parse_starting_line(line: &str) -> Result<(Method, Uri), HttpError> {
    const ERROR: HttpError =
        HttpError::InvalidSyntaxRequest("Некорректный синтаксис стартовый строки!");

    let (method, line) = line.split_once(' ').ok_or(ERROR)?;
    let (uri, _) = line.split_once(' ').ok_or(ERROR)?;

    Ok((method.try_into()?, parse_uri(uri)))
}

fn parse_uri(input: &str) -> Uri {
    if let Some((path, query_string)) = input.split_once('?') {
        Uri {
            path: path.to_string(),
            query_string: Some(query_string.to_string()),
        }
    } else {
        Uri {
            path: input.to_string(),
            query_string: None,
        }
    }
}

pub fn parse_header(line: &str) -> Result<(String, String), HttpError> {
    let (key, value) = line.split_once(':').ok_or(HttpError::InvalidSyntaxRequest(
        "Некорректный синтаксис заголовка!",
    ))?;

    Ok((key.to_string(), value.trim().to_string()))
}
