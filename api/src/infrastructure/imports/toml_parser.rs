use crate::application::imports::models::package::ImportPkg;

pub struct Parser;

impl Parser
{
    pub fn new() -> Self
    {
        Self {}
    }

    pub fn parse_toml(&self, contents: &str) -> Result<ImportPkg, toml::de::Error>
    {
        toml::from_str(contents)
    }
}
