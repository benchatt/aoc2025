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

fn squidize(cols: Vec<Vec<char>>) -> usize {
    let mut total: usize = 0;
    let mut buffer: Vec<usize> = vec![];
    for mut col in cols {
        if col.iter().collect::<String>().trim() == "" {
            continue;
        }
        col.pop();
        let op = col.pop();
        let stringcol: String = col.into_iter().filter(|c| c != &' ').collect();
        println!("sc %{}%", stringcol);
        let num: usize = stringcol.parse().unwrap();
        match op {
            Some('*') => {
                buffer.push(num);
                for item in &buffer {
                    print!("{} ", item);
                }
                println!("*\n\n");
                total += buffer.iter().product::<usize>();
                buffer = vec![];
            }
            Some('+') => {
                buffer.push(num);
                for item in &buffer {
                    print!("{} ", item);
                }
                println!("+\n\n");
                total += buffer.iter().sum::<usize>();
                buffer = vec![];
            }
            Some(_) => {
                buffer.push(num);
            }
            None => {
                panic!("at the disco");
            }
        }
    }
    total
}

pub fn squid_math() -> () {
    let file_to_open = "src/data/problem06";
    let file = File::open(file_to_open).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().to_owned())
        .collect();

    // let mut table: Vec<Vec<usize>> = vec![];
    // let mut ops: Vec<u8> = vec![];
    let charcols: Vec<Vec<char>> = (0..lines[0].len())
        .map(|n| {
            lines
                .iter()
                .map(|line| match line.chars().nth(n) {
                    Some(c) => c,
                    None => ' ',
                })
                .collect()
        })
        .collect();

    // for line in lines {
    //     let cs: Vec<char> = line.chars().collect();
    //     if cs[0] == '*' || cs[0] == '+' {
    //         ops = parse_math_line(line);
    //     } else {
    //         let tokenized: Vec<usize> = line
    //             .split_whitespace()
    //             .map(|token| token.parse::<usize>().unwrap())
    //             .collect();
    //         table.push(tokenized);
    //     }
    // }
    // let res: Vec<usize> = (0..ops.len())
    //     .map(|n| {
    //         let column = table.iter().map(|row| row[n]).collect();
    //         mathize(column, ops[n])
    //     })
    //     .collect();
    // let sum: usize = res.iter().sum();
    let revcols: Vec<Vec<char>> = charcols.into_iter().rev().collect();
    for cc in &revcols {
        let strize: String = cc.iter().collect();
        println!("@{}@", strize);
    }
    println!("{}", squidize(revcols));
}
