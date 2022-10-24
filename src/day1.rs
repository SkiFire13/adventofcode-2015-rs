#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = &'a [u8];

pub fn input_generator(input: &str) -> Input<'_> {
    input.as_bytes()
}

pub fn part1(input: &Input) -> isize {
    input.iter().map(|&b| 2 * (b == b'(') as isize - 1).sum()
}

pub fn part2(input: &Input) -> usize {
    let mut acc = 0;
    input
        .iter()
        .map(|&b| {
            acc += 2 * (b == b'(') as isize - 1;
            acc
        })
        .position(|sum| sum == -1)
        .map(|pos| pos + 1)
        .expect("Invalid input")
}
