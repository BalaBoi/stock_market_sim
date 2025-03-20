use axum::{
    Router,
    http::{HeaderName, Request},
    routing::get,
};
use sqlx::PgPool;
use state::ApiState;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{ServiceBuilderExt, request_id::MakeRequestUuid, trace::TraceLayer};

use health_check::health_check;
use tracing::info_span;

mod error;
mod health_check;
mod state;
pub mod stocks;

pub async fn serve_app(listener: TcpListener, pg_pool: PgPool) {
    let state = ApiState::new(pg_pool);
    let app = api_router(state);

    axum::serve(listener, app)
        .await
        .expect("error in http server")
}

fn api_router(state: ApiState) -> Router {
    let req_id_header = HeaderName::from_static("req-id");
    Router::new()
        .route("/health_check", get(health_check))
        .nest(stocks::ROOT, stocks::router())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .set_request_id(req_id_header.clone(), MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
                        let req_id = req
                            .headers()
                            .get("req-id")
                            .expect("req-id header should be set by the trace layer")
                            .to_str()
                            .unwrap();

                        info_span!(
                            "request",
                            method = %req.method(),
                            uri = %req.uri(),
                            req_id = %req_id
                        )
                    }),
                )
                .propagate_request_id(req_id_header),
        )
}
