pub struct Tokenizer {}

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<String> {
        let processes_input = if let Some(pos) = input.find(';') {
            &input[..pos]
        } else {
            input
        };

        processes_input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_string_literal() {
        assert_eq!(Tokenizer::tokenize("").len(), 0);
    }

    #[test]
    fn test_tokenize_query_literal() {
        assert_eq!(
            Tokenizer::tokenize("select * from users where id = 3"),
            vec![
                "select".to_string(),
                "*".to_string(),
                "from".to_string(),
                "users".to_string(),
                "where".to_string(),
                "id".to_string(),
                "=".to_string(),
                "3".to_string(),
            ]
        );
    }

    #[test]
    fn test_tokenize_query_literal_with_semicolon() {
        assert_eq!(
            Tokenizer::tokenize("select * from users where id = 3;"),
            vec![
                "select".to_string(),
                "*".to_string(),
                "from".to_string(),
                "users".to_string(),
                "where".to_string(),
                "id".to_string(),
                "=".to_string(),
                "3".to_string(),
            ]
        );
    }
}
