use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            data: None,
        }
    }
}
