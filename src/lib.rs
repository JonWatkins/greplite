use std::fs;

#[derive(Debug, PartialEq)]
pub enum ApplicationError {
    NotEnoughArguments,
    FileNotFound,
    HelpRequested,
}

impl ApplicationError {
    pub fn handle_error(&self) {
        match self {
            ApplicationError::NotEnoughArguments => eprintln!("Query or file paths are missing."),
            ApplicationError::FileNotFound => eprintln!("Error locating files."),
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
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ApplicationError> {
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            return Err(ApplicationError::HelpRequested);
        }

        let mut ignore_case = false;
        let mut show_line_numbers = false;
        let mut query = String::new();
        let mut file_paths = Vec::new();
        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-i" => ignore_case = true,
                "-n" => show_line_numbers = true,
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

        Ok(Config {
            query,
            file_paths,
            ignore_case,
            show_line_numbers,
        })
    }
}

pub fn run(config: Config) -> Result<(), ApplicationError> {
    for file_path in config.file_paths {
        println!("File: {}", file_path);

        let content = fs::read_to_string(file_path).map_err(|_| ApplicationError::FileNotFound)?;
        let results = search(&config.query, &content, config.ignore_case);

        if results.is_empty() {
            continue;
        }

        for (line_num, line) in results {
            if config.show_line_numbers {
                println!("{}: {}", line_num, line);
            } else {
                println!("{}", line);
            }
        }

        println!();
    }

    Ok(())
}

fn compare_lines(query: &str, line: &str, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(&query.to_lowercase())
    } else {
        line.contains(query)
    }
}

fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if compare_lines(query, line, ignore_case) {
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
    }

    #[test]
    fn config_with_flags() {
        let args = vec![
            "minigrep".to_string(),
            "-i".to_string(),
            "-n".to_string(),
            "rust".to_string(),
            "poem.txt".to_string(),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "rust");
        assert_eq!(config.file_paths, vec!["poem.txt".to_string()]);
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
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
    fn compare_lines_case_sensitive() {
        let query = "duct";
        let line = "duct tape";
        assert!(compare_lines(query, line, false));
        let line2 = "Duct tape";
        assert!(!compare_lines(query, line2, false));
    }

    #[test]
    fn compare_lines_case_insensitive() {
        let query = "rUsT";
        let line = "Rust is great";
        assert!(compare_lines(query, line, true));
        let line2 = "rust is great";
        assert!(compare_lines(query, line2, true));
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
            search(query, content, false)
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
            search(query, content, true)
        );
    }
}
