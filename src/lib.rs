// src/lib.rs

mod data;
mod hash;
mod json;

pub use hash::{hash_file, hash_file_list};
pub use json::write_json_file;
pub use data::{FileList, FileInfo};