#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(" can fly ").expect("Invalid input");
            let (speed, rest) = rest.split_once(" km/s for ").expect("Invalid input");
            let (runtime, rest) = rest
                .split_once(" seconds, but then must rest for ")
                .expect("Invalid input");
            let resttime = rest.strip_suffix(" seconds.").expect("Invalid input");
            let speed = speed.parse().expect("Invalid input");
            let runtime = runtime.parse().expect("Invalid input");
            let resttime = resttime.parse().expect("Invalid input");
            (speed, runtime, resttime)
        })
        .collect()
}

fn traveled(speed: usize, runtime: usize, resttime: usize, steps: usize) -> usize {
    let cycletime = runtime + resttime;
    let cycletravel = speed * runtime;
    let cycles = steps / cycletime;
    let remainder = steps % cycletime;
    cycles * cycletravel + min(remainder, runtime) * speed
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|&(speed, runtime, resttime)| traveled(speed, runtime, resttime, 2503))
        .max()
        .expect("Invalid input")
}

pub fn part2(input: &Input) -> usize {
    let mut points = vec![0; input.len()];

    for steps in 1..=2503 {
        let travel_iter = input
            .iter()
            .map(|&(speed, runtime, resttime)| traveled(speed, runtime, resttime, steps));

        let max = travel_iter.clone().max().expect("Invalid input");

        for (travel, points) in travel_iter.zip(&mut points) {
            if travel == max {
                *points += 1;
            }
        }
    }

    points.into_iter().max().expect("Invalid input")
}
