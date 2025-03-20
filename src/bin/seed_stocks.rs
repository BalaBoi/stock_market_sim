use rust_decimal::{Decimal, prelude::FromPrimitive};
use sqlx::{PgPool, query};
use stock_market_sim::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::load_settings();

    let pool = settings
        .postgres
        .create_pool()
        .await
        .expect("should be able to creat pool of connections to postgres");
    seed_stocks(pool).await;

    println!("Stocks seeded succesfully");
}

async fn seed_stocks(pool: PgPool) {
    let stocks = vec![
        ("AAPL", "Apple Inc.", 195.61),
        ("MSFT", "Microsoft Corporation", 431.24),
        ("GOOGL", "Alphabet Inc.", 171.83),
        ("AMZN", "Amazon.com Inc.", 187.42),
        ("META", "Meta Platforms Inc.", 519.73),
        ("TSLA", "Tesla Inc.", 176.85),
        ("NVDA", "NVIDIA Corporation", 136.82),
        ("JPM", "JPMorgan Chase & Co.", 208.97),
        ("V", "Visa Inc.", 275.30),
        ("WMT", "Walmart Inc.", 70.84),
        ("PG", "Procter & Gamble Co.", 170.36),
        ("JNJ", "Johnson & Johnson", 152.49),
        ("UNH", "UnitedHealth Group Inc.", 537.28),
        ("HD", "Home Depot Inc.", 361.57),
        ("MA", "Mastercard Inc.", 471.65),
        ("BAC", "Bank of America Corp.", 40.83),
        ("PFE", "Pfizer Inc.", 27.18),
        ("DIS", "The Walt Disney Company", 116.32),
        ("NFLX", "Netflix Inc.", 687.90),
        ("CSCO", "Cisco Systems Inc.", 50.42),
    ];
    for stock in stocks {
        let _ = query!(
            r#"
            INSERT INTO stocks (symbol, name, current_price)
            VALUES ($1, $2, $3)
            ON CONFLICT (symbol) DO UPDATE
            SET name = $2
            "#,
            stock.0,
            stock.1,
            Decimal::from_f64(stock.2)
        )
        .execute(&pool)
        .await
        .expect("should be able to insert into the table");
    }
}
