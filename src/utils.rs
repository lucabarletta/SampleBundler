pub fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return "".to_string();
    }

    let first_string = &strings[0];
    let lowercased_strings: Vec<String> = strings.iter().map(|s| s.to_lowercase()).collect();

    let mut prefix_len = 0;
    for (i, c) in lowercased_strings[0].chars().enumerate() {
        let all_match = lowercased_strings.iter().all(|s| s.chars().nth(i) == Some(c));
        if all_match {
            prefix_len += c.len_utf8();
        } else {
            break;
        }
    }
    first_string[..prefix_len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix_empty_list() {
        let strings: Vec<String> = Vec::new();
        assert_eq!(longest_common_prefix(&strings), "");
    }

    #[test]
    fn test_longest_common_prefix_single_string() {
        let strings = vec!["apple".to_string()];
        assert_eq!(longest_common_prefix(&strings), "apple");
    }

    #[test]
    fn test_longest_common_prefix_common_prefix() {
        let strings = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(&strings), "fl");
    }

    #[test]
    fn test_longest_common_prefix_no_common_prefix() {
        let strings = vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ];
        assert_eq!(longest_common_prefix(&strings), "");
    }

    #[test]
    fn test_longest_common_prefix_full_match() {
        let strings = vec![
            "apple".to_string(),
            "apple".to_string(),
            "apple".to_string(),
        ];
        assert_eq!(longest_common_prefix(&strings), "apple");
    }

    #[test]
    fn test_longest_common_prefix_with_different_lengths() {
        let strings = vec![
            "testing".to_string(),
            "test".to_string(),
            "tester".to_string(),
        ];
        assert_eq!(longest_common_prefix(&strings), "test");
    }

    #[test]
    fn test_longest_common_prefix_with_numbers_and_symbols() {
        let strings = vec![
            "123abc".to_string(),
            "123def".to_string(),
            "123ghi".to_string(),
        ];
        assert_eq!(longest_common_prefix(&strings), "123");
    }

    #[test]
    fn test_longest_common_prefix_case_insensitive() {
        let strings = vec![
            "Apple".to_string(),
            "apple".to_string(),
            "App".to_string(),
        ];
        assert_eq!(longest_common_prefix(&strings), "App");
    }
}
