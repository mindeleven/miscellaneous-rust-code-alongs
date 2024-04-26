use super::method::Method;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult
    },
    str::{
        self,
        Utf8Error
    }
};

// request we want to code
/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// TryFrom is generic over type T
// which is the type we're converting from (the byte array)
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // GET /search?name=abcd&sort=1 HTTP/1.1\r\n
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
        // converting the byteslice into a stringslice
        match str::from_utf8(buf) {
            Ok(request) => { },
            Err(_) => {
                return Err(ParseError::InvalidEncoding)
            },
        }
        // alternative syntax:
        // or() will return error or the string slice from the function call
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => { },
            Err(e) => return Err(e)
        }
        // alternative syntax with ? operator:
        // ? will try to convert the error type it receives if it does not 
        // match the error type the function returns
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        */
        // even shorter alternative syntax
        // `?` can convert error to `ParseError` if the `From` trait is implemented
        let request = str::from_utf8(buf)?;

        unimplemented!()
        
    }
}

// enum to represent different kinds of parsing errors
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding, // not valid UTF-8
    InvalidProtocol, // not HTTP 1.1
    InvalidMethod, // method not in Method enum
}

// implementing From trait to parse Utf8Error
// every time we receive an Utf8Error we return an InvalidEncoding error
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // formatting the message method from the ParseError implementation
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // formatting the message method from the ParseError implementation
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        // matching on all variants
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// to make error more ideomatic we use Error Trait from std
// ParseError has to implement Display and Debug Trait
impl Error for ParseError {}