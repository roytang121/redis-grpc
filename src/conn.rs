use redis::AsyncCommands;

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

    pub async fn keys(&self, pattern: &str) -> anyhow::Result<Vec<String>> {
        let mut conn = self.conn.clone();
        let response = conn.keys::<&str, Vec<String>>(pattern).await?;
        Ok(response)
    }
}
