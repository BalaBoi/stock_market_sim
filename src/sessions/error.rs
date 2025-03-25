use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct SessionError {

}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "session error")
    }
}