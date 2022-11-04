#[allow(unused_imports)]
use super::prelude::*;
type Input = (usize, HashMap<(usize, usize), usize>);

pub fn input_generator(input: &str) -> Input {
    let mut map = HashMap::new();
    let mut get_id = |s| {
        let next_id = map.len();
        *map.entry(s).or_insert(next_id)
    };
    let edges = input
        .lines()
        .map(|line| {
            let (first, rest) = line.split_once(" to ").expect("Invalid input");
            let (second, dist) = rest.split_once(" = ").expect("Invalid input");
            let first = get_id(first);
            let second = get_id(second);
            let dist = dist.parse().expect("Invalid input");
            ((first, second), dist)
        })
        .collect();
    (map.len(), edges)
}

pub fn part1(input: &Input) -> usize {
    let &(n, ref edges) = input;
    let mut queue = (0..n)
        .map(|n| (Reverse(0), 1u64 << n, n))
        .collect::<BinaryHeap<_>>();
    while let Some((Reverse(len), seen, curr)) = queue.pop() {
        if seen.count_ones() == n as u32 {
            return len;
        }
        for next in 0..n {
            if seen & (1 << next) == 0 {
                if let Some(&cost) = edges
                    .get(&(curr, next))
                    .or_else(|| edges.get(&(next, curr)))
                {
                    queue.push((Reverse(len + cost), seen | 1 << next, next));
                }
            }
        }
    }
    panic!("Invalid input")
}

pub fn part2(input: &Input) -> usize {
    let &(n, ref edges) = input;
    let mut queue = (0..n).map(|n| (0, 1u64 << n, n)).collect::<Vec<_>>();
    let mut max = None;
    while let Some((len, seen, curr)) = queue.pop() {
        if seen.count_ones() == n as u32 {
            max = std::cmp::max(max, Some(len));
            continue;
        }
        for next in 0..n {
            if seen & (1 << next) == 0 {
                if let Some(&cost) = edges
                    .get(&(curr, next))
                    .or_else(|| edges.get(&(next, curr)))
                {
                    queue.push((len + cost, seen | 1 << next, next));
                }
            }
        }
    }
    max.expect("Invalid input")
}
