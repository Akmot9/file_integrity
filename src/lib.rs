//! # File Integrity Library
//!
//! `file_integrity` is a Rust library that provides functions for ensuring the integrity of files.
//!
//! This library allows you to calculate MD5 hashes of file contents, generate JSON reports of file hashes,
//! and more. It is designed to help developers verify the integrity of files in various scenarios.
//!
//! ## Usage
//!
//! To use this library, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! file_integrity = "0.1"
//! ```
//!
//! Then, you can import and use the functions provided by the library in your code.
//!
//! ## Examples
//!
//! ```rust
//! use file_integrity::hash_file;
//!
//! let file_path = String::from("path/to/your/file.txt");
//! let result = hash_file(file_path.clone());
//!
//! // Add more examples and explanations here...
//! ```
//!
//! For more detailed information and examples, refer to the documentation of individual functions and modules.
//!
//! # License
//!
//! This project is licensed under the terms of the MIT license.
//!
//! For further details, please refer to the [LICENSE](https://github.com/Akmot9/file_integrity/blob/main/LICENSE) file.
//!
//! # Contributing
//!
//! Contributions to this library are welcome! Please follow the guidelines outlined in the [CONTRIBUTING](https://github.com/Akmot9/file_integrity) file.
mod data;
mod hash;
mod json;

#[doc(hidden)]
pub use hash::{hash_file, hash_file_list, calculate_md5_hash};
pub use json::write_json_file;
pub use data::{FileList, FileInfo};

