use rust_redis::{Result, client::Client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
