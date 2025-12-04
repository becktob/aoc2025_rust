use std::collections::HashSet;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_04.txt").expect("could not read file");
    if part2 {
        solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> usize {
    let diagram = parse_diagram(input);
    diagram
        .iter()
        .filter(|&roll| accessible(&diagram, roll))
        .count()
}

fn solve_2(input: &str) -> usize {
    let mut diagram = parse_diagram(input);
    let inital_count = diagram.len();
    let mut last_count = usize::MAX;

    while diagram.len() < last_count {
        last_count = diagram.len();
        diagram = remove_accessible(diagram)
    }
    inital_count - last_count
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
    let (y, x) = (roll.0, roll.1);
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let neighbors = deltas
        .iter()
        .filter(|&&delta| {
            // TODO: this goes wrong when there are rolls at usize.MAX
            diagram.contains(&(
                y.wrapping_add_signed(delta.0),
                x.wrapping_add_signed(delta.1),
            ))
        })
        .count();
    neighbors < 4
}

fn remove_accessible(diagram: Diagram) -> Diagram {
    diagram
        .iter()
        .filter(|&roll| !accessible(&diagram, roll))
        .cloned()
        .collect()
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
#[test]
fn test_remove_accessible() {
    let removed_once = remove_accessible(parse_diagram(EXAMPLE1));
    assert_eq!(removed_once.len(), 71 - 13);
    let removed_twice = remove_accessible(removed_once);
    assert_eq!(removed_twice.len(), 71 - 13 - 12);
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE1), 13);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "1393");
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE1), 43);
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "8643");
}
