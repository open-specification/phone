use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

mod response;
mod request;
#[macro_use] mod serializer;

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

fn okay_response(body:String, mime_type:&str) -> response::Response {

    return response::Response {
        response_code: 200,
        body: body.to_string(),
        headers: HashMap::from([("Content-Length".to_string(), body.len().to_string()), ("Content-Type".to_string(), mime_type.to_string()), ("Content-Disposition".to_string(), "inline".to_string())])
    }

}

fn get_e164_format(request_data:request::Request) -> response::Response {

    // Get the Country Name and Phone Number from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 4 { return bad_format(); }
    let country_name:&str = request_parts[2];
    if country_name.len() == 0 { return bad_format(); }
    let phone_number:&str = request_parts[3];
    if phone_number.len() == 0 { return bad_format(); }
    
    // Get the Formatted Number
    let formatted_number = phone::e164_format(country_name, phone_number);

    // Return the Country Name
    return okay_response(formatted_number, "text/html");

}

fn get_info(request_data:request::Request) -> response::Response {

    // Get the Country Name and Phone Number from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 4 { return bad_format(); }
    let country_name:&str = request_parts[2];
    if country_name.len() == 0 { return bad_format(); }
    let phone_number:&str = request_parts[3];
    if phone_number.len() == 0 { return bad_format(); }

    // Get the Phone Info
    let phone_info = phone::get_general_info(country_name, phone_number);

    // Convert Phone Info to String
    let content_type = "application/yaml";
    let phone_serialized = to_serialized!(content_type, phone_info, (number_type, area_code));

    // Return the Response
    return okay_response(phone_serialized, content_type);

}

fn get_country(request_data:request::Request) -> response::Response {

    // Get the Country Code from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 3 { return bad_format(); }
    let country_code:&str = request_parts[2];
    if country_code.len() == 0 { return bad_format(); }
    
    // Get the Country Code
    let country_name = phone::get_country(country_code);

    // Check if Country Code not Found
    if country_name == "" {

        return response::Response {
            response_code: 404,
            body: ("Country Not Found.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Country Not Found.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    }

    // Return the Country Name
    return okay_response(country_name.to_string(), "text/html");

}

fn get_branched_format(request_data:request::Request) -> response::Response {

    // Get the Country Name and Phone Number from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 4 { return bad_format(); }
    let country_name:&str = request_parts[2];
    let phone_number:&str = request_parts[3];
    if phone_number.len() == 0 { return bad_format(); }

    // Get the Formatted Number
    let formatted_number = phone::branched_format(country_name, phone_number);

    // Return the Country Name
    return okay_response(formatted_number.to_string(), "text/html");

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request_data = request::parse_request(buffer);
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    let request_intro = request_parts[1];

    let response_data:response::Response = match request_intro {

        "country" => get_country(request_data),
        "e164" => get_e164_format(request_data),
        "format" => get_branched_format(request_data),
        "info" => get_info(request_data),
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