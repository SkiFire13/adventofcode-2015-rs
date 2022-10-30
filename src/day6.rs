#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(Instruction, BoundingBox)>;

type BoundingBox = (usize, usize, usize, usize);

#[derive(Clone, Copy)]
pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (p2, _, p1, instr) = line.rsplitn(4, ' ').collect_tuple().expect("Invalid input");
            let instr = match instr {
                "toggle" => Instruction::Toggle,
                "turn off" => Instruction::TurnOff,
                "turn on" => Instruction::TurnOn,
                _ => panic!("Invalid input"),
            };
            let (x1, y1) = p1.split_once(',').expect("Invalid input");
            let x1 = x1.parse().expect("Invalid input");
            let y1 = y1.parse().expect("Invalid input");
            let (x2, y2) = p2.split_once(',').expect("Invalid input");
            let x2 = x2.parse().expect("Invalid input");
            let y2 = y2.parse().expect("Invalid input");
            (instr, (x1, y1, x2, y2))
        })
        .collect()
}

fn for_each_in_range<T: Copy>(
    grid: &mut [T; 1000 * 1000],
    (x1, y1, x2, y2): BoundingBox,
    mut f: impl FnMut(T) -> T,
) {
    let xmin = min(x1, x2);
    let xmax = max(x1, x2);
    let ymin = min(y1, y2);
    let ymax = max(y1, y2);
    assert!(ymax < 1000);
    assert!(xmax < 1000);

    grid[ymin * 1000..(ymax + 1) * 1000]
        .chunks_exact_mut(1000)
        .flat_map(|chunk| &mut chunk[xmin..=xmax])
        .for_each(|v| *v = f(*v));
}

pub fn part1(input: &Input) -> usize {
    let mut grid: Box<[_; 1000 * 1000]> = vec![false; 1000 * 1000]
        .into_boxed_slice()
        .try_into()
        .unwrap();

    for &(instr, bounds) in input {
        match instr {
            Instruction::TurnOn => for_each_in_range(&mut grid, bounds, |_| true),
            Instruction::TurnOff => for_each_in_range(&mut grid, bounds, |_| false),
            Instruction::Toggle => for_each_in_range(&mut grid, bounds, |v| !v),
        }
    }

    grid.iter().filter(|&&b| b).count()
}

pub fn part2(input: &Input) -> u64 {
    let mut grid: Box<[_; 1000 * 1000]> = vec![0u16; 1000 * 1000]
        .into_boxed_slice()
        .try_into()
        .unwrap();

    for &(instr, bounds) in input {
        match instr {
            Instruction::TurnOn => for_each_in_range(&mut grid, bounds, |v| v + 1),
            Instruction::TurnOff => for_each_in_range(&mut grid, bounds, |v| v.saturating_sub(1)),
            Instruction::Toggle => for_each_in_range(&mut grid, bounds, |v| v + 2),
        }
    }

    grid.iter().map(|&v| v as u64).sum()
}
