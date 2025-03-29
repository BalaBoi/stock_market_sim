use std::{collections::HashMap, str::FromStr};

use deadpool_redis::redis::cmd;
use uuid::Uuid;

use crate::RedisPool;

use super::session::Session;

#[derive(Debug, Clone, Copy)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for SessionId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid: Uuid = s.parse()?;
        Ok(Self(uuid))
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    redis_pool: RedisPool,
}

impl Store {
    pub fn new(redis_pool: RedisPool) -> Self {
        Self { redis_pool }
    }

    pub async fn save(&self, session: &Session) -> anyhow::Result<()> {
        let mut conn = self.redis_pool.get().await?;
        todo!()
    }

    pub async fn load(&self, session_id: SessionId) -> anyhow::Result<Session> {
        let mut conn = self.redis_pool.get().await?;
        let data: HashMap<String, String> = cmd("GET")
            .arg(session_id.0.as_bytes())
            .query_async(&mut conn)
            .await?;
        Ok(Session::new(session_id, data))
    }

    pub async fn delete(&self, session_id: SessionId) -> anyhow::Result<()> {
        todo!()
    }
}
