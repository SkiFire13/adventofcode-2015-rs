#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = &'a [u8];

pub fn input_generator(input: &str) -> Input<'_> {
    input.as_bytes()
}

fn fmt_to_buf(mut i: usize, buf: &mut [u8; 20]) -> &[u8] {
    let mut idx = 20;
    while i != 0 {
        idx -= 1;
        buf[idx] = (i % 10) as u8 + b'0';
        i /= 10;
    }
    &buf[idx..]
}

fn find_md5(input: &[u8], max_b3: u8) -> usize {
    use md5::{Digest, Md5};

    let base_hasher = Md5::new_with_prefix(input);

    (1..)
        .par_find_chunked(250_000, |i| {
            let mut hasher = base_hasher.clone();
            hasher.update(fmt_to_buf(i, &mut [0; 20]));
            matches!(&*hasher.finalize(), &[0, 0, b3, ..] if b3 <= max_b3)
        })
        .unwrap()
}

pub fn part1(input: &Input) -> usize {
    find_md5(input, 15)
}

pub fn part2(input: &Input) -> usize {
    find_md5(input, 0)
}
