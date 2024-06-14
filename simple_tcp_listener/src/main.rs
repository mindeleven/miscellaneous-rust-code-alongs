/// https://www.youtube.com/watch?v=hzSsOV2F7-s
/// Building an HTTP Server in Rust: Exploring TCP/IP, Socket Programming, and Asynchronous I/O 
/// by codemoon 
/// https://github.com/codemoonsxyz/toy-http-rs

use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write},
    thread
};

mod request;
mod response;
mod http;
mod server;
mod middleware;

fn handle_client(mut stream: TcpStream) {
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
}

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
