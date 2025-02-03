use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiRequestError {
    #[error("{0}")]
    ProcessError(String),
}
