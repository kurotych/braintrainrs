#[cfg(test)]
mod test {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::convert::From;

    // cells [i1, j2] and [i2, j2] are on the same diagonal,
    // if and only if |i1 - i2| == |j1 - j2|
    fn is_solution(perm: &[u32]) -> bool {
        // Check is all rows (j) are unique
        if (1..perm.len()).any(|i| perm[i..].contains(&perm[i - 1])) {
            return false;
        }

        for (column, row) in perm.iter().enumerate() {
            for (column2, row2) in perm[column + 1..].iter().enumerate() {
                let (i1, j1) = (i64::from(column as u32), i64::from(*row));
                let (i2, j2) = (i64::from((column2 + column + 1) as u32), i64::from(*row2));
                if (i1 - i2).abs() == (j1 - j2).abs() {
                    return false;
                }
            }
        }
        true
    }

    fn extend(perm: &mut Vec<u32>, n: usize, mut count_solutions: usize) -> usize {
        if perm.len() == n {
            println!("Solution for {:?}x{:?} table: {:?}", n, n, perm);
            return count_solutions + 1;
        }
        for i in 0..n {
            if !perm.contains(&(i as u32)) {
                perm.push(i as u32);
                if is_solution(perm) {
                    count_solutions = extend(perm, n, count_solutions);
                }
                perm.pop();
            }
        }
        count_solutions
    }

    #[test]
    fn back_tracking() {
        let count_solutions = extend(&mut Vec::new(), 8, 0);
        assert_eq!(count_solutions, 92);
    }

    #[test]
    fn brute_force_8x8() {
        loop {
            let mut vec: Vec<u32> = (0..8).collect();
            vec.shuffle(&mut thread_rng());
            if is_solution(&vec) == true {
                println!("{:?}", vec);
                break;
            }
        }
    }

    #[test]
    fn is_solution_test() {
        assert_eq!(is_solution(&[1, 3, 0, 2]), true);
        assert_eq!(is_solution(&[3, 1, 0, 2]), false);
        assert_eq!(is_solution(&[3, 3, 0, 2]), false);
    }
}
