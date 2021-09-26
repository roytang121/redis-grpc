use tonic::{transport::Server, Request, Response, Status};

use crate::conn::RedisFacade;
use crate::AppConfig;
use redis_grpc::redis_grpc_server::{RedisGrpc, RedisGrpcServer};
use redis_grpc::{
    CommandRequest, CommandResponse, GetRequest, GetResponse, KeysRequest, KeysResponse,
    SetRequest, SetResponse, SubscribeRequest, SubscribeResponse,
};
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::wrappers::ReceiverStream;

pub mod redis_grpc {
    tonic::include_proto!("redis_grpc");
}

pub struct RedisGrpcImpl {
    redis: RedisFacade,
}

impl RedisGrpcImpl {
    pub async fn new(app_config: &AppConfig) -> Self {
        let redis = RedisFacade::new(app_config.host.as_str()).await;
        return RedisGrpcImpl { redis };
    }
}

#[tonic::async_trait]
impl RedisGrpc for RedisGrpcImpl {
    type SubscribeStream = ReceiverStream<Result<SubscribeResponse, Status>>;

    async fn command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self.redis.command(request.command.as_str()).await.unwrap();
        let grpc_response = CommandResponse {
            message: redis_result,
        };
        Ok(Response::new(grpc_response))
    }

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        info!("Got a request: {:?}", request);
        let (mut tx, rx) = tokio::sync::mpsc::channel(4);
        let channels = request.into_inner().channels;
        // tokio::spawn(async move {
        //     let consumer = LambdaParamsMessageConsumer(tx);
        //     RedisBackedMessageBus::subscribe_channels(vec![&subscribe_key], &consumer).await.unwrap();
        // });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn keys(&self, request: Request<KeysRequest>) -> Result<Response<KeysResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self.redis.keys(request.pattern.as_str()).await;
        let grpc_response = match redis_result {
            Ok(result) => KeysResponse {
                success: true,
                error: String::default(),
                result,
            },
            Err(err) => KeysResponse {
                success: false,
                error: format!("{}", err),
                result: vec![],
            },
        };
        Ok(Response::new(grpc_response))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self
            .redis
            .set(request.key.as_str(), request.value.as_str())
            .await;
        let grpc_response = match redis_result {
            Ok(result) => SetResponse {
                success: true,
                error: String::default(),
                result,
            },
            Err(err) => SetResponse {
                success: false,
                error: format!("{}", err),
                result: String::default(),
            },
        };
        Ok(Response::new(grpc_response))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self.redis.get(request.key.as_str()).await;
        let grpc_response = match redis_result {
            Ok(result) => GetResponse {
                success: true,
                error: String::default(),
                result,
            },
            Err(err) => GetResponse {
                success: false,
                error: format!("{}", err),
                result: String::default(),
            },
        };
        Ok(Response::new(grpc_response))
    }
}

pub struct RedisGrpcService {}
impl RedisGrpcService {
    pub fn new() -> Self {
        RedisGrpcService {}
    }
    pub async fn subscribe(&self, app_config: &AppConfig) -> anyhow::Result<()> {
        let socket_addr = format!("0.0.0.0:{port}", port = app_config.port).parse()?;
        let redis_grpc = RedisGrpcImpl::new(app_config).await;
        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(RedisGrpcServer::new(redis_grpc)))
            .serve(socket_addr)
            .await?;

        Ok(())
    }
}
