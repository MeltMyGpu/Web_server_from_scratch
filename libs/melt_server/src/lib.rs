#![feature(format_args_capture)]
use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, BufRead, Write}, 
    fs,
};
mod http_response_builder;
mod http_request;
use http_request::{
    HttpRequest,
    HttpRequestType
};



/// Starts server and runs port listener
pub fn start_server() -> () {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
    
}

// Receives a connection
fn handle_connection(mut stream: TcpStream) -> () {
    let buffer = BufReader::new(&mut stream);

    // Deconstruct recieved data
    let http_request:Vec<_> = buffer.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request was {:#?}", http_request);

    // Wrap the request information in a single object
    let wrapped = HttpRequest::new(&http_request[0], stream).unwrap();
    successful_connection_responce(wrapped.request_stream);
    println!("Testing: {:?} {:?} {:?}",wrapped.request_type, wrapped.request_uri, wrapped.request_body );
    
    todo!()
}

/// A stand in OK 200 response for testing purposes
fn successful_connection_responce(mut stream : TcpStream) {
    let response_code = "HTTP/1.1 200 OK";
    let response_body = fs::read_to_string("testing_web_pages/index.html").unwrap();
    let response_body_length = response_body.len();
    let response = format!("{response_code}\r\nContent-Length: {response_body_length}\r\n\r\n{response_body}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn match_request_type(req : HttpRequest){
    match req.request_type {
        HttpRequestType::GET => handle_get_request(req),
        HttpRequestType::POST => handle_post_request(req),
    }
}

fn handle_get_request(req: HttpRequest) {
    todo!()
}

fn handle_post_request(req : HttpRequest){
    todo!()
}



