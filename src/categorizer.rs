use crate::config::Config;
use regex::Regex;
use std::path::Path;

pub fn categorize_sample(path: &Path, config: &Config) -> Option<String> {
    let filename = path.file_name()?.to_str()?.to_lowercase();
    for (category, patterns) in &config.patterns {
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&filename) {
                    return Some(category.clone());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_mock_config() -> Config {
        let mut patterns = HashMap::new();
        patterns.insert(
            "drums".to_string(),
            vec!["kick".to_string(), "snare".to_string()],
        );
        patterns.insert(
            "synth".to_string(),
            vec!["pad".to_string(), "lead".to_string()],
        );
        Config { patterns }
    }

    #[test]
    fn test_categorize_sample_match() {
        let config = create_mock_config();
        let path = Path::new("path/to/my_kick_sample.wav");
        assert_eq!(categorize_sample(path, &config), Some("drums".to_string()));
    }

    #[test]
    fn test_categorize_sample_no_match() {
        let config = create_mock_config();
        let path = Path::new("path/to/my_vocal_sample.wav");
        assert_eq!(categorize_sample(path, &config), None);
    }

    #[test]
    fn test_categorize_sample_multiple_patterns() {
        let config = create_mock_config();
        let path = Path::new("path/to/my_snare_drum.wav");
        assert_eq!(categorize_sample(path, &config), Some("drums".to_string()));
    }

    #[test]
    fn test_categorize_sample_case_insensitivity() {
        let config = create_mock_config();
        let path = Path::new("path/to/My_Pad_Sound.wav");
        assert_eq!(categorize_sample(path, &config), Some("synth".to_string()));
    }

    #[test]
    fn test_categorize_sample_empty_filename() {
        let config = create_mock_config();
        let path = Path::new("path/to/"); // No filename
        assert_eq!(categorize_sample(path, &config), None);
    }
}
