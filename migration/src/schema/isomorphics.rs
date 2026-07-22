use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum IsomorphicSets
{
    Table,

    /// UUID
    ///
    /// Example:
    /// 0197a5c6-4d7d-71d8-9b3b-a91b92d9d3e8
    Id,

    /// FK -> Concepts.Id
    ConceptId,

    /// Human-readable set name.
    ///
    /// Examples:
    /// Newton's Second Law
    /// Ownership Variants
    /// Binary Search Scenarios
    Title,

    /// Optional explanation for instructors.
    ///
    /// Example:
    /// Equivalent questions with different real-world contexts.
    Description,

    /// Soft delete / archive
    IsActive,

    CreatedAt,

    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum IsomorphicSetQuestions
{
    Table,

    /// FK -> IsomorphicSets.Id
    IsomorphicSetId,

    /// FK -> Questions.Id
    QuestionId,

    /// Position of the Question within the Isomorphic Set.
    ///
    /// Examples:
    /// 1
    /// 2
    /// 3
    DisplayOrder,
}
