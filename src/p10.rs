use std::fmt::Debug;
use std::str::FromStr;
use std::{iter, vec};

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_10.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
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

#[derive(Debug, Clone)]
pub(crate) struct Machine {
    goal: Vec<bool>,
    pub(crate) buttons: Vec<Vec<usize>>,
    pub(crate) joltage: Vec<i32>,
}

pub(crate) type ButtonPresses = Vec<usize>; // len == buttons.len; How often is button[i] pushed?

fn configure_machine(machine: &Machine) -> ButtonPresses {
    // optimal part 1 solution has 0 or 1 presses per button: 2 presses cancel out
    all_selections(machine.buttons.len())
        .into_iter()
        .filter(|presses| are_odd(result_of_presses(presses, machine)) == machine.goal)
        .min_by_key(|presses| presses.iter().sum::<usize>())
        .unwrap()
}

fn are_odd(state: Vec<u32>) -> Vec<bool> {
    state.iter().map(|n| n % 2 == 1).collect::<Vec<_>>()
}

fn result_of_presses(presses: &ButtonPresses, machine: &Machine) -> Vec<u32> {
    let nothing_pressed = iter::repeat_n(0, machine.goal.len()).collect();
    presses.iter().enumerate().fold(
        nothing_pressed,
        |mut times_pressed: Vec<_>, (i_button, &times_this_button)| {
            machine.buttons[i_button]
                .iter()
                .for_each(|&light| times_pressed[light] += times_this_button as u32);
            times_pressed
        },
    )
}

fn all_selections(positions: usize) -> Vec<ButtonPresses> {
    // all ways to toggle <n> buttons, i.e. 0 or 1 per button

    (0..2usize.pow(positions as u32))
        .map(|n| {
            (0..positions)
                .map(|p| {
                    if (2usize.pow(p as u32) & n) != 0 {
                        1
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
fn sort_machine(machine: &Machine) -> Machine {
    // least common lamps first
    let mut indices = (0..machine.joltage.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| machine.buttons.iter().filter(|&b| b.contains(&i)).count());

    let joltage: Vec<i32> = indices.iter().map(|&i| machine.joltage[i]).collect();
    let buttons: Vec<Vec<usize>> = machine
        .buttons
        .iter()
        .map(|b| {
            let mut mapped_buttons = b
                .iter()
                .map(|&i| indices.iter().position(|&x| x == i).unwrap())
                .collect::<Vec<_>>();
            mapped_buttons.sort();
            mapped_buttons
        })
        .collect();

    Machine {
        goal: machine.goal.clone(),
        buttons,
        joltage,
    }
}

pub(crate) fn parse_machines(input: &str) -> Vec<Machine> {
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

#[cfg(test)]
pub(crate) static EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
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
fn test_all_selections() {
    assert_eq!(all_selections(2), vec![[0, 0], [1, 0], [0, 1], [1, 1]]);
}

#[test]
fn test_result_of_presses() {
    let machine = &parse_machines(EXAMPLE)[0];
    let state = result_of_presses(&vec![0, 1, 0, 1, 0, 2], machine);
    assert_eq!(are_odd(state), machine.goal);
    let solution_state = result_of_presses(&vec![0, 0, 0, 0, 1, 1], machine);
    assert_eq!(are_odd(solution_state), machine.goal);
}

#[test]
fn test_sort_machine() {
    let machine = Machine {
        goal: vec![true, false, true],
        buttons: vec![vec![0, 1, 2], vec![0, 2], vec![0]],
        joltage: vec![3, 1, 2], // equals number of appearances in buttons
    };
    let sorted = sort_machine(&machine);
    assert_eq!(sorted.joltage, vec![1, 2, 3]);
    assert_eq!(sorted.buttons, vec![vec![0, 1, 2], vec![1, 2], vec![2]],);
}

#[test]
fn solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 7);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "477");
}
