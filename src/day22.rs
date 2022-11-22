#[allow(unused_imports)]
use super::prelude::*;
type Input = (u32, u32);

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
    (health, damage)
}

#[derive(Clone, Copy)]
struct State {
    boss_health: u32,
    player_health: u32,
    player_mana: u32,
    shield_effect: u8,
    poison_effect: u8,
    recharge_effect: u8,
}

fn tick_effects(state: &mut State) {
    if state.shield_effect > 0 {
        state.shield_effect -= 1;
    }

    if state.poison_effect > 0 {
        state.boss_health = state.boss_health.saturating_sub(3);
        state.poison_effect -= 1;
    }

    if state.recharge_effect > 0 {
        state.player_mana += 101;
        state.recharge_effect -= 1;
    }
}

fn solve_rec_boss(
    mut state: State,
    boss_dmg: u32,
    best: Option<u32>,
    curr_mana: u32,
    part2: bool,
) -> Option<u32> {
    if best.map_or(false, |best| best <= curr_mana) {
        return best;
    }

    if part2 {
        state.player_health -= 1;
        if state.player_health == 0 {
            return None;
        }
    }

    tick_effects(&mut state);
    if state.boss_health == 0 {
        return Some(curr_mana);
    }

    let player_shield = if state.shield_effect != 0 { 7 } else { 0 };
    let actual_dmg = max(1, boss_dmg.saturating_sub(player_shield));
    state.player_health = state.player_health.saturating_sub(actual_dmg);

    if state.player_health == 0 {
        return None;
    }

    solve_rec_player(state, boss_dmg, best, curr_mana, part2)
}

fn solve_rec_player(
    mut state: State,
    boss_dmg: u32,
    mut best: Option<u32>,
    curr_mana: u32,
    part2: bool,
) -> Option<u32> {
    if part2 {
        state.player_health -= 1;
        if state.player_health == 0 {
            return None;
        }
    }

    tick_effects(&mut state);
    if state.boss_health == 0 {
        return Some(curr_mana);
    }

    if state.player_mana >= 113 && state.shield_effect == 0 {
        let new_state = State {
            player_mana: state.player_mana - 113,
            shield_effect: 6,
            ..state
        };
        if let Some(new_best) = solve_rec_boss(new_state, boss_dmg, best, curr_mana + 113, part2) {
            best = Some(best.map_or(new_best, |best| min(best, new_best)));
        }
    }

    if state.player_mana >= 173 && state.poison_effect == 0 {
        let new_state = State {
            player_mana: state.player_mana - 173,
            poison_effect: 6,
            ..state
        };
        if let Some(new_best) = solve_rec_boss(new_state, boss_dmg, best, curr_mana + 173, part2) {
            best = Some(best.map_or(new_best, |best| min(best, new_best)));
        }
    }

    if state.player_mana >= 229 && state.recharge_effect == 0 {
        let new_state = State {
            player_mana: state.player_mana - 229,
            recharge_effect: 5,
            ..state
        };
        if let Some(new_best) = solve_rec_boss(new_state, boss_dmg, best, curr_mana + 229, part2) {
            best = Some(best.map_or(new_best, |best| min(best, new_best)));
        }
    }

    if state.player_mana >= 53 {
        let new_state = State {
            player_mana: state.player_mana - 53,
            boss_health: state.boss_health.saturating_sub(4),
            ..state
        };
        if let Some(new_best) = solve_rec_boss(new_state, boss_dmg, best, curr_mana + 53, part2) {
            best = Some(best.map_or(new_best, |best| min(best, new_best)));
        }
    }

    if state.player_mana >= 73 {
        let new_state = State {
            player_mana: state.player_mana - 73,
            boss_health: state.boss_health.saturating_sub(2),
            player_health: state.player_health + 2,
            ..state
        };
        if let Some(new_best) = solve_rec_boss(new_state, boss_dmg, best, curr_mana + 73, part2) {
            best = Some(best.map_or(new_best, |best| min(best, new_best)));
        }
    }

    best
}

fn default_state(boss_health: u32) -> State {
    State {
        boss_health,
        player_health: 50,
        player_mana: 500,
        shield_effect: 0,
        poison_effect: 0,
        recharge_effect: 0,
    }
}

pub fn part1(input: &Input) -> u32 {
    let &(boss_health, boss_dmg) = input;
    solve_rec_player(default_state(boss_health), boss_dmg, None, 0, false).expect("Invalid input")
}

pub fn part2(input: &Input) -> u32 {
    let &(boss_health, boss_dmg) = input;
    solve_rec_player(default_state(boss_health), boss_dmg, None, 0, true).expect("Invalid input")
}
