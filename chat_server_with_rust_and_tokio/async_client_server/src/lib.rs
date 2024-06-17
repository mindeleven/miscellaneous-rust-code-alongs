/// an asynchrouous client and server
/// coding along with the async-chat example in O'Reilly's Programming Rust
/// main goal of the example is to handle the backpressure well
/// backpressure:
/// -> a client having a slow connection or dropping out must never 
///    affect other clients ability to exchange messages
/// -> messages for clients who can't keep up are dropped by the server,
///    but clients get informed that their stream in incomplete
fn _somefunc() {
    unimplemented!()
}