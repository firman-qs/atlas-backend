use uuid::Uuid;

use crate::dto::learning_objective::update_lo_request::UpdateLoRequest;

pub struct UpdateLo {
    pub id: Uuid,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub display_order: Option<i32>,
}

impl From<UpdateLoRequest> for UpdateLo {
    fn from(request: UpdateLoRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            title: request.title,
            description: request.description,
            display_order: request.display_order,
        }
    }
}
