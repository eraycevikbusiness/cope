#[derive(Debug)]
pub enum ConfigError {
    PathDoesNotExist,
    ParsingError
}

pub fn read_json_from<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, ConfigError> {
    let content_as_str = std::fs::read_to_string(path).map_err(|_| ConfigError::PathDoesNotExist)?;
    let parsed = serde_json::from_str(&content_as_str).map_err(|_| ConfigError::ParsingError)?;
    Ok(parsed)
}