#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<[usize; 3]>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = line.split('x').collect_tuple().expect("Invalid input");
            [l, w, h].map(|d| d.parse().expect("Invalid input"))
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|&[l, w, h]| {
            let perim = 2 * (l * w + l * h + w * h);
            let extra = min(l * w, min(l * h, w * h));
            perim + extra
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|&[l, w, h]| {
            let max_side = max(l, max(w, h));
            let min_perim = 2 * (l + w + h - max_side);
            let volume = l * w * h;
            min_perim + volume
        })
        .sum()
}
