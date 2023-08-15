# file_integrity

[![Crates.io](https://img.shields.io/crates/v/file_integrity.svg)](https://crates.io/crates/file_integrity)

The *File Integrity Library* is a Rust crate designed to help you calculate MD5 hashes for files and generate JSON reports of the file integrity status. This can be useful for verifying the integrity of files in a list, such as during software distribution or system monitoring.

## Features

- Calculate hashes for a list of files.
- Calculate hashes for file contents.
- Generate JSON reports of file integrity status.

## Installation
Add this crate to your Cargo.toml:

```toml
[dependencies]
file_integrity = "0.1.2"
```

## Usage
### Hashing a the all computer files
```rust
fn main() {
    let folder_path = "/" ;
    let nbs_of_file = list_files(&folder_path);
    log!("INFOS: Number of files: {nbs_of_file}");

    let file_path_list = "file_list.txt";
    
    let hashs = hash_file_list(file_path_list);
    let name = "outp2ut.json" ; 
    write_json_file(&hashs, &name);
}
```
