#[macro_use]
extern crate lazy_static;

use std::fmt::Debug;
use std::pin::Pin;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tonic::metadata::{KeyAndValueRef, MetadataValue};
use tonic_health::server::HealthReporter;
use tonic_reflection::server::Builder;
use tokio::sync::mpsc;
use open_idempotency::{
    open_idempotency_server::{OpenIdempotency, OpenIdempotencyServer } ,
    ApiConfig, IdempotencyResponse, IdpempotencyId,
    IdempotencyDataMessage, IdempotencyStatus,
    IdempotencyMessage , Status as GRPCStatus
};
mod databases;

use databases::database::IDatabase;
use prost_types::Timestamp as grpcTimestamp;
use prost_types::Duration as grpcDuration;


fn do_stuff() {
    let c = databases::create_database();
}
// lazy_static! {
//     static ref DATABASE: IDatabase
// }

pub mod open_idempotency {
    tonic::include_proto!("open_idempotency");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("idempotency_descriptor");
}

#[derive(Debug, Default)]
pub struct OpenIdempotencyService {}

#[tonic::async_trait]
impl OpenIdempotency for OpenIdempotencyService {

    type StreamCheckStream =
    Pin<Box<dyn Stream<Item = Result<IdempotencyStatus, Status>> + 'static + Send + Sync >>;

    async fn stream_check(
        &self,
        request: Request<Streaming<IdempotencyMessage>>,
    ) -> Result<Response<Self::StreamCheckStream>, Status>{
        let (tx, rx) = mpsc::channel(1);

        let mut stream: Streaming<IdempotencyMessage> = request.into_inner();

        tokio::spawn(async move {
            while let Some(vote) = stream.next().await {
                let v_request: IdempotencyMessage = vote.unwrap();

                // Do some processing
                let temp = IdempotencyStatus{
                    success: false,
                };
                tx.send(Ok(temp)).await.unwrap();
            }

            info!("{}", "Client <data here> failed sending data from server");
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))

    }

    type StreamSaveStream =
    Pin<Box<dyn Stream<Item = Result<IdempotencyStatus, Status>> + 'static + Send + Sync >>;

    async fn stream_save(
        &self,
        request: Request<Streaming<IdempotencyDataMessage>>,
    ) -> Result<Response<Self::StreamSaveStream>, Status>{
        let (tx, rx) = mpsc::channel(1);

        let mut stream: Streaming<IdempotencyDataMessage> = request.into_inner();

        tokio::spawn(async move {
            while let Some(vote) = stream.next().await {
                let v_request: IdempotencyDataMessage = vote.unwrap();

                // Do some processing
                let temp = IdempotencyStatus{
                    success: false,
                };
                tx.send(Ok(temp)).await.unwrap();
            }

            info!("{}", "Client <data here> failed sending data from server");
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))

    }
    async fn delete(
        &self,
        _request: Request<IdempotencyMessage>,
    ) -> Result<Response<()>, Status>{
        Ok(Response::new(()))
    }

    async fn save(
        &self,
        _request: Request<IdempotencyId>,
    ) -> Result<Response<()>, Status>{
        Ok(Response::new(()))
    }

    async fn check(
        &self,
        request: Request<IdempotencyMessage>,
    ) -> Result<Response<IdempotencyResponse>, Status>{
        Ok(Response::new(
            IdempotencyResponse{
                status: 0,
                expire: None,
            }
        ))
    }

    async fn get_data(
        &self,
        request: Request<IdpempotencyId>,
    ) -> Result<Response<IdempotencyDataMessage>, Status>{
        Ok(Response::new(  IdempotencyDataMessage{
            id: "".to_string(),
            data: "".to_string(),
        }))
    }

    async fn config(
        &self,
        request: Request<()>,
    ) -> Result<Response<ApiConfig>, Status>{
        Ok(Response::new(  ApiConfig{
            api: 0,
        }))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Configuring Logging");
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    info!("Configuring Server");
    let address = "[::1]:8080".parse().unwrap();
    let oIdm_service = OpenIdempotencyService::default();

    info!("Configuring Authentication");
    let auth = open_idempotency::open_idempotency_server::OpenIdempotencyServer::with_interceptor(oIdm_service, check_auth);

    info!("Configuring Health Check");
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<OpenIdempotencyServer<OpenIdempotencyService>>()
        .await;


    info!("Configuring Reflection");
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(open_idempotency::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(tonic_health::proto::GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    println!("GreeterServer listening on {}", address);
    Server::builder()
        .add_service(reflection_service)
        .add_service(auth)
        .add_service(health_service)
        .serve(address)
        .await?;
    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    // FIXME
    let token: MetadataValue<_> = "Bearer some-auth-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) => {
            if t == token {
                Ok(req)
            }else {
                Err(Status::unauthenticated("No valid auth token"))
            }

        },
        _ => Err(Status::unauthenticated("No valid auth token")),
    }

}
