use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::application::auth::service::AuthService;
use crate::application::imports::curriculum::service::CurriculumPkgImportService;
use crate::application::imports::questions::service::QuestionPkgImportService;
use crate::application::learning::concepts::service::ConceptService;
use crate::application::users::service::UserService;
use crate::infrastructure::config::settings::Settings;
use crate::infrastructure::database::repositories::pg_concept_repository::PgConceptRepository;
use crate::infrastructure::database::repositories::pg_course_repository::PgCourseRepository;
use crate::infrastructure::database::repositories::pg_learning_objective_concept_repository::PgLearningObjectiveConceptRepository;
use crate::infrastructure::database::repositories::pg_learning_objective_repository::PgLearningObjectiveRepository;
use crate::infrastructure::database::repositories::pg_password_reset_tokens_repository::PgPasswordResetTokensRepository;
use crate::infrastructure::database::repositories::pg_question_repository::PgQuestionRepository;
use crate::infrastructure::database::repositories::pg_question_type_repository::PgQuestionTypeRepository;
use crate::infrastructure::database::repositories::pg_solo_level_repository::PgSoloLevelRepository;
use crate::infrastructure::database::repositories::pg_users_repository::PgUserRepository;
use crate::infrastructure::jwt::jwt_manager::JwtManager;
use crate::infrastructure::security::password_manager::PasswordManager;
use crate::infrastructure::security::token_manager::TokenManager;

pub struct AppState {
    pub settings: Settings,
    pub db: sea_orm::DatabaseConnection,
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub password_manager: Arc<PasswordManager>,
    pub jwt_manager: Arc<JwtManager>,
    pub concept_service: Arc<ConceptService>,
    pub question_pkg_import_service: Arc<QuestionPkgImportService>,
    pub curriculum_pkg_import_service: Arc<CurriculumPkgImportService>,
}

impl AppState {
    pub fn new(settings: Settings, db: DatabaseConnection) -> Self {
        //
        // Repositories
        //
        let pg_user_repository = Arc::new(PgUserRepository::new(db.clone()));
        let pg_concept_repository = Arc::new(PgConceptRepository::new(db.clone()));
        let prt_repository = Arc::new(PgPasswordResetTokensRepository::new(db.clone()));
        let pg_concept_repository = Arc::new(PgConceptRepository::new(db.clone()));
        let pg_question_repository = Arc::new(PgQuestionRepository::new(db.clone()));
        let pg_question_type_repository = Arc::new(PgQuestionTypeRepository::new(db.clone()));
        let pg_solo_level_repository = Arc::new(PgSoloLevelRepository::new(db.clone()));
        let pg_learning_objective_repository =
            Arc::new(PgLearningObjectiveRepository::new(db.clone()));

        let pg_course_repository = Arc::new(PgCourseRepository::new(db.clone()));
        let pg_learning_objective_concept_repository =
            Arc::new(PgLearningObjectiveConceptRepository::new(db.clone()));

        //
        // Infrastructure services (manager)
        //
        let jwt_manager = Arc::new(JwtManager::new(
            settings.jwt_secret.clone(),
            settings.access_token_exp_minutes,
            settings.refresh_token_exp_days,
        ));
        let token_manager = Arc::new(TokenManager::new());

        //
        // Services
        //
        let user_service = Arc::new(UserService::new(pg_user_repository.clone()));
        let password_manager = Arc::new(PasswordManager::new());

        let auth_service = Arc::new(AuthService::new(
            pg_user_repository,
            prt_repository,
            jwt_manager.clone(),
            password_manager.clone(),
            token_manager.clone(),
        ));

        let concept_service = Arc::new(ConceptService::new(pg_concept_repository.clone()));
        let curriculum_pkg_import_service = Arc::new(CurriculumPkgImportService::new(db.clone()));
        let question_pkg_import_service = Arc::new(QuestionPkgImportService::new(db.clone()));

        Self {
            settings,
            db,
            password_manager,
            user_service,
            auth_service,
            jwt_manager,
            concept_service,
            question_pkg_import_service,
            curriculum_pkg_import_service,
        }
    }
}
