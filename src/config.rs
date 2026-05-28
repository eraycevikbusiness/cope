use std::fs;

use serde::Serialize;

#[derive(Debug)]
pub enum ConfigError {
    ReadFailed(String),
    WriteFailed(String),
    Deserialize,
    Serialize
}

pub fn read_json_from<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, ConfigError> {
    let content_as_str = std::fs::read_to_string(path).map_err(|e| ConfigError::ReadFailed(e.to_string()))?;
    let parsed = serde_json::from_str(&content_as_str).map_err(|_| ConfigError::Deserialize)?;
    Ok(parsed)
}

pub fn write_to_path<T: Serialize>(path: &str, model: &T) -> Result<(), ConfigError>{
    let json = serde_json::to_string_pretty(&model).map_err(|_| ConfigError::Serialize)?;
    fs::write(path, json).map_err(|e| ConfigError::WriteFailed(e.to_string()))?;

    Ok(())
}