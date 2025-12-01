use std::collections::HashMap;

pub fn solve(part2: bool) -> String {
    let rotations = load_rotations();
    if part2 {
        solve_2(rotations).to_string()
    } else {
        solve_1(rotations).to_string()
    }
}

fn solve_1(rotations: Vec<i128>) -> u128 {
    apply_rotations(rotations, 50)
        .iter()
        .filter(|&&x| x == 0)
        .count()
        .try_into()
        .unwrap()
}

fn solve_2(rotations: Vec<i128>) -> u128 {
    let exact_zeros = solve_1(rotations.clone());
    let skipped_zeros: u128 = apply_rotations_skipped_zeros(rotations, 50).iter().sum();
    exact_zeros + skipped_zeros
}

fn load_rotations() -> Vec<i128> {
    std::fs::read_to_string("input_01.txt")
        .expect("could not read file")
        .lines()
        .map(parse_rotation)
        .collect()
}

fn parse_rotation(line: &str) -> i128 {
    let rl = line.chars().nth(0).unwrap();
    let direction_map = HashMap::from([('R', 1), ('L', -1)]);
    let sign = direction_map.get(&rl).unwrap_or(&0);
    let abs = line
        .trim_start_matches('R')
        .trim_start_matches('L')
        .parse::<i128>()
        .unwrap();
    sign * abs
}

fn apply_rotations(rotations: Vec<i128>, mut init: i128) -> Vec<i128> {
    rotations
        .iter()
        .map(|&rot| {
            init += rot;
            init = init.rem_euclid(100);
            init
        })
        .collect()
}

fn apply_rotations_skipped_zeros(rotations: Vec<i128>, mut init: i128) -> Vec<u128> {
    rotations
        .iter()
        .map(|&rot_raw| {
            let full_turns = rot_raw.abs().div_euclid(100);
            let rot = rot_raw - 100 * full_turns * rot_raw.signum();

            let prev = init;
            let next_raw = init + rot;
            let next = next_raw.rem_euclid(100);

            let sign_change = next_raw > 100 || next_raw < 0;
            let skips = if sign_change && prev != 0 && next != 0 {
                1
            } else {
                0
            };
            init = next;
            skips + full_turns.unsigned_abs()
        })
        .collect()
}

#[test]
fn test_load_rotations() {
    let rotations = load_rotations();
    assert_eq!(rotations.len(), 4036);
    assert_eq!(*rotations.first().unwrap(), -49);
    assert_eq!(*rotations.last().unwrap(), 39);
}

#[test]
fn test_parse_rotation() {
    assert_eq!(parse_rotation("R123"), 123);
    assert_eq!(parse_rotation("L45"), -45);
}

#[test]
fn test_apply_rotations() {
    let state = 50;
    let test_rotations = Vec::from([-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    let sequence = apply_rotations(test_rotations, state);
    assert_eq!(sequence, vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]);
}

#[test]
fn test_apply_rotations_skipped_zeros() {
    let state = 50;
    let test_rotations = Vec::from([-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    let skips = apply_rotations_skipped_zeros(test_rotations, state);
    assert_eq!(skips, vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
}

#[test]
fn test_apply_rotations_skipped_zeros_full_rotations() {
    let skips = apply_rotations_skipped_zeros(Vec::from([1000]), 50);
    assert_eq!(skips, vec![10]);
}

#[test]
fn test_apply_rotations_skipped_zeros_full_rotations_backwards() {
    let skips = apply_rotations_skipped_zeros(Vec::from([-1000]), 50);
    assert_eq!(skips, vec![10]);
}

#[test]
fn test_solve_part_1_test() {
    let test_rotations = Vec::from([-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    assert_eq!(solve_1(test_rotations), 3);
}

#[test]
fn test_solve_part_2_test() {
    let test_rotations = Vec::from([-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    assert_eq!(solve_2(test_rotations), 6);
}

#[test]
fn test_solve_part_1() {
    assert_eq!(solve(false), "984");
}
