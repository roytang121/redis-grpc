use clap::{App, Error};
use redis_grpc::grpc::server::RedisGrpcService;
use redis_grpc::AppConfig;

#[tracing::instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut app_config = AppConfig::default();

    let matches = App::new("redis-grpc")
        .version("0.0.1")
        .author("Roy Tang. <me@roytang.me>")
        .about("redis gRPC proxy")
        .arg("-p, --port=[PORT] 'Listen on port'")
        .arg("-h --host=[HOST] 'Target redis host to proxy from'")
        .get_matches();

    match matches.value_of_t::<usize>("port") {
        Ok(port) => app_config.port = port,
        Err(_) => {}
    }
    match matches.value_of_t::<String>("host") {
        Ok(host) => app_config.host = host,
        Err(_) => {}
    }
    tracing::info!(
        port = app_config.port,
        host = app_config.host.as_str(),
        "starting redis-grpc",
    );

    let service = RedisGrpcService::new();
    return match service.subscribe(&app_config).await {
        Ok(_) => {
            log::error!("view exited uncaught");
            Ok(())
        }
        Err(err) => {
            log::error!("{}", err);
            Err(anyhow::anyhow!(err))
        }
    };
}
