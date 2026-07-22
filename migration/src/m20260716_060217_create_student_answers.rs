use sea_orm_migration::prelude::*;

use crate::schema::assesment_attempts::AssessmentAttempts;
use crate::schema::question_options::QuestionOptions;
use crate::schema::questions::Questions;
use crate::schema::student_answers::StudentAnswers;

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
                    .table(StudentAnswers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StudentAnswers::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(
                        ColumnDef::new(StudentAnswers::AssessmentAttemptId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StudentAnswers::QuestionId).uuid().not_null())
                    .col(ColumnDef::new(StudentAnswers::AnswerText).text())
                    .col(ColumnDef::new(StudentAnswers::SelectedOptionId).uuid())
                    .col(ColumnDef::new(StudentAnswers::AnswerJson).json_binary())
                    .col(
                        ColumnDef::new(StudentAnswers::IsCorrect)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(StudentAnswers::Score)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .col(ColumnDef::new(StudentAnswers::Feedback).text())
                    .col(
                        ColumnDef::new(StudentAnswers::AnsweredAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(StudentAnswers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(StudentAnswers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_answers_attempt")
                            .from(StudentAnswers::Table, StudentAnswers::AssessmentAttemptId)
                            .to(AssessmentAttempts::Table, AssessmentAttempts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_answers_question")
                            .from(StudentAnswers::Table, StudentAnswers::QuestionId)
                            .to(Questions::Table, Questions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_answers_choice")
                            .from(StudentAnswers::Table, StudentAnswers::SelectedOptionId)
                            .to(QuestionOptions::Table, QuestionOptions::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_student_answers_question")
                    .table(StudentAnswers::Table)
                    .col(StudentAnswers::QuestionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_student_answers_attempt_question")
                    .table(StudentAnswers::Table)
                    .col(StudentAnswers::AssessmentAttemptId)
                    .col(StudentAnswers::QuestionId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(StudentAnswers::Table).to_owned())
            .await
    }
}
