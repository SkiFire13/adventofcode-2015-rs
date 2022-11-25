#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instr>;

#[derive(Copy, Clone)]
pub enum Instr {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

pub fn input_generator(input: &str) -> Input {
    fn parse_reg(raw: &str) -> usize {
        match raw {
            "a" => 0,
            "b" => 1,
            _ => panic!("Invalid input"),
        }
    }

    input
        .lines()
        .map(|line| {
            let (instr, rest) = line.split_once(' ').expect("Invalid input");
            match instr {
                "hlf" => Instr::Hlf(parse_reg(rest)),
                "tpl" => Instr::Tpl(parse_reg(rest)),
                "inc" => Instr::Inc(parse_reg(rest)),
                "jmp" => Instr::Jmp(rest.parse().expect("Invalid input")),
                "jie" => {
                    let (reg, offset) = rest.split_once(", ").expect("Invalid input");
                    Instr::Jie(parse_reg(reg), offset.parse().expect("Invalid input"))
                }
                "jio" => {
                    let (reg, offset) = rest.split_once(", ").expect("Invalid input");
                    Instr::Jio(parse_reg(reg), offset.parse().expect("Invalid input"))
                }
                _ => panic!("Invalid input"),
            }
        })
        .collect()
}

fn execute(mut regs: [u64; 2], instrs: &[Instr]) -> u64 {
    let mut ip = 0;

    while let Some(&instr) = instrs.get(ip as usize) {
        let mut next_ip = ip + 1;

        match instr {
            Instr::Hlf(reg) => regs[reg] /= 2,
            Instr::Tpl(reg) => regs[reg] *= 3,
            Instr::Inc(reg) => regs[reg] += 1,
            Instr::Jmp(offset) => next_ip += offset - 1,
            Instr::Jie(reg, offset) if regs[reg] % 2 == 0 => next_ip += offset - 1,
            Instr::Jio(reg, offset) if regs[reg] == 1 => next_ip += offset - 1,
            _ => {}
        }

        ip = next_ip;
    }

    regs[1]
}

pub fn part1(input: &Input) -> u64 {
    execute([0, 0], input)
}

pub fn part2(input: &Input) -> u64 {
    execute([1, 0], input)
}
