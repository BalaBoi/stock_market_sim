use serde::Serialize;
use sqlx::{query_as, PgPool};
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use super::super::error::Result;

#[instrument(
    skip_all,
    fields()
)]
pub async fn get_stocks(pool: &PgPool) -> Result<Vec<Stock>> {
    let stocks = query_as!(
        Stock,
        "SELECT * FROM stocks"
    )
    .fetch_all(pool)
    .await?;

    Ok(stocks)
}

#[derive(Debug, Serialize)]
pub struct Stock {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime
}