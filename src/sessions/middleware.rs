use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::response::IntoResponse;
use axum::{
    extract::Request,
    response::Response,
    http::StatusCode,
};
use cookie::Cookie;
use tower::{Layer, Service};
use tracing::Level;

use crate::sessions::Session;
use crate::{
    sessions::store::{SessionId, Store},
    util::ResultExt,
};

#[derive(Clone)]
pub struct SessionLayer {
    session_store: Arc<Store>,
}

impl SessionLayer {
    pub fn new(session_store: Arc<Store>) -> Self {
        Self { session_store }
    }
}

impl<S> Layer<S> for SessionLayer {
    type Service = SessionMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionMiddleware {
            inner,
            session_store: self.session_store.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionMiddleware<S> {
    inner: S,
    session_store: Arc<Store>,
}

impl<S> SessionMiddleware<S> {
    fn new(inner: S, session_store: Arc<Store>) -> Self {
        SessionMiddleware {
            inner,
            session_store,
        }
    }
}

impl<S> Service<Request> for SessionMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + 'static + Send,
    S::Future: Send,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let mut this = self.clone();
        std::mem::swap(&mut this, self);
        Box::pin(async move {
            let session_id = get_session_id_from_cookie(&req, "session-id");
            let session = match session_id {
                Some(id) => {
                    match this.session_store.load(id).await {
                        Ok(session) => session,
                        Err(_error) => {
                            return Ok(StatusCode::UNAUTHORIZED.into_response());
                        }
                    }
                },
                None => {
                    Session::new(SessionId::new(), HashMap::new())
                },
            };
            req.extensions_mut().insert(session);
            this.inner.call(req).await
        })
    }
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
