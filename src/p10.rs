use std::fmt::Debug;
use std::str::FromStr;
use std::{iter, vec};

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_10.txt").expect("could not read file");
    if part2 {
        solve_2(&input).to_string()
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

fn solve_2(input: &str) -> usize {
    let machines = parse_machines(input);

    machines
        .iter()
        .map(configure_machine_joltage)
        .enumerate()
        .map(|(i, presses)| {
            println!(
                "{:?}/{:?} - {:?}",
                i,
                machines.len(),
                presses.iter().sum::<usize>()
            );
            presses.iter().sum::<usize>()
        })
        .sum()
}

#[derive(Debug, Clone)]
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
        .find(|presses| are_odd(result_of_presses(presses, machine)) == machine.goal)
        .unwrap()
}

fn configure_machine_joltage(machine: &Machine) -> ButtonPresses {
    let mut buttons = machine.buttons.clone();
    buttons.sort_by_key(|buttons| buttons.len());
    buttons.reverse();

    let sorted_machine = Machine{goal: machine.goal.clone(), buttons, joltage: machine.joltage.clone()};

    all_sequences_exact_joltage(sorted_machine)
        .iter()
        .cloned()
        .min_by_key(|presses| presses.iter().sum::<usize>())
        .unwrap()
}

fn all_sequences_exact_joltage(machine: Machine) -> Vec<ButtonPresses> {
    // todo: return Impl Iterator here?
    // recursion: (how many times can this button be pushed) X (how can the Machine with n-1 buttons do the rest?)

    if machine.joltage.iter().all(|&i| i == 0) {
        return vec![iter::repeat_n(0, machine.buttons.len()).collect()];
    }

    if machine.buttons.len() == 0 {
        return vec![];
    }

    let this_button = &machine.buttons[0];
    let max_presses = this_button
        .iter()
        .map(|&i| machine.joltage[i])
        .min()
        .unwrap();

    let presses = (0..=max_presses)
        .flat_map(|times_this_button_pressed| {
            let remaining_joltage = machine
                .joltage
                .iter()
                .enumerate()
                .map(|(i, &joltage)| {
                    if this_button.contains(&i) {
                        joltage - times_this_button_pressed
                    } else {
                        joltage
                    }
                })
                .collect();

            let remaining_buttons = machine.buttons[1..].to_owned();
            let remaining_machine = Machine {
                goal: machine.goal.clone(),
                buttons: remaining_buttons,
                joltage: remaining_joltage,
            };
            all_sequences_exact_joltage(remaining_machine)
                .into_iter()
                .map(move |mut others| {
                    others.insert(0, times_this_button_pressed as usize);
                    others
                })
        })
        .collect::<Vec<_>>();
    presses
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

#[cfg(test)]
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
fn test_configure_machine_joltage_0() {
    let machine = &parse_machines(EXAMPLE)[0];
    let button_presses = configure_machine_joltage(machine).iter().sum::<usize>();
    assert_eq!(button_presses, 10);
}

#[test]
fn test_configure_machine_joltage_1() {
    let machine = &parse_machines(EXAMPLE)[1];
    let button_presses = configure_machine_joltage(machine).iter().sum::<usize>();
    assert_eq!(button_presses, 12);
}

#[test]
fn test_configure_machine_joltage_terminate() {
    let buttons = vec![vec![0, 1]];
    let finished_machine = Machine {
        goal: vec![true, false],
        buttons,
        joltage: vec![0, 0],
    };
    let button_presses = configure_machine_joltage(&finished_machine)
        .iter()
        .sum::<usize>();
    assert_eq!(button_presses, 0);
}

#[test]
fn test_configure_machine_joltage_terminate_no_buttons() {
    let buttons = vec![];
    let finished_machine = Machine {
        goal: vec![true, false],
        buttons,
        joltage: vec![0, 0],
    };
    let button_presses = configure_machine_joltage(&finished_machine)
        .iter()
        .sum::<usize>();
    assert_eq!(button_presses, 0);
}

#[test]
fn test_configure_machine_joltage_1_press() {
    let finished_machine = Machine {
        goal: vec![true, false],
        buttons: vec![vec![0, 1]],
        joltage: vec![1, 1],
    };
    let button_presses = configure_machine_joltage(&finished_machine)
        .iter()
        .sum::<usize>();
    assert_eq!(button_presses, 1);
}

#[test]
fn test_all_sequences_exact_joltage_0() {
    let machine = parse_machines(EXAMPLE);
    let button_presses = all_sequences_exact_joltage(machine[0].clone());
    let one_solution = vec![1, 3, 0, 3, 1, 2];
    assert!(button_presses.contains(&one_solution));
}

#[test]
fn test_all_sequences_exact_joltage_1() {
    let machine = parse_machines(EXAMPLE);
    let button_presses = all_sequences_exact_joltage(machine[1].clone());
    let one_solution = vec![2, 5, 0, 5, 0];
    assert!(button_presses.contains(&one_solution));
}

#[test]
fn test_all_sequences_exact_joltage_2() {
    let machine = parse_machines(EXAMPLE);
    let button_presses = all_sequences_exact_joltage(machine[2].clone());
    let one_solution = vec![5, 0, 5, 1];
    assert!(button_presses.contains(&one_solution));
}

#[test]
fn test_all_sequences_exact_joltage_2_presses() {
    let machine = Machine {
        goal: vec![true, false],
        buttons: vec![vec![0, 1], vec![1]],
        joltage: vec![2, 2],
    };
    let button_presses = all_sequences_exact_joltage(machine);
    assert_eq!(button_presses, vec![vec![2, 0]]);
}

#[test]
fn test_all_sequences() {
    assert_eq!(all_sequences(2, 2), vec![[2, 0], [1, 1], [0, 2]]);
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
fn solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 7);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "477");
}

#[test]
fn solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 33);
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "42");
}
