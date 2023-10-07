use std::io::BufRead;

use regex::Regex;

pub fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    reader
        .lines()
        .enumerate()
        .for_each(|(i, reader_result)| match reader_result {
            Ok(line) => match re.find(line.as_str()) {
                Some(_) => println!("{}: {}", i + 1, line),
                None => (),
            },
            _ => (),
        });
}
