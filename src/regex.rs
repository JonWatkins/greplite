use crate::error::ApplicationError;
use regex::{Regex, RegexBuilder};

pub fn compile_regex(
    query: &str,
    use_regex: bool,
    ignore_case: bool,
) -> Result<Option<Regex>, ApplicationError> {
    if use_regex {
        let mut builder = RegexBuilder::new(query);

        if ignore_case {
            builder.case_insensitive(true);
        }

        builder
            .build()
            .map(Some)
            .map_err(|_| ApplicationError::InvalidRegex(query.to_string()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_regex_no_regex() {
        let query = "rust";
        let use_regex = false;
        let ignore_case = false;

        let result = compile_regex(query, use_regex, ignore_case);

        match result {
            Ok(None) => (),
            _ => panic!("Expected Ok(None), got {:?}", result),
        }
    }

    #[test]
    fn test_compile_regex_valid_regex() {
        let query = "rust.*";
        let use_regex = true;
        let ignore_case = false;

        let result = compile_regex(query, use_regex, ignore_case);

        match result {
            Ok(Some(regex)) => {
                assert!(!regex.is_match("Rust is great"));
                assert!(regex.is_match("nothing about rust"));
            }
            _ => panic!("Expected Ok(Some(regex)), got {:?}", result),
        }
    }

    #[test]
    fn test_compile_regex_invalid_regex() {
        let query = "[rust";
        let use_regex = true;
        let ignore_case = false;

        let result = compile_regex(query, use_regex, ignore_case);

        match result {
            Err(ApplicationError::InvalidRegex(ref s)) => {
                assert_eq!(s, "[rust");
            }
            _ => panic!(
                "Expected Err(ApplicationError::InvalidRegex), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_compile_regex_case_insensitive() {
        let query = "rust.*";
        let use_regex = true;
        let ignore_case = true;

        let result = compile_regex(query, use_regex, ignore_case);

        match result {
            Ok(Some(regex)) => {
                assert!(regex.is_match("Rusty nails"));
                assert!(regex.is_match("rusty nails"));
                assert!(regex.is_match("nothing about rust"));
                assert!(!regex.is_match("fast, safe, productive."));
            }
            _ => panic!("Expected Ok(Some(regex)), got {:?}", result),
        }
    }
}
