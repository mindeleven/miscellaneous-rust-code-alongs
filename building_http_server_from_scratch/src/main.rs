#![allow(dead_code, unused_imports, unused_variables)]
/// code along with the Udemy course Learn Rust by Building Real Applications
/// by Lyubomir Gavadinov, https://www.udemy.com/course/rust-fundamentals/
/// Section 4: Building a HTTP server from scratch
/// https://www.udemy.com/course/rust-fundamentals/learn/lecture/20695638#overview
/// 
/// Server will have three main components (architecture):
/// (1) TCP Listener
/// (2) HTTP Parser
/// (3) Request Handler

mod server;
mod http;

use server::Server;
use http::Request;
use http::Method;

fn main() {    
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
