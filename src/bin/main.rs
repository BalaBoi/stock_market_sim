use stock_market_sim::{Settings, StockPriceGenerator, init_tracing_subscriber, serve_app};
use tokio::net::TcpListener;
use tracing::error;

#[tokio::main]
async fn main() {
    let settings = Settings::load_settings();
    init_tracing_subscriber();
    let listener = TcpListener::bind(settings.http.address())
        .await
        .expect("should be able to listen to configured address");
    let pg_pool = settings
        .postgres
        .create_pool()
        .await
        .expect("should be able to get a pool to the postgres instance");
    let redis_pool = settings
        .redis
        .create_pool()
        .await
        .expect("failure in creating redis pool");

    let price_generator = StockPriceGenerator::new(pg_pool.clone(), 50, 10);
    std::mem::drop(tokio::spawn(async move {
        if let Err(e) = price_generator.run().await {
            error!(error = ?e, "price generator errored");
        }
    }));
    serve_app(listener, pg_pool, redis_pool).await;
}
