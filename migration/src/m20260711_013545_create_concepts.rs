use sea_orm_migration::prelude::*;

use crate::schema::{
    concepts::{ConceptPrerequisites, Concepts, LearningObjectiveConcepts},
    learning_objectives::LearningObjectives,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let concepts = Table::create()
            .table(Concepts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Concepts::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(
                ColumnDef::new(Concepts::Code)
                    .string_len(50)
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(Concepts::Name).string_len(100).not_null())
            .col(
                ColumnDef::new(Concepts::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(ColumnDef::new(Concepts::Description).text())
            .col(
                ColumnDef::new(Concepts::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Concepts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(concepts).await?;

        let learning_objective_concepts = Table::create()
            .table(LearningObjectiveConcepts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LearningObjectiveConcepts::LearningObjectiveId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LearningObjectiveConcepts::ConceptId)
                    .uuid()
                    .not_null(),
            )
            .primary_key(
                Index::create()
                    .col(LearningObjectiveConcepts::LearningObjectiveId)
                    .col(LearningObjectiveConcepts::ConceptId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_loc_learning_objective")
                    .from(
                        LearningObjectiveConcepts::Table,
                        LearningObjectiveConcepts::LearningObjectiveId,
                    )
                    .to(LearningObjectives::Table, LearningObjectives::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_loc_concept")
                    .from(
                        LearningObjectiveConcepts::Table,
                        LearningObjectiveConcepts::ConceptId,
                    )
                    .to(Concepts::Table, Concepts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(learning_objective_concepts).await?;

        let concept_prerequisites = Table::create()
            .table(ConceptPrerequisites::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ConceptPrerequisites::ConceptId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ConceptPrerequisites::PrerequisiteConceptId)
                    .uuid()
                    .not_null(),
            )
            .primary_key(
                Index::create()
                    .col(ConceptPrerequisites::ConceptId)
                    .col(ConceptPrerequisites::PrerequisiteConceptId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_concept_prequisite_concept")
                    .from(ConceptPrerequisites::Table, ConceptPrerequisites::ConceptId)
                    .to(Concepts::Table, Concepts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_concept_prerequisite_prerequisite_concept")
                    .from(
                        ConceptPrerequisites::Table,
                        ConceptPrerequisites::PrerequisiteConceptId,
                    )
                    .to(Concepts::Table, Concepts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(concept_prerequisites).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_concept_prequisite_prerequisite_concept")
                    .table(ConceptPrerequisites::Table)
                    .col(ConceptPrerequisites::PrerequisiteConceptId)
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
                    .table(LearningObjectiveConcepts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(ConceptPrerequisites::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Concepts::Table).to_owned())
            .await?;

        Ok(())
    }
}
