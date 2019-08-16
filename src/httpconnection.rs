pub mod connection {
  use std::io::{Error, ErrorKind};
  use std::net::TcpStream;
  use std::net::TcpListener;
  use std::string::String;
  use std::boxed::Box;
  use std::fmt;
  use std::fs;
  use std::io::Write;
  use std::process::Command;

  #[derive(PartialEq)]
  pub enum Method {
    GET, POST, INVALID
  }

  pub struct Request {
    pub method: Method,
    pub resource: Box<String>,
  }

  impl Request{
    fn new(m: Method, r: Box<String>) -> Request {
      Request{
        method: m,
        resource: r,
      }
    }
  }

  pub fn parse_request (buffer: & [u8]) -> Result<Request, Error> {
    //let request = Request{};
    let mut counter = 0;
    let bad_method = Error::new(ErrorKind::InvalidData, "invalid method type");
    let bad_path = Error::new(ErrorKind::InvalidData, "you cannot go above current directory");
    let mut m = Method::INVALID;
    let mut file = Box::new(String::new());

    println!("serve_file: {} ", String::from_utf8_lossy(&buffer));
    for itr in String::from_utf8_lossy(&buffer[..]).split_whitespace() {
      println!("{}, ", itr);
      println!("counter= {}", counter);
      match counter {
        //method type
        0 => {
               if itr == "GET" {
                 println!("CORRECT! you entered: {}", itr);
                 m = Method::GET;
               }
               else if itr == "POST" {
                 println!("CORRECT! you entered: {}", itr);
                 m = Method::POST;
               }
               else {
                 println!("Incorrect!, you entered: {}", itr);
                 return Err(bad_method)
               }
        },
        1 => {
               println!("file wanted: {:?}", itr);
               if itr.contains("..") {
                   return Err(bad_path)
               }
               else if itr != "/" {
                  (*file).push_str("htdocs");
                  (*file).push_str(itr);
               }
               else {
                 (*file).push_str("htdocs/index.html");
               }
               println!("file to grab = {}", (*file));
        },
        //_ => println!("next value"),
        _ => break,
      }
      counter += 1;
    }
    Ok(Request::new(m, file))
  }
  pub fn send_data(r: &Request, mut stream: TcpStream) {
    let contents: String;
    let status_line;
    let mut filename = String::new();

    //GET Request
    if r.method == Method::GET || r.method == Method::POST {
        status_line = "HTTP/1.1 200 OK\r\n\r\n"; 
        filename.push_str(&*r.resource);
    }
    //all other requests
    else {
      status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
      filename.push_str("404.html");
    }

    //run file through php interpreter
    if filename.ends_with(".html") || filename.ends_with(".php") {
      let output = Command::new("php")
                          .arg(filename)
                          .output()
                          .expect("failed to execute process");

      contents = String::from_utf8_lossy(&output.stdout).to_string();
    }
    else {
      //stringify file and send over network
      contents = match fs::read_to_string(filename) {
          Ok(fstr) => fstr,
          Err(err) => fs::read_to_string("404.html").unwrap(),
      };
    }

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }
}
