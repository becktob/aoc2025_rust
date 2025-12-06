use std::collections::HashMap;
use std::ops;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_06.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //crate::p06::solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //crate::p06::solve_1(&input).to_string()
    }
}

struct Problem {
    numbers: Vec<u32>,
    operator: fn(u32, u32) -> u32,
}

type BinOp = fn(u32, u32) -> u32;

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn mul(x: u32, y: u32) -> u32 {
    x * y
}

fn parse_problems(input: &&str) -> Vec<Problem> {
    let n_cols = input.lines().next().unwrap().split_whitespace().count();

    let mut iterators_per_line: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| line.split_whitespace())
        .collect();

    let columns: Vec<Vec<&str>> = (0..n_cols)
        .map(|_| {
            iterators_per_line
                .iter_mut()
                .map(|line| line.next().unwrap())
                .collect()
        })
        .collect();

    let mut operators: HashMap<&str, BinOp> = HashMap::new();
    operators.insert("+", add);
    operators.insert("*", mul);
    columns
        .iter()
        .map(|strings| Problem {
            numbers: strings[0..(strings.len() - 1)]
                .iter()
                .map(|str| str.parse().unwrap())
                .collect(),
            operator: *operators.get(strings.last().unwrap()).unwrap(),
        })
        .collect()
}

#[cfg(test)]
static EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

#[test]
fn test_parse_problems() {
    let problems = parse_problems(&EXAMPLE);
    assert_eq!(problems.len(), 4);
    assert_eq!(problems[0].numbers, vec![123, 45, 6]);
    // Todo test operator equality?
}
