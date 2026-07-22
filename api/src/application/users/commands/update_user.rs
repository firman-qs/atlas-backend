use uuid::Uuid;

#[derive(Debug)]
pub struct UpdateUser {
    pub id: Uuid,
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub must_change_password: Option<bool>,
}

impl UpdateUser {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            username: None,
            password_hash: None,
            full_name: None,
            avatar_url: None,
            is_active: None,
            must_change_password: None,
        }
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn with_password_hash(mut self, password_hash: String) -> Self {
        self.password_hash = Some(password_hash);
        self
    }

    pub fn with_full_name(mut self, full_name: String) -> Self {
        self.full_name = Some(full_name);
        self
    }

    pub fn with_avatar_url(mut self, avatar_url: Option<String>) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn with_is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn with_must_change_password(mut self, must_change_password: bool) -> Self {
        self.must_change_password = Some(must_change_password);
        self
    }
}
