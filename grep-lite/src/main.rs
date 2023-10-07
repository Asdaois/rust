mod parser;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use grep_lite::process_lines;
use regex::Regex;

fn main() {
    let cli = parser::Cli::new();

    let re = Regex::new(&cli.pattern).unwrap();

    let input = cli.input;

    if let Ok(file) = File::open(input) {
        let reader = BufReader::new(file);
        process_lines(reader, re)
    } else {
        println!("The file is not founded fallback to default reader");
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }
}
