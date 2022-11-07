#[allow(unused_imports)]
use super::prelude::*;
type Input = (HashMap<(usize, usize), i32>, usize);

pub fn input_generator(input: &str) -> Input {
    let mut ids = HashMap::new();
    let mut get_id = |name| {
        let next_id = ids.len();
        *ids.entry(name).or_insert(next_id)
    };
    let map = input
        .lines()
        .map(|line| {
            let line = line.trim_end_matches('.');
            let (name, rest) = line.split_once(" would ").expect("Invalid input");
            let (gainlose, rest) = rest.split_once(' ').expect("Invalid input");
            let (n, rest) = rest.split_once(' ').expect("Invalid input");
            let (_, othername) = rest.rsplit_once(' ').expect("Invalid input");
            let nameid = get_id(name);
            let otherid = get_id(othername);
            let n = n.parse::<i32>().expect("Invalid input");
            let n = match gainlose {
                "gain" => n,
                "lose" => -n,
                _ => panic!("Invalid input"),
            };
            ((nameid, otherid), n)
        })
        .collect();
    (map, ids.len())
}

fn max_happiness(
    map: &HashMap<(usize, usize), i32>,
    remaining: &mut Vec<Option<usize>>,
    prev: usize,
) -> i32 {
    let mut best = None;
    for i in 0..remaining.len() {
        if let Some(curr) = remaining[i].take() {
            let base = map[&(curr, prev)] + map[&(prev, curr)];
            let new_best = base + max_happiness(map, remaining, curr);
            best = max(best, Some(new_best));
            remaining[i] = Some(curr);
        }
    }

    best.unwrap_or_else(|| map[&(0, prev)] + map[&(prev, 0)])
}

pub fn part1(input: &Input) -> i32 {
    let &(ref map, n) = input;
    max_happiness(map, &mut (1..n).map(Some).collect(), 0)
}

pub fn part2(input: &Input) -> i32 {
    let &(ref map, n) = input;
    let mut new_map = map.clone();
    new_map.extend((0..n).flat_map(|i| [((i, n), 0), ((n, i), 0)]));
    max_happiness(&new_map, &mut (1..=n).map(Some).collect(), 0)
}
