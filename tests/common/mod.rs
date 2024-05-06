use std::fs::File;
use std::io::{self, BufRead};

pub fn load_data(file_name: &str) -> Vec<f64> {
    let file = File::open(file_name).expect("Should exist, was commited to the repo");
    let reader = io::BufReader::new(file);

    let mut data: Vec<f64> = Vec::new();

    for line in reader.lines() {
        if let Ok(value_str) = line {
            if let Ok(value) = value_str.parse::<f64>() {
                data.push(value);
            } else {
                println!("Failed to parse line: {}", value_str);
            }
        } else {
            println!("Failed read line from file");
        }
    }

    data
}
