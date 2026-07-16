use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum QuestionPurpose {
    #[sea_orm(iden = "question_purpose_enum")]
    Enum,
    Assessment,
    Practice,
    Remediation,
    Reflection,
}
