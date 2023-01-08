#[macro_use]
extern crate lazy_static;
use tokio::sync::Mutex;
use std::sync::{Arc};
use std::fmt::Debug;
use std::pin::Pin;
use std::time::Duration;
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
    ApiConfig, IdempotencyResponse, IdempotencyId,
    IdempotencyDataMessage, IdempotencyStatus,
    IdempotencyMessage , Status as GRPCStatus
};
mod databases;
mod proto_bridge;

use databases::database::IDatabase;
use prost_types::Timestamp as grpcTimestamp;
use prost_types::Duration as grpcDuration;
use crate::databases::database::{IdempotencyTransaction, MessageStatusDef};
use crate::open_idempotency::MessageStatus;


// lazy_static! {
//     static ref DATABASE: Arc<Mutex<Box<dyn IDatabase + Send>>> = databases::create_database_mutex_sync();
// }
// let db = DATABASE.lock().await;

pub mod open_idempotency {
    tonic::include_proto!("open_idempotency");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("idempotency_descriptor");
}

#[derive(Debug, Default)]
pub struct OpenIdempotencyService {

}

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
                let id = v_request.id.unwrap().clone().uuid.clone();
                let app_id = v_request.app_id.clone();

                let mut db = databases::create_database().await;
                let check_result = db.exists(
                    id.clone(),
                    app_id.clone()
                ).await.expect("failed to check if id exists");
                if check_result.status == MessageStatusDef::None {
                    db.put(id.clone(),
                           app_id.clone(),
                           IdempotencyTransaction::new_default_in_progress(),
                           Some(Duration::from_secs(60 * 60))).await.expect("Failed to put key");
                }
                // Do some processing
                let temp = proto_bridge::convert_to_idempotency_status(
                    id.clone(),
                    check_result);
                tx.send(Ok(temp)).await.expect("failed to send response");
                // let status1 = Status::new(tonic::Code::Internal, "Failed to to handle request");
                // tx.send(Err(status1)).await.unwrap();
            }
        });

        info!("{}", "Client <data here> failed sending data from server");
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
                // let temp = IdempotencyStatus{
                //     status: MessageStatus::Completed
                //     id: "".to_string(),
                // };
                let status1 = Status::new(tonic::Code::Internal, "Failed to to handle request");
                tx.send(Err(status1)).await.unwrap();
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
        _request: Request<IdempotencyDataMessage>,
    ) -> Result<Response<()>, Status>{
        let req: IdempotencyDataMessage = _request.into_inner();
        let id = req.id.clone();
        // let app_id = req.app_id.clone();
        // let mut db = databases::create_database().await;
        Ok(Response::new(()))
    }

    async fn check(
        &self,
        request: Request<IdempotencyMessage>,
    ) -> Result<Response<IdempotencyStatus>, Status>{
        let req: IdempotencyMessage = request.into_inner();
        let id = req.id.unwrap().uuid.clone();
        let app_id = req.app_id.clone();
        let mut db = databases::create_database().await;
        //
        let check_result = db.exists(
            id.clone(),
            app_id.clone()
        ).await.expect("failed to check result");
        // Do some processing
        let temp = proto_bridge::convert_to_idempotency_status(
            id.clone(),
            check_result);
        Ok(Response::new(
            temp
        ))
        // let status1 = Status::new(tonic::Code::Internal, "Failed to to handle request");
        // tx.send(Err(status1)).await.unwrap();

        // let temp = proto_bridge::convert_to_idempotency_status(
        //     id.clone(),
        //     IdempotencyTransaction::new_default_none());
        // Ok(Response::new(
        //     temp
        // ))
    }

    async fn get_data(
        &self,
        request: Request<IdempotencyId>,
    ) -> Result<Response<IdempotencyDataMessage>, Status>{

        // let req: IdempotencyId = request.into_inner();
        // let id = req.id.clone();
        // let app_id = req.app_id.clone();

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
