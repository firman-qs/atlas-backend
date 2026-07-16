use migration::async_trait::async_trait;

use crate::models::assessment::assessment_result::AssessmentResult;

#[async_trait]
pub trait Evaluator {
    type Answer;
    async fn evaluate(&self, answer: Self::Answer) -> anyhow::Result<AssessmentResult>;
}
