pub mod connection {
  use std::io::{Error, ErrorKind};
  use std::string::String;
  use std::boxed::Box;
  use std::fmt;

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
        _ => println!("next value"),
      }
      counter += 1;
    }
    Ok(Request::new(m, file))
  }
  fn get_resource() {}
  fn send_data() {}
}
