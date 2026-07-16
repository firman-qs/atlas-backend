use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum QuestionConcepts {
    Table,
    QuestionId,
    ConceptId,
    SoloLevelId,
    IsPrimary,
    Purpose,
    DisplayOrder,
}
