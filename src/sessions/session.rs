use std::collections::HashMap;

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Serialize, de::DeserializeOwned};

use super::{error::SessionError, store::SessionId};

pub struct Session {
    id: SessionId,
    data: HashMap<String, String>,
}

impl Session {
    pub fn new(id: SessionId, data: HashMap<String, String>) -> Self {
        Self { id, data }
    }

    pub async fn set(&mut self, key: &str, value: impl Serialize) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<T>> {
        todo!()
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = SessionError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}
