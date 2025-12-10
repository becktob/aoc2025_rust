use std::fmt::Debug;
use std::str::FromStr;
use std::{iter, vec};

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_09.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> usize {
    let machines = parse_machines(input);

    machines
        .iter()
        .map(configure_machine)
        .map(|presses| presses.iter().sum::<usize>())
        .sum()
}

struct Machine {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

type ButtonPresses = Vec<usize>; // len == buttons.len; How often is button[i] pushed?

fn configure_machine(machine: &Machine) -> ButtonPresses {
    let n_buttons = machine.buttons.len();
    (0..)
        .flat_map(|n_presses| {
            (0..n_presses).flat_map(|n_pressed| all_sequences(n_buttons, n_pressed))
        })
        .find(|presses| result_of_presses(presses, machine) == machine.goal)
        .unwrap()
}

fn result_of_presses(presses: &ButtonPresses, machine: &Machine) -> Vec<bool> {
    presses
        .iter()
        .enumerate()
        .fold(
            iter::repeat_n(0usize, machine.goal.len()).collect(),
            |mut times_toggled: Vec<_>, (i_button, times_pressed)| {
                machine.buttons[i_button]
                    .iter()
                    .for_each(|&light| times_toggled[light] += times_pressed);
                times_toggled
            },
        )
        .iter()
        .map(|times_toggled| times_toggled % 2 == 1)
        .collect()
}

fn all_sequences(positions: usize, sum: usize) -> Vec<ButtonPresses> {
    // todo: return Impl Iterator here?

    if positions == 1 {
        return vec![vec![sum]];
    }
    (0..=sum)
        .flat_map(|times_this_button_pressed| {
            all_sequences(positions - 1, sum - times_this_button_pressed)
                .into_iter()
                .map(move |mut others| {
                    others.push(times_this_button_pressed);
                    others
                })
        })
        .collect::<Vec<_>>()
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}

fn parse_machine(line: &str) -> Machine {
    let (goal_raw, rest) = line.split_once(" ").unwrap();
    let (buttons_raw, jolt_raw) = rest.rsplit_once(" ").unwrap();
    let goal = goal_raw
        .trim_matches(|c| "[]".contains(c))
        .chars()
        .map(|x| x == '#')
        .collect();
    let buttons = buttons_raw.split(" ").map(split_parens).collect();
    let joltage = split_parens(jolt_raw);

    Machine {
        goal,
        buttons,
        joltage,
    }
}

fn split_parens<T: FromStr>(parens: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    parens
        .trim_matches(|c| "{}()[]".contains(c))
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

static EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[test]
fn test_parse_machines() {
    let machines = parse_machines(EXAMPLE);
    assert_eq!(machines.len(), 3);
    assert_eq!(machines[0].goal, vec![false, true, true, false]);
    assert_eq!(machines[0].buttons[5], vec![0, 1]);
    assert_eq!(machines[0].joltage, vec![3, 5, 4, 7]);
}

#[test]
fn test_configure_machine() {
    let machine = &parse_machines(EXAMPLE)[0];
    let buttons_presses = configure_machine(machine).iter().sum::<usize>();
    assert_eq!(buttons_presses, 2);
}

#[test]
fn test_all_sequences() {
    assert_eq!(all_sequences(2, 2), vec![[2, 0], [1, 1], [0, 2]]);
}

#[test]
fn test_result_of_presses() {
    let machine = &parse_machines(EXAMPLE)[0];
    let state = result_of_presses(&vec![0, 1, 0, 1, 0, 2], machine);
    assert_eq!(state, machine.goal);
    let solution_state = result_of_presses(&vec![0, 0, 0, 0, 1, 1], machine);
    assert_eq!(solution_state, machine.goal);
}

#[test]
fn solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 7);
}
