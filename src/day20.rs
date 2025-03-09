#[allow(unused_imports)]
use super::prelude::*;
type Input = u32;

pub fn input_generator(input: &str) -> Input {
    input.trim().parse().expect("Invalid input")
}

pub fn part1(input: &Input) -> u32 {
    fn solve_rec(primes: &[u32], curr: u32, factor_sum: u32, best: u32, goal: u32) -> u32 {
        let Some((&prime, primes)) = primes.split_first() else {
            return best;
        };

        if curr * prime > best {
            return best;
        }

        let mut best = best;
        let mut curr = curr;
        let mut factor_acc = 1;
        while factor_sum * factor_acc < goal && curr < best {
            let new_best = solve_rec(primes, curr, factor_sum * factor_acc, best, goal);
            best = min(best, new_best);
            curr *= prime;
            factor_acc = prime * factor_acc + 1;
        }

        min(best, curr)
    }

    let mut primes = vec![2, 3];
    (1..)
        .flat_map(|n| [6 * n - 1, 6 * n + 1])
        .take_while(|&n| n * n + 1 <= *input / 10)
        .for_each(|n| {
            if primes.iter().all(|&p| n % p != 0) {
                primes.push(n);
            }
        });

    solve_rec(&primes, 1, 1, u32::MAX, *input / 10)
}

pub fn part2(input: &Input) -> u32 {
    (*input as usize * 2 / 9 / 11..)
        .par_find_chunked(10_000, |n| {
            let n = n as u32;
            let sum = (1..=50).filter(|&p| n % p == 0).map(|p| n / p).sum::<u32>();
            sum * 11 >= *input
        })
        .unwrap() as u32
}
