use regex::{Regex, RegexBuilder};

use std::{
    fmt,
    fs::{self},
    io::{self, Read},
    path::Path,
};

const HIGHLIGHT_START: &str = "\x1b[1;33m";
const HIGHLIGHT_END: &str = "\x1b[0m";

#[derive(Debug)]
pub enum ApplicationError {
    NotEnoughArguments,
    InvalidRegex(String),
    FileNotFound(String),
    DirectoryReadError(String),
    DirectoryWithoutRecursive,
    InvalidFlag(String),
    IOError(io::Error),
    HelpRequested,
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationError::NotEnoughArguments => {
                write!(
                    f,
                    "Error: Not enough arguments provided. A query and file paths are required."
                )
            }
            ApplicationError::InvalidRegex(query) => {
                write!(f, "Error: Invalid regular expression: '{}'", query)
            }
            ApplicationError::FileNotFound(file) => {
                write!(f, "Error: File '{}' not found.", file)
            }
            ApplicationError::DirectoryReadError(path) => {
                write!(f, "Error reading directory '{}'.", path)
            }
            ApplicationError::DirectoryWithoutRecursive => {
                write!(f, "Error: You provided a directory, but did not use the '-R' option for recursive search.")
            }
            ApplicationError::InvalidFlag(flag) => {
                write!(f, "Error: Invalid flag '{}'.", flag)
            }
            ApplicationError::IOError(e) => write!(f, "I/O Error: {}", e),
            ApplicationError::HelpRequested => write!(f, "Help requested."),
        }
    }
}

impl ApplicationError {
    pub fn handle_error(&self) {
        match self {
            ApplicationError::HelpRequested => print_help(),
            ApplicationError::NotEnoughArguments => eprintln!("{}", self),
            ApplicationError::InvalidRegex(_) => eprintln!("{}", self),
            ApplicationError::FileNotFound(_) => eprintln!("{}", self),
            ApplicationError::InvalidFlag(_) => eprintln!("{}", self),
            ApplicationError::IOError(_) => eprintln!("{}", self),
            ApplicationError::DirectoryReadError(_) => eprintln!("{}", self),
            ApplicationError::DirectoryWithoutRecursive => eprintln!("{}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    file_paths: Vec<String>,
    ignore_case: bool,
    show_line_numbers: bool,
    use_regex: bool,
    enable_highlighting: bool,
    read_from_stdin: bool,
    recursive_search: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ApplicationError> {
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            return Err(ApplicationError::HelpRequested);
        }

        let mut ignore_case = false;
        let mut show_line_numbers = false;
        let mut use_regex = false;
        let mut enable_highlighting = false;
        let mut recursive_search = false;
        let mut query = String::new();
        let mut file_paths = Vec::new();
        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-i" | "--ignore-case" => ignore_case = true,
                "-n" | "--line-numbers" => show_line_numbers = true,
                "-R" | "--recursive" => recursive_search = true,
                "-r" | "--use-regex" => use_regex = true,
                "-c" | "--color" => enable_highlighting = true,
                _ => {
                    if arg.starts_with('-') {
                        return Err(ApplicationError::InvalidFlag(arg.clone()));
                    }

                    if query.is_empty() {
                        query = arg.clone();
                    } else {
                        file_paths.push(arg.clone());
                    }
                }
            }
        }

        let read_from_stdin = file_paths.is_empty();

        if query.is_empty() {
            return Err(ApplicationError::NotEnoughArguments);
        }

        if use_regex {
            if Regex::new(&query).is_err() {
                return Err(ApplicationError::InvalidRegex(query.to_string()));
            }
        }

        Ok(Config {
            query,
            file_paths,
            ignore_case,
            show_line_numbers,
            use_regex,
            enable_highlighting,
            read_from_stdin,
            recursive_search,
        })
    }
}

pub fn run(config: Config) -> Result<(), ApplicationError> {
    let regex = compile_regex(&config.query, config.use_regex, config.ignore_case)?;

    if config.read_from_stdin {
        process_input("stdin", &mut io::stdin().lock(), &config, &regex)?;
    } else {
        for file_path in &config.file_paths {
            let path = Path::new(file_path);

            if path.is_dir() && !config.recursive_search {
                return Err(ApplicationError::DirectoryWithoutRecursive);
            }

            if path.is_dir() && config.recursive_search {
                process_directory(path, &config, &regex)?;
            } else {
                process_file(file_path, &config, &regex)?;
            }
        }
    }

    Ok(())
}

fn process_input<R: Read>(
    source: &str,
    reader: &mut R,
    config: &Config,
    regex: &Option<Regex>,
) -> Result<(), ApplicationError> {
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .map_err(|e| ApplicationError::IOError(e))?;

    let results = search(&config.query, &input, config.ignore_case, regex);
    print_results(config, source, results, regex)
}

fn process_file(
    file_path: &str,
    config: &Config,
    regex: &Option<Regex>,
) -> Result<(), ApplicationError> {
    let content = fs::read_to_string(file_path)
        .map_err(|_| ApplicationError::FileNotFound(file_path.to_string()))?;

    let results = search(&config.query, &content, config.ignore_case, regex);
    print_results(config, file_path, results, regex)
}

fn process_directory(
    dir_path: &Path,
    config: &Config,
    regex: &Option<Regex>,
) -> Result<(), ApplicationError> {
    for entry in fs::read_dir(dir_path)
        .map_err(|_| ApplicationError::FileNotFound(dir_path.to_string_lossy().to_string()))?
    {
        let entry = entry
            .map_err(|_| ApplicationError::FileNotFound(dir_path.to_string_lossy().to_string()))?;
        let path = entry.path();

        if path.is_dir() {
            process_directory(&path, config, regex)?;
        } else {
            process_file(path.to_str().unwrap(), config, regex)?;
        }
    }

    Ok(())
}

fn print_results(
    config: &Config,
    source: &str,
    results: Vec<(usize, &str)>,
    regex: &Option<Regex>,
) -> Result<(), ApplicationError> {
    if results.is_empty() {
        return Ok(());
    }

    for (line_num, line) in results {
        let highlighted_line = if config.enable_highlighting {
            highlight_match(&config.query, line, config.ignore_case, &regex)
        } else {
            line.to_string()
        };

        if config.show_line_numbers {
            println!("{}:{}: {}", source, line_num, highlighted_line);
        } else {
            println!("{}:{}", source, highlighted_line);
        }
    }
    Ok(())
}

fn compile_regex(
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

fn compare_lines(query: &str, line: &str, ignore_case: bool, regex: &Option<Regex>) -> bool {
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

fn search<'a>(
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

fn apply_highlight(text: &str) -> String {
    format!("{}{}{}", HIGHLIGHT_START, text, HIGHLIGHT_END)
}

fn highlight_with_regex<'a>(regex: &Regex, line: &'a str) -> String {
    let mut highlighted_line = String::from(line);

    for mat in regex.find_iter(line) {
        let matched_string = &line[mat.start()..mat.end()];
        let highlighted = apply_highlight(matched_string);
        highlighted_line = highlighted_line.replace(matched_string, &highlighted);
    }

    highlighted_line
}

fn highlight_with_substring<'a>(query: &str, line: &'a str, ignore_case: bool) -> String {
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

fn highlight_match<'a>(
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

fn print_help() {
    println!("TinyGrep - A simplified version of the `grep` command");
    println!();
    println!("Usage:");
    println!("  tinygrep [OPTION]... PATTERN [FILE]...");
    println!();
    println!("Search for PATTERN in each FILE or standard input.");
    println!();
    println!("Options:");
    println!("  -i, --ignore-case       Perform case-insensitive matching");
    println!("  -n, --line-numbers      Show line numbers with output lines");
    println!("  -r, --use-regex         Treat PATTERN as a regular expression");
    println!("  -R, --recursive         Search recursively in directories.");
    println!("  -c, --color             Highlight matching text in output");
    println!("  -h, --help              Display this help and exit");
    println!();
    println!("Examples:");
    println!("  tinygrep -i \"rust\" file1.txt       # Case-insensitive search for 'rust'");
    println!("  tinygrep -n \"error\" file1.txt      # Search for 'error' and show line numbers");
    println!(
        "  tinygrep -r \"R\\w+\" file1.txt       # Search for words starting with 'R' using regex"
    );
    println!("  tinygrep -i -n \"hello\" file1.txt file2.txt # Case-insensitive search with line numbers");
    println!();
    println!("For more information, check the documentation or run the command with -h.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let args = vec![
            "minigrep".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(!config.ignore_case);
        assert!(!config.show_line_numbers);
        assert!(!config.use_regex);
        assert!(!config.enable_highlighting);
    }

    #[test]
    fn test_config_with_flags() {
        let args = vec![
            "minigrep".to_string(),
            "-i".to_string(),
            "-n".to_string(),
            "-r".to_string(),
            "-c".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert!(config.use_regex);
        assert!(config.enable_highlighting);
    }

    #[test]
    fn test_config_with_long_flags() {
        let args = vec![
            "minigrep".to_string(),
            "--ignore-case".to_string(),
            "--line-numbers".to_string(),
            "--use-regex".to_string(),
            "--color".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert!(config.use_regex);
        assert!(config.enable_highlighting);
    }

    #[test]
    fn test_invalid_flag() {
        let args = vec![
            "minigrep".to_string(),
            "rust".to_string(),
            "--unknown".to_string(),  // Invalid flag
            "poem.txt".to_string(),
        ];

        let result = Config::new(&args);

        assert!(
            matches!(result, Err(ApplicationError::InvalidFlag(ref flag)) if flag == "--unknown"),
            "Expected InvalidFlag error with '--unknown', but got {:?}",
            result
        );
    }

    #[test]
    fn test_not_enough_arguments() {
        let args = vec!["minigrep".to_string()];
        let result = Config::new(&args);
        assert!(
            matches!(result, Err(ApplicationError::NotEnoughArguments)),
            "Expected NotEnoughArguments error, but got {:?}",
            result
        );
    }

    #[test]
    fn test_missing_query_or_file_paths() {
        let args = vec!["minigrep".to_string(), "-i".to_string()];
        let result = Config::new(&args);
        assert!(
            matches!(result, Err(ApplicationError::NotEnoughArguments)),
            "Expected NotEnoughArguments error, but got {:?}",
            result
        );
    }

    #[test]
    fn test_help_requested() {
        let args = vec!["minigrep".to_string(), "--help".to_string()];
        let result = Config::new(&args);
        assert!(
            matches!(result, Err(ApplicationError::HelpRequested)),
            "Expected HelpRequested error, but got {:?}",
            result
        );
    }

    #[test]
    fn test_invalid_regex() {
        let args = vec![
            "minigrep".to_string(),
            "-r".to_string(),
            "[invalid".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args);
        assert!(
            matches!(config, Err(ApplicationError::InvalidRegex(ref s)) if s == "[invalid"),
            "Expected InvalidRegex error with '[invalid', but got {:?}",
            config
        );
    }

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
