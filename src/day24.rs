#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .rev()
        .collect()
}

fn solve<F>(input: &[usize], target: usize, check_remaining: F) -> usize
where
    F: Fn(&[usize]) -> bool,
{
    fn solve_rec<F: Fn(&[usize]) -> bool>(
        input: &[usize],
        check_remaining: &F,
        remaining: &[usize],
        stack: &mut Vec<usize>,
        curr_sum: usize,
        curr_prod: usize,
        target: usize,
        mut min_count: usize,
        mut min_quantum: usize,
    ) -> (usize, usize) {
        if curr_sum > target || stack.len() >= min_count || curr_prod >= min_quantum {
            return (usize::MAX, usize::MAX);
        }

        if curr_sum == target {
            if check_remaining(stack) {
                return (stack.len(), curr_prod);
            }
        }

        let Some(pos) = remaining.iter().position(|&r| r <= target) else {
            return (usize::MAX, usize::MAX);
        };

        let remaining = &remaining[pos..];

        for (i, &r) in remaining.iter().enumerate() {
            stack.push(r);

            let (new_min_count, new_min_quantum) = solve_rec(
                input,
                check_remaining,
                &remaining[i + 1..],
                stack,
                curr_sum + r,
                curr_prod * r,
                target,
                min_count,
                min_quantum,
            );

            min_count = min(min_count, new_min_count);
            min_quantum = min(min_quantum, new_min_quantum);

            stack.pop();
        }

        (min_count, min_quantum)
    }

    let (min_count, min_quantum) = solve_rec(
        input,
        &check_remaining,
        &input.to_vec(),
        &mut Vec::new(),
        0,
        1,
        target,
        usize::MAX,
        usize::MAX,
    );
    assert_ne!(min_count, usize::MAX);
    assert_ne!(min_quantum, usize::MAX);
    min_quantum
}

pub fn part1(input: &Input) -> usize {
    let target = input.iter().sum::<usize>() / 3;

    solve(input, target, |stack| {
        fn remaining_rec(remaining: &[usize], used: &[usize], target: usize) -> bool {
            if target == 0 {
                return true;
            }

            let &[rf, ref rrest @ ..] = remaining else { return false; };

            if let &[uf, ref urest @ ..] = used {
                if rf == uf {
                    return remaining_rec(rrest, urest, target);
                }
            }

            (rf <= target && remaining_rec(rrest, used, target - rf))
                || remaining_rec(rrest, used, target)
        }

        remaining_rec(input, stack, target)
    })
}

pub fn part2(input: &Input) -> usize {
    let target = input.iter().sum::<usize>() / 4;

    solve(input, target, |stack| {
        fn remaining_rec(
            remaining: &[usize],
            used: &[usize],
            target1: usize,
            target2: usize,
        ) -> bool {
            if target1 == 0 && target2 == 0 {
                return true;
            }

            let &[rf, ref rrest @ ..] = remaining else { return false; };

            if let &[uf, ref urest @ ..] = used {
                if rf == uf {
                    return remaining_rec(rrest, urest, target1, target2);
                }
            }

            (rf <= target1 && remaining_rec(rrest, used, target1 - rf, target2))
                || (rf <= target2 && remaining_rec(rrest, used, target1, target2 - rf))
                || remaining_rec(rrest, used, target1, target2)
        }

        remaining_rec(input, stack, target, target)
    })
}
