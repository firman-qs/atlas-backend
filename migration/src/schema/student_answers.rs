use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum StudentAnswers
{
    Table,
    Id,
    AssessmentAttemptId,
    QuestionId,
    AnswerText,
    SelectedOptionId,
    AnswerJson,
    IsCorrect,
    Score,
    Feedback,
    AnsweredAt,
    CreatedAt,
    UpdatedAt,
}
