#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<&'a str>;

pub fn input_generator(input: &str) -> Input<'_> {
    input.lines().collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|line| {
            let mut vocals = 0;
            let mut double = false;
            let mut invalid = false;

            let is_vocal = |c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u');

            vocals += is_vocal(line.as_bytes()[0]) as usize;
            for win in line.as_bytes().windows(2) {
                let [c1, c2]: [_; 2] = win.try_into().unwrap();
                vocals += is_vocal(c2) as usize;
                double |= c1 == c2;
                invalid |= matches!(win, b"ab" | b"cd" | b"pq" | b"xy");
            }

            vocals >= 3 && double && !invalid
        })
        .count()
}

pub fn part2(input: &Input) -> usize {
    let mut positions = HashMap::new();
    input
        .iter()
        .filter(|line| {
            let mut pair = false;
            let mut triple = false;

            positions.clear();
            positions.insert(line.as_bytes()[0..2].try_into().unwrap(), 0);
            for (i, win) in line.as_bytes().windows(3).enumerate() {
                let [c1, c2, c3]: [_; 3] = win.try_into().unwrap();
                pair = pair || *positions.entry([c2, c3]).or_insert(i + 1) < i;
                triple |= c1 == c3;
            }

            pair && triple
        })
        .count()
}
