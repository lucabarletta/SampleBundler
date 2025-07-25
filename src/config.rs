use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub patterns: HashMap<String, Vec<String>>,
}

pub fn load_config(path: &Path) -> Result<Config, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.toml");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "[patterns]\ncategory1 = [\"pattern1\"]\ncategory2 = [\"pattern2\"]").unwrap();

        let config = load_config(&file_path).unwrap();
        assert_eq!(config.patterns.len(), 2);
        assert_eq!(config.patterns["category1"], vec!["pattern1"]);
        assert_eq!(config.patterns["category2"], vec!["pattern2"]);
    }

    #[test]
    fn test_load_config_file_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("non_existent_config.toml");

        let err = load_config(&file_path).unwrap_err();
        eprintln!("Error: {}", err);
        assert!(err.contains("os error 2"));
    }

    #[test]
    fn test_load_config_invalid_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid_config.toml");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "[patterns]\ncategory1 = \"pattern1\"").unwrap(); // Malformed TOML

        let err = load_config(&file_path).unwrap_err();
        assert!(err.contains("TOML parse error"));
    }
}
