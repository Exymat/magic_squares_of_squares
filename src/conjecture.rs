use crate::{generate_squares, utils};

pub fn verify_if_N_matches_conjecture(N: u64) -> Option<(u64, u64)> {
    // Conjecture: N is of the form (k*3*p²)² with p=6n+1 and p prime. We'll first check if N is a multiple of 9.
    if N % 9 != 0 {
        println!("❌ N is not a multiple of 9");
        return None;
    }

    // Now we have to check if (N/9) is a square (as N/9==(kp²)²).
    let (is_ps, n) = generate_squares::is_perfect_square(N / 9);
    if !is_ps {
        println!("❌ N/9 is not a perfect square {}", N / 9);
        return None;
    }

    // Now, we have N/9 = (kp²)² = k * p² = n
    // We're going to iterate on primes of the form 6n+1 and check if n is divisible by p². If the divisor is also a square, we'll have find k.
    for p in 0..n {
        if utils::is_prime(p) && p % 6 == 1 {
            if (n % p.pow(2)) == 0 {
                let k = n / p.pow(2);
                return Some((p, k));
            }
        }
    }

    None
}
