
use client::hello_world;
use client::connect;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hello_world();
    connect().await?;
    Ok(())
}



