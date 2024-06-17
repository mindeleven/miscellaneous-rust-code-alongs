/// https://www.youtube.com/watch?v=hzSsOV2F7-s
/// Building an HTTP Server in Rust: Exploring TCP/IP, Socket Programming, and Asynchronous I/O 
/// by codemoon 6:18
/// https://github.com/codemoonsxyz/toy-http-rs

use std::{
    io::{Read, Write}, net::{SocketAddr, TcpStream}, thread
};

mod request;
mod response;
mod http;
mod server;
mod middleware;

use std::collections::HashMap;

use http::HttpStatusCode;
use http::HttpMethod;
use middleware::logger::LoggerMiddleware;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;


use request::*;

use response::*;

use server::*;


/* fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        // declaring a mutable buffer
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            },
            Err(err) => {
                panic!("{} caused panic in the streets of London", err);
            }
        }

    }
} */

fn hello_handler(_req: Request) -> FutureResponse<'static> {
    let html = "<html><body><h1>Hello, world!</h1></body></html>";
    let response = Response {
        version: "HTTP/1.1".to_string(),
        status_code: 200,
        status_text: "OK".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            headers
        },
        body: Some(html.to_string()),
    };
    Box::pin(async move { Ok(response) })
}

// main() that's consuming the finished library
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    
    // instantiate server and setting up hello world route
    let server = ServerBuilder::new()
        .bind(addr)
        .route("/",HttpMethod::GET, hello_handler).accept(LoggerMiddleware)
        .build()?
        .run()
        .await?;

    println!("Hello, world!");
    Ok(())
}

/* Initial version of main, before library got finished
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // iterrating over all incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // create thread to handle each incoming connection
                // creating multiple threads so we can have multiple connections at the same time
                thread::spawn(move || {
                    handle_client(stream);
                });

            },
            Err(_) => {}
        }

    }

}
*/