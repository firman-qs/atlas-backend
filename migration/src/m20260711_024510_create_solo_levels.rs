use sea_orm_migration::prelude::*;
use sea_query::Expr;
use sea_query::Query;

use crate::schema::solo_levels::SoloLevels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .create_table(
                Table::create()
                    .table(SoloLevels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SoloLevels::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(
                        ColumnDef::new(SoloLevels::Code)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(SoloLevels::Name).string_len(100).not_null())
                    .col(
                        ColumnDef::new(SoloLevels::OrderIndex)
                            .small_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(SoloLevels::Description).text())
                    .col(
                        ColumnDef::new(SoloLevels::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(SoloLevels::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SoloLevels::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(SoloLevels::Table)
                    .columns([
                        SoloLevels::Code,
                        SoloLevels::Name,
                        SoloLevels::OrderIndex,
                        SoloLevels::Description,
                    ])
                    .values_panic([
                        "PRESTRUCTURAL".into(),
                        "Prestructural".into(),
                        1.into(),
                        "The learner shows little or no understanding of the concept.".into(),
                    ])
                    .values_panic([
                        "UNISTRUCTURAL".into(),
                        "Unistructural".into(),
                        2.into(),
                        "The learner understands one relevant aspect of the concept.".into(),
                    ])
                    .values_panic([
                        "MULTISTRUCTURAL".into(),
                        "Multistructural".into(),
                        3.into(),
                        "The learner understands several relevant aspects but treats them \
                         independently."
                            .into(),
                    ])
                    .values_panic([
                        "RELATIONAL".into(),
                        "Relational".into(),
                        4.into(),
                        "The learner integrates multiple aspects into a coherent understanding."
                            .into(),
                    ])
                    .values_panic([
                        "EXTENDED_ABSTRACT".into(),
                        "Extended Abstract".into(),
                        5.into(),
                        "The learner generalizes, theorizes, or transfers understanding to new \
                         situations."
                            .into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(SoloLevels::Table).to_owned())
            .await
    }
}
