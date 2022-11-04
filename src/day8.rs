#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<&'a [u8]>;

pub fn input_generator(input: &str) -> Input<'_> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|bytes| {
            let mut remaining = *bytes;
            let mut count = 0;
            loop {
                match remaining {
                    [] => break bytes.len() - count + 2,
                    [b'\\', b'\\' | b'"', rest @ ..] => remaining = rest,
                    [b'\\', b'x', _, _, rest @ ..] => remaining = rest,
                    [_, rest @ ..] => remaining = rest,
                }
                count += 1;
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|bytes| 2 + bytes.iter().filter(|&&b| b == b'"' || b == b'\\').count())
        .sum()
}
