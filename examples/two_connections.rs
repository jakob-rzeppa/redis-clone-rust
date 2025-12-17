#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    let mut client1 = mini_redis::client::connect("127.0.0.1:6379").await?;
    let mut client2 = mini_redis::client::connect("127.0.0.1:6379").await?;

    client1.set("hello", "world".into()).await?;

    let result = client2.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    client2.set("hello", "not world".into()).await?;
    let result = client1.get("hello").await?;
    
    println!("got value from the server; result={:?}", result);

    Ok(())
}
