use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    Username,
    PasswordHash,
    FullName,
    AvatarUrl,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
