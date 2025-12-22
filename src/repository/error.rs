use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub(crate) enum DatabaseError {
    #[error("entry with id {0} not found")]
    NotFound(u32),

    #[error("entry with id {0} already exists")]
    AlreadyExists(u32),

    #[error("someone else is currently using the entry with id {0}")]
    WriteBlocked(u32),
}