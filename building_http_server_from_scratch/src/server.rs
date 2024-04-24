// creating a TCP socket and bind it to the address we want to use
// using the std::net module
// https://doc.rust-lang.org/std/net/index.html

// TcpListener -> a TCP socket server, listening for connections
// Struct std::net::TcpListener
use std::net::TcpListener;

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
            listener.accept();
            println!("Listening on {}", self.addr);
        }

        

    }
}