#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

type Individual = (f64, Vec<usize>);
type Population = Vec<Individual>;

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

fn stop_criteria_reached(start: Instant) -> bool {
    return start.elapsed().as_secs_f64() > 15.0;
}

fn selection(pop: &Population) -> Population {
    let mut random_select: Population = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..pop.len() {
        let individual: usize = rng.gen_range(0..pop.len());
        random_select.push((pop[individual].0, pop[individual].1.clone()));
    }

    random_select
}

fn mutation(individual: &mut Individual) {
    let mut rng = rand::thread_rng();
    let mut lb: usize = rng.gen_range(0..individual.1.len());
    let mut ub: usize = rng.gen_range(0..individual.1.len());

    (individual.1[lb], individual.1[ub]) = (individual.1[ub], individual.1[lb]);
}

fn all_unique(vec: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    vec.into_iter().all(|x| seen.insert(x))
}

fn crossover(parent1: &Individual, parent2: &Individual) -> Vec<Individual> {
    let n: usize = parent1.1.len();
    let (mut off1, mut off2): (Vec<usize>, Vec<usize>) = (vec![n + 1; n], vec![n + 1; n]);

    let (lb, ub): (usize, usize) = {
        let mut rng = rand::thread_rng();
        let mut lb_: usize = rng.gen_range(0..n - 1);
        let mut ub_: usize = rng.gen_range(0..n - 1);

        while lb_ == ub_ {
            ub_ = rng.gen_range(0..n - 1);
        }

        if lb_ > ub_ {
            std::mem::swap(&mut lb_, &mut ub_);
        }

        (lb_, ub_)
    };

    let mut replacement1: Vec<i32> = vec![-1; n];
    let mut replacement2: Vec<i32> = vec![-1; n];

    for idx in lb..=ub {
        off1[idx] = parent2.1[idx];
        off2[idx] = parent1.1[idx];

        replacement1[parent2.1[idx]] = parent1.1[idx] as i32;
        replacement2[parent1.1[idx]] = parent2.1[idx] as i32;
    }

    for i in 0..n {
        if i >= lb && i <= ub {
            continue;
        }

        let mut n1: usize = parent1.1[i];
        let mut m1: i32 = replacement1[n1];

        while m1 != -1 {
            n1 = m1 as usize;
            m1 = replacement1[m1 as usize];
        }

        let mut n2 = parent2.1[i];
        let mut m2 = replacement2[n2];

        while m2 != -1 {
            n2 = m2 as usize;
            m2 = replacement2[m2 as usize];
        }

        off1[i] = n1;
        off2[i] = n2;
    }

    vec![(0.0, off1), (0.0, off2)]
}

fn reproduction(pop: &Population) -> Population {
    let mut offspring: Population = Vec::new();
    for i in (0..pop.len()).step_by(2) {
        let (p1, p2) = {
            let i1 = (pop[0].0, pop[0].1.clone());
            let i2 = (pop[1].0, pop[1].1.clone());

            (i1, i2)
        };

        let mut children: Vec<Individual> = crossover(&p1, &p2);

        for child in &mut children {
            mutation(child);
            offspring.push((child.0, child.1.clone()));
        }
    }

    offspring
}

fn evaluate_population(pop: &mut Population, adj: &Vec<Vec<f64>>) {
    pop.iter_mut().for_each(|(val, route)| {
        *val = route.windows(2).map(|l| adj[l[0]][l[1]]).sum();
        *val += adj[route[route.len() - 1]][route[0]];
    });
}

fn create_population(size: &usize, adj: &Vec<Vec<f64>>, start: &usize) -> Population {
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

fn replacement(mut population: Population, mut offspring: Population) -> Population {
    population.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    offspring.extend(population[0..=1].into_iter().cloned());
    offspring.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    offspring[0..(offspring.len() - 2)].to_vec()
}

fn genetic_algorithm(
    adj: &Vec<Vec<f64>>,
    start: usize,
    pop_size: usize,
) -> (Population, Population) {
    let mut population: Population = create_population(&pop_size, &adj, &start);

    let init_pop: Population = population.clone();
    let start = Instant::now();
    loop {
        if stop_criteria_reached(start) {
            break;
        }

        let mating = selection(&population);
        let mut offspring = reproduction(&mating);

        evaluate_population(&mut offspring, &adj);

        population = replacement(population, offspring);
    }
    (init_pop, population)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let adj = read_and_build_adjacency_matrix(&args[1]);
    let root: usize = 0;
    let pop_size = 15;

    let (init_pop, population) = genetic_algorithm(&adj.unwrap(), 0, pop_size);

    println!("{}", args[1]);
    println!("Initial_Population,Final_Population");
    for i in 0..pop_size {
        println!("{:.2},{:.2}", init_pop[i].0, population[i].0);
    }
}
