use crate::{
    config::Config,
    error::ApplicationError,
    io::{process_directory, process_file, process_input},
    regex::compile_regex,
};
use std::io;

pub fn run(config: Config) -> Result<(), ApplicationError> {
    let regex = compile_regex(&config.query, config.use_regex, config.ignore_case)?;

    if config.read_from_stdin {
        process_input("stdin", &mut io::stdin().lock(), &config, &regex)?;
    } else {
        for file_path in &config.file_paths {
            let path = std::path::Path::new(file_path);

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
