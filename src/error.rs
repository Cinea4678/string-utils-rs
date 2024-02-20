use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

pub type UtilsResult<T> = Result<T, UtilsError>;
