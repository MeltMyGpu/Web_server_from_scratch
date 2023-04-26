use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, BufRead, Write},
};
mod http_request;

pub fn start_server() -> () {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
    
}

fn handle_connection(mut stream: TcpStream) -> () {
    let buffer = BufReader::new(&mut stream);

    // Deconstruct recieved data
    let http_request:Vec<_> = buffer.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request was {:#?}", http_request);

    successful_connection_responce(stream);
    
    todo!()
}

fn successful_connection_responce(mut stream : TcpStream) {
    let response = "HTTP/1.1 200 OK/r/n/r/n";
    stream.write_all(response.as_bytes()).unwrap();
}







#[cfg(test)]
mod tests {

    #[test]
    fn successful_connection_response() {
        
    }
}
