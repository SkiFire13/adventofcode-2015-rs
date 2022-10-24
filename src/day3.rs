#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Direction>;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn input_generator(input: &str) -> Input {
    input
        .bytes()
        .map(|b| match b {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn apply_dir(curr: (i16, i16), dir: Direction) -> (i16, i16) {
    match dir {
        Direction::Up => (curr.0, curr.1 + 1),
        Direction::Down => (curr.0, curr.1 - 1),
        Direction::Left => (curr.0 - 1, curr.1),
        Direction::Right => (curr.0 + 1, curr.1),
    }
}

pub fn part1(input: &Input) -> usize {
    let mut visited = HashSet::from([(0, 0)]);
    let mut curr = (0, 0);
    for &dir in input {
        curr = apply_dir(curr, dir);
        visited.insert(curr);
    }
    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut visited = HashSet::from([(0, 0)]);
    let mut curr1 = (0, 0);
    let mut curr2 = (0, 0);
    for (i, &dir) in input.iter().enumerate() {
        let curr = if i % 2 == 0 { &mut curr1 } else { &mut curr2 };
        *curr = apply_dir(*curr, dir);
        visited.insert(*curr);
    }
    visited.len()
}
