use axum::{response::Json, routing::get, extract::State, Router};
use sqlx::PgPool;
use tracing::instrument;

use crate::http::{error::Result, state::ApiState};
use super::db::{self, Stock};

pub const ROOT: &str = "/stocks";

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/", get(get_stocks))
}

#[instrument(skip_all, fields(action = "stocks::get_stocks route"))]
pub async fn get_stocks(State(pool): State<PgPool>) -> Result<Json<Vec<Stock>>> {
    Ok(Json(db::get_stocks(&pool).await?))
}