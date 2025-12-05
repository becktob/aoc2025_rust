use std::collections::HashSet;
use std::ops;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_05.txt").expect("could not read file");
    if part2 {
        "wip".to_string()
        //solve_2(&input).to_string()
    } else {
        "wip".to_string()
        //solve_1(&input).to_string()
    }
}

type Id = u32;
type FreshRange = ops::RangeInclusive<Id>;

fn parse(input: &str) -> (HashSet<FreshRange>, HashSet<Id>) {
    let (ranges_raw, ingredients_raw) = input.split_once("\n\n").unwrap();
    let ranges = ranges_raw
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(from, to)| from.parse::<Id>().unwrap()..=to.parse::<Id>().unwrap())
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

    assert!(ranges.contains(&(12..=18)));
    assert!(ingredients.contains(&8));
}
