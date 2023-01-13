use crate::{files, Error, MAX_NB_HEADERS};
use httparse;
use std::{fs::File, io::Read};

pub fn handle_http(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut headers = [httparse::EMPTY_HEADER; MAX_NB_HEADERS];
    let mut request = httparse::Request::new(&mut headers);
    if request.parse(input).is_err() {
        return Err(Error::HttpParsing);
    }
    if let Some("GET") = request.method {
    } else {
        return Err(Error::HttpMethod);
    }
    deal_with_path(&request)
}

fn deal_with_path(request: &httparse::Request) -> Result<Vec<u8>, Error> {
    match request.path {
        Some(files::FLAG) => serve_flag(request),
        Some("/") => serve_file(crate::ACCUEIL),
        Some(filename) if files::LIST.contains(&filename) => serve_file(filename),
        _ => Err(Error::NoSuchFile),
    }
}

fn serve_file(filename: &str) -> Result<Vec<u8>, Error> {
    let file = File::open(files::get(filename).expect("Alredy checked that file existed"));
    if file.is_err() {
        return Err(Error::FileError(filename.to_string()));
    }
    let mut content = Vec::new();
    let result = file
        .expect("File should be successfully open")
        .read_to_end(&mut content);
    if result.is_err() {
        return Err(Error::FileError(filename.to_owned()));
    }
    Ok(content)
}

fn serve_flag(request: &httparse::Request) -> Result<Vec<u8>, Error> {
    if !is_admin(request.headers) {
        return Err(Error::Forbidden);
    }
    serve_file(files::FLAG)
}

fn is_admin(headers: &[httparse::Header]) -> bool {
    for h in headers {
        if h.name == crate::HEADER_NAME && h.value == crate::ADMIN_COOKIE.as_bytes() {
            return true;
        }
    }
    false
}

pub fn build_reply(
    buffer: std::result::Result<Vec<u8>, Error>,
    hex_iv: &str,
    hex_request: &str,
) -> std::result::Result<Vec<u8>, http::Error> {
    let http_response = _build_http_reply(buffer, hex_iv, hex_request)?;
    let (parts, mut body) = http_response.into_parts();
    let mut headers = String::new();
    for h in parts.headers {
        if let Some(header) = h.0 {
            let value = h.1.to_str();
            if value.is_err() {
                continue;
            }
            headers = format!(
                "{}{}: {}\r\n",
                headers,
                header,
                value.expect("already checked for error")
            );
        }
    }
    let mut bytes_head = format!("{:?} {:?}\r\n{}\r\n", parts.version, parts.status, headers)
        .as_bytes()
        .to_vec();
    bytes_head.append(&mut body);
    Ok(bytes_head)
}

fn _build_http_reply(
    buffer: std::result::Result<Vec<u8>, Error>,
    hex_iv: &str,
    hex_request: &str,
) -> std::result::Result<http::Response<Vec<u8>>, http::Error> {
    match buffer {
        Err(Error::Forbidden) => {
            let body =
                b"An admin cookie is required in order to connect to this page!".to_vec();
            http::Response::builder()
                .status(http::StatusCode::FORBIDDEN)
                .header("Content-Length", body.len())
                .header(hex_iv, hex_request)
                .body(body)
        }

        Err(Error::HttpMethod) => {
            let body = b"Only GET requests are supported!".to_vec();
            http::Response::builder()
                .status(http::StatusCode::METHOD_NOT_ALLOWED)
                .header("Content-Length", body.len())
                .header(hex_iv, hex_request)
                .body(body)
        }

        Err(Error::NoSuchFile) => {
            let body = b"This file does not exist!".to_vec();
            http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .header("Content-Length", body.len())
                .header(hex_iv, hex_request)
                .body(body)
        }

        Err(Error::HttpParsing) => {
            let body = b"This is not valid HTTP!".to_vec();
            http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .header("Content-Length", body.len())
                .header(hex_iv, hex_request)
                .body(body)
        }

        Err(_) => {
            let body =
                b"Unexpected error. Not part of the challenge.".to_vec();
            http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Length", body.len())
                .header(hex_iv, hex_request)
                .body(body)
        }
        Ok(reply) => http::Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Length", reply.len())
            .header(hex_iv, hex_request)
            .body(reply),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_http_parsing() {
        let req = b"GET /lol HTTP/1.1\r\ncookie: lel\r\n\r\n";
        let mut headers = [httparse::EMPTY_HEADER; 10];
        let mut parser = httparse::Request::new(&mut headers);
        parser.parse(req).unwrap();
        // panic!("{:?}", parser);
    }
    #[test]
    #[should_panic]
    fn test_http_parsing_failure() {
        let req = b"G\x04ET /lol HTTP/1.1\r\ncookie: lel\r\n\r\n";
        let mut headers = [httparse::EMPTY_HEADER; 10];
        let mut parser = httparse::Request::new(&mut headers);
        parser.parse(req).unwrap();
    }
}
