#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

type Solution = (f64, Vec<usize>);

fn read_and_build_adjacency_matrix(file_path: &str) -> Option<Vec<Vec<f64>>> {
    // println!("Reading Instance");
    let input: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut coordinate: Vec<[f64; 2]> = vec![];
    for i in 6..input.len() - 1 {
        let mut parts = input[i].trim().split_whitespace();
        let words: Vec<&str> = input[i].trim().split_whitespace().collect();
        println!("{:?}", words);

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

    // println!("Read instance");

    return Some(adj);
}

fn explore_neighborhood(adj: &Vec<Vec<f64>>, inc: &Solution, mut k: usize) -> Solution {
    // println!("Exploring neighborhood");
    // println!("inc solution is {:?}", inc.1);

    let (mut cost, mut tour): (f64, Vec<usize>) = (0.0, Vec::new());
    while k > 0 {
        (cost, tour) = (inc.0, inc.1.clone());
        let mut rng = rand::thread_rng();
        let range = 1..inc.1.len() - 1;

        // 1. Select the sub-tour that is going to be reversed
        let (sub_i, sub_j): (usize, usize) = {
            let mut sub_tour: Vec<usize> = range.choose_multiple(&mut rng, 2);
            if sub_tour[0] > sub_tour[1] {
                sub_tour.swap(0, 1);
            }

            (sub_tour[0], sub_tour[1])
        };

        // 2. Update solution info
        tour = tour[..=sub_i]
            .iter()
            .chain(tour[sub_i + 1..=sub_j].iter().rev())
            .chain(tour[sub_j + 1..].iter())
            .copied()
            .collect();

        assert_eq!(inc.1.len(), tour.len());

        cost = tour.windows(2).map(|l| adj[l[0]][l[1]]).sum();
        cost += adj[tour[tour.len() - 1]][tour[0]];
        k -= 1;
    }

    return (cost, tour);
}

fn vnd(adj: &Vec<Vec<f64>>, k_max: usize, root: usize) -> (Solution, f64, Solution, f64) {
    // TODO: must guarantee that k_max <= adj.len()
    let start = Instant::now(); // Start the timer
    let initial_sol: Solution = run_nearest_neighbor(&adj, root);
    let ch_time = start.elapsed().as_secs_f64();

    let mut incumbent = (initial_sol.0, initial_sol.1.clone());

    let vnd_start = Instant::now();
    loop {
        let mut k: usize = 1;
        let mut improved: bool = false;
        while k <= k_max {
            improved = false;
            let local_optima = explore_neighborhood(&adj, &incumbent, k);

            if local_optima.0.lt(&incumbent.0) {
                // println!("IMPROVED SOLUTION");
                // println!("Solution OF {}, tour {:?}", local_optima.0, local_optima.1);
                improved = true;
                incumbent = local_optima;
                k = 1;
            } else {
                k += 1;
            }
        }

        if improved == false {
            break;
        }
    }
    let vnd_time = vnd_start.elapsed().as_secs_f64();
    return (initial_sol, ch_time, incumbent, vnd_time);
}

fn run_nearest_neighbor(adj: &Vec<Vec<f64>>, start: usize) -> Solution {
    // println!("Running CH");
    let mut val: f64 = 0.0;
    let n = adj.len();

    let mut route: Vec<usize> = vec![];

    let mut visited: HashSet<usize> = HashSet::new();
    let mut current: usize = 0;

    visited.insert(current);
    loop {
        route.push(current);
        let mut next: Option<usize> = None;
        let mut dist_next: f64 = 100000.0;
        for j in 0..n {
            if current == j {
                continue;
            }

            if adj[current][j].lt(&dist_next) && !visited.contains(&j) {
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

    val += adj[route[route.len() - 1]][start];
    // println!("Built solution with OF {:.2}", val);
    return (val, route);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let adj = read_and_build_adjacency_matrix(&args[1]);
    let root: usize = 0;
    let k_max: usize = 120;

    // let ans: (Solution, f64, Solution, f64) = vnd(&adj.unwrap(), k_max, root);

    let (ch_sol, ch_time, vnd_sol, vnd_time): (Solution, f64, Solution, f64) =
        vnd(&adj.unwrap(), k_max, root);
    println!(
        "{} & {:.2} & {:.2} & {:.2} & {:.2} \\\\",
        args[1], ch_sol.0, ch_time, vnd_sol.0, vnd_time
    );
}
