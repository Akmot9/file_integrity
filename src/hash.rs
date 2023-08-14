use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use md5::{Md5, Digest};
use crate::data::{FileInfo, FileList};
use my_logger::log;
use chrono::Local;

pub fn hash_file_list(filename: &str) -> FileList {
    log!("STATUS: Hashing files: Please wait...");
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut liste = FileList {
        date,
        files: vec![],
    };
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            log!("ERROR: Can't open file '{}': {}", filename, error);
            return liste;
        }
    };
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(error) => {
                log!("ERROR: Can't read line from file '{}': {}", filename, error);
                continue;
            }
        };

        // Exclude files that start with '/sys/', '/dev/', '/run/', and '/proc/' from being hashed
        if !line.starts_with("/sys/") && !line.starts_with("/dev/")
            && !line.starts_with("/run/") && !line.starts_with("/proc/") {
            let file_info = hash_file(line);
            liste.files.push(file_info);
        }
    }
    log!("STATUS: Hashing files: Success !");
    liste
}

pub fn hash_file(filename: String) -> FileInfo {
    let mut file = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(_) => {
            log!("ERROR: Can't open file: {}", filename);
            return FileInfo {
                filename,
                md5_hash: String::from("none"),
            };
        }
    };
    let mut buffer = Vec::new();
    if let Err(error) = file.read_to_end(&mut buffer) {
        log!("ERROR: Can't read file: {}", error);
        return FileInfo {
            filename,
            md5_hash: String::from("none"),
        };
    }
    let input = match String::from_utf8(buffer) {
        Ok(input) => input,
        Err(_) => {
            //log!("ERROR: Can't decode file contents: {}", error);
            return FileInfo {
                filename,
                md5_hash: String::from("none"),
            };
        }
    };
    FileInfo {
        filename,
        md5_hash: calculate_md5_hash(&input),
    }
}

fn calculate_md5_hash(input: &String) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    hash_string
}

#[cfg(test)]
mod tests {
    use super::calculate_md5_hash;
    #[test]
    fn test_calculate_md5_hash() {
        let input = String::from("test");
        let expected_hash = "098f6bcd4621d373cade4e832627b4f6";
        let calculated_hash = calculate_md5_hash(&input);
        assert_eq!(calculated_hash, expected_hash);
    }
    
    #[test]
    fn test_hash_file_valid_file() {
        // Test the hash_file function with a valid file path
        // Ensure the correct MD5 hash is generated
    }

    #[test]
    fn test_hash_file_nonexistent_file() {
        // Test the hash_file function with a non-existent file path
        // Ensure the "none" hash is generated
    }

    #[test]
    fn test_hash_file_list_with_test_files() {
        // Create a temporary test file and test the hash_file_list function
        // Ensure correct hash information is generated for each file
    }

    #[test]
    fn test_hash_file_list_exclude_files() {
        // Test scenarios where certain files should be excluded based on their paths
    }

    // Additional tests for other functions can be added here
}