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

fn solve_1(input: &str) -> usize {
    let manifold = parse_manifold(input);

    let start_beams = BeamRow::from([manifold.start]);
    manifold
        .splitters
        .iter()
        .scan(start_beams, |beams, splitters| {
            let splits;  // Todo: can I mix this into the tuple-destructuring?
            (*beams, splits) = pass_row_count_splits(beams, splitters);
            Some(splits)
        })
        .sum()
}

type BeamRow = HashSet<usize>;
type SplitterRow = HashSet<usize>;
struct Manifold {
    start: usize,
    splitters: Vec<SplitterRow>,
}

fn parse_manifold(input: &str) -> Manifold {
    let start = input.lines().nth(0).unwrap().find('S').unwrap();
    let splitters = input
        .lines()
        .map(|line| line.match_indices('^').map(|(i, _)| i).collect())
        .collect();
    Manifold { start, splitters }
}

fn pass_row(beams: &BeamRow, splitters: &SplitterRow) -> BeamRow {
    let (new_beams, splits) = pass_row_count_splits(beams, splitters);
    new_beams
}

fn pass_row_count_splits(beams: &BeamRow, splitters: &SplitterRow) -> (BeamRow, usize) {
    let mut splits = 0;
    let new_beams = beams
        .iter()
        .cloned()
        .flat_map(|beam| {
            if splitters.contains(&beam) {
                splits += 1;
                vec![beam.sub(1), beam.add(1)]
            } else {
                vec![beam]
            }
        })
        .collect();
    (new_beams, splits)
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
fn test_parse_manifold() {
    let manifold = parse_manifold(EXAMPLE);
    assert_eq!(manifold.start, 7);
    assert_eq!(manifold.splitters[2], SplitterRow::from([7]));
    assert_eq!(
        manifold.splitters[14],
        SplitterRow::from([1, 3, 5, 7, 9, 13])
    );
}

#[test]
fn test_pass_row() {
    let manifold = parse_manifold(EXAMPLE);
    let start_beam = BeamRow::from([manifold.start]);
    let beams_after_first_splitter = pass_row(&start_beam, &manifold.splitters[2]);
    let beams_after_second = pass_row(&beams_after_first_splitter, &manifold.splitters[4]);
    assert_eq!(beams_after_first_splitter, BeamRow::from([6, 8]));
    assert_eq!(beams_after_second, BeamRow::from([5, 7, 9]));
}

#[test]
fn test_pass_row_count_splits() {
    let manifold = parse_manifold(EXAMPLE);
    let beam = BeamRow::from([manifold.start]);
    let (beam, splits_first) = pass_row_count_splits(&beam, &manifold.splitters[2]);
    let (_, splits_second) = pass_row_count_splits(&beam, &manifold.splitters[4]);
    assert_eq!(splits_first, 1);
    assert_eq!(splits_second, 2);
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 21);
}
