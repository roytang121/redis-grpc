#[macro_use]
extern crate tracing;

pub mod conn;
pub mod grpc;

#[derive(Debug)]
pub struct AppConfig {
    pub port: usize,
    pub host: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            port: 50051,
            host: String::from("redis://0.0.0.0:6349"),
        }
    }
}
