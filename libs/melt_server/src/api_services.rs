use std::{env::Args, collections::HashMap};
use crate::http_request::{
    HttpRequestType, HttpRequest
};


#[derive(Debug,PartialEq)]
enum ResonseFormat {
    Http,
    ApplicationJson,
}

#[derive(Debug,PartialEq)]
enum ControllerError {
    RegisteredInncorrectType(String),
}


//////// TESTING AREA
struct Controller {
    get_handlers : HashMap<String, Handler>,
    post_handlers : HashMap<String, Handler>,
    put_handlers : HashMap<String, Handler>,
    delete_handlers : HashMap<String, Handler>,
}
impl Controller {
    fn new() -> Self {
        Controller { 
            get_handlers : HashMap::new(), 
            post_handlers: HashMap::new(), 
            put_handlers: HashMap::new(), 
            delete_handlers: HashMap::new() }
    }
    fn register_get(mut self, handlers : Vec<Handler>) -> Result<(),ControllerError>{
        // handlers.into_iter().for_each(|handler| match handler.http_method {
        //     HttpRequestType::GET => {
        //         self.get_handlers.insert(handler.route, handler);
                
        //     }
        //     HttpRequestType::POST => {}
        // });
        for handler in handlers {
            match handler.http_method {
                    HttpRequestType::GET => {
                        self.get_handlers.insert(handler.route, handler);
                        // FUCK THIS
                    }
                    HttpRequestType::POST => {}
                }
        }
        todo!()
    }
}


struct Handler {
    route : String,
    http_method : HttpRequestType,
    response_format: ResonseFormat,
    action : fn(HttpRequest) -> dyn ResponseTypes,
}
impl Handler {
    fn new(
        route : String,
        htthttp_method : HttpRequestType,
        response_format: ResonseFormat,
        action : fn(HttpRequest) -> dyn ResponseTypes,
    ) -> Self {
        todo!()
    }
}

 



/// Interface for response types
trait ResponseTypes {
    /// All responses must be converted to byte arrays before sending
    fn response_as_bytes(&self) -> &[u8];
}


// fn gen_fun<T: Default>() -> T {
//     T::default()
// }

// Need to work out how exactly im going to go about implementing this...
// Need some way for users to register services of unknow args and return type