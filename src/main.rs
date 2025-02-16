#![allow(warnings)]

use rayon::prelude::*;
use std::{sync::Arc, time::Instant};

mod conjecture;
mod generate_squares;
mod magic_squares;
mod utils;

// if correct_axes == 3 {
//     println!("------------------------------------------------------");
//     println!(">>>> OMG WE FOUND A PERFECT SOLUTION!!! <<<<");
//     println!(">>>> WAKE UP HONEY A NEW SOLUTION JUST DROPPED <<<<");
//     println!("Solution for N = {}, e = {}", N, e);
//     println!(
//         "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}",
//         a, b, c, d, e, f, g, h, i_val
//     );
//     std::process::exit(0);
// }

// if correct_axes >= 2 {
//     println!("------------------------------------------------------");
//     println!("Partial magic square for N = {}, e = {}, X = {}", N, e, X);

//     // Ensure it verifies the conjecture.
//     if let Some((p, k)) = verify_if_N_matches_conjecture(N) {
//         println!("✅ N is in the form of (k*3*p²)² with p={} and k={}", p, k);
//     } else {
//         println!("❌😱😱😱😱😱😱😱😱😱 N is not a multiple of (k*3*p)²");
//         panic!("N is not a multiple of (k*3*p²)²");
//     }
//     println!(
//         "a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {} | S6 = c² + e² + g² = {}",
//         a, b, c, d, e, f, g, h, i_val, S6
//     );
//     println!(
//         "{}\t{}\t{}\n{}\t{}\t{}\n{}\t{}\t{}",
//         a, b, c, d, e, f, g, h, i_val
//     );
//     return true;

fn main() {
    let start = Instant::now();

    let N_MIN = 0;
    let N_MAX = 1000_000;
    let rep_map = generate_squares::generate_squares_sum_fast(N_MAX);

    println!("Checking N = {}..{} ", N_MIN, N_MAX);

    // Process the range in parallel.
    let numbers: Vec<u64> = (N_MIN..N_MAX).collect();
    let rep_map = Arc::new(rep_map);

    let mut responses: Vec<magic_squares::Solution> = numbers
        .par_chunks(1000)
        .flat_map_iter(|batch| {
            // Clone the Arc pointer (not the entire map)
            let rep_map = Arc::clone(&rep_map);

            batch
                .iter()
                .map(|i| magic_squares::find_perfect_squares(&rep_map, *i))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

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
