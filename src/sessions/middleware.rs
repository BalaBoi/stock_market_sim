use axum::{extract::Request, middleware::Next, response::Response};

pub async fn session_middleware(request: Request, next: Next) -> Response {
    todo!()
}