use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
use std::env;
use std::io::Read;
use std::ops::Div;

// Function to output the solutions to both parts
fn output(result: &Result) {
    println!("Part 1: {}", &result.part_1);
    println!("Part 2: {}", &result.part_2);
}

fn main() {
    // Vector of the command line arguments
    let args: Vec<String> = env::args().collect();

    // Read in the input
    let mut file_handle = std::fs::File::open(&args[1]).unwrap();
    let mut inp = String::new();
    file_handle.read_to_string(&mut inp).unwrap();
    let inp: Vec<&str> = inp.split("\n").filter(|line| !line.is_empty()).collect();

    // Struct storing the resulting values
    let mut result: Result = Result {
        part_1: 0,
        part_2: 0,
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: usize,
    part_2: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SecretNumber {
    num: usize,
}

impl SecretNumber {
    fn mix(&self, num_to_mix: usize) -> SecretNumber {
        SecretNumber {
            num: self.num ^ num_to_mix,
        }
    }

    fn prune(&self) -> SecretNumber {
        SecretNumber {
            num: self.num % 16777216,
        }
    }

    fn next_naive(&self) -> SecretNumber {
        // Calculate the result of multiplying the secret number by 64
        // + mix + prune
        let res = self.mix(self.num << 6).prune();
        // Calculate the result of dividing the secret number by 32
        // + mix + prune
        let res = res.mix(res.num >> 5).prune();
        // Calculate the result of multiplying the secret number by 2048
        // + mix + prune
        let res = res.mix(res.num << 11).prune();

        res
    }

    fn get_price(&self) -> i8 {
        (self.num % 10) as i8
    }
}

// Goes through num_its numbers and counts how many bananas
// you get using the given sequence
fn test_sequence(sequence: &[i8], secret_num: SecretNumber) -> usize {
    let num_its = 2000;
    let mut nums = vec![secret_num];
    let mut changes = Vec::new();
    for _ in 0..num_its {
        let last = nums.last().unwrap().clone();
        let next = last.next_naive();
        nums.push(next);
        changes.push(next.get_price() - last.get_price());
        if let Some(window) = changes.last_chunk::<4>() {
            if window == sequence {
                return nums.last().unwrap().get_price() as usize;
            }
        }
    }
    return 0;
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let secret_numbers = inp
        .into_iter()
        .map(|l| l.parse::<usize>().expect("Couldn't parse number"))
        .map(|n| SecretNumber { num: n })
        .collect::<Vec<_>>();

    let num_its = 2000;

    res.part_1 = secret_numbers
        .par_iter()
        .map(|n| {
            let mut n = n.clone();
            for _ in 0..num_its {
                n = n.next_naive();
            }
            n.num
        })
        .sum();

    res.part_2 = (0..4).map(|_| (-9..=9))
        .multi_cartesian_product()
        .into_iter()
        .par_bridge()
        .progress_count(20 * 20 * 20 * 20)
        .map(|seq| secret_numbers.iter().map(|n| test_sequence(&seq, *n)).sum())
        .max()
        .unwrap();
}

// 1604 too low

#[cfg(test)]
mod tests {
    use crate::SecretNumber;

    #[test]
    fn mul_by_64() {
        assert_eq!(1 << 6, 64);
    }

    #[test]
    fn divide_by_32() {
        assert_eq!(64 >> 5, 2);
        assert_eq!(32 >> 5, 1);
        assert_eq!(41 >> 5, 1);
    }

    #[test]
    fn mul_by_2048() {
        assert_eq!(1 << 11, 2048);
    }

    #[test]
    fn mix_test_1() {
        let test = SecretNumber { num: 42 };
        assert_eq!(test.mix(15).num, 37);
    }

    #[test]
    fn prune_test_1() {
        let test = SecretNumber { num: 100000000 };
        assert_eq!(test.prune().num, 16113920);
    }

    #[test]
    fn secret_123() {
        let test = SecretNumber { num: 123 };
        let mut results = vec![test];
        for _ in 0..10 {
            results.push(results.last().unwrap().next_naive());
        }
        assert_eq!(
            results
                .into_iter()
                .skip(1)
                .map(|n| n.num)
                .collect::<Vec<_>>(),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }

    #[test]
    fn prices_123() {
        let test = SecretNumber { num: 123 };
        let mut results = vec![test];
        for _ in 1..10 {
            results.push(results.last().unwrap().next_naive());
        }
        let results = results
            .into_iter()
            .map(|n| n.get_price())
            .collect::<Vec<_>>();
        assert_eq!(results, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }
}

#[cfg(test)]
mod tests_sequence {
    use crate::{test_sequence, SecretNumber};

    #[test]
    fn test_sequences() {
        let seq = [-2, 1, -1, 3];
        let secret_numbers = vec![
            SecretNumber { num: 1 },
            SecretNumber { num: 2 },
            SecretNumber { num: 3 },
            SecretNumber { num: 2024 },
        ];
        assert_eq!(test_sequence(&seq, secret_numbers[0]), 7);
        assert_eq!(test_sequence(&seq, secret_numbers[1]), 7);
        assert_eq!(test_sequence(&seq, secret_numbers[2]), 0);
        assert_eq!(test_sequence(&seq, secret_numbers[3]), 9);
        assert_eq!(
            secret_numbers
                .iter()
                .map(|n| test_sequence(&seq, *n))
                .sum::<usize>(),
            23
        )
    }

    #[test]
    fn test_not_using_last_possible_change() {
        let seq = [3,1,4,1];
        let secret_numbers = vec![
            SecretNumber { num: 2021 },
            SecretNumber { num: 5017 },
            SecretNumber { num: 19751 },
        ];
        assert_eq!(
            secret_numbers
                .iter()
                .map(|n| test_sequence(&seq, *n))
                .sum::<usize>(),
            27
        )
    }

    #[test]
    fn test_not_using_first_possible_change() {
        let seq = [-1,0,-1,8];
        let secret_numbers = vec![
            SecretNumber { num: 5053 },
            SecretNumber { num: 10083 },
            SecretNumber { num: 11263 },
        ];
        assert_eq!(
            secret_numbers
                .iter()
                .map(|n| test_sequence(&seq, *n))
                .sum::<usize>(),
            27
        )
    }
}
