use std::collections::HashMap;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_06.txt").expect("could not read file");
    if part2 {
        solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> u128 {
    let problems = parse_problems(&input);
    problems.iter().map(compute).sum()
}

fn solve_2(input: &str) -> u128 {
    let problems = parse_input_cephalopod(&input);
    problems.iter().map(compute).sum()
}

fn compute(problem: &Problem) -> u128 {
    let op = problem.operator;
    problem.numbers[1..]
        .iter()
        .fold(problem.numbers[0], |acc, &i| op(acc, i))
}

struct Problem {
    numbers: Vec<u128>,
    operator: fn(u128, u128) -> u128,
}

type BinOp = fn(u128, u128) -> u128;

fn add(x: u128, y: u128) -> u128 {
    x + y
}

fn mul(x: u128, y: u128) -> u128 {
    x * y
}

fn parse_problems(input: &&str) -> Vec<Problem> {
    let n_problems = input.lines().next().unwrap().split_whitespace().count();

    let mut iterators_per_line: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| line.split_whitespace())
        .collect();

    let columns: Vec<Vec<&str>> = (0..n_problems)
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

fn parse_input_cephalopod(input: &str) -> Vec<Problem> {
    let mut operators: HashMap<&str, BinOp> = HashMap::new();
    operators.insert("+", add);
    operators.insert("*", mul);
    let operators: Vec<BinOp> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|c| *operators.get(c).unwrap())
        .collect();

    let n_number_rows = input.lines().count() - 1;
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let number_chars = chars[..n_number_rows].to_vec();

    let columns = transpose(number_chars);

    let blocks = columns
        .split(|col| col.iter().all(|&char| char == ' '))
        .to_owned();

    let numbers_s: Vec<Vec<u128>> = blocks
        .map(|cols| {
            cols.iter()
                .rev()
                .map(|c| c.iter().collect::<String>().trim().parse().unwrap())
                .collect()
        })
        .collect();

    numbers_s
        .into_iter()
        .zip(operators)
        .map(|(numbers, operator)| Problem { numbers, operator })
        .collect()
}

fn transpose<T>(number_chars: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n_cols = number_chars[0].len();
    let n_rows = number_chars.len();

    let mut iterators_per_line: Vec<_> =
        number_chars.into_iter().map(|line| line.into_iter()).collect();

    let columns: Vec<Vec<T>> = (0..n_cols)
        .map(|_| {
            iterators_per_line[0..n_rows]
                .iter_mut()
                .map(|line| line.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect();
    columns
}

#[cfg(test)]
// trailing whitespace is relevant!
static EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

#[test]
fn test_parse_problems() {
    let problems = parse_problems(&EXAMPLE);
    assert_eq!(problems.len(), 4);
    assert_eq!(problems[0].numbers, vec![123, 45, 6]);
    // Todo test operator equality?
}

#[test]
fn test_parse_cephalopod() {
    let problems = parse_input_cephalopod(&EXAMPLE);
    assert_eq!(problems.len(), 4);
    assert_eq!(problems[0].numbers, vec![356, 24, 1]);
    assert_eq!(problems[3].numbers, vec![4, 431, 623]);
    assert_eq!(compute(&problems[0]), 8544);
    assert_eq!(compute(&problems[3]), 1058);
}

#[test]
fn test_compute() {
    let problems = parse_problems(&EXAMPLE);
    assert_eq!(compute(&problems[0]), 33210);
    assert_eq!(compute(&problems[1]), 490);
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 4277556);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "4693159084994");
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 3263827);
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "11643736116335");
}
