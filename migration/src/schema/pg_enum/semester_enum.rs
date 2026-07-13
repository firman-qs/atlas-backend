use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum SemesterEnum {
    #[sea_orm(iden = "semester_enum")]
    Enum,
    Odd,
    Even,
    Ganjil,
    Genap,
    Antara,
    Spring,
    Fall,
    Summer,
}
