#[macro_use]
extern crate lazy_static;
use tokio::sync::Mutex;
use std::sync::{Arc};
use std::fmt::Debug;
use std::future::Future;
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
    ApiConfig, IdempotencyId,
    IdempotencyData, IdempotencyStructure,
    IdempotencyRequest , Status as GRPCStatus
};
mod databases;
mod proto_bridge;

use databases::database::IDatabase;
use prost_types::Timestamp as grpcTimestamp;
use prost_types::Duration as grpcDuration;

use crate::databases::database::{IdempotencyTransaction, MessageStatusDef};
use crate::open_idempotency::{IdempotencyCompleteRequest, IdempotencyDataSaveRequest, MessageStatus};


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

fn parse_idempotency_request(req: &IdempotencyRequest) -> (String, String, Duration) {
    let wrapper_id = req.id.as_ref().expect("Id is required");
    (
        wrapper_id.id.clone(),
        wrapper_id.app_id.clone(),
        Duration::from_secs(req.custom_ttl as u64)
    )
}

fn parse_idempotency_id(id: &IdempotencyId) -> (String, String) {
    (
        id.id.clone(),
        id.app_id.clone()
    )
}

#[tonic::async_trait]
impl OpenIdempotency for OpenIdempotencyService {

    // type StreamCheckStream =
    // Pin<Box<dyn Stream<Item = Result<IdempotencyStructure, Status>> + 'static + Send + Sync >>;
    //
    // async fn stream_check(
    //     &self,
    //     request: Request<Streaming<IdempotencyRequest>>,
    // ) -> Result<Response<Self::StreamCheckStream>, Status>{
    //     let (tx, rx) = mpsc::channel(1);
    //
    //     let mut stream: Streaming<IdempotencyRequest> = request.into_inner();
    //
    //     tokio::spawn(async move {
    //         while let Some(vote) = stream.next().await {
    //             let v_request: IdempotencyRequest = vote.expect("request should have a value");
    //             let (id, app_id, ttl) = parse_idempotency_request(&v_request);
    //
    //             let mut db = databases::create_database().await;
    //             let check_result = db.exists(
    //                 id.clone(),
    //                 app_id.clone()
    //             ).await.expect("failed to check if id exists");
    //             if check_result.status == MessageStatusDef::None {
    //                 db.put(id.clone(),
    //                        app_id.clone(),
    //                        IdempotencyTransaction::new_default_in_progress(),
    //                        Some(Duration::from_secs(60 * 60))).await.expect("Failed to put key");
    //             }
    //             // Do some processing
    //             let temp = proto_bridge::convert_to_idempotency_status(
    //                 id.clone(), app_id.clone(),
    //                 check_result);
    //             tx.send(Ok(temp)).await.expect("failed to send response");
    //             // let status1 = Status::new(tonic::Code::Internal, "Failed to to handle request");
    //             // tx.send(Err(status1)).await.unwrap();
    //         }
    //     });
    //
    //     info!("{}", "Client <data here> failed sending data from server");
    //     Ok(Response::new(Box::pin(
    //         tokio_stream::wrappers::ReceiverStream::new(rx),
    //     )))
    //
    // }

    // type StreamSaveStream =
    // Pin<Box<dyn Stream<Item = Result<(), Status>> + 'static + Send + Sync >>;
    //
    // async fn stream_save(
    //     &self,
    //     request: Request<Streaming<IdempotencyDataSaveRequest>>,
    // ) -> Result<Response<Self::StreamSaveStream>, Status>{
    //     let (tx, rx) = mpsc::channel(1);
    //
    //     let mut stream: Streaming<IdempotencyDataSaveRequest> = request.into_inner();
    //
    //     tokio::spawn(async move {
    //         while let Some(vote) = stream.next().await {
    //             let v_request: IdempotencyDataSaveRequest = vote.unwrap();
    //             tx.send(Ok(())).await.unwrap();
    //             // Do some processing
    //             // let temp = IdempotencyStructure{
    //             //     status: MessageStatus::Completed
    //             //     id: "".to_string(),
    //             // };
    //             // let status1 = Status::new(tonic::Code::Internal, "Failed to to handle request");
    //             // tx.send(Err(status1)).await.unwrap();
    //         }
    //
    //         info!("{}", "Client <data here> failed sending data from server");
    //     });
    //
    //     Ok(Response::new(Box::pin(
    //         tokio_stream::wrappers::ReceiverStream::new(rx),
    //     )))
    //
    // }

    async fn delete(
        &self,
        _request: Request<IdempotencyRequest>,
    ) -> Result<Response<()>, Status>{
        Ok(Response::new(()))
    }

    async fn save_stage(
        &self,
        _request: Request<IdempotencyDataSaveRequest>,
    ) -> Result<Response<()>, Status>{
        // let req: IdempotencyDataSaveRequest = _request.into_inner();
        // let id = req.id.clone();
        let req = _request.into_inner();
        let (id, app_id)  = parse_idempotency_id(&req.id.expect("Id is required"));
        let mut db = databases::create_database().await;
        let check_result = db.exists(
            id.clone(),
            app_id.clone()
        ).await.expect("failed to check result");

        if check_result.status == MessageStatusDef::InProgress
        {
            db.update(
                id.clone(),
                app_id.clone(),
                IdempotencyTransaction {
                    status: MessageStatusDef::Completed,
                    stage: req.stage,
                    response: req.data.clone()
                }
            ).await.expect("failed to check result");
            Ok(Response::new(()))
        } else {
            let message = match check_result.status {
                MessageStatusDef::None => { "ERR_NONE" },
                MessageStatusDef::Completed => { "ERR_COMPLETED" },
                MessageStatusDef::InProgress => { panic!("Should not hit this") }
                MessageStatusDef::Failed => { "ERR_FAILED" }
            };
            Err( Status::new(tonic::Code::InvalidArgument, message))
        }
    }

    async fn check(&self, request: Request<IdempotencyRequest>) -> Result<Response<IdempotencyStructure>, Status>{
        let req: IdempotencyRequest = request.into_inner();
        let (id, app_id, ttl) = parse_idempotency_request(&req);
        let mut db = databases::create_database().await;
        //
        let check_result = db.exists(
            id.clone(),
            app_id.clone()
        ).await.expect("failed to check result");
        db.insert(id.clone(),app_id.clone(), IdempotencyTransaction {
            status: MessageStatusDef::InProgress,
            response: String::from(""),
            stage: String::from("")
        }, Some(ttl)).await.expect("failed to check result");
        // Do some processing
        let temp = proto_bridge::convert_to_idempotency_status(
            id.clone(), app_id.clone(), check_result);
        Ok(Response::new(temp))
    }


    async fn config(
        &self,
        request: Request<()>,
    ) -> Result<Response<ApiConfig>, Status>{
        Ok(Response::new(  ApiConfig{
            api: 0,
        }))
    }

    async fn complete(&self, request: Request<IdempotencyCompleteRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let (id, app_id)  = parse_idempotency_id(&req.id.expect("Id is required"));
        let mut db = databases::create_database().await;
        let check_result = db.exists(
            id.clone(),
            app_id.clone()
        ).await.expect("failed to check result");

        if check_result.status == MessageStatusDef::InProgress
        {
            db.update(
                id.clone(),
                app_id.clone(),
                IdempotencyTransaction {
                    status: MessageStatusDef::Completed,
                    stage: String::from(""),
                    response: req.data.clone()
                }
            ).await.expect("failed to check result");
            Ok(Response::new(()))
        } else {
            let message = match check_result.status {
                MessageStatusDef::None => { "ERR_NONE" },
                MessageStatusDef::Completed => { "ERR_COMPLETED" },
                MessageStatusDef::InProgress => { panic!("Should not hit this") }
                MessageStatusDef::Failed => { "ERR_FAILED" }
            };
            Err( Status::new(tonic::Code::InvalidArgument, message))

        }
    }
}
// use futures::future::{Abortable, AbortHandle, Aborted};

pub async fn start_server<F: Future<Output = ()>>(f: F) -> Result<(), Box<dyn std::error::Error>> {


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
        .serve_with_shutdown(address, f)
        .await
        .unwrap();
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