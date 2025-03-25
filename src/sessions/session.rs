use serde::{de::DeserializeOwned, Serialize};

pub struct Session {}

impl Session {
    pub async fn set(&mut self, key: &str, value: impl Serialize) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<T>> {
        todo!()
    }
}