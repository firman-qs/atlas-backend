use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum UserRole
{
    #[sea_orm(iden = "user_role_enum")]
    Enum,
    Admin,
    Teacher,
    Student,
}
