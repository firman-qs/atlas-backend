use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::schema::concepts::Concepts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .alter_table(
                Table::alter()
                    .table(Concepts::Table)
                    .add_column(
                        ColumnDef::new(Concepts::DisplayOrder)
                            .integer()
                            .not_null()
                            .default(0),
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
                    .table(Concepts::Table)
                    .drop_column(Concepts::DisplayOrder)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
