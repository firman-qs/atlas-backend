use uuid::Uuid;

pub struct Impersonate {
    pub user_id: Uuid,
    pub as_user_id: Uuid,
}
