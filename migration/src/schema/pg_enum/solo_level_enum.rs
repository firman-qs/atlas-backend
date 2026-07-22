use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum SoloLevelEnum
{
    #[sea_orm(iden = "solo_level_enum")]
    Enum,

    /// Student misses the point entirely.
    Prestructural,

    /// One relevant aspect understood.
    Unistructural,

    /// Several aspects understood independently.
    Multistructural,

    /// Integrated understanding.
    Relational,

    /// Generalizes beyond the learned material.
    ExtendedAbstract,
}
