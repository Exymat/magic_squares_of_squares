use fxhash::FxHashMap;

/// Returns the “bit‐length” of x (i.e. ⌊log₂(x)⌋+1).
fn bit_length(x: u64) -> u32 {
    if x == 0 {
        0
    } else {
        64 - x.leading_zeros()
    }
}

/// Verifies if a number can be written as the sum of three squares. This is known as the Legendre's three-square theorem, and the A004215 sequence in OEIS.
pub fn can_be_written_as_sum_of_three_squares(n: u64) -> bool {
    let m = bit_length((!n) & (n - 1));
    if m == 0 && ((n >> m) & 7 == 7) {
        false
    } else {
        true
    }
}

/// Check if a number is prime
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut p = 5;
    let mut w = 2;
    while p * p <= n {
        if n % p == 0 {
            return false;
        }
        p += w;
        w = 6 - w;
    }
    true
}

/// Given a slice `lst`, returns all non-symmetric 3‑element combinations.
pub fn nonsymetric_permutations_3(lst: &[(u64, u64)]) -> Vec<((u64, u64), (u64, u64), (u64, u64))> {
    let n = lst.len();
    // Preallocate the vector with the final size:
    let mut results = Vec::with_capacity(n * (n - 1) * (n - 2) / 2);

    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            // Ensure we don’t pick an index smaller than j for k,
            // to avoid duplicate pairs.
            for k in (j + 1)..n {
                if k == i {
                    continue;
                }
                results.push((lst[i], lst[j], lst[k]));
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonsymetric_permutations_3() {
        let input_4 = &[(0, 0), (1, 1), (2, 2), (3, 3)];
        let expected_4 = vec![
            ((0, 0), (1, 1), (2, 2)),
            ((0, 0), (1, 1), (3, 3)),
            ((0, 0), (2, 2), (3, 3)),
            ((1, 1), (0, 0), (2, 2)),
            ((1, 1), (0, 0), (3, 3)),
            ((1, 1), (2, 2), (3, 3)),
            ((2, 2), (0, 0), (1, 1)),
            ((2, 2), (0, 0), (3, 3)),
            ((2, 2), (1, 1), (3, 3)),
            ((3, 3), (0, 0), (1, 1)),
            ((3, 3), (0, 0), (2, 2)),
            ((3, 3), (1, 1), (2, 2)),
        ];

        let mut result_4 = nonsymetric_permutations_3(input_4);
        result_4.sort();
        let mut expected_4 = expected_4;
        expected_4.sort();
        assert_eq!(result_4, expected_4);

        let input_3 = &[(10, 10), (20, 20), (30, 30)];
        let expected_3 = vec![
            ((10, 10), (20, 20), (30, 30)),
            ((20, 20), (10, 10), (30, 30)),
            ((30, 30), (10, 10), (20, 20)),
        ];

        let mut result_3 = nonsymetric_permutations_3(input_3);
        result_3.sort();
        let mut expected_3 = expected_3;
        expected_3.sort();
        assert_eq!(result_3, expected_3);
    }
}
