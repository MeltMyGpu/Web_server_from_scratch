
use std::{
    error::Error,
    fmt,
};

#[derive(Debug,)]
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
pub struct HttpRequest {
    pub request_type:HttpRequestType,
    pub request_uri: String,
    pub request_body: String,
}
impl HttpRequest {
    // basic ctor
    pub fn new(request: &String) -> Result<Self,HttpRequestError> {
        let parts = request.split(" ");
        let parts = parts.collect::<Vec<&str>>();
        let request_type = match parts[0] {
            "GET" => HttpRequestType::GET,
            "POST" => HttpRequestType::POST,
            _ => return Err(HttpRequestError::RequestWrapError)
        };
        Ok(
            HttpRequest { 
                request_type,
                request_uri: String::from(parts[1]),
                request_body: String::new() 
            })
    }
}

/// Htpp request type enum 
#[derive(Debug)]
pub enum HttpRequestType {
    GET,
    POST,
}