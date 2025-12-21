use std::fs::File;
use std::io::{BufReader, prelude::*};

fn step_beam(prev: String, curr: String) -> (String, usize) {
    // println!("prev: {}\ncurr: {}", prev, curr);
    let incoming: Vec<usize> = prev
        .chars()
        .map(|c| if c == 'S' || c == '|' { 1 } else { 0 })
        .collect();
    let existing: Vec<usize> = curr.chars().map(|c| if c == '^' { 1 } else { 0 }).collect();
    let hits: Vec<usize> = (0..incoming.len())
        .filter(|&i| incoming[i] & existing[i] == 1)
        .collect();
    let lefts: Vec<usize> = hits.iter().filter(|&h| h > &0).map(|h| h - 1).collect();
    let rights: Vec<usize> = hits
        .iter()
        .map(|h| h + 1)
        .filter(|&h| h < existing.len())
        .collect();
    let outgoing: String = (0..existing.len())
        .map(|i| {
            if lefts.contains(&i) || rights.contains(&i) || (incoming[i] == 1 && !hits.contains(&i))
            {
                '|'
            } else {
                curr.chars().nth(i).unwrap()
            }
        })
        .collect();
    // println!("out : {}\n===", outgoing);
    (outgoing, hits.iter().count())
}

fn step_quantum(incoming: Vec<usize>, curr: String) -> Vec<usize> {
    let existing: Vec<usize> = curr.chars().map(|c| if c == '^' { 1 } else { 0 }).collect();
    let hits: Vec<usize> = (0..incoming.len())
        .map(|i| {
            if incoming[i] > 0 && existing[i] > 0 {
                1
            } else {
                0
            }
        })
        .collect();
    let follows: Vec<usize> = incoming
        .iter()
        .enumerate()
        .map(|(i, &val)| if hits[i] == 1 { 0 } else { val })
        .collect();
    let lefts: Vec<usize> = (0..incoming.len())
        .map(|i| {
            if i == incoming.len() - 1 {
                0
            } else {
                if hits[i + 1] == 1 { incoming[i + 1] } else { 0 }
            }
        })
        .collect();
    let rights: Vec<usize> = (0..incoming.len())
        .map(|i| {
            if i == 0 {
                0
            } else {
                if hits[i - 1] == 1 { incoming[i - 1] } else { 0 }
            }
        })
        .collect();
    let outgoing: Vec<usize> = (0..existing.len())
        .map(|i| follows[i] + lefts[i] + rights[i])
        .collect();
    outgoing
}

pub fn count_splits() -> () {
    let file_to_open = "src/data/problem07";
    let file = File::open(file_to_open).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect();

    let mut this_line: Vec<usize> = lines[0]
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();
    for i in 1..lines.len() - 1 {
        let changed_line = step_quantum(this_line, lines[i].clone());
        this_line = changed_line;
    }
    let count: usize = this_line.iter().sum();

    // {
    //     let mut this_line: String = lines[0].clone();
    //     let mut count = 0;
    //     for i in 1..lines.len() - 1 {
    //         let (changed_line, incr) = step_beam(this_line, lines[i].clone());
    //         this_line = changed_line;
    //         count += incr;
    //     }
    // }
    println!("{}", count);
}
