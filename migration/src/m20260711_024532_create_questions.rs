use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::schema::{
    pg_enum::{question_purpose::QuestionPurpose, solo_level_enum::SoloLevelEnum},
    question_concepts::QuestionConcepts,
    question_types::QuestionTypes,
    questions::Questions,
    users::Users,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---------------------------------------------------------------------
        // PostgreSQL Enum
        //
        // SOLO Taxonomy
        //
        // Prestructural
        // Unistructural
        // Multistructural
        // Relational
        // Extended Abstract
        // ---------------------------------------------------------------------

        manager
            .create_type(
                Type::create()
                    .as_enum(SoloLevelEnum::Enum)
                    .values([
                        SoloLevelEnum::Prestructural,
                        SoloLevelEnum::Unistructural,
                        SoloLevelEnum::Multistructural,
                        SoloLevelEnum::Relational,
                        SoloLevelEnum::ExtendedAbstract,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(QuestionPurpose::Enum)
                    .values([
                        QuestionPurpose::Assessment,
                        QuestionPurpose::Practice,
                        QuestionPurpose::Remediation,
                        QuestionPurpose::Reflection,
                    ])
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Questions
        //
        // Represents one assessment item.
        //
        // Each question belongs to one primary Concept.
        //
        // Examples:
        //
        // "What is ownership?"
        // "Which statement is correct?"
        // "Implement Binary Search."
        // ---------------------------------------------------------------------

        let table = Table::create()
            .table(Questions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Questions::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(ColumnDef::new(Questions::QuestionTypeId).uuid().not_null())
            .col(ColumnDef::new(Questions::CreatedBy).uuid().not_null())
            .col(ColumnDef::new(Questions::Title).string_len(255).not_null())
            .col(ColumnDef::new(Questions::QuestionText).text().not_null())
            .col(
                ColumnDef::new(Questions::EstimatedMinutes)
                    .integer()
                    .not_null()
                    .default(60),
            )
            .col(ColumnDef::new(Questions::ReferenceExplanation).text())
            .col(ColumnDef::new(Questions::Feedback).text())
            .col(
                ColumnDef::new(Questions::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Questions::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Questions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_questions_question_type")
                    .from(Questions::Table, Questions::QuestionTypeId)
                    .to(QuestionTypes::Table, QuestionTypes::Id)
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_questions_created_by")
                    .from(Questions::Table, Questions::CreatedBy)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .to_owned();

        manager.create_table(table).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_questions_created_by")
                    .table(Questions::Table)
                    .col(Questions::CreatedBy)
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Questions Concepts
        // ---------------------------------------------------------------------
        let question_concepts = Table::create()
            .table(QuestionConcepts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(QuestionConcepts::QuestionId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionConcepts::ConceptId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionConcepts::SoloLevelId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionConcepts::Purpose)
                    .enumeration(
                        QuestionPurpose::Enum,
                        [
                            QuestionPurpose::Assessment,
                            QuestionPurpose::Practice,
                            QuestionPurpose::Remediation,
                            QuestionPurpose::Reflection,
                        ],
                    )
                    .not_null(),
            )
            .col(
                ColumnDef::new(QuestionConcepts::IsPrimary)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(QuestionConcepts::DisplayOrder)
                    .integer()
                    .not_null()
                    .default(0),
            )
            .primary_key(
                Index::create()
                    .col(QuestionConcepts::QuestionId)
                    .col(QuestionConcepts::ConceptId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_question_concepts_question")
                    .from(QuestionConcepts::Table, QuestionConcepts::QuestionId)
                    .to(Questions::Table, Questions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_question_concepts_concept")
                    .from(QuestionConcepts::Table, QuestionConcepts::ConceptId)
                    .to(
                        crate::schema::concepts::Concepts::Table,
                        crate::schema::concepts::Concepts::Id,
                    )
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_question_concepts_solo_level")
                    .from(QuestionConcepts::Table, QuestionConcepts::SoloLevelId)
                    .to(
                        crate::schema::solo_levels::SoloLevels::Table,
                        crate::schema::solo_levels::SoloLevels::Id,
                    )
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .to_owned();

        manager.create_table(question_concepts).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_question_concepts_concept_question")
                    .table(QuestionConcepts::Table)
                    .col(QuestionConcepts::ConceptId)
                    .col(QuestionConcepts::QuestionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(QuestionConcepts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Questions::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(SoloLevelEnum::Enum)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(QuestionPurpose::Enum)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
