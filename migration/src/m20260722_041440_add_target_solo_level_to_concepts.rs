use sea_orm_migration::prelude::*;

use crate::schema::concepts::Concepts;
use crate::schema::solo_levels::SoloLevels;

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
                        ColumnDef::new(Concepts::TargetSoloLevelId)
                            .uuid()
                            .not_null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_concepts_target_solo_level")
                            .from_tbl(Concepts::Table)
                            .from_col(Concepts::TargetSoloLevelId)
                            .to_tbl(SoloLevels::Table)
                            .to_col(SoloLevels::Id)
                            .on_delete(ForeignKeyAction::Restrict),
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
                    .drop_foreign_key(Alias::new("fk_concepts_target_solo_level"))
                    .drop_column(Concepts::TargetSoloLevelId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
