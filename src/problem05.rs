use std::fs::File;
use std::io::{BufReader, prelude::*};

fn run_check(id: usize, range: &(usize, usize)) -> bool {
    if id >= range.0 && id <= range.1 {
        return true;
    }
    false
}

fn combiner(r1: &(usize, usize), r2: &(usize, usize)) -> Vec<(usize, usize)> {
    let low = if r1.0 < r2.0 { r1 } else { r2 };
    let high = if low == r1 { r2 } else { r1 };
    if (low.1 + 1) < high.0 {
        return vec![*low, *high];
    }
    let bound = if low.1 > high.1 { low.1 } else { high.1 };
    vec![(low.0, bound)]
}

pub fn check_freshness() -> () {
    let file = File::open("src/data/problem05").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect();

    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut ids: Vec<usize> = vec![];

    let mut flag: bool = false;
    for line in lines {
        // println!("{}", line);
        if flag {
            ids.push(line.parse::<usize>().unwrap())
        } else {
            if line == "" {
                flag = true;
                continue;
            }
            let range: Vec<usize> = line
                .split("-")
                .map(|r| r.parse::<usize>().unwrap())
                .collect();
            if range.len() != 2 {
                panic!("Incorrect range")
            }
            ranges.push((range[0], range[1]))
        }
    }
    ranges.sort_by_key(|n| n.0);
    loop {
        // for range in &ranges {
        //     println!("#{}-{}", range.0, range.1);
        // }
        // println!("\n===");
        let prerange = ranges.len();
        let starter = combiner(&ranges[0], &ranges[1]);
        ranges = ranges.into_iter().fold(starter, |mut acc, range| {
            let last = acc.pop().unwrap();
            acc.append(&mut combiner(&last, &range));
            acc
        });
        ranges.dedup();
        if ranges.len() == prerange {
            break;
        }
    }

    let mut rangeadd = 0;
    for range in &ranges {
        println!("{}-{}", range.0, range.1);
        rangeadd += range.1 - range.0 + 1;
    }
    println!("{}", rangeadd);

    // let mut count = 0;
    // for id in ids {
    //     for range in &ranges {
    //         if run_check(id, range) {
    //             count += 1;
    //             break;
    //         }
    //     }
    // }
    // println!("{}", count);
}
