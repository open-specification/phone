use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

mod response;
mod request;

mod payment;

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

fn get_valid_thru(request_data:request::Request) -> response::Response {

    // Get the Parts of the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 4 { return bad_format(); }

    // Get the Month and Year String
    let month_string:&str = request_parts[2];
    let year_string:&str = request_parts[3];

    // Make Sure the Month and Year are Correct Length
    if month_string.len() == 0 { return bad_format(); }
    if year_string.len() == 0 { return bad_format(); }

    // Convert Strings to U32
    let month:u32 = month_string.parse().unwrap();
    let year:u32 = year_string.parse().unwrap();

    // Check if its Valid
    if payment::get_valid_thru(month, year) {

        // If Valid, Respond "Valid"
        return response::Response {
            response_code: 200,
            body: ("Valid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Valid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    } else {

        // If Invalid, Respond "Invalid"
        return response::Response {
            response_code: 200,
            body: ("Invalid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Invalid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    }

}

fn get_luhn(request_data:request::Request) -> response::Response {

    // Get the Credit Card Number from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 3 { return bad_format(); }
    let credit_number:&str = request_parts[2];
    if credit_number.len() == 0 { return bad_format(); }

    // Return Valid or Invalid
    if payment::luhn_method(credit_number) {

        return response::Response {
            response_code: 200,
            body: ("Valid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Valid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    } else {
        
        return response::Response {
            response_code: 400,
            body: ("Invalid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Invalid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    }

}

fn get_issuer(request_data:request::Request) -> response::Response {

    // Get the Credit Card Number, and Run it Through the Issuer Function
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 3 { return bad_format(); }
    let credit_number = request_parts[2];
    let issuer_name = payment::get_issuer(credit_number);

    // Check if Issuer Exists
    if issuer_name == "" { 

        // If Not, Return a Response Saying "Invalid."
        return response::Response {
            response_code: 200,
            body: ("Invalid.".to_string()),
            headers: HashMap::from([("Content-Length".to_string(), "Invalid.".len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
        };

    }

    // Return the Issuer
    return response::Response {
        response_code: 200,
        body: (issuer_name.to_string()),
        headers: HashMap::from([("Content-Length".to_string(), issuer_name.len().to_string()), ("Content-Type".to_string(), "text/html".to_string())])
    };

}

fn get_info(request_data:request::Request) -> response::Response {

    // Get the Credit Card Number from the Path
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    if request_parts.len() < 3 { return bad_format(); }
    let credit_number:&str = request_parts[2];
    if credit_number.len() == 0 { return bad_format(); }

    // Get Info about Card Number
    let industry = payment::get_card_industry(credit_number);
    let network = payment::get_card_network(credit_number);
    let issuer = payment::get_issuer(credit_number);
    let valid = payment::luhn_method(credit_number);

    // Check if its Valid
    if industry == "" { return bad_format(); }
    if network == "" { return bad_format(); }
    if issuer == "" { return bad_format(); }

    // Create the Response Body
    let body = format!("{{\"industry\":\"{}\", \"network\": \"{}\", \"issuer\": \"{}\", \"valid\": {}}}", industry, network, issuer, valid);

    // Return the Response
    return response::Response {
        response_code: 200,
        body: (body.to_string()),
        headers: HashMap::from([("Content-Length".to_string(), body.len().to_string()), ("Content-Type".to_string(), "application/json".to_string())])
    };

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request_data = request::parse_request(buffer);
    let request_parts: Vec<&str> = request_data.path.split('/').collect();
    let request_intro = request_parts[1];

    let response_data:response::Response = match request_intro {

        "issuer" => get_issuer(request_data),
        "luhn" => get_luhn(request_data),
        "date" => get_valid_thru(request_data),
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