# TinyGrep

TinyGrep is a simplified version of the `grep` command written in Rust. It allows you to search for a pattern within 
files, with support for case-insensitive searching and line numbers. This is my modified version of the minigrep
implementation in the [Rust Book](https://doc.rust-lang.org).

## Features

* Pattern Search: Search for a pattern (string or regex) within files.
* Case-Insensitive Search: Use the `-i` option for case-insensitive searching.
* Line Numbers: Use the `-n` option to display line numbers alongside matching lines.
* Regular Expression Support: Use the `-r` option to treat the pattern as a regular expression.
* Recursive Search: Use the `-R` option to search files in subdirectories.
* Highlight Matching Text: Use the `-c` option to highlight matching text in the output.
* Help: Use the `-h` option to display help and usage information.
* Pipe Support: Pass input through pipes from other commands, allowing you to use TinyGrep in conjunction with other 
  Unix-like tools.

## Example usage

Once you've built TinyGrep, you can use it as a simplified version of the grep command. Below are a few examples of how 
to use it:

### 1. Basic Search

Search for the pattern "rust" in a file:

```bash
target/release/tinygrep "rust" file.txt
```

### 2. Case-insensitive Search

Perform a case-insensitive search using the `-i` option:

```bash
target/release/tinygrep -i "rust" file.txt
```

### 3. Search with Line Numbers

Show line numbers of the matching lines using the `-n` option:

```bash
target/release/tinygrep -n "error" log.txt
```

### 4. Search Across Multiple Files

Search in multiple files:

```bash
target/release/tinygrep "hello" file1.txt file2.txt
```

### 5. Search Using Regular Expressions

TinyGrep supports regular expressions with the `-r` option. For example, to search for lines starting with "Rust" 
(case-sensitive), you can use:

```bash
target/release/tinygrep -r "^Rust" file.txt
```

To make the regex search case-insensitive, use both -r and -i:

```bash
target/release/tinygrep -r -i "^rust" file.txt
```

### 6. Recursive Search in Directories

Use the `-R` option to search recursively through all files in the specified directory and its subdirectories.

```
target/release/tinygrep -R "pattern" ./my_directory
```

### 7. Piping Output into TinyGrep

TinyGrep can also be used in combination with commands like cat, echo, or even complex command pipelines. For 
instance, if you want to search for a specific string in a file after filtering the contents with grep, you can 
chain the commands like this:

```bash
cat file.txt | target/release/tinygrep "pattern"
```

Using TinyGrep in a Pipeline with Other Filters:

```bash
cat large_log.txt | target/release/tinygrep -i "error" | sort | uniq
```

### 8. Display Help

To see the available options and usage instructions, run the command with the `-h` option:
```
target/release/tinygrep -h
```

## Install Rust

To get started with TinyGrep, you'll need to have Rust installed on your machine. You can easily install Rust by following 
the instructions on the official website:

1. Go to the [Rust installation page](https://www.rust-lang.org/learn/get-started).
2. Follow the instructions for your operating system to install `rustup`, the Rust toolchain installer.

Once installed, you can check if Rust is successfully installed by running:

```sh
rustc --version
```

This should print the installed Rust version.

## Install TinyGrep

TinyGrep is a command-line tool written in Rust, which can be installed by cloning this repository.

1. Clone the repository:
   ```bash
   git clone https://github.com/JonWatkins/tinygrep.git
   ```

2. Change into the project directory:
   ```bash
   cd tinygrep
   ```

## Build TinyGrep

To build TinyGrep, you need to use `cargo`, the Rust package manager and build system. After navigating to 
the project directory, run the following command to build the project:

```bash
cargo build --release
```

This will build the project in release mode, which optimizes for performance.

Once the build completes, you can find the `tinygrep` executable in the `target/release/` directory.

## Testing TinyGrep
TinyGrep comes with a simple set of tests to ensure everything works as expected. You can run the tests using the 
built-in test suite provided by Rust.
To run the tests, simply execute the following command:

```bash
cargo test
```

## Troubleshooting

If you run into any issues, here are a few things to check:

* Make sure that you've installed Rust correctly by running rustc --version.
* Ensure that you're building TinyGrep in the correct directory by running cargo build --release from the tinygrep project folder.
* Check for error messages in the terminal if any commands fail to run and ensure that the file paths are correct.
