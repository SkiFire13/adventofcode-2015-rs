#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<[Option<u32>; 10]>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(": ").expect("Invalid input");
            let mut acc = [None; 10];
            for detection in rest.split(", ") {
                let (detected, n) = detection.split_once(": ").expect("Invalid input");
                let elem = match detected {
                    "children" => &mut acc[0],
                    "cats" => &mut acc[1],
                    "samoyeds" => &mut acc[2],
                    "pomeranians" => &mut acc[3],
                    "akitas" => &mut acc[4],
                    "vizslas" => &mut acc[5],
                    "goldfish" => &mut acc[6],
                    "trees" => &mut acc[7],
                    "cars" => &mut acc[8],
                    "perfumes" => &mut acc[9],
                    _ => panic!("Invalid input"),
                };
                let old = elem.replace(n.parse().expect("Invalid input"));
                assert!(old.is_none(), "Invalid input");
            }
            acc
        })
        .collect()
}

fn solve(input: &Input, expected: [(u32, Ordering); 10]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, detected)| {
            detected
                .iter()
                .zip(&expected)
                .filter_map(|(&det, &exp)| Some((det?, exp)))
                .all(|(det, (expn, expo))| Ord::cmp(&det, &expn) == expo)
        })
        .map(|(i, _)| i + 1)
        .exactly_one()
        .expect("Invalid input")
}

pub fn part1(input: &Input) -> usize {
    let expected = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    solve(input, expected.map(|n| (n, Ordering::Equal)))
}

pub fn part2(input: &Input) -> usize {
    let mut expected = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1].map(|exp| (exp, Ordering::Equal));
    expected[1].1 = Ordering::Greater;
    expected[7].1 = Ordering::Greater;
    expected[3].1 = Ordering::Less;
    expected[6].1 = Ordering::Less;
    solve(input, expected)
}
