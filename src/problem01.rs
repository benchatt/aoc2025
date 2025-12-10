use std::fs::File;
use std::io::Error;
use std::io::{self, BufReader, prelude::*};

pub fn main() -> () {
    let file = File::open("src/data/problem01").unwrap();
    let reader = BufReader::new(file);

    let mut n: i16 = 50;
    let mut zeroes: u16 = 0;
    for line_opt in reader.lines() {
        let line = match line_opt {
            Ok(x) => x,
            Err(_) => panic!("at the disco"),
        };
        println!("Starting point: {}, Input line: {}", n, line);
        let sign: i16 = match line.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => panic!("Line must start with L or R"),
        };
        let numchars = &line[1..];
        let addnum = match numchars.parse::<i16>() {
            Ok(x) => x,
            Err(_) => panic!("Couldn't parse"),
        };
        let num = sign * addnum;
        n = n + num;
        while n < 0 {
            n = n + 100;
            zeroes += 1;
        }
        while n > 99 {
            n = n - 100;
            zeroes += 1;
        }
        // if n == 0 {
        //     zeroes += 1;
        // }
    }
    println!("The password is {}", zeroes);
}
