#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = &'a [u8];

pub fn input_generator(input: &str) -> Input {
    input.as_bytes()
}

fn sum_field(json: &[u8], ignore_red: bool) -> (i64, bool, &[u8]) {
    let (_, json) = read_string(json);
    let [b':', json @ ..] = json else {
        panic!("Invalid input");
    };
    sum_json(json, ignore_red)
}

fn read_string(initial: &[u8]) -> (&[u8], &[u8]) {
    let [b'"', json @ ..] = initial else {
        panic!("Invalid input");  
    };
    let mut json = json;
    loop {
        match json {
            [b'"', rest @ ..] => return (initial.strip_suffix(rest).unwrap(), rest),
            [b'\\', b'"', rest @ ..] | [b'\\', b'\\', rest @ ..] | [_, rest @ ..] => json = rest,
            [] => panic!("Invalid input"),
        }
    }
}

fn sum_json(json: &[u8], ignore_red: bool) -> (i64, bool, &[u8]) {
    match json {
        [b'[', json @ ..] => {
            let (mut acc, _, mut json) = sum_json(json, ignore_red);
            loop {
                match json {
                    [b',', rest @ ..] => {
                        let (to_add, _, rest) = sum_json(rest, ignore_red);
                        acc += to_add;
                        json = rest;
                    }
                    [b']', rest @ ..] => return (acc, false, rest),
                    _ => panic!("Invalid input"),
                }
            }
        }
        [b'{', json @ ..] => {
            let (mut acc, mut has_red, mut json) = sum_field(json, ignore_red);
            loop {
                match json {
                    [b',', rest @ ..] => {
                        let (to_add, is_red, rest) = sum_field(rest, ignore_red);
                        acc += to_add;
                        has_red |= is_red;
                        json = rest;
                    }
                    [b'}', rest @ ..] => {
                        let remove = has_red && ignore_red;
                        return (acc * (!remove as i64), false, rest);
                    }
                    _ => panic!("Invalid input, rest={}", std::str::from_utf8(json).unwrap()),
                }
            }
        }
        [b'"', ..] => {
            let (string, rest) = read_string(json);
            (0, string == b"\"red\"", rest)
        }
        [c @ (b'0'..=b'9' | b'-'), json @ ..] => {
            let mut json = json;
            let sign = if *c == b'-' { -1 } else { 1 };
            let mut acc = if *c == b'-' { 0 } else { (*c - b'0') as i64 };
            while let [c @ b'0'..=b'9', rest @ ..] = json {
                acc = 10 * acc + (*c - b'0') as i64;
                json = rest;
            }
            (sign * acc, false, json)
        }
        _ => panic!("Invalid input, rest"),
    }
}

pub fn part1(input: &Input) -> i64 {
    let (sum, _, rest) = sum_json(input, false);
    assert!(rest.is_empty(), "Invalid input");
    sum
}

pub fn part2(input: &Input) -> i64 {
    let (sum, _, rest) = sum_json(input, true);
    assert!(rest.is_empty(), "Invalid input");
    sum
}
