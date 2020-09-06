use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot find binary {0}. Are you using the correct alias?")]
    BinaryNotFoundError(String),
    #[error("Cannot find version {1} of binary {0}. Have you added this version?")]
    BinaryVersionNotFoundError(String, String),
    #[error("Something went wrong.")]
    GenericError,
}
