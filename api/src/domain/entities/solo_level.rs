use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SoloLevel
{
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub order_index: i16,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug)]
pub struct SoloLevelNew
{
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i16,
}

#[derive(Debug)]
pub struct SoloLevelUpdate
{
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub order_index: Option<i16>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SoloLevelCode
{
    Prestructural,
    Unistructural,
    Multistructural,
    Relational,
    ExtendedAbstract,
}

impl SoloLevelCode
{
    pub fn as_db_str(&self) -> &'static str
    {
        match self
        {
            SoloLevelCode::Prestructural => "PRESTRUCTURAL",
            SoloLevelCode::Unistructural => "UNISTRUCTURAL",
            SoloLevelCode::Multistructural => "MULTISTRUCTURAL",
            SoloLevelCode::Relational => "RELATIONAL",
            SoloLevelCode::ExtendedAbstract => "EXTENDED_ABSTRACT",
        }
    }
}
