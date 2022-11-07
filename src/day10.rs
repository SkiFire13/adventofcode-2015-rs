#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u8>;

pub fn input_generator(input: &str) -> Input {
    input.chars().map(|c| c as u8 - b'0').collect()
}

fn execute_steps(base: &[u8], steps: usize) -> usize {
    let mut acc = base.to_vec();
    let mut tmp = Vec::new();
    for _ in 0..steps {
        tmp.clear();
        tmp.extend(
            acc.iter()
                .dedup_with_count()
                .flat_map(|(c, &i)| [c as u8, i]),
        );
        std::mem::swap(&mut acc, &mut tmp);
    }
    acc.len()
}

pub fn part1(input: &Input) -> usize {
    execute_steps(input, 40)
}

pub fn part2(input: &Input) -> usize {
    execute_steps(input, 50)
}
