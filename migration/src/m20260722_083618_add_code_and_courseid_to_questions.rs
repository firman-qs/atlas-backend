use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::m20260710_161513_create_courses_and_learning_objectives::Courses;
use crate::schema::questions::Questions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]

impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .alter_table(
                Table::alter()
                    .table(Questions::Table)
                    .add_column(ColumnDef::new(Questions::CourseId).uuid().not_null())
                    .add_column(ColumnDef::new(Questions::Code).string().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_questions_course")
                            .from_tbl(Questions::Table)
                            .from_col(Questions::CourseId)
                            .to_tbl(Courses::Table)
                            .to_col(Courses::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_questions_course_code")
                    .table(Questions::Table)
                    .col(Questions::CourseId)
                    .col(Questions::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_index(
                Index::drop()
                    .name("uq_questions_course_code")
                    .table(Questions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Questions::Table)
                    .drop_foreign_key(Alias::new("fk_questions_course"))
                    .drop_column(Questions::CourseId)
                    .drop_column(Questions::Code)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post
{
    Table,
    Id,
    Title,
    Text,
}
