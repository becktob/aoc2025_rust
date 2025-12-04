use std::collections::HashSet;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_04.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
    } else {
        "WIP".to_string()
    }
}

type Diagram = HashSet<(usize, usize)>;

fn parse_diagram(diagram: &str) -> Diagram {
    diagram
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x,c)| (y,x,c)))
        .filter_map(|(y, x, c)| if c == '@' { Some((y, x)) } else { None })
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