use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum SoloLevels
{
    Table,
    Id,
    Code,
    Name,
    OrderIndex,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
