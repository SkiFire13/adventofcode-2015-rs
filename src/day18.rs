#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<bool>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c == '#')
}

fn solve(input: &Input, on_each: impl Fn(&mut Input)) -> usize {
    let mut curr = input.clone();
    let mut tmp = input.clone();

    for _ in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                let neighbors = curr
                    .square_neighbours((x, y))
                    .filter(|&pos| curr[pos])
                    .count();

                tmp[(x, y)] = match (curr[(x, y)], neighbors) {
                    (true, 2 | 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        on_each(&mut tmp);

        std::mem::swap(&mut curr, &mut tmp);
    }

    curr.vec.into_iter().filter(|&b| b).count()
}

pub fn part1(input: &Input) -> usize {
    solve(input, |_| {})
}

pub fn part2(input: &Input) -> usize {
    solve(input, |tmp| {
        tmp[(0usize, 0usize)] = true;
        tmp[(0usize, 99usize)] = true;
        tmp[(99usize, 0usize)] = true;
        tmp[(99usize, 99usize)] = true;
    })
}
