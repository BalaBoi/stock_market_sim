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
        ("AAPL", "Apple Inc."),
        ("MSFT", "Microsoft Corporation"),
        ("GOOGL", "Alphabet Inc."),
        ("AMZN", "Amazon.com Inc."),
        ("META", "Meta Platforms Inc."),
        ("TSLA", "Tesla Inc."),
        ("NVDA", "NVIDIA Corporation"),
        ("JPM", "JPMorgan Chase & Co."),
        ("V", "Visa Inc."),
        ("WMT", "Walmart Inc."),
        ("PG", "Procter & Gamble Co."),
        ("JNJ", "Johnson & Johnson"),
        ("UNH", "UnitedHealth Group Inc."),
        ("HD", "Home Depot Inc."),
        ("MA", "Mastercard Inc."),
        ("BAC", "Bank of America Corp."),
        ("PFE", "Pfizer Inc."),
        ("DIS", "The Walt Disney Company"),
        ("NFLX", "Netflix Inc."),
        ("CSCO", "Cisco Systems Inc."),
    ];

    for stock in stocks {
        let _ = query!(
            r#"
            INSERT INTO stocks (symbol, name)
            VALUES ($1, $2)
            ON CONFLICT (symbol) DO UPDATE
            SET name = $2
            "#,
            stock.0,
            stock.1
        )
        .execute(&pool)
        .await
        .expect("should be able to insert into the table");
    }
}
