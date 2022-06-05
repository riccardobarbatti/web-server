//TCP Listening
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

//custom trait handler response
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
//SERVER -----------------------/
//struct
    pub struct Server {
        addr: String,
    }

    //array function use reference point (any value)
    // fn arr(a: &[u8]) {}

    //implement server
    impl Server{
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        //method run (ownership) - pass handler
        pub fn run(self, mut handler: impl Handler) {
            println!("Listening on {}", self.addr);

            // use unwrap like exception
            let listener = TcpListener::bind(&self.addr).unwrap();

            //use loop like while
            loop {
                //use match bind results
              match listener.accept() {
                  //ok want a tuple argument or _
                  Ok((mut stream, _)) => {
                      //define array sample
                      // let a = [1, 2, 3, 3, 3, 4];
                      // array(&a[1..3]);
                      let mut buffer = [0; 1024];
                      match stream.read(&mut buffer){
                          //Pass reference &buffer
                          Ok(_) => {
                              //{:?} println also traits
                              println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                              let response = match Request::try_from(&buffer[..]) {
                                  Ok(request) => handler.handle_request(&request),
                                  Err(e) => handler.handle_bad_request(&e),
                              };
                              //OLD Before handler custom
                              // let response = match Request::try_from(&buffer[..]){
                              //     Ok(request) => {
                              //         Response::new(
                              //             StatusCode::Ok,
                              //             Some("<h1> IT WORKS!</h1>".to_string()),
                              //         );
                              //         response.send(&mut stream);
                              //
                              //     }
                              //     Err(e) => {
                              //         println!("Failed to parse request: {}", e);
                              //         Response::new(StatusCode::BadRequest, None).send(&mut stream);
                              //     }
                              // };
                              //----
                              //check response
                              if let Err(e) = response.send(&mut stream) {
                                  println!("Failed to send response: {}", e)
                              }
                              //try to convert
                             // let res:&Result<Request, _> = &buffer[..].try_into();
                          }
                          Err(e) => println!("Failed to read from connection: {}", e)
                      }
                      // let a = 5;
                      // println!("OK");
                  },
                  Err(e) => println!("Failed to establish connection: {}", e),
              }

                //Ok - tuple result
                //let (stream, addr) = res.unwrap();
            }
            //tuples
            //let tup = (5, "a", listener);
        }
    }
