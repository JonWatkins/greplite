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

### 1. Basic Search

Search for the pattern "rust" in a file:

```bash
tinygrep "rust" file.txt
```

### 2. Case-insensitive Search

Perform a case-insensitive search using the `-i` option:

```bash
tinygrep -i "rust" file.txt
```

### 3. Search with Line Numbers

Show line numbers of the matching lines using the `-n` option:

```bash
tinygrep -n "error" log.txt
```

### 4. Search Across Multiple Files

Search in multiple files:

```bash
tinygrep "hello" file1.txt file2.txt
```

### 5. Search Using Regular Expressions

TinyGrep supports regular expressions with the `-r` option. For example, to search for lines starting with "Rust" 
(case-sensitive), you can use:

```bash
tinygrep -r "^Rust" file.txt
```

To make the regex search case-insensitive, use both -r and -i:

```bash
tinygrep -r -i "^rust" file.txt
```

### 6. Recursive Search in Directories

Use the `-R` option to search recursively through all files in the specified directory and its subdirectories.

```bash
tinygrep -R "pattern" ./my_directory
```

### 7. Piping Output into TinyGrep

TinyGrep can also be used in combination with commands like cat, echo, or even complex command pipelines. For 
instance, if you want to search for a specific string in a file after filtering the contents with grep, you can 
chain the commands like this:

```bash
cat file.txt | tinygrep "pattern"
```

Using TinyGrep in a Pipeline with Other Filters:

```bash
cat large_log.txt | tinygrep -i "error" | sort | uniq
```

### 8. Display Help

To see the available options and usage instructions, run the command with the `-h` option:
```bash
tinygrep -h
```
