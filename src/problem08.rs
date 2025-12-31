use rust_3d::*;
use std::f64;
use std::fs::File;
use std::io::{BufReader, prelude::*};

struct Pair<'a> {
    nodes: (&'a Point3D, &'a Point3D),
    distance: f64,
}

impl<'a> Pair<'a> {
    fn new(a: &'a Point3D, b: &'a Point3D) -> Pair<'a> {
        let distance: f64 = rust_3d::dist_3d(a, b);
        Pair {
            nodes: (a, b),
            distance: distance,
        }
    }
}

struct Circuit<'a> {
    nodes: Vec<&'a Point3D>,
}

impl<'a> Circuit<'a> {
    fn from_pair(pair: &Pair<'a>) -> Circuit<'a> {
        Circuit {
            nodes: vec![pair.nodes.0, pair.nodes.1],
        }
    }

    fn add_pair(&mut self, pair: &Pair<'a>) -> () {
        if !self.nodes.contains(&pair.nodes.0) {
            self.nodes.push(&pair.nodes.0)
        }
        if !self.nodes.contains(&pair.nodes.1) {
            self.nodes.push(&pair.nodes.1)
        }
    }

    fn join(&mut self, circuit: &mut Circuit<'a>) -> () {
        for node in &circuit.nodes {
            if !self.nodes.contains(&node) {
                self.nodes.push(node)
            }
        }
        circuit.nodes.clear();
    }

    fn connects(&self, pair: &Pair<'a>) -> bool {
        self.nodes.contains(&pair.nodes.0) || self.nodes.contains(&pair.nodes.1)
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }
}

fn pt_from(line: &String) -> Point3D {
    let pts: Vec<f64> = line.split(",").map(|s| s.parse::<f64>().unwrap()).collect();
    if pts.len() != 3 {
        panic!("at the point parser");
    }
    Point3D::new(pts[0], pts[1], pts[2])
}

fn get_circuit_count<'a>(pairs: Vec<&'a Pair>) -> Vec<usize> {
    let mut circuits: Vec<Circuit> = vec![];
    for pair in &pairs {
        println!("----{} {}----", pair.nodes.0, pair.nodes.1);
        let mut matched: &mut Circuit = &mut Circuit { nodes: vec![] };
        for circuit in &mut circuits {
            if circuit.connects(pair) {
                if matched.nodes.len() > 0 {
                    circuit.join(matched);
                } else {
                    circuit.add_pair(pair);
                }
                for el in &circuit.nodes {
                    print!("{} ", el);
                }
                println!("add {}, {}", pair.nodes.0, pair.nodes.1);
                matched = circuit;
            }
        }
        if matched.nodes.len() == 0 {
            circuits.push(Circuit::from_pair(*pair))
        }
    }
    println!("----- -----");
    for circuit in &circuits {
        for node in &circuit.nodes {
            print!("{} ", node);
        }
        println!("===");
    }

    circuits.iter().map(|circuit| circuit.size()).collect()
}

fn product_of_largest(mut sizes: Vec<usize>) -> usize {
    sizes.sort();
    sizes.reverse();
    sizes[..3].iter().product()
}

pub fn connect_lights() -> () {
    let target_num = 10;
    let file_to_open = "src/data/problem08ex";
    let file = File::open(file_to_open).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect();
    let points: Vec<Point3D> = lines.iter().map(|line| pt_from(line)).collect();
    let mut pairs: Vec<Pair> = vec![];
    for i in 0..points.len() - 1 {
        for j in (i + 1)..points.len() {
            pairs.push(Pair::new(&points[i], &points[j]))
        }
    }
    let mut shortest: Vec<&Pair> = vec![];
    for pair in &pairs {
        if shortest.len() < target_num {
            shortest.push(pair);
            continue;
        }
        for i in 0..target_num {
            if pair.distance < shortest[i].distance {
                shortest.insert(i, pair);
                shortest.pop();
                break;
            }
        }
    }

    for s in &shortest {
        println!("{}, {}: {}", s.nodes.0, s.nodes.1, s.distance);
    }

    let circuit_sizes = get_circuit_count(shortest);
    println!("{}", product_of_largest(circuit_sizes));
}
