# TinyGrep

TinyGrep is a simplified version of the `grep` command written in Rust. It allows you to search for a pattern within 
files, with support for case-insensitive searching and line numbers. This is my modified version of the minigrep
implementation in the [Rust Book](https://doc.rust-lang.org).

## Example usage

Once you've built TinyGrep, you can use it as a simplified version of the grep command. Below are a few examples of how 
to use it:

### Basic Search

Search for the pattern "rust" in a file:

```bash
target/release/tinygrep "rust" file.txt
```

### Case-insensitive Search

Perform a case-insensitive search using the `-i` option:

```bash
target/release/tinygrep -i "rust" file.txt
```

### Search with Line Numbers

Show line numbers of the matching lines using the `-n` option:

```bash
target/release/tinygrep -n "error" log.txt
```

### Search Across Multiple Files

Search in multiple files:

```bash
target/release/tinygrep "hello" file1.txt file2.txt
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

## Testing TinyGrep
TinyGrep comes with a simple set of tests to ensure everything works as expected. You can run the tests using the 
built-in test suite provided by Rust.
To run the tests, simply execute the following command:

```bash
cargo test
```
