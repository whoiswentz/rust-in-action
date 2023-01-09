
use std::{io::{BufRead, BufReader, self}, fs::File};

use clap::{arg, Command};
use regex::Regex;

fn main() {
    let matches = Command::new("GrepLite")
        .version("1.0")
        .author("Vinicios Wentz")
        .about("search for pattern")
        .arg(arg!(-p --pattern <PATTERN> "Pattern to look at").required(true))
        .arg(arg!(-i --input <INPUT> "Input to match with pattern").required(false))
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    if let Some(input) = matches.get_one::<String>("input") {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        process_line(reader, re);
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_line(reader, re);
    }
}

fn process_line<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let unwraped_line = line.unwrap();
        match re.find(&unwraped_line) {
            Some(_) => println!("{}", unwraped_line),
            None => ()
        }
    }
}