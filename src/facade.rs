use redis::{AsyncCommands, Value};
use tokio_stream::StreamExt;

#[tonic::async_trait]
pub trait MessageConsumer {
    async fn consume(&self, msg: redis::Msg) -> anyhow::Result<()>;
}

pub struct RedisFacade {
    pub conn: redis::aio::MultiplexedConnection,
    pub url: String,
}

impl RedisFacade {
    pub async fn new(url: &str) -> Self {
        let client = redis::Client::open(url)
            .expect(format!("Failed to establish redis connection at {}", url).as_str());
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .expect("Failed to get multiplexed async connection.");
        RedisFacade {
            conn,
            url: url.to_string(),
        }
    }

    fn get_conn(&self) -> redis::aio::MultiplexedConnection {
        self.conn.clone()
    }

    fn format_redis_value(value: redis::Value) -> Option<String> {
        match value {
            Value::Nil => None,
            Value::Int(value) => Some(format!("{}", value)),
            Value::Data(data) => Some(String::from_utf8(data).unwrap()),
            Value::Bulk(data) => Some(format!(
                "{:?}",
                data.into_iter()
                    .map(Self::format_redis_value)
                    .map(|val| val.unwrap_or_default())
                    .collect::<Vec<String>>()
            )),
            Value::Status(status) => Some(status),
            Value::Okay => Some(String::from("OK")),
        }
    }

    pub async fn command(&self, command: &str) -> anyhow::Result<Option<String>> {
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

    pub async fn set(&self, k: &str, v: &str) -> anyhow::Result<Option<String>> {
        let mut conn = self.get_conn();
        let response = conn.set::<&str, &str, Value>(k, v).await?;
        Ok(RedisFacade::format_redis_value(response))
    }

    pub async fn get(&self, k: &str) -> anyhow::Result<Option<String>> {
        let mut conn = self.get_conn();
        let response = conn.get::<&str, Value>(k).await?;
        Ok(RedisFacade::format_redis_value(response))
    }

    pub async fn del(&self, k: &str) -> anyhow::Result<i64> {
        let mut conn = self.get_conn();
        let response = conn.del::<&str, i64>(k).await?;
        Ok(response)
    }

    pub async fn lpush(&self, k: &str, element: &str) -> anyhow::Result<i64> {
        let mut conn = self.get_conn();
        let response = conn.lpush::<&str, &str, i64>(k, element).await?;
        Ok(response)
    }

    pub async fn rpush(&self, k: &str, element: &str) -> anyhow::Result<i64> {
        let mut conn = self.get_conn();
        let response = conn.rpush::<&str, &str, i64>(k, element).await?;
        Ok(response)
    }

    pub async fn subscribe_channels<T>(
        url: &str,
        channels: &Vec<String>,
        consumer: &T,
    ) -> anyhow::Result<()>
    where
        T: MessageConsumer,
    {
        let client = redis::Client::open(url)?;
        let conn = client.get_async_connection().await?;
        let mut pubsub = conn.into_pubsub();
        for channel in channels {
            info!("subscribe channel: {}", channel);
            pubsub.subscribe(channel).await?;
        }
        let mut stream = pubsub.on_message();
        while let Some(msg) = stream.next().await {
            consumer.consume(msg).await?
        }
        Ok(())
    }

    pub async fn publish(&self, channel: &str, message: &str) -> anyhow::Result<i64> {
        let mut conn = self.conn.clone();
        let result = conn.publish::<&str, &str, i64>(channel, message).await;
        match result {
            Ok(n) => Ok(n),
            Err(err) => Err(anyhow::anyhow!("{}", err)),
        }
    }
}
