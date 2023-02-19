use client::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect("http://[::1]:8080").await?;
    client.check(String::from("123"), String::from("app1")).await?;
    Ok(())
}



