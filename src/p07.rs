use std::collections::HashSet;
use std::ops::{Add, Sub};

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

type BeamRow = HashSet<usize>;
type SplitterRow = HashSet<usize>;
struct Manifest {
    start: usize,
    splitters: Vec<SplitterRow>,
}

fn parse_manifold(input: &str) -> Manifest {
    let start = input.lines().nth(0).unwrap().find('S').unwrap();
    let splitters = input
        .lines()
        .map(|line| line.match_indices('^').map(|(i, _)| i).collect())
        .collect();
    Manifest { start, splitters }
}

fn pass_row(beams: &BeamRow, splitters: &SplitterRow) -> BeamRow {
    beams
        .iter()
        .cloned()
        .flat_map(|beam| {
            if splitters.contains(&beam) {
                vec![beam.sub(1), beam.add(1)]
            } else {
                vec![beam]
            }
        })
        .collect()
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
    assert_eq!(manifest.splitters[2], SplitterRow::from([7]));
    assert_eq!(
        manifest.splitters[14],
        SplitterRow::from([1, 3, 5, 7, 9, 13])
    );
}

#[test]
fn test_pass_row() {
    let manifest = parse_manifold(EXAMPLE);
    let start_beam = BeamRow::from([manifest.start]);
    let beams_after_first_splitter = pass_row(&start_beam, &manifest.splitters[2]);
    let beams_after_second = pass_row(&beams_after_first_splitter, &manifest.splitters[4]);
    assert_eq!(beams_after_first_splitter, BeamRow::from([6, 8]));
    assert_eq!(beams_after_second, BeamRow::from([5, 7, 9]));
}
