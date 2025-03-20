use anyhow::anyhow;
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::{PgPool, query, query_as};
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use super::super::error::{Error, Result};

#[instrument(skip_all, fields())]
pub async fn get_stocks(pool: &PgPool) -> Result<Vec<Stock>> {
    let stocks = query_as!(Stock, "SELECT * FROM stocks")
        .fetch_all(pool)
        .await?;

    Ok(stocks)
}

#[instrument(skip_all)]
pub async fn update_stock_price(pool: &PgPool, stock_id: Uuid, price: Decimal) -> Result<()> {
    let res = query!(
        r#"
        UPDATE stocks
        SET current_price = $1
        WHERE id = $2
        "#,
        price,
        stock_id
    )
    .execute(pool)
    .await?;

    if res.rows_affected() != 1 {
        return Err(Error::Other(anyhow!(
            "number of rows affected in update_stock_price op was not 1 but {}",
            res.rows_affected()
        )));
    }

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct Stock {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub current_price: Decimal,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
