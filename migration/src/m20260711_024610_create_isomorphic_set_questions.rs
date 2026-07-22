use sea_orm_migration::prelude::*;

use crate::schema::isomorphics::IsomorphicSetQuestions;
use crate::schema::isomorphics::IsomorphicSets;
use crate::schema::questions::Questions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        // ---------------------------------------------------------------------
        // Isomorphic Set Questions
        //
        // Associates Questions with an Isomorphic Set.
        //
        // Every Question may belong to at most one Isomorphic Set.
        //
        // Example:
        //
        // Isomorphic Set:
        // Newton's Second Law
        //
        // Questions:
        // 1. Box on a table
        // 2. Shopping cart
        // 3. Rocket launch
        // ---------------------------------------------------------------------

        let table = Table::create()
            .table(IsomorphicSetQuestions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IsomorphicSetQuestions::IsomorphicSetId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IsomorphicSetQuestions::QuestionId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IsomorphicSetQuestions::DisplayOrder)
                    .integer()
                    .not_null()
                    .default(1),
            )
            .primary_key(
                Index::create()
                    .col(IsomorphicSetQuestions::IsomorphicSetId)
                    .col(IsomorphicSetQuestions::QuestionId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_isomorphic_set_questions_set")
                    .from(
                        IsomorphicSetQuestions::Table,
                        IsomorphicSetQuestions::IsomorphicSetId,
                    )
                    .to(IsomorphicSets::Table, IsomorphicSets::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_isomorphic_set_questions_question")
                    .from(
                        IsomorphicSetQuestions::Table,
                        IsomorphicSetQuestions::QuestionId,
                    )
                    .to(Questions::Table, Questions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        println!("Creating table: {}", table.to_string(PostgresQueryBuilder));

        manager.create_table(table).await?;

        // A Question may belong to only one Isomorphic Set.
        manager
            .create_index(
                Index::create()
                    .name("uq_isomorphic_set_questions_question")
                    .table(IsomorphicSetQuestions::Table)
                    .col(IsomorphicSetQuestions::QuestionId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Preserve ordering inside the set.
        manager
            .create_index(
                Index::create()
                    .name("uq_isomorphic_set_questions_order")
                    .table(IsomorphicSetQuestions::Table)
                    .col(IsomorphicSetQuestions::IsomorphicSetId)
                    .col(IsomorphicSetQuestions::DisplayOrder)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(
                Table::drop()
                    .table(IsomorphicSetQuestions::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
