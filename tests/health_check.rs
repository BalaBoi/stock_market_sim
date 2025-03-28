use reqwest::StatusCode;
use sqlx::{PgPool, test};
use stock_market_sim::{Settings, serve_app};
use tokio::net::TcpListener;

#[test]
async fn health_check_returns_ok(pool: PgPool) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("should be able to able to listen on a free port from localhost");
    let addr = listener
        .local_addr()
        .expect("should be able to get local address from listener");
    let settings = Settings::load_settings();
    std::mem::drop(tokio::spawn(serve_app(
        listener,
        pool,
        settings.redis.create_pool().await.unwrap(),
    )));
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("should be able to send request");

    assert_eq!(response.status(), StatusCode::OK);
}
