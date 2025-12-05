use std::iter;
use std::vec::Vec;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_05.txt").expect("could not read file");
    if part2 {
        solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);
    let fresh_ingredients = ingredients
        .iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)));
    fresh_ingredients.count()
}

fn solve_2(input: &str) -> u64 {
    let (ranges, _) = parse(input);
    let mut union = Vec::new();

    for r in ranges {
        union = union_into(&union, &r.clone());
    }

    union.iter().map(|r| r.end - r.start + 1).sum()
}

fn union_into(ranges: &Vec<FreshRange>, new_range: &FreshRange) -> Vec<FreshRange> {
    let ranges_to_union: Vec<FreshRange> = ranges
        .iter()
        .filter(|existing| existing.intersects(new_range))
        .chain(iter::once(new_range))
        .cloned()
        .collect();

    let lower_bound = ranges_to_union.iter().map(|r| r.start).min().unwrap();
    let upper_bound = ranges_to_union.iter().map(|r| r.end).max().unwrap();

    let new_union = FreshRange {
        start: lower_bound,
        end: upper_bound,
    };

    ranges
        .iter()
        .filter(|existing| !existing.intersects(new_range))
        .chain(iter::once(&new_union))
        .cloned()
        .collect()
}

type Id = u64;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FreshRange {
    start: Id,
    end: Id,
}

impl FreshRange {
    fn contains(&self, id: &Id) -> bool {
        self.start <= *id && *id <= self.end
    }

    fn intersects(&self, other: &FreshRange) -> bool {
        self.contains(&other.end) | self.contains(&other.start)
    }
}

fn parse(input: &str) -> (Vec<FreshRange>, Vec<Id>) {
    let (ranges_raw, ingredients_raw) = input.split_once("\n\n").unwrap();
    let ranges = ranges_raw
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(from, to)| FreshRange {
            start: from.parse().unwrap(),
            end: to.parse().unwrap(),
        })
        .collect();
    let ingredients = ingredients_raw
        .lines()
        .map(|l| l.parse::<Id>().unwrap())
        .collect();
    (ranges, ingredients)
}

#[cfg(test)]
static EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn test_parse() {
    let (ranges, ingredients) = parse(&EXAMPLE);
    assert_eq!(ranges.len(), 4);
    assert_eq!(ingredients.len(), 6);

    assert!(ranges.contains(&FreshRange { start: 12, end: 18 }));
    assert!(ingredients.contains(&8));
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 3);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "640");
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 14);
}

#[test]
fn test_solve_2_union_into_fill_gap() {
    let with_gap = vec![
        FreshRange { start: 0, end: 10 },
        FreshRange { start: 20, end: 30 },
    ];

    let middle = FreshRange { start: 10, end: 20 };
    let union = union_into(&with_gap, &middle);
    assert_eq!(union, vec![FreshRange { start: 0, end: 30 }]);
}

#[ignore]
#[test]
fn test_solve_2_union_into_fill_gap_non_overlapping() {
    let with_gap = vec![
        FreshRange { start: 0, end: 10 },
        FreshRange { start: 20, end: 30 },
    ];

    let middle = FreshRange { start: 11, end: 19 };
    let union = union_into(&with_gap, &middle);
    assert_eq!(union, vec![FreshRange { start: 0, end: 30 }]);
}

#[test]
fn test_solve_2_union_into_respects_gaps_of_size_1() {
    let with_gap = vec![
        FreshRange { start: 0, end: 10 },
        FreshRange { start: 20, end: 30 },
    ];

    let middle = FreshRange { start: 12, end: 18 };
    let union = union_into(&with_gap, &middle);
    assert_eq!(union.len(), 3);
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "640");
    // 369482253727747 too high
    // 371869364553730 too high
    // 371481987128973
    // 369577397844941 too high
    // 381245701025211
    // ... getting unstable, but similar results without changing code :(
    // 400495066357532 using Vec
}
