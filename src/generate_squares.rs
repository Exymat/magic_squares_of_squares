use fxhash::FxHashMap;
use std::time::Instant;

use num_cpus;
use rayon::prelude::*;

/// Return true if `n` has a prime factor p ≡ 3 (mod 4) with an odd exponent.
fn has_forbidden_prime_factor(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    let mut m = n;
    // Remove all factors of 2.
    while m % 2 == 0 {
        m /= 2;
    }
    let mut limit = num_integer::sqrt(m);
    let mut p = 3;
    while p <= limit && m > 1 {
        if m % p == 0 {
            let mut count = 0;
            while m % p == 0 {
                count += 1;
                m /= p;
            }
            if p % 4 == 3 && (count % 2 == 1) {
                return true;
            }
            limit = num_integer::sqrt(m);
        }
        p += 2;
    }
    m > 1 && (m % 4 == 3)
}

/// Returns true if `n` can be expressed as x² + y².
fn can_be_sum_of_two_squares(n: u64) -> bool {
    !has_forbidden_prime_factor(n)
}

/// Return all pairs (x, y) such that x² + y² == n using a two-pointers approach.
/// If n is not expressible, an empty vector is returned.
pub fn find_pairs_two_pointers(n: u64) -> Vec<(u64, u64)> {
    if !can_be_sum_of_two_squares(n) {
        return Vec::new();
    }

    let mut pairs = Vec::new();
    let mut x = 0;
    let mut y = num_integer::sqrt(n);

    while x <= y {
        let sum_squares = x * x + y * y;
        if sum_squares == n {
            pairs.push((x, y));
            x += 1;
            // Prevent underflow.
            if y > 0 {
                y -= 1;
            } else {
                break;
            }
        } else if sum_squares < n {
            x += 1;
        } else {
            if y > 0 {
                y -= 1;
            } else {
                break;
            }
        }
    }
    pairs
}

/// Process a single number n and return Some((n, pairs)) if n is expressible as a sum of two squares,
/// otherwise return None.
fn process_n(n: u64) -> Option<(u64, Vec<(u64, u64)>)> {
    let pairs = find_pairs_two_pointers(n);
    if !pairs.is_empty() {
        Some((n, pairs))
    } else {
        None
    }
}

/// Process a batch (slice) of numbers and return a vector of (n, pairs) for those n that are expressible.
fn batch_process(batch: &[u64]) -> Vec<(u64, Vec<(u64, u64)>)> {
    let mut results = Vec::new();
    for &n in batch {
        if let Some(res) = process_n(n) {
            results.push(res);
        }
    }
    results
}

/// For each n in 1..=N, determine if n is expressible as the sum of two squares.
/// Returns a HashMap where the key is n and the value is the vector of (x, y) pairs.
/// Numbers that are not expressible are not included.
pub fn generate_squares_sum_fast(n: u64) -> FxHashMap<u64, Box<[(u64, u64)]>> {
    let cpus = num_cpus::get() as u64;
    // Compute batch size: ceil(n / cpus) // 8, but at least 1.
    let batch_size = (((n + cpus - 1) / cpus) / 8).max(1) as usize;

    (1..=n)
        .collect::<Vec<u64>>() // Collecting the range into a Vec
        .par_chunks(batch_size)
        .flat_map_iter(batch_process)
        .map(|(n, pairs)| (n, pairs.into_boxed_slice()))
        .collect()
}
