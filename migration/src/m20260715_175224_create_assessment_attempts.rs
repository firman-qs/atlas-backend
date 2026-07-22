use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;

use crate::schema::assesment_attempts::AssessmentAttempts;
use crate::schema::concepts::Concepts;
use crate::schema::pg_enum::attempt_status_enum::AttemptStatus;
use crate::schema::solo_levels::SoloLevels;
use crate::schema::users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        // ---------------------------------------------------------------------
        // Attempt Status Enum
        // ---------------------------------------------------------------------
        manager
            .create_type(
                Type::create()
                    .as_enum(AttemptStatus::Enum)
                    .values([
                        AttemptStatus::InProgress,
                        AttemptStatus::Completed,
                        AttemptStatus::Abandoned,
                    ])
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Assessment Attempts Table
        // ---------------------------------------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(AssessmentAttempts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssessmentAttempts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::StudentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::ConceptId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AssessmentAttempts::CompletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(AssessmentAttempts::Status)
                            .enumeration(
                                AttemptStatus::Enum,
                                [
                                    AttemptStatus::InProgress,
                                    AttemptStatus::Completed,
                                    AttemptStatus::Abandoned,
                                ],
                            )
                            .not_null()
                            .default(AttemptStatus::InProgress.to_string()),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::CurrentSoloLevelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::TargetSoloLevelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::IsMastered)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AssessmentAttempts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assessment_attempts_student")
                            .from(AssessmentAttempts::Table, AssessmentAttempts::StudentId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assessment_attempts_concept")
                            .from(AssessmentAttempts::Table, AssessmentAttempts::ConceptId)
                            .to(Concepts::Table, Concepts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assessment_attempts_current_solo")
                            .from(
                                AssessmentAttempts::Table,
                                AssessmentAttempts::CurrentSoloLevelId,
                            )
                            .to(SoloLevels::Table, SoloLevels::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_assessment_attempts_target_solo")
                            .from(
                                AssessmentAttempts::Table,
                                AssessmentAttempts::TargetSoloLevelId,
                            )
                            .to(SoloLevels::Table, SoloLevels::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_assessment_attempts_concept")
                    .table(AssessmentAttempts::Table)
                    .col(AssessmentAttempts::ConceptId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_assessment_attempts_status")
                    .table(AssessmentAttempts::Table)
                    .col(AssessmentAttempts::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_assessment_attempts_student_concept")
                    .table(AssessmentAttempts::Table)
                    .col(AssessmentAttempts::StudentId)
                    .col(AssessmentAttempts::ConceptId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(AssessmentAttempts::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AttemptStatus::Enum).to_owned())
            .await?;

        Ok(())
    }
}
