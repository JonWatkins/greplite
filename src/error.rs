use std::{fmt, io};

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

    // Test the Display implementation for various errors
    #[test]
    fn test_display_not_enough_arguments() {
        let err = ApplicationError::NotEnoughArguments;
        let result = format!("{}", err);
        assert_eq!(
            result,
            "Error: Not enough arguments provided. A query and file paths are required."
        );
    }

    #[test]
    fn test_display_invalid_regex() {
        let err = ApplicationError::InvalidRegex("^[a-z".to_string());
        let result = format!("{}", err);
        assert_eq!(result, "Error: Invalid regular expression: '^[a-z'");
    }

    #[test]
    fn test_display_file_not_found() {
        let err = ApplicationError::FileNotFound("file.txt".to_string());
        let result = format!("{}", err);
        assert_eq!(result, "Error: File 'file.txt' not found.");
    }

    #[test]
    fn test_display_directory_read_error() {
        let err = ApplicationError::DirectoryReadError("/some/path".to_string());
        let result = format!("{}", err);
        assert_eq!(result, "Error reading directory '/some/path'.");
    }

    #[test]
    fn test_display_directory_without_recursive() {
        let err = ApplicationError::DirectoryWithoutRecursive;
        let result = format!("{}", err);
        assert_eq!(result, "Error: You provided a directory, but did not use the '-R' option for recursive search.");
    }

    #[test]
    fn test_display_invalid_flag() {
        let err = ApplicationError::InvalidFlag("-f".to_string());
        let result = format!("{}", err);
        assert_eq!(result, "Error: Invalid flag '-f'.");
    }

    #[test]
    fn test_display_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let err = ApplicationError::IOError(io_err);
        let result = format!("{}", err);
        assert_eq!(result, "I/O Error: File not found");
    }

    #[test]
    fn test_display_help_requested() {
        let err = ApplicationError::HelpRequested;
        let result = format!("{}", err);
        assert_eq!(result, "Help requested.");
    }
}
