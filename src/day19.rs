#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (Vec<(&'a str, &'a str)>, &'a str);

pub fn input_generator(input: &str) -> Input<'_> {
    let (rules, molecules) = input.rsplit_once("\n\n").expect("Invalid input");
    let rules = rules
        .lines()
        .map(|line| line.split_once(" => ").expect("Invalid input"))
        .collect();
    (rules, molecules)
}

pub fn part1(input: &Input) -> usize {
    let (rules, molecules) = input;
    let mut seen = HashSet::new();

    for &(from, to) in rules {
        let mut rest = *molecules;
        while let Some(idx) = rest.find(from) {
            let base_len = molecules.len() - rest.len();
            rest = &rest[idx + from.len()..];
            seen.insert(molecules[..base_len + idx].to_string() + to + rest);
        }
    }

    seen.len()
}

pub fn part2(input: &Input) -> u8 {
    let (raw_rules, molecules) = input;

    let curr_id = Cell::new(0);
    let next_id = || {
        curr_id.set(curr_id.get() + 1);
        curr_id.get()
    };

    let mut idmap = HashMap::from([("e", 0)]);
    let mut get_rule_id = |ident| *idmap.entry(ident).or_insert_with(next_id);

    fn split_mol(mut mol: &str) -> impl Iterator<Item = &str> + '_ {
        mol = mol.trim_end();
        std::iter::from_fn(move || {
            if mol.is_empty() {
                return None;
            }
            let mut chars = mol.chars();
            assert!(matches!(chars.next(), Some('A'..='Z')));
            while let Some('a'..='z') = chars.clone().next() {
                chars.next();
            }
            let rest = chars.as_str();
            let matched = mol.strip_suffix(rest).unwrap();
            mol = rest;
            Some(matched)
        })
    }

    let mut rules = Vec::new();
    let mut dedup_rules = HashMap::new();
    for (rule, molecule) in raw_rules {
        let rule = get_rule_id(rule);

        let mut atoms = split_mol(molecule);
        let mut first = get_rule_id(atoms.next().expect("Invalid input"));
        let mut second = get_rule_id(atoms.next().expect("Invalid input"));
        for next in atoms {
            let new_rule = *dedup_rules.entry((first, second)).or_insert_with(next_id);
            rules.push((new_rule, false, first, second));
            (first, second) = (new_rule, get_rule_id(next));
        }

        rules.push((rule, true, first, second));
    }

    let input = split_mol(molecules).map(get_rule_id).collect::<Vec<_>>();

    let rules_len = curr_id.get() + 1;
    let idx = |i, j, k| (i * input.len() + j - 1) * rules_len + k;
    let mut cache = vec![u8::MAX; input.len() * input.len() * rules_len];
    let filter_idx = |i, j| i * input.len() + j - 1;
    let mut filter = vec![false; input.len() * input.len()];

    for (i, &term) in input.iter().enumerate() {
        cache[idx(i, i + 1, term)] = 0;
        filter[filter_idx(i, i + 1)] = true;
    }

    for l in 2..input.len() + 1 {
        for i in 0..input.len() - l + 1 {
            for s in 1..l {
                if !filter[filter_idx(i, i + s)] || !filter[filter_idx(i + s, i + l)] {
                    continue;
                }
                for &(rule, original, left, right) in &rules {
                    let left_steps = cache[idx(i, i + s, left)];
                    let right_steps = cache[idx(i + s, i + l, right)];
                    if left_steps != u8::MAX && right_steps != u8::MAX {
                        let new_best = left_steps + right_steps + original as u8;
                        cache[idx(i, i + l, rule)] = min(cache[idx(i, i + l, rule)], new_best);
                        filter[filter_idx(i, i + l)] = true;
                    }
                }
            }
        }
    }

    let result = cache[idx(0, input.len(), 0)];
    assert_ne!(result, u8::MAX);
    result
}
