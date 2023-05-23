

use std::{
    error::Error,
    fmt, 
    net::TcpStream,
};

/// Htpp request type enum 
#[derive(Debug)]
pub enum HttpRequestType {
    GET,
    POST,
}

#[derive(Debug,PartialEq)]
pub enum HttpRequestError {
    RequestWrapError,
}
impl Error for HttpRequestError {}
impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There has been an error when wrapping the Http request")
    }
}

/// Http request content wrapper
/// Stores both the key request information and the stream it was from.
#[derive(Debug)]
pub struct HttpRequest {
    pub request_type:HttpRequestType,
    pub request_uri: String,
    pub request_body: String,
    pub request_stream: TcpStream,
}
impl HttpRequest {
    // basic ctor
    pub fn new(request: &String, stream: TcpStream) -> Result<Self,HttpRequestError> {
        let parts = request.split(" ").collect::<Vec<&str>>();
        let request_type = match parts[0] {
            "GET" => HttpRequestType::GET,
            "POST" => HttpRequestType::POST,
            _ => return Err(HttpRequestError::RequestWrapError)
        };
        Ok(
            HttpRequest { 
                request_type,
                request_uri: String::from(parts[1]),
                request_body: String::new(),
                request_stream: stream,
            })
    }
}


// TESTS //
#[cfg(test)]
mod tests {
    use std::net::{TcpStream,TcpListener};
    use crate::http_request::*;

    #[test]
    fn provide_incorrect_status_code_returns_error() {
        let _ = TcpListener::bind("127.0.0.1:7878").unwrap();
        let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let fail_code: String = "crap".to_string();

        match HttpRequest::new(&fail_code, stream) {
            Ok(_) => assert!(false),
            Err(e) => assert!(e == HttpRequestError::RequestWrapError),
        };
    }
}