use regex::Regex;

const HIGHLIGHT_START: &str = "\x1b[1;33m";
const HIGHLIGHT_END: &str = "\x1b[0m";

pub fn apply_highlight(text: &str) -> String {
    format!("{}{}{}", HIGHLIGHT_START, text, HIGHLIGHT_END)
}

pub fn highlight_with_regex<'a>(regex: &Regex, line: &'a str) -> String {
    let mut highlighted_line = String::from(line);

    for mat in regex.find_iter(line) {
        let matched_string = &line[mat.start()..mat.end()];
        let highlighted = apply_highlight(matched_string);
        highlighted_line = highlighted_line.replace(matched_string, &highlighted);
    }

    highlighted_line
}

pub fn highlight_with_substring<'a>(query: &str, line: &'a str, ignore_case: bool) -> String {
    let search_line = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    if let Some(pos) = search_line.find(query) {
        let matched_str = &line[pos..pos + query.len()];
        let highlighted = apply_highlight(matched_str);
        line.replace(matched_str, &highlighted)
    } else {
        String::from(line)
    }
}

pub fn highlight_match<'a>(
    query: &str,
    line: &'a str,
    ignore_case: bool,
    regex: &Option<Regex>,
) -> String {
    if let Some(regex) = regex {
        highlight_with_regex(regex, line)
    } else {
        let query = if ignore_case {
            query.to_lowercase()
        } else {
            query.to_string()
        };
        highlight_with_substring(&query, line, ignore_case)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_highlight() {
        let input = "Rust is powerful";
        let expected = "\x1b[1;33mRust is powerful\x1b[0m";
        let result = apply_highlight(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_with_regex() {
        let regex = Regex::new(r"R\w+").unwrap();
        let input = "Rust is powerful, and Rocks are heavy.";
        let expected = "\x1b[1;33mRust\x1b[0m is powerful, and \x1b[1;33mRocks\x1b[0m are heavy.";

        let result = highlight_with_regex(&regex, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_with_substring_case_sensitive() {
        let input = "Rust is powerful, Rocks are heavy.";
        let query = "Rust";
        let expected = "\x1b[1;33mRust\x1b[0m is powerful, Rocks are heavy.";

        let result = highlight_with_substring(query, input, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_with_substring_case_insensitive() {
        let input = "Rust is powerful, Rocks are heavy.";
        let query = "rust";
        let expected = "\u{1b}[1;33mRust\u{1b}[0m is powerful, Rocks are heavy.";

        let result = highlight_with_substring(query, input, true);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_match_with_regex() {
        let query = "R\\w+";
        let regex = Regex::new(query).unwrap();
        let input = "Rust is powerful, and Rocks are heavy.";
        let expected = "\x1b[1;33mRust\x1b[0m is powerful, and \x1b[1;33mRocks\x1b[0m are heavy.";

        let result = highlight_match(query, input, false, &Some(regex));
        assert_eq!(result, expected);
    }
}
