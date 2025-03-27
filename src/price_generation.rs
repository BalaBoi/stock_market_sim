use std::time::Duration;

use anyhow::anyhow;
use rand::Rng;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use sqlx::{PgPool, query_as, query_scalar};
use tracing::debug;

use crate::http::stocks_db::{Stock, update_stock_price};

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

pub struct StockPriceGenerator {
    pool: PgPool,
    max_price_delta: u32,
    max_wait_time: u32,
}

impl StockPriceGenerator {
    pub fn new(pool: PgPool, max_price_delta: u32, max_wait_time: u32) -> Self {
        Self {
            pool,
            max_price_delta,
            max_wait_time,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        loop {
            let random_stock = get_random_stock(&self.pool).await?;

            let random_price_delta = Decimal::from_i64(
                rand::rng()
                    .random_range(-(self.max_price_delta as i64)..(self.max_price_delta as i64)),
            )
            .unwrap();
            let updated_price = random_stock.current_price + random_price_delta;
            debug!(updated_stock = %random_stock.symbol, %updated_price, old_price = %random_stock.current_price, "Updating stock price");
            update_stock_price(&self.pool, random_stock.id, updated_price).await?;

            let sleep_seconds = rand::rng().random_range(0..self.max_wait_time) as u64;
            tokio::time::sleep(Duration::from_secs(sleep_seconds)).await;
        }
    }
}
