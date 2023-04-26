
/// Http request content wrapper
pub struct HttpRequest {
    pub request_type:HttpRequestType,
    pub request_uri: String,
    pub request_body: String,
}
impl HttpRequest {
    // basic ctor
    pub fn new(request: String) -> Self {
        let parts = request.split(" ");
        let parts = parts.collect::<Vec<&str>>();
        HttpRequest { 
            request_type: match parts[0] {
                "GET" => HttpRequestType::GET,
                "POST" => HttpRequestType::POST,
                _ => HttpRequestType::ERR
            },
            request_uri: String::from(parts[1]),
            request_body: String::new() 
        }
    }
}

/// Htpp request type enum 
pub enum HttpRequestType {
    GET ,
    POST,
    ERR,
}