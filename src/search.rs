use regex::Regex;

pub fn compare_lines(query: &str, line: &str, ignore_case: bool, regex: &Option<Regex>) -> bool {
    if let Some(regex) = regex {
        regex.is_match(line)
    } else {
        if ignore_case {
            line.to_lowercase().contains(&query.to_lowercase())
        } else {
            line.contains(query)
        }
    }
}

pub fn search<'a>(
    query: &str,
    content: &'a str,
    ignore_case: bool,
    regex: &Option<Regex>,
) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if compare_lines(query, line, ignore_case, regex) {
            results.push((line_num + 1, line));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ApplicationError;
    use crate::regex::compile_regex;

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
                assert_eq!(s, "[rust"); // Expect the correct error message format
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

    #[test]
    fn test_compile_regex_case_sensitive() {
        let query = "Rust.*";
        let use_regex = true;
        let ignore_case = false;

        let result = compile_regex(query, use_regex, ignore_case);

        match result {
            Ok(Some(regex)) => {
                assert!(regex.is_match("Rusty nails"));
                assert!(!regex.is_match("rusty nails"));
            }
            _ => panic!("Expected Ok(Some(regex)), got {:?}", result),
        }
    }

    #[test]
    fn test_compare_lines_case_sensitive() {
        let query = "duct";
        let line = "duct tape";
        assert!(compare_lines(query, line, false, &None));
        let line2 = "Duct tape";
        assert!(!compare_lines(query, line2, false, &None));
    }

    #[test]
    fn test_compare_lines_case_insensitive() {
        let query = "rUsT";
        let line = "Rust is great";
        assert!(compare_lines(query, line, true, &None));
        let line2 = "rust is great";
        assert!(compare_lines(query, line2, true, &None));
    }

    #[test]
    fn test_compare_lines_with_regex() {
        let query = "^Rust";
        let use_regex = true;
        let ignore_case = false;

        let regex = compile_regex(query, use_regex, ignore_case)
            .unwrap()
            .unwrap();
        let line = "Rust is great";
        assert!(compare_lines("Rust", line, false, &Some(regex)));
    }

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, content, false, &None)
        );
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search(query, content, true, &None)
        );
    }

    #[test]
    fn test_regex_search_case_sensitive() {
        let query = "Rust.*";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Rusty nails.";

        let regex = compile_regex(query, true, false).unwrap().unwrap();
        assert_eq!(
            vec![(1, "Rust:"), (4, "Rusty nails.")],
            search(query, content, false, &Some(regex))
        );
    }

    #[test]
    fn test_regex_search_case_insensitive() {
        let query = "(?i)rust.*";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Rusty nails.";

        let regex = compile_regex(query, true, true).unwrap().unwrap();
        assert_eq!(
            vec![(1, "Rust:"), (4, "Rusty nails.")],
            search(query, content, false, &Some(regex))
        );
    }
}
