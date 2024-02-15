use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Invalid argument")]
    InvalidArgument
}

pub type UtilsResult<T> = Result<T, UtilsError>;
