use grid::*;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn parse_math_line(line: String) -> Vec<u8> {
    line.split_whitespace()
        .map(|op| if op == "*" { 1 } else { 0 })
        .collect()
}

fn mathize(column: Vec<usize>, op: u8) -> usize {
    // println!("doing {} to {}", op, format!("{:?}", column));
    if op == 0 {
        column.iter().sum()
    } else {
        column.iter().product()
    }
}

pub fn squid_math() -> () {
    let file_to_open = "src/data/problem06";
    let file = File::open(file_to_open).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect();

    let mut table: Vec<Vec<usize>> = vec![];
    // let mut table: Grid<usize> = Grid::new(lines.len() - 1, lines[0].split_whitespace().count());
    let mut ops: Vec<u8> = vec![];
    for line in lines {
        // println!("{}", &line);
        let cs: Vec<char> = line.chars().collect();
        if cs[0] == '*' || cs[0] == '+' {
            ops = parse_math_line(line);
        } else {
            let tokenized: Vec<usize> = line
                .split_whitespace()
                .map(|token| token.parse::<usize>().unwrap())
                .collect();
            // println!("|{}", format!("{:?}", tokenized));
            table.push(tokenized);
        }
    }
    let res: Vec<usize> = (0..ops.len())
        .map(|n| {
            let column = table.iter().map(|row| row[n]).collect();
            mathize(column, ops[n])
        })
        .collect();
    let sum: usize = res.iter().sum();
    println!("{}", sum);
}
