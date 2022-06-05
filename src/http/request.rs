//use crate::http::{ParseError, Response, StatusCode};
use super::method::{MethodError, Method};
//hashmap
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

//use std::string::ParseError;

//REQUEST - query_string: Option<u8> ecc.. <'buf> use generic - lifetime - pointer to buffer
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

//convert byte array
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
    //fn from_byte_array(bytes: &[u8]) -> Result<Self, String> {}
}

//trait - like interface - request
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
// type Error = String;
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    //reference 'buf
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        //-----
        //error convert in one line from_utf8
        // match str::from_utf8(buf){
        //     Ok(request) => {}
        //     Err(e) => return Err(ParserError::IvalidEncoding),
        // }
        // //use OR method
        // match str::from_utf8(buf).or(Err(ParserError::IvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }
        //--------------------------
        //special Syntax ?
        let request = str::from_utf8(buf)?;
        //call get next word
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        //convert method in string - parse (method.rs) -convert enum
        //? because could return a error
        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            //path: path.to_string(),
            path,
            query_string,
            method,
        })
        // let string = String::from("asd");
        //use trait encrypt
        //string.encrypt();
        //buf.encrypt();
        //macro
       // unimplemented!()
    }
}
//'a 'b is lifetime
//fn get_next_word<'a,'b>(request: &'a str, b: &'b str) -> Option<(&'a str, &'b str)> {
fn get_next_word(request: &str) -> Option<(&str, &str)> {
   // let mut iter = request.chars();
   //  loop {
   //      let item = iter.next();
   //      match item {
   //          Some(c) => {}
   //          None => break,
   //      }
   //  }
    //enumerate split in 2 strings return
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
    //unimplemented!()
}

//Custom Error -
pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
//Display
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        //write macro
        write!(f, "{}", self.message())
    }
}
//Debug
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        //write macro
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError { }


// //encriptions trait
// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }
// //for string
// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
// //for buffer type
// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> self {
//         unimplemented!()
//     }
// }