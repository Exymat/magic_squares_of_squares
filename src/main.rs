#![allow(warnings)]

use rayon::prelude::*;
use std::time::Instant;

mod conjecture;
mod generate_squares;
mod magic_squares;
mod utils;

/// Test all values of N in the range [n_min, n_max). When `multiprocessing` is true,
/// a parallel iterator is used.
fn test_all_N_up_in_range(n_min: u64, n_max: u64, multiprocessing: bool) {
    let start = Instant::now();

    // Precompute common data.
    let squares_sum = generate_squares::generate_squares_sum_fast(n_max);
    let perfect_squares = generate_squares::precompute_perfect_squares(n_max);

    println!("Checking N = {}..{}", n_min, n_max);

    // Define a closure that attempts to find a solution for a given N.
    // In the parallel branch we pass `None` for perfect_squares (if that's desired);
    // otherwise we pass a reference.
    let find_solution = |n: u64| {
        let ps = if multiprocessing {
            None
        } else {
            Some(&perfect_squares)
        };
        magic_squares::find_perfect_squares(ps, Some(&squares_sum), n)
    };

    // Choose parallel or sequential processing.
    let solutions: Vec<magic_squares::Solution> = if multiprocessing {
        (n_min..n_max)
            .into_par_iter()
            .filter_map(find_solution)
            .collect()
    } else {
        (n_min..n_max)
            .into_iter()
            .filter_map(find_solution)
            .collect()
    };

    // Sort solutions by N.
    let mut responses = solutions;
    responses.sort_by_key(|sol| sol.N);

    // Print each solution.
    for sol in responses {
        println!("------------------------------------------------------");
        if sol.solution_type == magic_squares::SolutionType::Perfect {
            println!(">>>> OMG WE FOUND A PERFECT SOLUTION!!! <<<<");
            println!(">>>> WAKE UP HONEY A NEW SOLUTION JUST DROPPED <<<<");
            println!("Perfect Solution for N = {}, e = {}", sol.N, sol.e);
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                sol.a, sol.b, sol.c, sol.d, sol.e, sol.f, sol.g, sol.h, sol.i
            );
            std::process::exit(0);
        } else {
            println!(
                "Partial magic square for N = {}, e = {}, incorrect axes = {:?}",
                sol.N, sol.e, sol.incorrect_axis_values
            );
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                sol.a, sol.b, sol.c, sol.d, sol.e, sol.f, sol.g, sol.h, sol.i
            );

            match conjecture::verify_if_N_matches_conjecture(sol.N) {
                Some((p, k)) => {
                    println!("✅ N is in the form of (k*3*p²)² with p={} and k={}", p, k);
                }
                None => {
                    println!("❌ N is not a multiple of (k*3*p²)²");
                    panic!("N is not a multiple of (k*3*p²)²");
                }
            }
        }
    }

    println!("Time: {:.2} seconds", start.elapsed().as_secs_f64());
}

/// Test all numbers of the form (k*3*p²)² with k in [1, max_k] and p prime in [1, max_p]
/// (with the extra condition that p ≡ 1 (mod 6)).
fn test_kp_form_up_to(max_k: u64, max_p: u64) {
    let start = Instant::now();

    // Generate candidate (k, p, N) tuples.
    let candidates: Vec<(u64, u64, u64)> = (1..=max_k)
        .flat_map(|k| {
            (1..=max_p)
                .filter(|&p| utils::is_prime(p) && p % 6 == 1)
                .map(move |p| (k, p, (k * 3 * p.pow(2)).pow(2)))
        })
        .collect();

    // Process each candidate.
    let mut responses: Vec<(u64, u64, u64, Option<magic_squares::Solution>)> = candidates
        .into_iter()
        .map(|(k, p, N)| (k, p, N, magic_squares::find_perfect_squares(None, None, N)))
        .collect();

    responses.sort_by_key(|(k, p, _N, _)| (*k, *p));

    for (k, p, N, sol_opt) in responses {
        println!("-----------------------");
        if let Some(sol) = sol_opt {
            if sol.solution_type == magic_squares::SolutionType::Perfect {
                panic!(
                    "😱 HONEY WAKE UP: perfect solution found for N = {}",
                    N
                );
            }
            println!(
                "✅ [p={}, k={}] N = {} is a quasi magic square (incorrect axes = {:?})",
                p, k, N, sol.incorrect_axis_values
            );
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                sol.a, sol.b, sol.c, sol.d, sol.e, sol.f, sol.g, sol.h, sol.i
            );
        } else {
            println!(
                "❌ N = {} is NOT a quasi magic square in the form of (k*3*p²)² with p={} and k={}",
                N, p, k
            );
        }
    }

    println!("Time: {:.2} seconds", start.elapsed().as_secs_f64());
}

/// Generate a single large quasi magic square for the parameters (k, p).
fn generate_large_quasi_magic_square(k: u64, p: u64) {
    let start = Instant::now();

    let N = (k * 3 * p.pow(2)).pow(2);
    match magic_squares::find_perfect_squares(None, None, N) {
        Some(sol) => {
            if sol.solution_type == magic_squares::SolutionType::Perfect {
                panic!("😱 HONEY WAKE UP: perfect solution found for N = {}", N);
            }

            println!(
                "✅ [p={}, k={}] N = {} is a quasi magic square (incorrect axes = {:?})",
                p, k, N, sol.incorrect_axis_values
            );
            println!(
                "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
                sol.a, sol.b, sol.c, sol.d, sol.e, sol.f, sol.g, sol.h, sol.i
            );
        }
        None => {
            println!(
                "❌ N = {} is NOT a quasi magic square in the form of (k*3*p²)² with p={} and k={}",
                N, p, k
            );
        }
    }

    println!("Time: {:.2} seconds", start.elapsed().as_secs_f64());
}

const USAGE: &str = "
Usage:
    cargo test_n <n_min> <n_max>  # Test all N in the range [n_min, n_max)
    cargo test_kp <max_k> <max_p> # Test all N in the form of (k*3*p²)² with k in [1, max_k] and p prime in [1, max_p]
    cargo generate <k> <p>        # Generate a large quasi magic square in the form of (k*3*p²)²
";

fn main() {
    let mut args = std::env::args().skip(1);

    let command = args.next().unwrap_or_else(|| {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    });

    let arg1 = args
        .next()
        .unwrap_or_else(|| {
            eprintln!("{}", USAGE);
            std::process::exit(1);
        })
        .replace('_', "");
    let arg2 = args
        .next()
        .unwrap_or_else(|| {
            eprintln!("{}", USAGE);
            std::process::exit(1);
        })
        .replace('_', "");

    let arg1: u64 = arg1.parse().unwrap_or_else(|_| {
        eprintln!("Invalid argument for arg1");
        std::process::exit(1);
    });
    let arg2: u64 = arg2.parse().unwrap_or_else(|_| {
        eprintln!("Invalid argument for arg2");
        std::process::exit(1);
    });

    match command.as_str() {
        "test_n" => test_all_N_up_in_range(arg1, arg2, true),
        "benchmark_n" => test_all_N_up_in_range(arg1, arg2, false),
        "test_kp" => test_kp_form_up_to(arg1, arg2),
        "generate" => generate_large_quasi_magic_square(arg1, arg2),
        _ => {
            eprintln!("{}", USAGE);
            std::process::exit(1);
        }
    }
}
