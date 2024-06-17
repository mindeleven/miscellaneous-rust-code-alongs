/// Creating a Chat Server with async Rust and Tokio by Lily Mara
/// https://www.youtube.com/watch?v=T2mWg91sx-o
/// tokio is asynchronous and non blocking, allows to write high performance network applications
/// example project: chat server
/// -> allowing multiple tcp clients to connect to the chat server
/// -> when one client sends a message to the server, the message will be broadcast 
///    to all the different clients
/// (Step 1) Making a TCP echo server
///          -> server that waits for a client to connect
///          -> once a client connects it will take any message the client sends 
///             and bounce it right back to the client

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // first thing we need is a TCP listener
    // TcpListener is a type that comes from tokio
    let listener = TcpListener::bind("localhost:8080")
        .await // suspend current task until future is ready to be acted on
        .unwrap();

    println!("Hello, world!");
}
