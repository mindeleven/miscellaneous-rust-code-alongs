use mini_redis::{
    client, 
    Result
};

#[tokio::main]
async fn main() -> Result<()> {
    // opening a connection to the mini_redis address
    // the client::connect function is provided by the mini-redis crate
    // connection returns client handle
    let mut client = client::connect("127.0.0.1:6379").await?;

    let op = say_world();
    
    // proceeding with the client handle
    // setting the key "hello" with value "world"
    client.set("hello", "world".into()).await?;
    
    // calls to .await within the async fn yield control back to the thread
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    println!("hello");

    op.await;

    Ok(())
}

async fn say_world() {
    println!("world");
}