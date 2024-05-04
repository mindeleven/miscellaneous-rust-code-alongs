/// list of status codes @ https://www.restapitutorial.com/httpstatuscodes.html
/// the ones we want to implemet are:
/// 200 OK
/// 400 Bad Request
/// 404 Not Found

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK = 200, 
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "Ok", 
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

// functionality for casting the status code to an integer
// to make this easier to use we implement the display trait on the status code
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // casting the enum
        // we can't cast the reference so we've to dereference self
        // `status_code::StatusCode` needs to implement `Clone` for this
        write!(f, "{}", *self as u16)
    }
}