#[allow(unused_imports)]
use super::prelude::*;
type Input = (u32, u32, u32);

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let health = lines
        .next()
        .expect("Invalid input")
        .strip_prefix("Hit Points: ")
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");
    let damage = lines
        .next()
        .expect("Invalid input")
        .strip_prefix("Damage: ")
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");
    let armor = lines
        .next()
        .expect("Invalid input")
        .strip_prefix("Armor: ")
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");
    (health, damage, armor)
}

fn iter_combinations(
    boss_health: u32,
    boss_dmg: u32,
    boss_armor: u32,
) -> impl Iterator<Item = (u32, bool)> {
    const WEAPONS: &[(u32, u32)] = &[(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
    const ARMOR: &[(u32, u32)] = &[(13, 1), (31, 2), (53, 3), (75, 4), (102, 5), (0, 0)];
    const RINGS: &[(u32, u32, u32)] = &[
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
        (0, 0, 0),
    ];

    let rings_iter = RINGS
        .iter()
        .copied()
        .tuple_combinations()
        .chain([((0, 0, 0), (0, 0, 0))]);
    itertools::iproduct!(WEAPONS, ARMOR, rings_iter).map(
        move |(&weapon, &armor, (ring1, ring2))| {
            let (weapon_cost, weapon_atk) = weapon;
            let (armor_cost, armor_armor) = armor;
            let (ring1_cost, ring1_atk, ring1_armor) = ring1;
            let (ring2_cost, ring2_atk, ring2_armor) = ring2;

            let cost = weapon_cost + armor_cost + ring1_cost + ring2_cost;
            let atk = weapon_atk + ring1_atk + ring2_atk;
            let armor = armor_armor + ring1_armor + ring2_armor;

            let dmg_dealt = max(1, atk.saturating_sub(boss_armor));
            let dmg_taken = max(1, boss_dmg.saturating_sub(armor));

            let turns_to_win = (boss_health + dmg_dealt - 1) / dmg_dealt;
            let turns_to_lose = (100 + dmg_taken - 1) / dmg_taken;

            (cost, turns_to_win <= turns_to_lose)
        },
    )
}

pub fn part1(input: &Input) -> u32 {
    let (boss_health, boss_dmg, boss_armor) = *input;
    iter_combinations(boss_health, boss_dmg, boss_armor)
        .filter(|&(_, win)| win)
        .map(|(cost, _)| cost)
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> u32 {
    let (boss_health, boss_dmg, boss_armor) = *input;
    iter_combinations(boss_health, boss_dmg, boss_armor)
        .filter(|&(_, win)| !win)
        .map(|(cost, _)| cost)
        .max()
        .unwrap()
}
