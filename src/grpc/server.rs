use tonic::{transport::Server, Code, Request, Response, Status};

use self::redis_grpc::redis_grpc_server::{RedisGrpc, RedisGrpcServer};
use self::redis_grpc::{
    CommandRequest, CommandResponse, DelRequest, GetRequest, IntResponse, KeysRequest,
    LPushRequest, PublishRequest, RPushRequest, SetRequest, StringListResponse, StringResponse,
    SubscribeRequest, SubscribeResponse,
};
use crate::facade::{MessageConsumer, RedisFacade};
use crate::AppConfig;
use tokio_stream::wrappers::ReceiverStream;

pub mod redis_grpc {
    tonic::include_proto!("redis_grpc");
}

pub struct RedisGrpcImpl {
    redis: RedisFacade,
    redis_host: String,
}

impl RedisGrpcImpl {
    pub async fn new(app_config: &AppConfig) -> Self {
        let redis = RedisFacade::new(app_config.host.as_str()).await;
        return RedisGrpcImpl {
            redis,
            redis_host: app_config.host.clone(),
        };
    }
}

fn handle_string_result(
    result: anyhow::Result<Option<String>>,
) -> anyhow::Result<StringResponse, Status> {
    return match result {
        Ok(result) => Ok(StringResponse { result }),
        Err(error) => Err(Status::new(Code::Internal, error.to_string())),
    };
}

fn handle_int_result(result: anyhow::Result<i64>) -> anyhow::Result<IntResponse, Status> {
    return match result {
        Ok(result) => Ok(IntResponse { result }),
        Err(error) => Err(Status::new(Code::Internal, error.to_string())),
    };
}

fn handle_string_array_result(
    result: anyhow::Result<Vec<String>>,
) -> anyhow::Result<StringListResponse, Status> {
    return match result {
        Ok(result) => Ok(StringListResponse { result }),
        Err(error) => Err(Status::new(Code::Internal, error.to_string())),
    };
}

#[tonic::async_trait]
impl RedisGrpc for RedisGrpcImpl {
    type subscribeStream = ReceiverStream<Result<SubscribeResponse, Status>>;

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
    ) -> Result<Response<Self::subscribeStream>, Status> {
        info!("Got a request: {:?}", request);
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let channels = request.into_inner().channels;
        let url = self.redis_host.clone();
        tokio::spawn(async move {
            let consumer = SubscribeMessageConsumer(tx);
            RedisFacade::subscribe_channels(&url, &channels, &consumer)
                .await
                .expect(format!("subscribe_channels exited: {:?}", channels).as_str());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<IntResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self
            .redis
            .publish(request.channel.as_str(), request.message.as_str())
            .await;
        Ok(Response::new(handle_int_result(redis_result)?))
    }

    async fn keys(
        &self,
        request: Request<KeysRequest>,
    ) -> Result<Response<StringListResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self.redis.keys(request.pattern.as_str()).await;
        Ok(Response::new(handle_string_array_result(redis_result)?))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<StringResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self
            .redis
            .set(request.key.as_str(), request.value.as_str())
            .await;
        Ok(Response::new(handle_string_result(redis_result)?))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<StringResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self.redis.get(request.key.as_str()).await;
        Ok(Response::new(handle_string_result(redis_result)?))
    }

    async fn del(&self, request: Request<DelRequest>) -> Result<Response<IntResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self.redis.del(request.key.as_str()).await;
        Ok(Response::new(handle_int_result(redis_result)?))
    }

    async fn lpush(&self, request: Request<LPushRequest>) -> Result<Response<IntResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self
            .redis
            .lpush(request.key.as_str(), request.element.as_str())
            .await;
        Ok(Response::new(handle_int_result(redis_result)?))
    }

    async fn rpush(&self, request: Request<RPushRequest>) -> Result<Response<IntResponse>, Status> {
        let request = request.into_inner();
        let redis_result = self
            .redis
            .rpush(request.key.as_str(), request.element.as_str())
            .await;
        Ok(Response::new(handle_int_result(redis_result)?))
    }
}

pub struct SubscribeMessageConsumer(
    tokio::sync::mpsc::Sender<Result<SubscribeResponse, tonic::Status>>,
);
#[tonic::async_trait]
impl MessageConsumer for SubscribeMessageConsumer {
    async fn consume(&self, message: redis::Msg) -> anyhow::Result<()> {
        let response = SubscribeResponse {
            channel: message.get_channel::<String>().unwrap(),
            // pattern: message.get_pattern::<redis::Value>().unwrap(),
            message: message.get_payload::<String>().unwrap(),
        };
        self.0.send(Ok(response)).await?;
        Ok(())
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
