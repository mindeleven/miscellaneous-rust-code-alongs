/// Creating a Chat Server with async Rust and Tokio by Lily Mara
/// https://www.youtube.com/watch?v=T2mWg91sx-o
/// tokio is asynchronous and non blocking, allows to write high performance network applications
/// example project: chat server
/// -> allowing multiple tcp clients to connect to the chat server
/// -> when one client sends a message to the server, the message will be broadcast 
///    to all the different clients

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
