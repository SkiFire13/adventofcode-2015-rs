#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<[i32; 5]>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (cap, dur, fla, tex, cal) = line.split(',').collect_tuple().expect("Invalid input");
            let (_, cap) = cap.rsplit_once(' ').expect("Invalid input");
            let (_, dur) = dur.rsplit_once(' ').expect("Invalid input");
            let (_, fla) = fla.rsplit_once(' ').expect("Invalid input");
            let (_, tex) = tex.rsplit_once(' ').expect("Invalid input");
            let (_, cal) = cal.rsplit_once(' ').expect("Invalid input");
            let cap = cap.parse().expect("Invalid input");
            let dur = dur.parse().expect("Invalid input");
            let fla = fla.parse().expect("Invalid input");
            let tex = tex.parse().expect("Invalid input");
            let cal = cal.parse().expect("Invalid input");
            [cap, dur, fla, tex, cal]
        })
        .collect()
}

fn solve(input: &Input, f: impl Copy + Fn(i32) -> bool) -> i32 {
    fn solve_rec(
        acc: [i32; 5],
        sum: i32,
        input: &[[i32; 5]],
        f: impl Copy + Fn(i32) -> bool,
    ) -> i32 {
        match input {
            [] => panic!("Invalid input"),
            [last] => {
                let acc = array::from_fn::<_, 5, _>(|i| acc[i] + last[i] * sum);
                if acc[..4].iter().all(|&n| n > 0) && f(acc[4]) {
                    acc[..4].iter().product()
                } else {
                    i32::MIN
                }
            }
            [curr, rest @ ..] => {
                let mut m = i32::MIN;
                for x in 0..=sum {
                    let acc = array::from_fn::<_, 5, _>(|i| acc[i] + curr[i] * x);
                    m = max(m, solve_rec(acc, sum - x, rest, f));
                }
                m
            }
        }
    }

    solve_rec([0; 5], 100, input, f)
}

pub fn part1(input: &Input) -> i32 {
    solve(input, |_| true)
}

pub fn part2(input: &Input) -> i32 {
    solve(input, |e| e == 500)
}
