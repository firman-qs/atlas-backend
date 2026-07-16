use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Concepts {
    Table,
    Id,
    Code,
    Name,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum LearningObjectiveConcepts {
    Table,
    LearningObjectiveId,
    ConceptId,
    DisplayOrder,
}

// #[derive(DeriveIden)]
// pub enum ConceptPrerequisites {
//     Table,
//     ConceptId,
//     PrerequisiteConceptId,
// }
