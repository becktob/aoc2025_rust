use std::collections::HashSet;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_04.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
    } else {
        "WIP".to_string()
    }
}
type Roll = (usize, usize);
type Diagram = HashSet<Roll>;

fn parse_diagram(diagram: &str) -> Diagram {
    diagram
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (y, x, c)))
        .filter_map(|(y, x, c)| if c == '@' { Some((y, x)) } else { None })
        .collect()
}

fn accessible(diagram: &Diagram, roll: &Roll) -> bool {
    let (y, x) = (roll.0 as i32, roll.1 as i32);
    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter(|&&delta| diagram.contains(&((y + delta.0) as usize, (x + delta.1) as usize)))
    .count();
    neighbors < 4
}

#[cfg(test)]
static EXAMPLE1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

#[test]
fn test_parse_diagram() {
    let diagram = parse_diagram(EXAMPLE1);
    assert_eq!(diagram.len(), 71);
    assert!(!diagram.contains(&(0, 0)));
    assert!(diagram.contains(&(1, 1)));
}

#[test]
fn test_accessible() {
    let diagram = parse_diagram(EXAMPLE1);
    assert!(accessible(&diagram, &(0, 2)));
    assert!(!accessible(&diagram, &(1, 1)));
}
