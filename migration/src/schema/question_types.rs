use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum QuestionTypes
{
    Table,

    /// UUID
    ///
    /// Example:
    /// 0197a5c6-4d7d-71d8-9b3b-a91b92d9d3e8
    Id,

    /// Stable machine-readable identifier.
    ///
    /// Examples:
    /// MCQ
    /// TRUE_FALSE
    /// ESSAY
    /// PROGRAMMING
    /// SIMULATION
    Code,

    /// Human-readable display name.
    ///
    /// Examples:
    /// Multiple Choice
    /// True / False
    /// Essay
    /// Programming
    /// Simulation
    Name,

    /// Optional description shown to administrators.
    Description,

    /// Whether this type stores selectable options.
    ///
    /// Examples:
    /// true  -> MCQ
    /// true  -> TRUE_FALSE
    /// false -> ESSAY
    /// false -> SIMULATION
    SupportsOptions,

    /// Whether answers can be automatically graded.
    ///
    /// Examples:
    /// true  -> MCQ
    /// true  -> TRUE_FALSE
    /// false -> ESSAY
    /// true  -> PROGRAMMING
    SupportsAutograde,

    /// Soft delete / archive
    IsActive,

    CreatedAt,

    UpdatedAt,
}
