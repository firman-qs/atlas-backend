pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub password_hash: String,
}

impl CreateUser {
    pub fn new(
        email: impl Into<String>,
        username: impl Into<String>,
        full_name: impl Into<String>,
        password_hash: impl Into<String>,
    ) -> Self {
        Self {
            email: email.into(),
            username: username.into(),
            full_name: full_name.into(),
            password_hash: password_hash.into(),
        }
    }
}
