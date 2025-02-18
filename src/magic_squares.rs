use fxhash::{FxHashMap, FxHashSet};
use std::sync::Arc;
use std::time::Instant;

use crate::generate_squares;
use crate::generate_squares::PrecomputedPerfectSquares;
use crate::utils;

/// Checks if the given 3 pairs (with extra value `e` and target N) form a (partial or perfect) magic square.
/// In the ordering the three pairs correspond to (a,i), (b,h) and (d,f) respectively.
fn is_magic_square(
    perfect_squares_precomp: Option<&PrecomputedPerfectSquares>,
    ordering: &[(u64, u64); 3],
    e: u64,
    N: u64,
    X: u64,
) -> Option<Solution> {
    let (a, i_val) = ordering[0];
    let (b, h) = ordering[1];
    let (d, f) = ordering[2];

    // S4: a² + b² + c² = N  =>  c² = N - a² - b².
    let sum_ab = a * a + b * b;
    if sum_ab > N {
        return None;
    }
    let c2 = N - sum_ab;
    let (is_ps_c, c) = match perfect_squares_precomp {
        Some(prec) => match prec.get(c2) {
            Some((is_ps, c)) => (is_ps, c),
            None => return None,
        },
        None => generate_squares::is_perfect_square(c2),
    };
    if !is_ps_c {
        return None;
    }

    // S2: g² + h² + i² = N  =>  g² = N - h² - i².
    let sum_hi = h * h + i_val * i_val;
    if sum_hi > N {
        return None;
    }
    let g2 = N - sum_hi;
    let (is_ps_g, g) = match perfect_squares_precomp {
        Some(prec) => match prec.get(g2) {
            Some((is_ps, g)) => (is_ps, g),
            None => return None,
        },
        None => generate_squares::is_perfect_square(g2),
    };

    if !is_ps_g {
        return None;
    }

    // Check that all 9 numbers are distinct.
    let assignment = vec![a, b, c, d, e, f, g, h, i_val];
    let unique: FxHashSet<_> = assignment.iter().cloned().collect();
    if unique.len() < 9 {
        return None;
    }

    // Compute three extra sums.
    let S1 = a * a + d * d + g * g;
    let S3 = i_val * i_val + f * f + c * c;
    let S6 = c * c + e * e + g * g;

    let mut incorrect_axes = vec![];
    if S1 != N {
        incorrect_axes.push(S1);
    }
    if S3 != N {
        incorrect_axes.push(S3);
    }
    if S6 != N {
        incorrect_axes.push(S6);
    }

    let correct_axes = 3 - incorrect_axes.len();

    if correct_axes >= 2 {
        return Option::from(Solution {
            N,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i: i_val,
            // If all axes are correct, it's a perfect solution.
            solution_type: if correct_axes == 3 {
                SolutionType::Perfect
            } else {
                SolutionType::Partial
            },
            incorrect_axis_values: incorrect_axes,
        });
    }

    None
}

#[derive(PartialEq)]
pub enum SolutionType {
    Perfect,
    Partial,
}

pub struct Solution {
    pub N: u64,
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
    pub e: u64,
    pub f: u64,
    pub g: u64,
    pub h: u64,
    pub i: u64,
    pub solution_type: SolutionType,
    pub incorrect_axis_values: Vec<u64>,
}

/// Finds “perfect squares” (i.e. candidate magic squares) for a given N.
pub fn find_perfect_squares(
    perfect_squares_precomp: Option<&PrecomputedPerfectSquares>,
    precomputed_square_sums: Option<&FxHashMap<u64, Box<[(u64, u64)]>>>,
    N: u64,
) -> Option<Solution> {
    // Check if N can be written as the sum of three squares. If it can't, then no magic square can be formed.
    if !utils::can_be_written_as_sum_of_three_squares(N) {
        return None;
    }

    // Maximum value for any of the 9 numbers.
    // Max value for e is sqrt(N/3), since N is the sum of three squares.
    let max_val = num_integer::sqrt(N / 3);

    // Store all e tested because we know that if we try a a->e->i diagonal, if e ever takes `a` or `i` later,
    // it will be a duplicate of the same square (with some translations).
    let mut tested_es: FxHashSet<u64> = FxHashSet::default();

    // Loop over candidate extra number e (with e² <= N/3).
    for e in 1..max_val {
        let X = N - e * e;

        // We want the pairs (a,i), (b,h), (d,f) to satisfy x²+y² = X.
        // Get from precomputed_square_sums if possible
        let pairs_list = match precomputed_square_sums {
            Some(prec) => match prec.get(&X) {
                Some(pairs) => pairs,
                None => continue,
            },
            None => &generate_squares::find_sum_of_squares_pairs(X).into_boxed_slice(),
        };

        if pairs_list.len() <= 3 {
            continue;
        }

        // For each non-symmetric permutation of three pairs...
        let orderings = utils::nonsymetric_permutations_3(&pairs_list);
        for ordering in orderings {
            // Create the four full orderings (reversing some of the pairs)
            let (p1, p2, p3) = ordering;

            /// We believe (empirically) that if either a or i is less than e,
            /// the current square is a duplicate from a previously-tested square.
            /// This is because we can always apply transformations to the square to make a or i the center (e), while preserving the sum of different axes.
            /// We don't have proof of this, but it seems to work.
            /// We prefer to disable that optimization for proof purposes, as we can't prove it's true.
            ///
            // let (a, i) = p1;
            // if (a < e || i < e) {
            //     continue;
            // }
            let full_orderings = [
                [p1, p2, p3],
                [p1, p2, (p3.1, p3.0)],
                [p1, (p2.1, p2.0), p3],
                [p1, (p2.1, p2.0), (p3.1, p3.0)],
            ];
            for o in full_orderings {
                match is_magic_square(perfect_squares_precomp, &o, e, N, X) {
                    Some(solution) => {
                        return Some(solution);
                    }
                    None => {}
                }
            }
        }
    }
    None
}
