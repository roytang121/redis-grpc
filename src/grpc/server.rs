use tonic::{transport::Server, Request, Response, Status};

use crate::conn::RedisFacade;
use crate::AppConfig;
use redis_grpc::redis_grpc_server::{RedisGrpc, RedisGrpcServer};
use redis_grpc::{KeysRequest, KeysResponse, ParamsRequest, ParamsResponse};
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
    type StreamParamsStream = ReceiverStream<Result<ParamsResponse, Status>>;

    async fn stream_params(
        &self,
        request: Request<ParamsRequest>,
    ) -> Result<Response<Self::StreamParamsStream>, Status> {
        info!("Got a request: {:?}", request);
        let (mut tx, rx) = tokio::sync::mpsc::channel(4);
        let subscribe_key = request.into_inner().key;
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
