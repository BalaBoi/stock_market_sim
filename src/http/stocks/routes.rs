use axum::{Router, extract::State, response::Json, routing::get};
use sqlx::PgPool;
use tracing::instrument;

use super::db::{self, Stock};
use crate::http::{error::Result, state::ApiState};

pub const ROOT: &str = "/stocks";

pub fn router() -> Router<ApiState> {
    Router::new().route("/", get(get_stocks))
}

#[instrument(skip_all, fields(action = "stocks::get_stocks route"))]
pub async fn get_stocks(State(pool): State<PgPool>) -> Result<Json<Vec<Stock>>> {
    Ok(Json(db::get_stocks(&pool).await?))
}
