use crate::{highlight::highlight_match, search::search, ApplicationError, Config};
use regex::Regex;
use std::{fs, io::Read};

pub fn process_input<R: Read>(
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

pub fn process_file(
    file_path: &str,
    config: &Config,
    regex: &Option<Regex>,
) -> Result<(), ApplicationError> {
    let content = fs::read_to_string(file_path)
        .map_err(|_| ApplicationError::FileNotFound(file_path.to_string()))?;

    let results = search(&config.query, &content, config.ignore_case, regex);
    print_results(config, file_path, results, regex)
}

pub fn process_directory(
    dir_path: &std::path::Path,
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
