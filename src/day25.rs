#[allow(unused_imports)]
use super::prelude::*;
type Input = (u64, u64);

pub fn input_generator(input: &str) -> Input {
    let (_, rest) = input.split_once("row ").expect("Invalid input");
    let (row, column) = rest.split_once(", column ").expect("Invalid input");
    let column = column.trim_end_matches('.');
    let row = row.parse().expect("Invalid input");
    let column = column.parse().expect("Invalid input");
    (row, column)
}

pub fn part1(input: &Input) -> u64 {
    let &(row, column) = input;

    let mut n = (row + column - 2) * (row + column - 1) / 2 + column - 1;

    let mut acc = 20151125;
    let mut base = 252533;

    while n != 0 {
        if n % 2 == 1 {
            acc = (acc * base) % 33554393;
        }

        base = (base * base) % 33554393;
        n /= 2;
    }

    acc
}
