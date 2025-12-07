use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Sub};

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_07.txt").expect("could not read file");
    if part2 {
        solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> usize {
    let manifold = parse_manifold(input);

    let start_beams = BeamRow::from([manifold.start]);
    manifold
        .splitters
        .iter()
        .scan(start_beams, |beams, splitters| {
            let splits; // Todo: can I mix this into the tuple-destructuring?
            (*beams, splits) = pass_row_count_splits(beams, splitters);
            Some(splits)
        })
        .sum()
}

fn solve_2(input: &str) -> usize {
    let manifold = parse_manifold(input);
    let start_beams = QuantumBeamRow::from([(manifold.start, 1)]);

    let mut line = 0;
    manifold
        .splitters
        .iter()
        .fold(start_beams, |beams, splitters| {
            // let disp: String = (0..144).map(|i| if beams.contains_key(&i) {'|'} else {'.'}).collect();
            // println!("{:?} - {:?}", line, disp);
            line += 1;
            pass_row_quantum(&beams, splitters)
        })
        .values()
        .sum()
}

type BeamRow = HashSet<usize>;
type QuantumBeamRow = HashMap<usize, usize>;
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

#[cfg(test)]
fn pass_row(beams: &BeamRow, splitters: &SplitterRow) -> BeamRow {
    let (new_beams, _) = pass_row_count_splits(beams, splitters);
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

fn pass_row_quantum(beams: &QuantumBeamRow, splitters: &SplitterRow) -> QuantumBeamRow {
    let mut new_beams = QuantumBeamRow::new();

    beams.iter().for_each(|(&beam, count)| {
        if splitters.contains(&beam) {
            new_beams.entry(beam - 1).or_insert(0).add_assign(count);
            new_beams.entry(beam + 1).or_insert(0).add_assign(count);
        } else {
            new_beams.entry(beam).or_insert(0).add_assign(count);
        }
    });
    new_beams
}

#[cfg(test)]
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
fn test_pass_row_quantum() {
    let manifold = parse_manifold(EXAMPLE);
    let beams = QuantumBeamRow::from([(manifold.start, 1)]);
    let beams = pass_row_quantum(&beams, &manifold.splitters[2]);
    assert_eq!(beams, QuantumBeamRow::from([(6, 1), (8, 1)]));
    let beams = pass_row_quantum(&beams, &manifold.splitters[4]);
    assert_eq!(beams, QuantumBeamRow::from([(5, 1), (7, 2), (9, 1)]));
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 21);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "1656");
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 40);
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "76624086587804");
}
