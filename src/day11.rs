#[allow(unused_imports)]
use super::prelude::*;
type Input = [u8; 8];

pub fn input_generator(input: &str) -> Input {
    let arr: [u8; 8] = input.as_bytes().try_into().expect("Invalid input");
    let arr = arr.map(|c| c - b'a');
    arr.iter().for_each(|&c| assert!(c < 26, "Invalid input"));
    arr
}

fn next_password(initial: [u8; 8]) -> [u8; 8] {
    let mut arr = initial;
    loop {
        let mut i = 7;
        while arr[i] == 25 {
            arr[i] = 0;
            i -= 1;
        }
        arr[i] += 1;

        if let Some(pos) = arr.iter().position(|&c| matches!(c, b'i' | b'o' | b'l')) {
            arr[pos] += 1;
            arr[pos + 1..].fill(0);
        }

        let mut pairs = arr.windows(2).filter(|w| w[0] == w[1]);
        let (p1, p2) = (pairs.next(), pairs.next());
        if p1 == p2 || p2.is_none() {
            continue;
        }

        let mut triples = arr
            .windows(3)
            .filter(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
        if triples.next().is_none() {
            continue;
        }

        break arr;
    }
}

fn print_password(password: [u8; 8]) -> String {
    password.iter().map(|&c| (c + b'a') as char).collect()
}

pub fn part1(input: &Input) -> String {
    print_password(next_password(*input))
}

pub fn part2(input: &Input) -> String {
    print_password(next_password(next_password(*input)))
}
