use thiserror::Error;


/// The error type for SDTP requests
#[derive(Error, Debug, Clone)]
pub enum SMDTPRequestError {
    #[error("There are too many bytes within the payload. Payload may not exceed 16 bytes.")]
    TooManyBytes,
    #[error("Invalid authentication credentials were provided.")]
    Forbidden,
    #[error("An unknown byte was provided.")]
    UnmatchedByte,
    #[error("An internal server error has occured, please try again later.")]
    InternalServerError,
}


/// The error type for SDTP responses
#[derive(Error, Debug, Clone)]
pub enum SDTPResponseError {
    #[error("Invalid data was provided.")]
    BadData
}