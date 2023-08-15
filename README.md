# file_intergity

[![Crates.io](https://img.shields.io/crates/v/file_integrity.svg)](https://crates.io/crates/file_integrity)

The *File Integrity Library* is a Rust crate designed to help you calculate MD5 hashes for files and generate JSON reports of the file integrity status. This can be useful for verifying the integrity of files in a list, such as during software distribution or system monitoring.

## Features

- Calculate MD5 hashes for file contents.
- Generate JSON reports of file integrity status.

## Installation
Add this crate to your Cargo.toml:

```toml
[dependencies]
file_integrity_lib = "0.1.0"
```

## Usage
### Hashing a Single File
```rust
use file_integrity_lib::hash_file;

fn main() {
    let filename = "path/to/your/file.txt".to_string();
    let result = hash_file(filename.clone());
    println!("Filename: {}", result.filename);
    println!("MD5 Hash: {}", result.md5_hash);
}
```
### Hashing Files from a List
```rust
use file_integrity_lib::hash_file_list;

fn main() {
    let filename = "path/to/your/file_list.txt";
    let file_list = hash_file_list(filename);
    println!("Date: {}", file_list.date);
    println!("Number of Files: {}", file_list.files.len());
}
```
### Writing JSON Reports
```rust
use file_integrity_lib::{hash_file_list, write_json_file, FileList};

fn main() {
    let filename = "path/to/your/file_list.txt";
    let file_list = hash_file_list(filename);
    write_json_file(&file_list);
}
```