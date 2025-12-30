use itertools::Itertools;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn rect_size(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let x = vec![a.0, b.0];
    let y = vec![a.1, b.1];
    let width = x.iter().max().unwrap() - x.iter().min().unwrap() + 1;
    let height = y.iter().max().unwrap() - y.iter().min().unwrap() + 1;
    println!("({},{}), ({},{})", a.0, a.1, b.0, b.1);
    println!("Width: {}, Height: {}", width, height);
    println!("Area: {}\n\n", width * height);
    width * height
}

pub fn find_rectangle() -> () {
    let file_to_open = "src/data/problem09";
    let file = File::open(file_to_open).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect();

    let points: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| {
            let vecpts: Vec<usize> = line
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (vecpts[0], vecpts[1])
        })
        .collect();
    let areas: Vec<usize> = points
        .iter()
        .combinations(2)
        .map(|pts| rect_size(pts[0], pts[1]))
        .collect();
    println!("{}", areas.iter().max().unwrap());
}
