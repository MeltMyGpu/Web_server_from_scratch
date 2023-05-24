use std::{ 
    collections::HashMap
};
use crate::http_request::{
    HttpRequestType, HttpRequest
};

/// Enum for registering an API service repsonse format
#[derive(Debug,PartialEq,Clone,Copy)]
enum ResonseFormat {
    Http,
    ApplicationJson,
}

/// Custom error container for controller. 
#[derive(Debug,PartialEq,Clone)]
pub enum ControllerError {
    RegisteredInncorrectType(String),
}

/// Interface for response types
pub trait ResponseTypes {
    /// All responses must be converted to byte arrays before sending
    fn response_as_bytes(&self) -> &[u8];
}


/// Api Endpoint Controller.
/// Searches registered endpoints for matches upon receiving a HttpRequest and calls the related fucntion, or returns a 404 response. 
pub struct Controller {
    get_handlers : HashMap<String, Service>,
    post_handlers : HashMap<String, Service>,
    put_handlers : HashMap<String, Service>,
    delete_handlers : HashMap<String, Service>,
}
impl Controller {
    // inits controller HashMaps, may not be needed
    pub fn new() -> Self {
        Controller { 
            get_handlers : HashMap::new(), 
            post_handlers: HashMap::new(), 
            put_handlers: HashMap::new(), 
            delete_handlers: HashMap::new() }
    }

    /// Used to register endpoints to the Controller
    /// Can be called mulitple times
    pub fn register_endpoints(mut self, handlers : Vec<Endpoint>) {
        handlers.into_iter()
            .for_each(|service| 
                match service.handler.http_method {
                    HttpRequestType::GET => {self.get_handlers.insert(service.route, service.handler);
                    },
                    HttpRequestType::POST => {
                        self.get_handlers.insert(service.route, service.handler);
                    },
                }
            );
    }
}

/// Structured container for an API Service
#[derive(Clone)]
pub struct Service {
    http_method : HttpRequestType,
    response_format: ResonseFormat,
    action : fn(HttpRequest) -> Box<dyn ResponseTypes>,
}


/// Wrapper for registering API endpoints with Controller.
/// Implements .into() for tuples of type (String, Service)
/// 
/// # Examples
/// ```
/// fn test () {
///     let endpoint: Endpoint = (String::new(), Service {
///         http_method: HttpRequestType::GET,
///         response_format: ResonseFormat::Http,
///         action: action_example,
///     }).into() ;
/// }
/// fn action_example(req: HttpRequest) -> Box<dyn ResponseTypes> {
///     Box::new(TestReturn{ test_value :String::new()})
/// }
/// ```
pub struct Endpoint {
    route : String,
    handler: Service,
}
impl Endpoint {
    pub fn new(route:String, handler:Service) -> Self {
        Endpoint { route, handler }
    }
}
impl From<(String, Service)> for Endpoint{
    fn from(other: (String, Service)) -> Endpoint {
        Endpoint::new( other.0, other.1 )
        
    }
}



struct TestReturn{
    test_value : String
}
impl ResponseTypes for TestReturn {
    fn response_as_bytes(&self) -> &[u8] {
        return self.test_value.as_bytes()
    }
}

// fn test () {
//     let endpoint: Endpoint = (String::new(), Service {
//         http_method: HttpRequestType::GET,
//         response_format: ResonseFormat::Http,
//         action: action_example,
//     }).into() ;
// }

// fn action_example(req: HttpRequest) -> Box<dyn ResponseTypes> {
//     Box::new(TestReturn{ test_value :String::new()})
// }
// fn gen_fun<T: Default>() -> T {
//     T::default()
// }

// Need to work out how exactly im going to go about implementing this...
// Need some way for users to register services of unknow args and return type