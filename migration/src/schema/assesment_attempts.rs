use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum AssessmentAttempts
{
    Table,
    Id,
    StudentId,
    ConceptId,
    StartedAt,
    CompletedAt,
    Status,
    CurrentSoloLevelId,
    TargetSoloLevelId,
    IsMastered,
    CreatedAt,
    UpdatedAt,
}
