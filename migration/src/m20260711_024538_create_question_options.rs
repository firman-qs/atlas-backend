use sea_orm_migration::prelude::*;

use crate::schema::question_options::QuestionOptions;
use crate::schema::questions::Questions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        // ---------------------------------------------------------------------
        // Question Options
        //
        // Represents one selectable answer belonging to a Question.
        //
        // Used by:
        //
        // - Multiple Choice
        // - True / False
        //
        // Not used by:
        //
        // - Essay
        // - Programming
        // - Simulation
        // ---------------------------------------------------------------------

        let table = Table::create()
            .table(QuestionOptions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(QuestionOptions::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(
                ColumnDef::new(QuestionOptions::QuestionId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionOptions::OptionText)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionOptions::IsCorrect)
                    .boolean()
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionOptions::DisplayOrder)
                    .integer()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_question_options_question")
                    .from(QuestionOptions::Table, QuestionOptions::QuestionId)
                    .to(Questions::Table, Questions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        println!("Creating table: {}", table.to_string(PostgresQueryBuilder));

        manager.create_table(table).await?;

        // Retrieve options ordered by display order
        manager
            .create_index(
                Index::create()
                    .name("idx_question_options_question_order")
                    .table(QuestionOptions::Table)
                    .col(QuestionOptions::QuestionId)
                    .col(QuestionOptions::DisplayOrder)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(QuestionOptions::Table).to_owned())
            .await?;

        Ok(())
    }
}
