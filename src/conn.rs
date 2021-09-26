use redis::{AsyncCommands, Value};

pub struct RedisFacade {
    pub conn: redis::aio::MultiplexedConnection,
}

impl RedisFacade {
    pub async fn new(url: &str) -> Self {
        let client = redis::Client::open(url)
            .expect(format!("Failed to establish redis connection at {}", url).as_str());
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .expect("Failed to get multiplexed async connection.");
        RedisFacade { conn }
    }

    fn get_conn(&self) -> redis::aio::MultiplexedConnection {
        self.conn.clone()
    }

    fn format_redis_value(value: redis::Value) -> String {
        match value {
            Value::Nil => String::from("(nil)"),
            Value::Int(value) => format!("{}", value),
            Value::Data(data) => String::from_utf8(data).unwrap(),
            Value::Bulk(data) => format!("{:?}", data),
            Value::Status(status) => format!("{}", status),
            Value::Okay => String::from("OK"),
        }
    }

    pub async fn command(&self, command: &str) -> anyhow::Result<String> {
        let mut conn = self.get_conn();
        let args = command.split(" ").collect::<Vec<&str>>();
        let mut cmd = redis::cmd(args.get(0).unwrap());
        for arg in args[1..].iter() {
            cmd.arg(arg);
        }
        let msg = cmd
            .query_async::<redis::aio::MultiplexedConnection, redis::Value>(&mut conn)
            .await?;
        Ok(RedisFacade::format_redis_value(msg))
    }

    pub async fn keys(&self, pattern: &str) -> anyhow::Result<Vec<String>> {
        let mut conn = self.get_conn();
        let response = conn.keys::<&str, Vec<String>>(pattern).await?;
        Ok(response)
    }

    pub async fn set(&self, k: &str, v: &str) -> anyhow::Result<String> {
        let mut conn = self.get_conn();
        let response = conn.set::<&str, &str, Value>(k, v).await?;
        Ok(RedisFacade::format_redis_value(response))
    }

    pub async fn get(&self, k: &str) -> anyhow::Result<String> {
        let mut conn = self.get_conn();
        let response = conn.get::<&str, Value>(k).await?;
        Ok(RedisFacade::format_redis_value(response))
    }
}
