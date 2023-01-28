use crate::{
    http_connection::{HttpError, IResult, InvalidBadRequestKind},
    request::{Method, Uri},
};

pub fn parse_starting_line(line: &str) -> IResult<(Method, Uri)> {
    const ERROR: HttpError = HttpError::BadRequest(InvalidBadRequestKind::StaringLine);

    let (method, line) = line.split_once(' ').ok_or(ERROR)?;
    let (uri, _) = line.split_once(' ').ok_or(ERROR)?;

    Ok((method.try_into()?, uri.into()))
}

pub fn parse_header(line: &str) -> IResult<(String, String)> {
    let (key, value) = line
        .split_once(':')
        .ok_or(HttpError::BadRequest(InvalidBadRequestKind::HeaderSyntax))?;

    Ok((key.to_string(), value.trim().to_string()))
}
