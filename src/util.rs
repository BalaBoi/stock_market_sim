use std::fmt::Debug;

use tracing::event;

pub trait ResultExt {
    fn log_err(self, level: tracing::Level, msg: &str) -> Self;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: Debug,
{
    fn log_err(self, level: tracing::Level, msg: &str) -> Self {
        if let Err(e) = self {
            use tracing::Level;
            match level {
                Level::TRACE => tracing::trace!(error = ?e, msg),
                Level::DEBUG => tracing::debug!(error = ?e, msg),
                Level::INFO => tracing::info!(error = ?e, msg),
                Level::WARN => tracing::warn!(error = ?e, msg),
                Level::ERROR => tracing::error!(error = ?e, msg),
            }
            return Err(e);
        }
        self
    }
}

pub type RedisPool = deadpool_redis::Pool;
