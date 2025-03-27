use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;
use tracing::{Level, info};

use crate::{
    RedisPool,
    sessions::store::{SessionId, Store},
    util::ResultExt,
};

pub async fn session_middleware(
    State(redis_pool): State<RedisPool>,
    request: Request,
    next: Next,
) -> Response {
    let session_id = get_session_id_from_cookie(&request, "session-id");

    let store = Store::new(redis_pool);

    let session = match session_id {
        Some(id) => store.load(id).await.unwrap(),
        None => todo!(),
    };
    todo!()
}

fn get_session_id_from_cookie(request: &Request, cookie_name: &str) -> Option<SessionId> {
    let raw_cookies = request
        .headers()
        .get("Cookie")?
        .to_str()
        .log_err(Level::DEBUG, "cookie contained invalid data")
        .ok()?;

    let session_cookie = raw_cookies.split(";").find_map(|raw| {
        let cookie = Cookie::parse_encoded(raw)
            .log_err(Level::DEBUG, "cookie contained invalid data")
            .ok()?;
        if cookie.name() == cookie_name {
            Some(cookie)
        } else {
            None
        }
    })?;

    session_cookie
        .value()
        .parse::<SessionId>()
        .log_err(Level::WARN, "invalid session id")
        .ok()
}
