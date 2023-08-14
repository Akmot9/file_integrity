use std::fs::File;
use std::io::Write;
use serde_json;
use crate::data::FileList;
use my_logger::log;

pub fn write_json_file(hash_set: &FileList) {
    log!("STATUS: Writing json: Please wait...");
    let json_output = match serde_json::to_string_pretty(&hash_set) {
        Ok(output) => output,
        Err(err) => {
            log!("ERROR: Failed to serialize to JSON: {}", err);
            return;
        }
    };

    let output_file = "output.json";
    match File::create(output_file) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(json_output.as_bytes()) {
                log!("ERROR: Failed to write JSON to file: {}", err);
            } else {
                log!("STATUS: Writing json: Success !");
            }
        }
        Err(err) => {
            log!("ERROR: Failed to create output file: {}", err);
        }
    }
}