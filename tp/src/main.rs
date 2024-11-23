#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::env;
use std::fs;

fn read_and_build_adjacency_matrix(file_path: &str) -> Option<Vec<Vec<f64>>> {
    let input: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut coordinate: Vec<[f64; 2]> = vec![];
    for i in 6..input.len() - 1 {
        let mut parts = input[i].trim().split_whitespace();
        let words: Vec<&str> = input[i].trim().split_whitespace().collect();

        let x: f64 = words[1].parse().unwrap();
        let y: f64 = words[2].parse().unwrap();
        coordinate.push([x, y]);

        let seila = x.powi(2);
        let seila2 = y.powi(2);
        let rst = (seila + seila2).sqrt();
    }

    let mut adj: Vec<Vec<f64>> = Vec::new();

    for i in 0..coordinate.len() {
        let a = coordinate[i];
        adj.push(vec![]);
        for j in 0..coordinate.len() {
            let b = coordinate[j];
            let dist = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt();
            adj[i].push(dist);
        }
    }

    return Some(adj);
}

fn run_nearest_neighbor(adj: &Vec<Vec<f64>>, start: usize) -> (f64, Vec<usize>) {
    let mut val: f64 = 0.0;
    let n = adj.len();

    let mut route: Vec<usize> = vec![];
    route.push(start);

    // 1. Compute initial solution

    let mut visited: HashSet<usize> = HashSet::new();
    let mut current: usize = 0;

    visited.insert(current);
    loop {
        let mut next: Option<usize> = None;
        let mut dist_next: f64 = 100000.0;
        for j in 0..n {
            if current == j {
                continue;
            }

            if adj[current][j] < dist_next && !visited.contains(&j) {
                dist_next = adj[current][j];
                next = Some(j);
            }
        }

        if next.is_none() {
            break;
        } else {
            current = next.unwrap();
            visited.insert(current);
            val += dist_next;
        }
    }

    // println!(
    //     "initial objective function is {}, visited {} nodes",
    //     val,
    //     visited.len()
    // );

    return (val, route);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let adj = read_and_build_adjacency_matrix(&args[1]);
    let root: usize = 0;

    let ans: (f64, Vec<usize>) = run_nearest_neighbor(&adj.unwrap(), root);
    println!("{} & {:.2} \\\\", args[1], ans.0);
}
