// creating a TCP socket and bind it to the address we want to use
// using the std::net module
// https://doc.rust-lang.org/std/net/index.html

// TcpListener -> a TCP socket server, listening for connections
// Struct std::net::TcpListener
use std::{
    io::{
        Read,
        Write
    }, 
    net::{
        TcpListener, 
        TcpStream
    }
};
use crate::http::{
    Request,
    Response,
    StatusCode
};
use std::convert::TryFrom;

// TcpStream -> a TCP stream between a local and a remote socket
// Struct std::net::TcpStream

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
        println!("Listening on {:?}", self.addr);

        // bind wraps TcpListener into a result
        // bind returns a std::io::Result<()>
        // turning recoverable in unrecoverable error:
        // we catch the listener with unwrap() or let the program panic in case of error
        // (like port already in use, -> netcat -l -p 8080)
        // % nc -nlvp 8080
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // checking (listening) for new connections for every iteration of the loop
            // listener.accept() blocks the code until a new connection arrives
            // returns io::Result<(TcpStream, SocketAddr)>
            match listener.accept() {
                // sending data for testing with
                // echo "TEST" | netcat 127.0.0.1 8080
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // buffer needs to be converted from a refernce to a byte slice
                            // one way to do it -> &buffer as &[u8]
                            // shorter way: 
                            // [..] creates a byte slice the omits bounds and contains entire array
                            // returns result so we've to match
                            match Request::try_from(&buffer[..]) {
                                // Ok() wraps request
                                Ok(request) => {
                                    dbg!(request);
                                    // creating a 404 response
                                    /* 
                                    let response = Response::new(
                                        StatusCode::NotFound,
                                        None
                                    );
                                    */
                                    // creating a 200 response
                                    let response: Response = Response::new(
                                        StatusCode::OK,
                                        Some("<h1>It's alive!!!!!!</h1>".to_string())
                                    );
                                    // using the write macro to send data to the request
                                    // we can pass the response directly 
                                    // because it implements Display
                                    // write!(stream, "{}", response);
                                    
                                    // we're writing to the stream directly now with send()
                                    response.send(&mut stream);
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(
                                        StatusCode::BadRequest,
                                        None
                                    ).send(&mut stream);
                                },
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {:?}", e),
                    }
                    
                },
                Err(e) => println!("Failed to establish a connection: {:?}", e),
            }            
        }

        

    }
}
