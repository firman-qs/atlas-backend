use sea_orm_migration::prelude::*;

use crate::schema::question_types::QuestionTypes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---------------------------------------------------------------------
        // Question Types
        //
        // Defines how a question is presented and graded.
        //
        // This is intentionally a lookup table instead of a PostgreSQL enum
        // because new question types may be introduced in the future without
        // requiring a database migration.
        //
        // Examples:
        //
        // MCQ
        // TRUE_FALSE
        // ESSAY
        // PROGRAMMING
        // SIMULATION
        // ---------------------------------------------------------------------

        let table = Table::create()
            .table(QuestionTypes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(QuestionTypes::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(
                ColumnDef::new(QuestionTypes::Code)
                    .string_len(30)
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionTypes::Name)
                    .string_len(100)
                    .not_null(),
            )
            .col(ColumnDef::new(QuestionTypes::Description).text())
            .col(
                ColumnDef::new(QuestionTypes::SupportsOptions)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(QuestionTypes::SupportsAutograde)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(QuestionTypes::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(QuestionTypes::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(QuestionTypes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        println!("Creating table: {}", table.to_string(PostgresQueryBuilder));

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_question_types_code")
                    .table(QuestionTypes::Table)
                    .col(QuestionTypes::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuestionTypes::Table).to_owned())
            .await?;

        Ok(())
    }
}
