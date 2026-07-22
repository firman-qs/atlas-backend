pub use sea_orm_migration::prelude::*;

mod m20260707_103243_create_users_table;
mod m20260708_180646_add_change_password_to_users_table;
mod m20260710_161513_create_courses_and_learning_objectives;
mod m20260711_013545_create_concepts;
mod m20260711_024510_create_solo_levels;
mod m20260711_024524_create_question_types;
mod m20260711_024532_create_questions;
mod m20260711_024538_create_question_options;
mod m20260711_024558_create_isomorphic_sets;
mod m20260711_024610_create_isomorphic_set_questions;
mod m20260715_175224_create_assessment_attempts;
mod m20260716_060217_create_student_answers;
mod m20260716_121041_password_reset_tokens;
mod m20260717_171642_create_user_role;
mod m20260722_041440_add_target_solo_level_to_concepts;
mod m20260722_045739_add_concepts_displayorder;
mod m20260722_083618_add_code_and_courseid_to_questions;
pub mod schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>>
    {
        vec![
            Box::new(m20260707_103243_create_users_table::Migration),
            Box::new(m20260708_180646_add_change_password_to_users_table::Migration),
            Box::new(m20260710_161513_create_courses_and_learning_objectives::Migration),
            Box::new(m20260711_013545_create_concepts::Migration),
            Box::new(m20260711_024510_create_solo_levels::Migration),
            Box::new(m20260711_024524_create_question_types::Migration),
            Box::new(m20260711_024532_create_questions::Migration),
            Box::new(m20260711_024538_create_question_options::Migration),
            Box::new(m20260711_024558_create_isomorphic_sets::Migration),
            Box::new(m20260711_024610_create_isomorphic_set_questions::Migration),
            Box::new(m20260715_175224_create_assessment_attempts::Migration),
            Box::new(m20260716_060217_create_student_answers::Migration),
            Box::new(m20260716_121041_password_reset_tokens::Migration),
            Box::new(m20260717_171642_create_user_role::Migration),
            Box::new(m20260722_041440_add_target_solo_level_to_concepts::Migration),
            Box::new(m20260722_045739_add_concepts_displayorder::Migration),
            Box::new(m20260722_083618_add_code_and_courseid_to_questions::Migration),
        ]
    }
}
