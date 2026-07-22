use sea_orm_migration::prelude::*;

use crate::schema::concepts::Concepts;
use crate::schema::isomorphics::IsomorphicSets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        // ---------------------------------------------------------------------
        // Isomorphic Sets
        //
        // Groups multiple equivalent questions that assess the same Concept.
        //
        // The questions should differ in surface representation while measuring
        // the same underlying knowledge component.
        //
        // Example:
        //
        // Concept:
        // Newton's Second Law
        //
        // Set:
        // "Newton's Second Law - Form A"
        //
        // Questions:
        // - Box pushed across a floor
        // - Rocket acceleration
        // - Shopping cart
        // ---------------------------------------------------------------------

        let table = Table::create()
            .table(IsomorphicSets::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IsomorphicSets::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(ColumnDef::new(IsomorphicSets::ConceptId).uuid().not_null())
            .col(
                ColumnDef::new(IsomorphicSets::Title)
                    .string_len(255)
                    .not_null(),
            )
            .col(ColumnDef::new(IsomorphicSets::Description).text())
            .col(
                ColumnDef::new(IsomorphicSets::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(IsomorphicSets::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(IsomorphicSets::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_isomorphic_sets_concept")
                    .from(IsomorphicSets::Table, IsomorphicSets::ConceptId)
                    .to(Concepts::Table, Concepts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        println!("Creating table: {}", table.to_string(PostgresQueryBuilder));

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_isomorphic_sets_concept")
                    .table(IsomorphicSets::Table)
                    .col(IsomorphicSets::ConceptId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(IsomorphicSets::Table).to_owned())
            .await?;

        Ok(())
    }
}
