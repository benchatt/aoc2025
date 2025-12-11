use std::fs;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn bat_to_jolts(batteries: Vec<usize>) -> usize {
    let mut accu: usize = 0;
    for (i, battery) in batteries.iter().enumerate() {
        accu += battery * (10usize.pow((batteries.len() - i - 1) as u32))
    }
    accu
}

fn top_jolts(joltimeter: &String, n: usize) -> usize {
    let batteries: Vec<usize> = (0..joltimeter.len())
        .map(|i| joltimeter[i..i + 1].parse().unwrap())
        .collect();
    let mut front = 0;
    let mut batvec: Vec<usize> = vec![];
    for back_offset in 0..n {
        let back = batteries.len() - (n - back_offset - 1);
        if front >= back {
            continue;
        }
        println!("{} ({}): {}-{}", joltimeter, batteries.len(), front, back);
        let maxpos = batteries[front..back]
            .iter()
            .position(|&b| &b == batteries[front..back].iter().max().unwrap())
            .unwrap()
            + front;
        front = maxpos + 1;
        batvec.push(batteries[maxpos])
    }
    bat_to_jolts(batvec)
}

pub fn find_joltage() -> () {
    let file = File::open("src/data/problem03").unwrap();
    let reader = BufReader::new(file);

    let mut total_jolts: usize = 0;
    for line_opt in reader.lines() {
        let line = match line_opt {
            Ok(x) => x,
            Err(_) => panic!("at the disco"),
        };
        let jolts = top_jolts(&line, 12);
        println!("b: {}, j: {}", line, jolts);
        total_jolts += jolts;
    }
    println!("Total jolts: {}", total_jolts);
}
