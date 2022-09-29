use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct HTTPRequest<'request> {
    request_vec: Vec<String>,
    pub stream: &'request TcpStream,
}

impl HTTPRequest<'_> {
    pub fn new(mut stream: &TcpStream) -> HTTPRequest {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        HTTPRequest {
            request_vec: http_request,
            stream,
        }
    }

    pub fn request(&self) -> Option<ParsedHTTPRequest> {
        let result = match self.request_vec.len() > 0 {
            true => { let req = &self.request_vec[0];
            Some(ParsedHTTPRequest::new(&req[..]))
            }
            false => {
                println!("Request was of length 0");
                None
            }
        };
        result
    }
}

pub struct ParsedHTTPRequest {
    pub method: String,
    pub version: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl ParsedHTTPRequest {
    fn new(request: &str) -> ParsedHTTPRequest {
        let request_parts = request.split_whitespace().collect::<Vec<&str>>();
        let (method, path, version) = (request_parts[0].to_string(), request_parts[1].to_string(), request_parts[2].to_string());
        ParsedHTTPRequest {
            method,
            path,
            version,
            // TODO: implement below
            headers: Default::default(),
            body: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_HTTP_POST: &str = "POST / HTTP/1.1\r\nContent-Length: 19\r\nContent-Type: application/x-www-form-urlencoded;boundary=\"\"\r\n\r\nsomething=something\r\n\r\n";

    #[test]
    fn it_works() {
        let expected_request = ParsedHTTPRequest {
            method: "POST".to_string(),
            version: "HTTP/1.1".to_string(),
            path: "/".to_string(),
            headers: Default::default(),
            body: None,
        };
        let parsed_request = ParsedHTTPRequest::new(EXAMPLE_HTTP_POST);
        assert_eq!(parsed_request.method, expected_request.method);
        assert_eq!(parsed_request.version, expected_request.version);
        assert_eq!(parsed_request.path, expected_request.path);
        assert_eq!(parsed_request.headers, expected_request.headers);
        assert_eq!(parsed_request.body, expected_request.body);
    }
}
