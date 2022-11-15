#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .sorted()
        .collect()
}

pub fn part1(input: &Input) -> usize {
    fn solve(cache: &mut HashMap<(usize, usize), usize>, input: &[usize], tot: usize) -> usize {
        if tot == 0 {
            return 1;
        }

        let [first, rest @ ..] = input else { return 0; };

        if let Some(&cached) = cache.get(&(tot, input.len())) {
            return cached;
        }

        let mut sol = solve(cache, rest, tot);
        if let Some(new_tot) = tot.checked_sub(*first) {
            sol += solve(cache, rest, new_tot);
        }

        cache.insert((tot, input.len()), sol);
        sol
    }

    solve(&mut HashMap::new(), input, 150)
}

pub fn part2(input: &Input) -> usize {
    fn solve(
        cache: &mut HashMap<(usize, usize), (usize, usize)>,
        input: &[usize],
        tot: usize,
    ) -> (usize, usize) {
        if tot == 0 {
            return (1, 0);
        }

        let [first, rest @ ..] = input else { return (0, usize::MAX - 1); };

        if let Some(&cached) = cache.get(&(tot, input.len())) {
            return cached;
        }

        let (mut sol, mut min_used) = solve(cache, rest, tot);
        if let Some(new_tot) = tot.checked_sub(*first) {
            let (new_sol, new_min_used) = solve(cache, rest, new_tot);
            if new_min_used + 1 == min_used {
                sol += new_sol;
            } else if new_min_used + 1 < min_used {
                sol = new_sol;
                min_used = new_min_used + 1;
            }
        }

        cache.insert((tot, input.len()), (sol, min_used));
        (sol, min_used)
    }

    let (sol, _) = solve(&mut HashMap::new(), input, 150);
    sol
}
