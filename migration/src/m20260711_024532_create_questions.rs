use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::schema::{
    concepts::Concepts, pg_enum::solo_level_enum::SoloLevelEnum, question_types::QuestionTypes,
    questions::Questions, users::Users,
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
            .col(ColumnDef::new(Questions::ConceptId).uuid().not_null())
            .col(ColumnDef::new(Questions::QuestionTypeId).uuid().not_null())
            .col(ColumnDef::new(Questions::CreatedBy).uuid().not_null())
            .col(ColumnDef::new(Questions::Title).string_len(255).not_null())
            .col(ColumnDef::new(Questions::QuestionText).text().not_null())
            .col(
                ColumnDef::new(Questions::SoloLevel)
                    .enumeration(
                        SoloLevelEnum::Enum,
                        [
                            SoloLevelEnum::Prestructural,
                            SoloLevelEnum::Unistructural,
                            SoloLevelEnum::Multistructural,
                            SoloLevelEnum::Relational,
                            SoloLevelEnum::ExtendedAbstract,
                        ],
                    )
                    .not_null(),
            )
            .col(
                ColumnDef::new(Questions::Difficulty)
                    .decimal_len(4, 2)
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new(Questions::EstimatedTime)
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
                    .name("fk_questions_concept")
                    .from(Questions::Table, Questions::ConceptId)
                    .to(Concepts::Table, Concepts::Id)
                    .on_delete(ForeignKeyAction::Restrict),
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

        println!("Creating table: {}", table.to_string(PostgresQueryBuilder));

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_questions_concept")
                    .table(Questions::Table)
                    .col(Questions::ConceptId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_questions_created_by")
                    .table(Questions::Table)
                    .col(Questions::CreatedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_questions_concept_solo")
                    .table(Questions::Table)
                    .col(Questions::ConceptId)
                    .col(Questions::SoloLevel)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Questions::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(SoloLevelEnum::Enum)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
