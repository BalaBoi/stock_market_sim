use anyhow::anyhow;
use rand::Rng;
use sqlx::{query_as, query_scalar, PgPool};

use crate::http::stocks::db::Stock;

async fn get_random_stock(pool: &PgPool) -> anyhow::Result<Stock> {
    let n_stocks = query_scalar!(
        r#"
        SELECT COUNT(*) FROM stocks
        "#
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    if n_stocks == 0 {
        return Err(anyhow!("Stocks table is empty"));
    }

    let rand_offset = rand::rng().random_range(0..n_stocks);

    let stock = query_as!(
        Stock,
        r#"
        SELECT * FROM stocks LIMIT 1 OFFSET $1
        "#,
        rand_offset
    )
    .fetch_one(pool)
    .await?;

    Ok(stock)
}