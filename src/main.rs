use stock_market_sim::{Settings, init_tracing_subscriber, serve_app};
use tokio::net::TcpListener;

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
    serve_app(listener, pg_pool).await;
}
