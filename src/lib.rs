use regex::{Regex, RegexBuilder};
use std::fs;

#[derive(Debug, PartialEq)]
pub enum ApplicationError {
    NotEnoughArguments,
    FileNotFound,
    InvalidRegex,
    HelpRequested,
}

impl ApplicationError {
    pub fn handle_error(&self) {
        match self {
            ApplicationError::NotEnoughArguments => eprintln!("Query or file paths are missing."),
            ApplicationError::FileNotFound => eprintln!("Error locating files."),
            ApplicationError::InvalidRegex => eprintln!("Invalid regular expression"),
            ApplicationError::HelpRequested => print_help(),
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
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ApplicationError> {
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            return Err(ApplicationError::HelpRequested);
        }

        let mut ignore_case = false;
        let mut show_line_numbers = false;
        let mut use_regex = false;
        let mut query = String::new();
        let mut file_paths = Vec::new();
        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "--ignore-case" => ignore_case = true,
                "--line-numbers" => show_line_numbers = true,
                "--use-regex" => use_regex = true,
                "-i" => ignore_case = true,
                "-n" => show_line_numbers = true,
                "-r" => use_regex = true,
                _ => {
                    if query.is_empty() {
                        query = arg.clone();
                    } else {
                        file_paths.push(arg.clone());
                    }
                }
            }
        }

        if query.is_empty() || file_paths.is_empty() {
            return Err(ApplicationError::NotEnoughArguments);
        }

        if use_regex {
            if Regex::new(&query).is_err() {
                return Err(ApplicationError::InvalidRegex);
            }
        }

        Ok(Config {
            query,
            file_paths,
            ignore_case,
            show_line_numbers,
            use_regex,
        })
    }
}

pub fn run(config: Config) -> Result<(), ApplicationError> {
    let regex = compile_regex(&config.query, config.use_regex, config.ignore_case)?;

    for file_path in config.file_paths {
        let content = fs::read_to_string(&file_path).map_err(|_| ApplicationError::FileNotFound)?;
        let results = search(&config.query, &content, config.ignore_case, &regex);

        if results.is_empty() {
            continue;
        }

        for (line_num, line) in results {
            if config.show_line_numbers {
                println!("{}:{}: {}", file_path, line_num, line);
            } else {
                println!("{}:{}", file_path, line);
            }
        }
    }

    Ok(())
}

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
            .map_err(|_| ApplicationError::InvalidRegex)
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
    println!("  -r, --use-regex         Show line numbers with output lines");
    println!("  -h, --help              Display this help and exit");
    println!();
    println!("Examples:");
    println!("  tinygrep -i \"rust\" poem.txt");
    println!("  tinygrep -n \"error\" log.txt");
    println!("  tinygrep \"hello\" file1.txt file2.txt");
    println!();
    println!("For more information, check the documentation or run the command with -h.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {
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
    }

    #[test]
    fn config_with_flags() {
        let args = vec![
            "minigrep".to_string(),
            "-i".to_string(),
            "-n".to_string(),
            "-r".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert!(config.use_regex);
    }

    #[test]
    fn config_with_long_flags() {
        let args = vec![
            "minigrep".to_string(),
            "--ignore-case".to_string(),
            "--line-numbers".to_string(),
            "--use-regex".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert!(config.use_regex);
    }

    #[test]
    fn not_enough_arguments() {
        let args = vec!["minigrep".to_string()];
        let result = Config::new(&args);
        assert!(
            matches!(result, Err(ApplicationError::NotEnoughArguments)),
            "Expected NotEnoughArguments error, but got {:?}",
            result
        );
    }

    #[test]
    fn missing_query_or_file_paths() {
        let args = vec!["minigrep".to_string(), "query".to_string()];
        let result = Config::new(&args);
        assert!(
            matches!(result, Err(ApplicationError::NotEnoughArguments)),
            "Expected NotEnoughArguments error, but got {:?}",
            result
        );
    }

    #[test]
    fn help_requested() {
        let args = vec!["minigrep".to_string(), "--help".to_string()];
        let result = Config::new(&args);
        assert!(
            matches!(result, Err(ApplicationError::HelpRequested)),
            "Expected HelpRequested error, but got {:?}",
            result
        );
    }

    #[test]
    fn invalid_regex() {
        let args = vec![
            "minigrep".to_string(),
            "-r".to_string(),
            "[invalid".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args);
        assert!(
            matches!(config, Err(ApplicationError::InvalidRegex)),
            "Expected InvalidRegex error, but got {:?}",
            config
        );
    }

    #[test]
    fn compile_regex_no_regex() {
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
    fn compile_regex_valid_regex() {
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
    fn compile_regex_invalid_regex() {
        let query = "[rust";
        let use_regex = true;
        let ignore_case = false;

        let result = compile_regex(query, use_regex, ignore_case);

        match result {
            Err(ApplicationError::InvalidRegex) => (),
            _ => panic!("Expected Err(ApplicationError::InvalidRegex), got {:?}", result),
        }
    }

    #[test]
    fn compile_regex_case_insensitive() {
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
    fn compile_regex_case_sensitive() {
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
    fn compare_lines_case_sensitive() {
        let query = "duct";
        let line = "duct tape";
        assert!(compare_lines(query, line, false, &None));
        let line2 = "Duct tape";
        assert!(!compare_lines(query, line2, false, &None));
    }

    #[test]
    fn compare_lines_case_insensitive() {
        let query = "rUsT";
        let line = "Rust is great";
        assert!(compare_lines(query, line, true, &None));
        let line2 = "rust is great";
        assert!(compare_lines(query, line2, true, &None));
    }

    #[test]
    fn compare_lines_with_regex() {
        let query = "^Rust";
        let use_regex = true;
        let ignore_case = false;
        
        let regex = compile_regex(query, use_regex, ignore_case).unwrap().unwrap();
        let line = "Rust is great";
        assert!(compare_lines("Rust", line, false, &Some(regex)));
    }

    #[test]
    fn case_sensitive() {
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
    fn case_insensitive() {
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
    fn regex_search_case_sensitive() {
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
    fn regex_search_case_insensitive() {
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
