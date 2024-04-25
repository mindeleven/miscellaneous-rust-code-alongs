use super::method::Method;
use std::convert::TryFrom;

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
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
       unimplemented!()
    }
}
