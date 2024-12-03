use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Session ended.")]
    SessionEnded,
    #[error("Authentication failed.")]
    AuthFailed,
    #[error("User account was not found.")]
    UserNotFound,
}
