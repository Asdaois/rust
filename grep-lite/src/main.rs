mod parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let cli = parser::Cli::new();

    let re = Regex::new(&cli.pattern).unwrap();

    let file = File::open(cli.file_path).unwrap();
    let reader = BufReader::new(file);

    for (i, reader_result) in reader.lines().enumerate() {
        if let Ok(line) = reader_result {
            match re.find(line.as_str()) {
                Some(_) => println!("{}: {}", i + 1, line),
                None => (),
            }
        } else {
            todo!()
        }
    }
}
