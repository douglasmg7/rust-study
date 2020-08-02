use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to a mini-reids address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "Hello" with value "world".
    client.set("hello", "world".into()).await?;

    // Get key "hello".
    let result = client.get("hello").await?;

    println!("got value from de server; result={:?}", result);

    Ok(())
}
