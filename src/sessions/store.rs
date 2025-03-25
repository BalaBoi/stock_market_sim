use uuid::Uuid;

use super::session::Session;

#[derive(Debug, Clone, Copy)]
pub struct SessionId(Uuid);

pub struct Store {}

impl Store {
    pub async fn save(&self, session: &Session) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn load(&self, session_id: SessionId) -> anyhow::Result<Session> {
        todo!()
    }

    pub async fn delete(&self, session_id: SessionId) -> anyhow::Result<()> {
        todo!()
    }
}