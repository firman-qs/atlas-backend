use garde::{Error, Result};

use crate::dto::learning_objective::update_lo_request::UpdateLoRequest;

pub fn validate_update_lo(value: &UpdateLoRequest, _: &()) -> Result {
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
