use sea_orm::Database;
use sea_orm::DatabaseConnection;

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(database_url).await
}
