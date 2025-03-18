use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone, FromRef)]
pub struct ApiState {
    pool: PgPool,
}

impl ApiState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
