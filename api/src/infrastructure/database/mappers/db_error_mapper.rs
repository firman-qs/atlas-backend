use anyhow::anyhow;
use sea_orm::DbErr;
use sea_orm::RuntimeErr;
use sea_orm::SqlErr;

use crate::domain::errors::repository_error::RepositoryError;

impl From<DbErr> for RepositoryError {
    fn from(err: DbErr) -> Self {
        map_db_error(err)
    }
}

pub fn map_db_error(err: DbErr) -> RepositoryError {
    if let Some(sql_err) = err.sql_err() {
        return match sql_err {
            SqlErr::UniqueConstraintViolation(_) => {
                let constraint = extract_constraint_name(&err);
                RepositoryError::UniqueConstraint { constraint }
            }
            SqlErr::ForeignKeyConstraintViolation(_) => {
                let constraint = extract_constraint_name(&err);
                RepositoryError::ForeignKeyConstraint { constraint }
            }
            _ => RepositoryError::Unexpected(anyhow!(err)),
        };
    }

    match err {
        DbErr::RecordNotFound(_) => RepositoryError::NotFound,
        DbErr::ConnectionAcquire(_) | DbErr::Conn(_) => RepositoryError::Connection,
        DbErr::Exec(_) | DbErr::Query(_) => RepositoryError::Unexpected(anyhow!(err)),
        _ => RepositoryError::Unexpected(anyhow!(err)),
    }
}

fn extract_constraint_name(err: &DbErr) -> Option<String> {
    match err {
        DbErr::Query(RuntimeErr::SqlxError(sea_orm::sqlx::Error::Database(db)))
        | DbErr::Exec(RuntimeErr::SqlxError(sea_orm::sqlx::Error::Database(db))) => {
            db.constraint().map(ToOwned::to_owned)
        }

        _ => None,
    }
}
