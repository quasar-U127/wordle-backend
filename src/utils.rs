use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

pub fn get_word_list(path:&Path)->Vec<String>{
    let file = match File::open(path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open the file {}: {}",path.display(),reason),
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let words:Vec<String> = lines.filter(|s| s.is_ok()).map(|s| s.unwrap()).collect();
    return words;
}