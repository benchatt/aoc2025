use grid::*;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn parseline(line: String) -> Vec<u8> {
    return line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect();
}

pub fn route_forklifts() -> () {
    let file = File::open("src/data/problem03").unwrap();
    let reader = BufReader::new(file);
    let floor = reader.lines().map(|line| parseline(line.unwrap()));
}
