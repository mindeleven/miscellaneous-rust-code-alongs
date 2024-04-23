#![allow(dead_code, unused_variables)]
/// code along with the Udemy course Learn Rust by Building Real Applications
/// by Lyubomir Gavadinov, https://www.udemy.com/course/rust-fundamentals/
/// Section 4: Building a HTTP server from scratch
/// https://www.udemy.com/course/rust-fundamentals/learn/lecture/20695638#overview
/// 
/// Server will have three main components (architecture):
/// (1) TCP Listener
/// (2) HTTP Parser
/// (3) Request Handler

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Server {
                addr
            }
        }

        pub fn run(&self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    pub mod request {
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
    }

    pub mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}

use server::Server;
use http::request::Request;

fn main() {    
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
