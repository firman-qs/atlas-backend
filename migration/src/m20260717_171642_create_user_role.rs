use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use crate::schema::pg_enum::user_role_enum::UserRole;
use crate::schema::users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserRole::Enum)
                    .values([UserRole::Admin, UserRole::Teacher, UserRole::Student])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::Role)
                            .enumeration(
                                UserRole::Enum,
                                [UserRole::Admin, UserRole::Teacher, UserRole::Student],
                            )
                            .not_null()
                            .default(UserRole::Student.to_string()),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Role)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(UserRole::Enum).to_owned())
            .await?;

        Ok(())
    }
}
