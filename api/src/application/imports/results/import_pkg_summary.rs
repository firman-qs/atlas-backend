use serde::Serialize;

use crate::application::imports::curriculum::results::curriculum_pkg_summary::ImportCurriculumPkgSummary;
use crate::application::imports::questions::results::question_pkg_summary::ImportQuestionPkgSummary;

#[derive(Debug, Serialize)]
pub struct ImportPkgSummary {
    pub question_pkg_summary: ImportQuestionPkgSummary,
    pub curriculum_pkg_summary: ImportCurriculumPkgSummary,
}
