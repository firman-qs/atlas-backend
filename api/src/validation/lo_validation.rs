use garde::{Error, Result};

use crate::dto::learning_objective::update_learning_objective_request::UpdateLearningObjectiveRequest;

pub fn validate_update_lo(value: &UpdateLearningObjectiveRequest, _: &()) -> Result {
    if value.code.is_none()
        && value.title.is_none()
        && value.description.is_none()
        && value.display_order.is_none()
    {
        return Err(Error::new(
            "At least one field must be provided for update.",
        ));
    }

    Ok(())
}
