use grid::*;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn parseline(line: &String) -> Vec<u8> {
    return line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect();
}

fn neighbors(grid: &Grid<u8>, row: usize, col: usize) -> Vec<u8> {
    let (rowbound, colbound) = grid.size();
    let rowmin = if row == 0 { 0 } else { row - 1 };
    let rowmax = if row == (rowbound - 1) {
        rowbound - 1
    } else {
        row + 1
    };
    let colmin = if col == 0 { 0 } else { col - 1 };
    let colmax = if col == (colbound - 1) {
        colbound - 1
    } else {
        col + 1
    };

    let mut neighbor_list: Vec<u8> = vec![];
    for irow in rowmin..=rowmax {
        for icol in colmin..=colmax {
            if irow == row && icol == col {
                continue;
            }
            neighbor_list.push(grid[(irow, icol)]);
        }
    }
    neighbor_list
}

fn remove_rolls(grid: &mut Grid<u8>, changes: Vec<(usize, usize)>) -> () {
    changes.into_iter().for_each(|change| -> () {
        if grid[change] == 1 {
            grid[change] = 0
        } else {
            panic!("Can't move a non-existent roll")
        }
    })
}

pub fn route_forklifts() -> () {
    let file = File::open("src/data/problem04").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut floor: Grid<u8> = Grid::new(lines.len(), lines[0].len());
    lines
        .iter()
        .for_each(|line| floor.push_row(parseline(line)));
    let mut counter: u64 = 0;
    let (rows, cols) = floor.size();
    loop {
        let mut changes: Vec<(usize, usize)> = vec![];
        for irow in 0..rows {
            for icol in 0..cols {
                let gridval = floor.get(irow, icol);
                match gridval {
                    Some(1) => {
                        ();
                    }
                    Some(_) => {
                        continue;
                    }
                    None => {
                        ();
                    }
                }
                let nsum = neighbors(&floor, irow, icol).iter().sum::<u8>();
                if nsum < 4 {
                    println!("({}, {}): {}", irow, icol, nsum);
                    changes.push((irow, icol));
                    counter += 1;
                }
            }
        }
        let prerolls = floor.flatten().iter().map(|&u| u as usize).sum::<usize>();
        remove_rolls(&mut floor, changes);
        let postrolls = floor.flatten().iter().map(|&u| u as usize).sum::<usize>();
        if prerolls == postrolls {
            break;
        }
    }
    println!("{}", counter);
}
