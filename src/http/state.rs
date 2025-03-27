use axum::extract::FromRef;
use sqlx::PgPool;

use crate::RedisPool;

#[derive(Clone, FromRef)]
pub struct ApiState {
    pg_pool: PgPool,
    redis_pool: RedisPool,
}

impl ApiState {
    pub fn new(pg_pool: PgPool, redis_pool: RedisPool) -> Self {
        Self {
            pg_pool,
            redis_pool,
        }
    }
}
