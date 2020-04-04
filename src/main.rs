use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

pub mod httpconnection;

fn main() {
    // binding to addr localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming() returns connections between the client and server
    // a single stream is an open connection between client and server
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let request: httpconnection::connection::Request = 
        match httpconnection::connection::parse_request(& buffer) {
        Ok(temp) => temp,
        Err(_) => panic!("unable to parse"),
    };

    println!("request: {:?}", (*request.resource));

    if request.method == httpconnection::connection::Method::GET {
      println!("method = GET\nresource = {}", *request.resource)
    }
    else {
     println!("method = ???\nresource = {}", *request.resource)
    }

    httpconnection::connection::send_data(&request, stream);
}
