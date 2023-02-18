use open_idempotency::{
    open_idempotency_client::{OpenIdempotencyClient },
    IdempotencyRequest
};
pub fn hello_world() {
    println!("hello World!")
}

pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OpenIdempotencyClient::connect("http://[::1]:8080").await?;

    let request = tonic::Request::new(IdempotencyRequest {
        id: Some(open_idempotency::IdempotencyId { id: String::from("123"), app_id: String::from("app1") }),
        custom_ttl: 9999
    });
    //
    let response = client.check(request).await?;
    //
    println!("RESPONSE={:?}", response);

    Ok(())
}

pub mod open_idempotency {
    tonic::include_proto!("open_idempotency");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("idempotency_descriptor");
}
//
// use futures::stream::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
//
// use open_idempotency_client::{echo_client::EchoClient, EchoRequest};
//
// pub fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
//     tokio_stream::iter(1..usize::MAX).map(|i| EchoRequest {
//         message: format!("msg {:02}", i),
//     })
// }
//
// async fn streaming_echo(client: &mut EchoClient<Channel>, num: usize) {
//     let stream = client
//         .server_streaming_echo(EchoRequest {
//             message: "foo".into(),
//         })
//         .await
//         .unwrap()
//         .into_inner();
//
//     // stream is infinite - take just 5 elements and then disconnect
//     let mut stream = stream.take(num);
//     while let Some(item) = stream.next().await {
//         println!("\treceived: {}", item.unwrap().message);
//     }
//     // stream is droped here and the disconnect info is send to server
// }
//
// async fn bidirectional_streaming_echo(client: &mut EchoClient<Channel>, num: usize) {
//     let in_stream = echo_requests_iter().take(num);
//
//     let response = client
//         .bidirectional_streaming_echo(in_stream)
//         .await
//         .unwrap();
//
//     let mut resp_stream = response.into_inner();
//
//     while let Some(received) = resp_stream.next().await {
//         let received = received.unwrap();
//         println!("\treceived message: `{}`", received.message);
//     }
// }
//
// async fn bidirectional_streaming_echo_throttle(client: &mut EchoClient<Channel>, dur: Duration) {
//     let in_stream = echo_requests_iter().throttle(dur);
//
//     let response = client
//         .bidirectional_streaming_echo(in_stream)
//         .await
//         .unwrap();
//
//     let mut resp_stream = response.into_inner();
//
//     while let Some(received) = resp_stream.next().await {
//         let received = received.unwrap();
//         println!("\treceived message: `{}`", received.message);
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = EchoClient::connect("http://[::1]:50051").await.unwrap();
//
//     println!("Streaming echo:");
//     streaming_echo(&mut client, 5).await;
//     tokio::time::sleep(Duration::from_secs(1)).await; //do not mess server println functions
//
//     // Echo stream that sends 17 requests then graceful end that connection
//     println!("\r\nBidirectional stream echo:");
//     bidirectional_streaming_echo(&mut client, 17).await;
//
//     // Echo stream that sends up to `usize::MAX` requets. One request each 2s.
//     // Exiting client with CTRL+C demonstrate how to distinguish broken pipe from
//     //graceful client disconnection (above example) on the server side.
//     println!("\r\nBidirectional stream echo (kill client with CTLR+C):");
//     bidirectional_streaming_echo_throttle(&mut client, Duration::from_secs(2)).await;
//
//     Ok(())
// }