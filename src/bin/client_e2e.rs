use client::connect;
use tokio::sync::mpsc;
use server::start_server;
use std::{thread, time::Duration};
use futures_util::FutureExt;
use tokio::sync::mpsc::Sender;
use open_idempotency::server::start_server;
use open_idempotency::client::connect;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (tx, mut rx) = mpsc::channel::<()>(1);
    let jh = tokio::spawn(async move {
        start_server(rx.recv().map(|_| ())).await.unwrap();
    });

    let mut client = connect("http://[::1]:8080").await?;
    client.check(String::from("123"), String::from("app1")).await?;
    // shutdown server
    tx.send(()).await.unwrap();
    jh.await.expect("TODO: panic message");
    Ok(())
}



