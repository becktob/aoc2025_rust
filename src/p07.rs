use std::collections::HashSet;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_06.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //solve_1(&input).to_string()
    }
}

struct Manifest {
    start: usize,
    splitters: Vec<HashSet<usize>>
}

fn parse_manifold(input: &str) ->  Manifest{
    let start = input.lines().nth(0).unwrap().find('S').unwrap();
    let splitters = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect()
        })
        .collect();
    Manifest{start, splitters}
}

static EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

#[test]
fn test_parse_manifest() {
    let manifest = parse_manifold(EXAMPLE);
    assert_eq!(manifest.start, 7);
    assert_eq!(manifest.splitters[2], HashSet::from_iter([7]));
    assert_eq!(manifest.splitters[14], HashSet::from_iter([1,3,5,7,9,13]));
}
