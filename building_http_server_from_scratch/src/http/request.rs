use super::method::Method;
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

