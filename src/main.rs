#![allow(warnings)]

use num_integer::sqrt;
use rayon::prelude::*;
use std::{sync::Arc, time::Instant};

mod conjecture;
mod generate_squares;
mod magic_squares;
mod utils;

fn test_all_N_up_in_range(n_min: u64, n_max: u64, multiprocessing: bool) {
    let start = Instant::now();

    let precomputed_squares_sum = generate_squares::generate_squares_sum_fast(n_max);
    let precomputed_perfect_squares = generate_squares::precompute_perfect_squares(n_max);
    println!("Checking N = {}..{} ", n_min, n_max);

    // Process the range in parallel.
    let numbers: Vec<u64> = (n_min..n_max).collect();

    let mut responses: Vec<magic_squares::Solution> = if multiprocessing {
        numbers
            .par_chunks(1000)
            .flat_map_iter(|batch| {
                batch
                    .iter()
                    .map(|i| {
                        magic_squares::find_perfect_squares(
                            None,
                            Option::from(&precomputed_squares_sum),
                            *i,
                        )
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect::<Vec<_>>()
            })
            .collect()
    } else {
        // Without parallelism
        numbers
            .iter()
            .map(|i| {
                magic_squares::find_perfect_squares(
                    Option::from(&precomputed_perfect_squares),
                    Option::from(&precomputed_squares_sum),
                    *i,
                )
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    };

    responses.sort_by_key(|x| x.N);

    for r in responses {
        println!("------------------------------------------------------");
        if r.solution_type == magic_squares::SolutionType::Perfect {
            println!(">>>> OMG WE FOUND A PERFECT SOLUTION!!! <<<<");
            println!(">>>> WAKE UP HONEY A NEW SOLUTION JUST DROPPED <<<<");
            println!("Perfect Solution for N = {}, e = {}", r.N, r.e);
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                r.a, r.b, r.c, r.d, r.e, r.f, r.g, r.h, r.i
            );
            std::process::exit(0); // Panic to stop the program, making it easier to see the output. Not like it's gonna happen anyway.
        } else {
            // We found a partial solution.
            println!(
                "Partial magic square for N = {}, e = {}, incorrect axes = {:?}",
                r.N, r.e, r.incorrect_axis_values
            );
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                r.a, r.b, r.c, r.d, r.e, r.f, r.g, r.h, r.i
            );

            match conjecture::verify_if_N_matches_conjecture(r.N) {
                Some((p, k)) => {
                    println!("✅ N is in the form of (k*3*p²)² with p={} and k={}", p, k);
                }
                None => {
                    println!("❌😱😱😱😱😱😱😱😱😱 N is not a multiple of (k*3*p)²");
                    panic!("N is not a multiple of (k*3*p²)²");
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Time: {:.2} seconds", duration.as_secs_f64());
}

fn test_kp_form_up_to(max_k: u64, max_p: u64) {
    let start = Instant::now();

    // List all N we are gonna generate (tuples of (k, p, N))
    let N_list: Vec<(u64, u64, u64)> = (1..(max_k + 1))
        .flat_map(|k| {
            (1..(max_p + 1))
                .filter(|&p| utils::is_prime(p) && p % 6 == 1)
                .map(move |p| (k, p, (k * 3 * p.pow(2)).pow(2)))
        })
        .collect();

    // Process without parallelism
    let mut responses: Vec<(u64, u64, u64, Option<magic_squares::Solution>)> = N_list
        .iter()
        .map(|(k, p, N)| {
            (
                *k,
                *p,
                *N,
                magic_squares::find_perfect_squares(None, None, *N),
            )
        })
        .collect();

    responses.sort_by_key(|x| (x.0, x.1));

    for (k, p, N, r) in responses {
        println!("-----------------------");

        match r {
            Some(r) => {
                if r.solution_type == magic_squares::SolutionType::Perfect {
                    panic!(
                        "😱 HONEY WAKE UP we just found a perfect solution found for N = {}",
                        N
                    );
                }

                println!(
                    "✅ [p={}, k={}] N = {} is a quasi magic square in the form of (k*3*p²)² (incorrect axes = {:?})",
                    p, k, N, r.incorrect_axis_values
                );
                println!(
                    "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                    r.a, r.b, r.c, r.d, r.e, r.f, r.g, r.h, r.i
                );
            }
            None => {
                println!("❌ N = {} is NOT a quasi magic square in the form of (k*3*p²)² with p={} and k={}", N, p, k);
            }
        }
    }

    let duration = start.elapsed();
    println!("Time: {:.2} seconds", duration.as_secs_f64());
}

fn generate_large_quasi_magic_square(k: u64, p: u64) {
    let start = Instant::now();

    let N = (k * 3 * p.pow(2)).pow(2);

    match magic_squares::find_perfect_squares(None, None, N) {
        Some(r) => {
            if r.solution_type == magic_squares::SolutionType::Perfect {
                panic!(
                    "😱 HONEY WAKE UP we just found a perfect solution found for N = {}",
                    N
                );
            }

            println!(
                "✅ [p={}, k={}] N = {} is a quasi magic square in the form of (k*3*p²)² (incorrect axes = {:?})",
                p, k, N, r.incorrect_axis_values
            );
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                r.a, r.b, r.c, r.d, r.e, r.f, r.g, r.h, r.i
            );
        }
        None => {
            println!(
                "❌ N = {} is NOT a quasi magic square in the form of (k*3*p²)² with p={} and k={}",
                N, p, k
            );
        }
    }

    let duration = start.elapsed();
    println!("Time: {:.2} seconds", duration.as_secs_f64());
}

const USAGE: &str = "
Usage:
    cargo test_n <n_min> <n_max>  # Test all N in the range [n_min, n_max)
    cargo test_kp <max_k> <max_p> # Test all N in the form of (k*3*p²)² with k in [1, max_k] and p prime in [1, max_p]
    cargo generate <k> <p>        # Generate a large quasi magic square in the form of (k*3*p²)²
";

fn main() {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("{}", USAGE));

    let arg1 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("{}", USAGE))
        .replace("_", "")
        .parse()
        .unwrap();
    let arg2 = std::env::args()
        .nth(3)
        .unwrap_or_else(|| panic!("{}", USAGE))
        .replace("_", "")
        .parse()
        .unwrap();

    if command == "test_n" {
        test_all_N_up_in_range(arg1, arg2, true);
    } else if command == "benchmark_n" {
        test_all_N_up_in_range(arg1, arg2, false);
    } else if command == "test_kp" {
        test_kp_form_up_to(arg1, arg2);
    } else if command == "generate" {
        generate_large_quasi_magic_square(arg1, arg2);
    } else {
        panic!("{}", USAGE);
    }
}
