use anyhow::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("resource not found")]
    NotFound,

    #[error("unique constraint violation")]
    UniqueConstraint { constraint: Option<String> },

    #[error("foreign key constraint violation")]
    ForeignKeyConstraint { constraint: Option<String> },

    #[error("database connection error")]
    Connection,

    #[error("database timeout")]
    Timeout,

    #[error("unexpected repository error")]
    Unexpected(#[source] Error),
}
