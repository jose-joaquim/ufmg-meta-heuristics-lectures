#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

type Individual = (f64, Vec<usize>);
type Population = Vec<Individual>;

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

fn stop_criteria_reached(foo: i32) -> bool {
    return true;
}

fn selection(pop: &Population) -> Population {
    // Binary Tournament Selection
    let mut random_select: Population = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..pop.len() {
        let individual: usize = rng.gen_range(0..pop.len());
        random_select.push((pop[individual].0, pop[individual].1.clone()));
    }

    random_select
}

fn mutation(individual: &Individual) -> Individual {
    (0.0, Vec::new())
}

fn reproduction(pop: &Population) -> Population {
    let mut offspring: Population = Vec::new();
    for i in (0..pop.len()).step_by(2) {
        let (p1, p2) = {
            let i1 = (pop[0].0, pop[0].1.clone());
            let i2 = (pop[1].0, pop[1].1.clone());

            (i1, i2)
        };

        let children: Vec<Individual> = Vec::new();

        for child in children.iter() {
            mutation(child);
            offspring.push((child.0, child.1.clone()));
        }
    }

    offspring
}

fn evaluate_population(pop: &Population) {}

fn create_population(size: &i32, adj: &Vec<Vec<f64>>, start: usize) -> Population {
    (0..*size)
        .map(|_| {
            let n = adj.len();
            let mut route: Vec<usize> = (0..n).collect();
            let mut rng = rand::thread_rng();

            let slice = &mut route[1..];
            slice.shuffle(&mut rng);

            let mut val: f64 = route.windows(2).map(|l| adj[l[0]][l[1]]).sum();
            val += adj[route[route.len() - 1]][route[0]];

            (val, route)
        })
        .collect()
}

fn genetic_algorithm() -> Population {
    let mut population: Population = Vec::new();
    evaluate_population(&population);
    loop {
        if stop_criteria_reached(10) {
            break;
        }

        selection(&population);
        reproduction(&population);
        evaluate_population(&population);
    }

    population
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let adj = read_and_build_adjacency_matrix(&args[1]);
    let root: usize = 0;
    let k_max: usize = 120;

    // let ans: (Individual, f64, Individual, f64) = vnd(&adj.unwrap(), k_max, root);

    // let (ch_sol, ch_time, vnd_sol, vnd_time): (Individual, f64, Individual, f64) =
    //     vnd(&adj.unwrap(), k_max, root);
    // println!(
    //     "{} & {:.2} & {:.2} & {:.2} & {:.2} \\\\",
    //     args[1], ch_sol.0, ch_time, vnd_sol.0, vnd_time
    // );
}
