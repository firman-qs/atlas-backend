use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum AttemptStatus
{
    #[sea_orm(iden = "attempt_status_enum")]
    Enum,
    InProgress,
    Completed,
    Abandoned,
}
