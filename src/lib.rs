use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Binary '{0}' is not registered with evm.")]
    BinaryNotFoundError(String),
    #[error(
        "'{0} {1}' is not registered with evm. Check which versions are registered with: evm list {0}"
    )]
    BinaryVersionNotFoundError(String, String),
    #[error("Cannot delete the active version of a binary. Please switch to another version and try again.")]
    DeleteActiveBinaryError,
    #[error("Something went wrong.")]
    GenericError,
}
