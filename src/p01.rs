use std::collections::HashMap;

pub fn solve(part2: bool) -> String {
    42.to_string()
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
    let abs = line.trim_start_matches('R').trim_start_matches('L').parse::<i128>().unwrap();
    sign * abs
}


#[test]
fn test_load_rotations() {
    let rotations = load_rotations();
    assert_eq!(rotations.len(), 4036);
    assert_eq!(*rotations.first().unwrap(), -49);
    assert_eq!(*rotations.last().unwrap(), 39 );
}

#[test]
fn test_parse_rotation() {
    assert_eq!(parse_rotation("R123"), 123);
    assert_eq!(parse_rotation("L45"), -45);
}
