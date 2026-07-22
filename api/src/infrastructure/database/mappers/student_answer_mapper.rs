use entity::student_answers;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::student_answer::StudentAnswer;
use crate::domain::entities::student_answer_update::StudentAnswerUpdate;

impl IntoActiveModel<student_answers::ActiveModel> for StudentAnswer {
    fn into_active_model(self) -> student_answers::ActiveModel {
        student_answers::ActiveModel {
            assessment_attempt_id: Set(self.assessment_attempt_id),
            question_id: Set(self.question_id),
            answer_text: Set(self.answer_text),
            selected_option_id: Set(self.selected_option_id),
            answer_json: Set(self.answer_json),
            ..Default::default()
        }
    }
}

impl From<student_answers::Model> for StudentAnswer {
    fn from(model: student_answers::Model) -> Self {
        StudentAnswer {
            assessment_attempt_id: model.assessment_attempt_id,
            question_id: model.question_id,
            answer_text: model.answer_text,
            selected_option_id: model.selected_option_id,
            answer_json: model.answer_json,
        }
    }
}

impl IntoActiveModel<student_answers::ActiveModel> for StudentAnswerUpdate {
    fn into_active_model(self) -> student_answers::ActiveModel {
        let mut active_model = student_answers::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(answer_text) = self.answer_text {
            active_model.answer_text = sea_orm::ActiveValue::Set(Some(answer_text));
        }

        if let Some(selected_option_id) = self.selected_option_id {
            active_model.selected_option_id = sea_orm::ActiveValue::Set(Some(selected_option_id));
        }

        if let Some(answer_json) = self.answer_json {
            active_model.answer_json = sea_orm::ActiveValue::Set(Some(answer_json.into()));
        }

        if let Some(is_correct) = self.is_correct {
            active_model.is_correct = sea_orm::ActiveValue::Set(is_correct);
        }

        if let Some(score) = self.score {
            active_model.score = sea_orm::ActiveValue::Set(score);
        }

        if let Some(feedback) = self.feedback {
            active_model.feedback = sea_orm::ActiveValue::Set(Some(feedback));
        }

        active_model.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());

        active_model
    }
}
