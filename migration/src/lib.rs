pub use sea_orm_migration::prelude::*;

mod m20260707_103243_create_users_table;
mod m20260708_180646_add_change_password_to_users_table;
mod m20260710_161513_create_courses_and_learning_objectives;
mod m20260711_013545_create_concepts;
mod m20260711_024524_create_question_types;
mod m20260711_024532_create_questions;
mod m20260711_024538_create_question_options;
mod m20260711_024558_create_isomorphic_sets;
mod m20260711_024610_create_isomorphic_set_questions;
pub mod schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260707_103243_create_users_table::Migration),
            Box::new(m20260708_180646_add_change_password_to_users_table::Migration),
            Box::new(m20260710_161513_create_courses_and_learning_objectives::Migration),
            Box::new(m20260711_013545_create_concepts::Migration),
            Box::new(m20260711_024524_create_question_types::Migration),
            Box::new(m20260711_024532_create_questions::Migration),
            Box::new(m20260711_024538_create_question_options::Migration),
            Box::new(m20260711_024558_create_isomorphic_sets::Migration),
            Box::new(m20260711_024610_create_isomorphic_set_questions::Migration),
        ]
    }
}
