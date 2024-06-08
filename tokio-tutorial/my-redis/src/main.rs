use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // opening a connection to the mini_redis address
    let mut client = client::connect("127.0.0.1:6379").await?;
    
    // setting the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
