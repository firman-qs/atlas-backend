use serde::de::DeserializeOwned;

use crate::application::app_error::AppError;

pub struct TomlPkgParser;

impl TomlPkgParser {
    pub fn parse<T>(contents: &str) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        let result = toml::from_str::<T>(contents)?;
        Ok(result)
    }
}
