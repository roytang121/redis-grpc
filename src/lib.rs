#[macro_use]
extern crate tracing;

pub mod facade;
pub mod grpc;

#[derive(Debug)]
pub struct AppConfig {
    pub port: String,
    pub host: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let port = std::env::var("REDIS_GRPC_PORT").unwrap_or("50051".to_string());
        let host = std::env::var("REDIS_GRPC_HOST").unwrap_or("redis://0.0.0.0:6379".to_string());
        AppConfig { port, host }
    }
}
