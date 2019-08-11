pub mod connection {
  use std::io::{Error, ErrorKind};

  pub enum Method {
      GET, POST,
  }

  pub struct Request {
    method: Method,
  }

  impl Request{
    pub fn new(m: Method) -> Request {
      Request{method: m}
    }
  }

  pub fn parse_request(buffer: & [u8]) -> Result<Request, Error> {
    //let request = Request{};
    let mut counter = 0;
    let bad_method = Error::new(ErrorKind::InvalidData, "invalid method type");

    println!("serve_file: {} ", String::from_utf8_lossy(&buffer));
    for itr in String::from_utf8_lossy(&buffer[..]).split_whitespace() {
      println!("{}, ", itr);
      if counter == 0 {
        if itr == "GET" {
          println!("CORRECT!, you entered: {}", itr);
          let m = Method::GET;
          return Ok(Request::new(m))
        }
        else {
          println!("Incorrect!, you entered: {}", itr);
          return Err(bad_method)
        }
      }
      counter += 1;
    }
    Err(bad_method)
  }
  fn get_resource() {}
  fn send_data() {}
}
