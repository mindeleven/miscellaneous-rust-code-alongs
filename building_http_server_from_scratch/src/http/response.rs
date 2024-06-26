/// server response example:
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Ranges
/// it begins with the status line HTTP/1.1 200 OK
/// at the end there are two new lines after which the body of the response follows
/* 
HTTP/1.1 200 OK
Date: Thu, 19 Sep 2019 20:02:31 EST
Server: Apache
Last-Modified: Wed, 21 Oct 2015 07:28:00 GMT
ETag: "33a64df551425fcc55e4d42a148795d9f25f89d4"
Accept-Ranges: bytes
Content-Length: 51
Vary: Accept-Encoding
Content-Type: text/plain

Hello world! This is the content of the response.
*/
/// reduced example we'll use here:
/* 
HTTP/1.1 200 OK

Hello world! This is the content of the response.
*/

use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    }, 
    io::{
        Result as IoResult,
        Write
    },
    net::TcpStream
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
        }
    }

    // moving write logic to the Response intself
    // pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
    
    // making the response more generic by replacing TcpStream with Write
    // dyn = dynamic dispatch -> the concrete function implementation will be called at runtime
    // pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
    
    // alternative approach: static dispatch with is resolved at compile time
    // impl -> function accepts any parameter that implements Write trait
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phrase(),
            body
        )
    }

}

// implementing Display for the response so that we can sent back a message with write!
/* no longer needed because now we're writing to the stream directly with send()
impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(
            f, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phrase(),
            body
        )
    }
}
*/