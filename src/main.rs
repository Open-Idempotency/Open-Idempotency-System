use tokio::sync::mpsc;
use server::start_server;
use std::{thread, time::Duration};
use futures_util::FutureExt;
use tokio::sync::mpsc::Sender;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (tx, mut rx) = mpsc::channel::<()>(1);
    let jh = tokio::spawn(async move {
        start_server(rx.recv().map(|_| ())).await.unwrap();
    });
    // tokio::time::sleep(Duration::from_secs(5)).await;

    ctrlc::set_handler(move || ctrl_c_handler(tx.to_owned()))
        .expect("Error setting Ctrl-C handler");
    jh.await.expect("TODO: panic message");
    println!("Server shutting down");
    Ok(())

}

fn ctrl_c_handler(tx: Sender<()>) {
    tokio::spawn(async move {
        tx.send(()).await.unwrap();
    });
}