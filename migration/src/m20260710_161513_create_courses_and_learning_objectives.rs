use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::schema::{
    learning_objectives::LearningObjectives, pg_enum::semester_enum::SemesterEnum, users::Users,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---------------------------------------------------------------------
        // Courses
        //
        // Represents the academic subject itself.
        //
        // Example:
        // Code  : CS101
        // Title : Introduction to Programming
        // ---------------------------------------------------------------------
        let course_table = Table::create()
            .table(Courses::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Courses::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(
                ColumnDef::new(Courses::Code)
                    .string_len(30)
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(Courses::Title).string_len(255).not_null())
            .col(ColumnDef::new(Courses::Description).text())
            .col(
                ColumnDef::new(Courses::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Courses::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Courses::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(course_table).await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_courses_code")
                    .table(Courses::Table)
                    .col(Courses::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Semester Enum
        // ---------------------------------------------------------------------
        manager
            .create_type(
                Type::create()
                    .as_enum(SemesterEnum::Enum)
                    .values([
                        SemesterEnum::Odd,
                        SemesterEnum::Even,
                        SemesterEnum::Ganjil,
                        SemesterEnum::Genap,
                        SemesterEnum::Antara,
                        SemesterEnum::Spring,
                        SemesterEnum::Fall,
                        SemesterEnum::Summer,
                    ])
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Academic Terms
        //
        // Represents a specific academic term.
        //
        // Example:
        // Year     : 2026
        // Semester : Ganjil
        // Displayed by the application as:
        // 2026-Ganjil
        // ---------------------------------------------------------------------
        let academic_terms_table = Table::create()
            .table(AcademicTerms::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AcademicTerms::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(
                ColumnDef::new(AcademicTerms::Year)
                    .small_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AcademicTerms::Semester)
                    .enumeration(
                        SemesterEnum::Enum,
                        [
                            SemesterEnum::Odd,
                            SemesterEnum::Even,
                            SemesterEnum::Ganjil,
                            SemesterEnum::Genap,
                            SemesterEnum::Antara,
                            SemesterEnum::Spring,
                            SemesterEnum::Fall,
                            SemesterEnum::Summer,
                        ],
                    )
                    .not_null(),
            )
            .col(ColumnDef::new(AcademicTerms::StartsAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AcademicTerms::EndsAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AcademicTerms::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(AcademicTerms::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AcademicTerms::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(academic_terms_table).await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_academic_terms_year_semester")
                    .table(AcademicTerms::Table)
                    .col(AcademicTerms::Year)
                    .col(AcademicTerms::Semester)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Course Offerings
        //
        // Represents one teaching instance of a course.
        //
        // Example:
        //
        // Course  : CS101
        // Term    : 2026-Ganjil
        // Section : A
        //
        // Displayed by the application as:
        //
        // CS101 2026-Ganjil Offering-A
        // ---------------------------------------------------------------------
        let course_offering_table = Table::create()
            .table(CourseOfferings::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CourseOfferings::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(ColumnDef::new(CourseOfferings::CourseId).uuid().not_null())
            .col(
                ColumnDef::new(CourseOfferings::AcademicTermId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CourseOfferings::Section)
                    .string_len(5)
                    .not_null(),
            )
            .col(ColumnDef::new(CourseOfferings::LecturerId).uuid())
            .col(
                ColumnDef::new(CourseOfferings::Capacity)
                    .integer()
                    .not_null()
                    .default(30),
            )
            .col(ColumnDef::new(CourseOfferings::StartsAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CourseOfferings::EndsAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(CourseOfferings::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(CourseOfferings::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CourseOfferings::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_course_offerings_course")
                    .from(CourseOfferings::Table, CourseOfferings::CourseId)
                    .to(Courses::Table, Courses::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_course_lecturer")
                    .from(CourseOfferings::Table, CourseOfferings::LecturerId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_course_offerings_academic_term")
                    .from(CourseOfferings::Table, CourseOfferings::AcademicTermId)
                    .to(AcademicTerms::Table, AcademicTerms::Id)
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .to_owned();

        manager.create_table(course_offering_table).await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_course_offering")
                    .table(CourseOfferings::Table)
                    .col(CourseOfferings::CourseId)
                    .col(CourseOfferings::AcademicTermId)
                    .col(CourseOfferings::Section)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_course_offerings_term")
                    .table(CourseOfferings::Table)
                    .col(CourseOfferings::AcademicTermId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_course_offerings_lecturer")
                    .table(CourseOfferings::Table)
                    .col(CourseOfferings::LecturerId)
                    .to_owned(),
            )
            .await?;

        // ---------------------------------------------------------------------
        // Learning Objectives
        //
        // Example:
        //
        // LO-1  Understand ownership
        // LO-2  Write async Rust
        //
        // They belong to the Course,
        // NOT to individual Course Offerings.
        // ---------------------------------------------------------------------
        let learning_objectives_table = Table::create()
            .table(LearningObjectives::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LearningObjectives::Id)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .default(Expr::cust("uuidv7()")),
            )
            .col(
                ColumnDef::new(LearningObjectives::CourseId)
                    .uuid()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LearningObjectives::Code)
                    .string_len(30)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LearningObjectives::Title)
                    .string_len(255)
                    .not_null(),
            )
            .col(ColumnDef::new(LearningObjectives::Description).text())
            .col(
                ColumnDef::new(LearningObjectives::DisplayOrder)
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new(LearningObjectives::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(LearningObjectives::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LearningObjectives::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_learning_objectives_course")
                    .from(LearningObjectives::Table, LearningObjectives::CourseId)
                    .to(Courses::Table, Courses::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(learning_objectives_table).await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_learning_objective_course_code")
                    .table(LearningObjectives::Table)
                    .col(LearningObjectives::CourseId)
                    .col(LearningObjectives::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_learning_objectives_order")
                    .table(LearningObjectives::Table)
                    .col(LearningObjectives::CourseId)
                    .col(LearningObjectives::DisplayOrder)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LearningObjectives::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CourseOfferings::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AcademicTerms::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Courses::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(SemesterEnum::Enum).to_owned())
            .await?;

        Ok(())
    }
}

//
// -----------------------------------------------------------------------------
// Table identifiers
// -----------------------------------------------------------------------------

#[derive(DeriveIden)]
enum Courses {
    Table,

    /// UUID
    /// Example:
    /// 0197a5c6-4d7d-71d8-9b3b-a91b92d9d3e8
    Id,

    /// Example:
    /// CS101
    /// IF2050
    /// MATH101
    Code,

    /// Example:
    /// Introduction to Programming
    Title,

    /// Long course description
    Description,

    /// Soft delete / archive
    IsActive,

    CreatedAt,

    UpdatedAt,
}

#[derive(DeriveIden)]
enum CourseOfferings {
    Table,

    /// UUID
    Id,

    /// FK -> Courses.Id
    CourseId,

    /// Academic term
    ///
    /// Examples:
    /// 2026-Ganjil
    /// 2026-Genap
    /// 2027-Pendek
    AcademicTermId,

    /// Student group / class
    ///
    /// Examples:
    /// A
    /// B
    /// C
    Section,

    /// FK -> Users.Id (nullable until assigned)
    LecturerId,

    /// Maximum enrolled students
    ///
    /// Example:
    /// 30
    Capacity,

    /// First lecture date/time
    StartsAt,

    /// Last lecture date/time
    EndsAt,

    IsActive,

    CreatedAt,

    UpdatedAt,
}

#[derive(DeriveIden)]
enum AcademicTerms {
    Table,

    Id,

    /// Example: 2026
    Year,

    /// Examples:
    /// Ganjil
    /// Genap
    /// Pendek
    Semester,

    StartsAt,

    EndsAt,

    IsActive,

    CreatedAt,

    UpdatedAt,
}
