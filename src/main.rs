use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

mod response;
mod request;

mod phone;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn bad_format() -> response::Response {

    return response::Response {
        response_code: 400,
        body: ("Bad Format.".to_string()),
        headers: HashMap::from([("Content-Length".to_string(), "Bad Format.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
    };

}

fn get_country(request_data:request::Request) -> response::Response {

    

    return bad_format();

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request_data = request::parse_request(buffer);
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    let request_intro = request_parts[1];

    let response_data:response::Response = match request_intro {

        "country" => get_country(request_data),
        _ => response::Response {
            response_code: 404,
            body: ("Endpoint Not Found".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Endpoint Not Found".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        }

    };

    let response = response_data.response_full();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}