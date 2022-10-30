#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, Source)>;

#[derive(Clone, Copy)]
pub enum Source {
    Direct(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

#[derive(Clone, Copy)]
pub enum Operand {
    Wire(usize),
    Const(u16),
}

pub fn input_generator(input: &str) -> Input {
    let mut ident_map = HashMap::from([("a", 0), ("b", 1)]);
    let mut mk_wire = |ident| {
        let new_id = ident_map.len();
        *ident_map.entry(ident).or_insert(new_id)
    };
    input
        .lines()
        .map(|line| {
            let (src, wire) = line.split_once(" -> ").expect("Invalid input");
            let wire = mk_wire(wire);

            macro_rules! mk_operand {
                ($value:expr) => {{
                    let value = $value;
                    match value.parse() {
                        Ok(value) => Operand::Const(value),
                        Err(_) => Operand::Wire(mk_wire(value)),
                    }
                }};
            }

            let src = if !src.contains(' ') {
                Source::Direct(mk_operand!(src))
            } else if let Some(operand) = src.strip_prefix("NOT ") {
                Source::Not(mk_operand!(operand))
            } else {
                let (v1, op, v2) = src.splitn(3, ' ').collect_tuple().expect("Invalid input");
                let v1 = mk_operand!(v1);
                let v2 = mk_operand!(v2);
                match op {
                    "AND" => Source::And(v1, v2),
                    "OR" => Source::Or(v1, v2),
                    "LSHIFT" => Source::LShift(v1, v2),
                    "RSHIFT" => Source::RShift(v1, v2),
                    _ => panic!("Invalid input"),
                }
            };
            (wire, src)
        })
        .collect()
}

fn neighbors(src: Source) -> impl Iterator<Item = usize> {
    let (w, other) = match src {
        Source::Direct(w) | Source::Not(w) => (w, None),
        Source::And(w1, w2)
        | Source::Or(w1, w2)
        | Source::LShift(w1, w2)
        | Source::RShift(w1, w2) => (w1, Some(w2)),
    };
    std::iter::once(w).chain(other).filter_map(|op| match op {
        Operand::Wire(w) => Some(w),
        Operand::Const(_) => None,
    })
}

fn extract_and_sort(input: &Input) -> (Vec<Source>, Vec<usize>, Vec<usize>) {
    let mut sources = vec![Source::Direct(Operand::Wire(input.len())); input.len()];
    let mut degrees = vec![0; input.len()];
    for &(w, src) in input {
        sources[w] = src;
        neighbors(src).for_each(|wire| degrees[wire] += 1)
    }

    let mut ordered = Vec::with_capacity(input.len());
    ordered.extend((0..degrees.len()).filter(|&w| degrees[w] == 0));
    let mut inverse = vec![input.len(); input.len()];
    let mut current = 0;
    while let Some(&w) = ordered.get(current) {
        inverse[w] = current;
        current += 1;
        neighbors(sources[w]).for_each(|wire| {
            degrees[wire] -= 1;
            if degrees[wire] == 0 {
                ordered.push(wire);
            }
        });
    }

    (sources, ordered, inverse)
}

fn find_a(sources: &[Source], ordered: &[usize], inverse: &[usize]) -> u16 {
    let mut values = Vec::new();
    for w in ordered.iter().copied().rev() {
        let get = |op| match op {
            Operand::Const(v) => v,
            Operand::Wire(w) => values[sources.len() - 1 - inverse[w]],
        };
        values.push(match sources[w] {
            Source::Direct(op) => get(op),
            Source::And(op1, op2) => get(op1) & get(op2),
            Source::Or(op1, op2) => get(op1) | get(op2),
            Source::Not(op) => !get(op),
            Source::LShift(op1, op2) => get(op1) << get(op2),
            Source::RShift(op1, op2) => get(op1) >> get(op2),
        });
    }
    values[sources.len() - 1 - inverse[0]]
}

pub fn part1(input: &Input) -> u16 {
    let (sources, ordered, inverse) = extract_and_sort(input);

    find_a(&sources, &ordered, &inverse)
}

pub fn part2(input: &Input) -> u16 {
    let (mut sources, ordered, inverse) = extract_and_sort(input);

    let a = find_a(&sources, &ordered, &inverse);
    assert!(matches!(sources[1], Source::Direct(Operand::Const(_))));
    sources[1] = Source::Direct(Operand::Const(a));

    find_a(&sources, &ordered, &inverse)
}
