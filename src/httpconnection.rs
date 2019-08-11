pub mod connection {
  use std::io::{Error, ErrorKind};
  use std::net::TcpStream;
  use std::net::TcpListener;
  use std::string::String;
  use std::boxed::Box;
  use std::fmt;
  use std::fs;
  use std::io::Write;

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

  /*
  impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      if self.method == Method::GET {
        write!("method = GET\nresource = {}", *self.resource)
      }
      write!("method = ???\nresource = {}", *self.resource)
    }
  }
  */

  pub fn parse_request (buffer: & [u8]) -> Result<Request, Error> {
    //let request = Request{};
    let mut counter = 0;
    let bad_method = Error::new(ErrorKind::InvalidData, "invalid method type");
    let mut m = Method::INVALID;
    let mut file = Box::new(String::from("index.html"));

    println!("serve_file: {} ", String::from_utf8_lossy(&buffer));
    for itr in String::from_utf8_lossy(&buffer[..]).split_whitespace() {
      println!("{}, ", itr);
      println!("counter= {}", counter);
      match counter {
        //method type
        0 => {
               if itr == "GET" {
                 println!("CORRECT!, you entered: {}", itr);
                 m = Method::GET;
               }
               else {
                 println!("Incorrect!, you entered: {}", itr);
                 return Err(bad_method)
               }
        },
        1 => {
               println!("file wanted: {:?}", itr);
               if itr != "/" {
                  (*file).clear();
                  (*file).push_str(itr);
               }
        },
        //_ => println!("next value"),
        _ => break,
      }
      counter += 1;
    }
    Ok(Request::new(m, file))
  }
  pub fn send_data(r: &Request, mut stream: TcpStream) {
    let status_line;
    let mut filename = String::new();

    //GET Request
    if r.method == Method::GET {
        status_line = "HTTP/1.1 200 OK\r\n\r\n"; 
        filename.push_str(&*r.resource);
    }
    //all other requests
    else {
      status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
      filename.push_str("404.html");
    }

    //stringify file and send over network
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }
}
