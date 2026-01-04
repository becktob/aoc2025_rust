use std::fmt::Debug;
use std::iter;
use std::str::FromStr;

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
        .map(shortest_goal_configuration)
        .map(|presses| presses.iter().sum::<usize>())
        .sum()
}

#[derive(Debug, Clone)]
pub(crate) struct Machine {
    goal: Vec<bool>,
    pub(crate) buttons: Vec<Vec<usize>>,
    pub(crate) joltage: Vec<u32>,
}

pub(crate) type ButtonPresses = Vec<usize>; // len == buttons.len; How often is button[i] pushed?

fn configure_joltage(machine: Machine) -> Vec<ButtonPresses> {
    // TODO: how to test recursion, i.e. "this call leads to correct recurring call"?
    if machine.joltage.iter().all(|&j| j == 0) {
        let no_presses = iter::repeat_n(0usize, machine.buttons.len()).collect();
        return vec![no_presses];
    }

    let parity_goal = are_odd(&machine.joltage);

    let parity_machine = Machine {
        goal: parity_goal,
        buttons: machine.buttons.clone(),
        joltage: machine.joltage.clone(),
    };

    let parity_configurations = goal_configurations(&parity_machine);
    let parity_configurations = parity_configurations
        .iter()
        .filter(|&presses| {
            result_of_presses(presses, &machine) // todo: compute result_of_presses() only once
                .iter()
                .zip(machine.joltage.iter())
                .all(|(r, j)| r <= j)
        })
        .collect::<Vec<_>>();

    parity_configurations
        .iter()
        .flat_map(|parity_configuration| {
            // IDEA: subtract parity, recurse on "halved" machine
            let pressed_joltages = result_of_presses(parity_configuration, &machine);
            let even_machine = Machine {
                goal: iter::repeat_n(false, machine.goal.len()).collect(),
                buttons: machine.buttons.clone(),
                joltage: machine
                    .joltage
                    .iter()
                    .zip(pressed_joltages.iter())
                    .map(|(j, p)| j - p)
                    .collect(),
            };
            // TODO: don't need entire machine, only joltage changes
            let half_machine = Machine {
                goal: even_machine.goal,
                buttons: even_machine.buttons,
                joltage: even_machine.joltage.iter().map(|&j| j / 2).collect(),
            };
            let half_machine_solutions = configure_joltage(half_machine);
            let even_machine_solutions = half_machine_solutions
                .into_iter()
                .map(|ps| ps.into_iter().map(|p| p * 2).collect::<ButtonPresses>())
                .collect::<Vec<_>>();

            even_machine_solutions
                .iter()
                .map(move |even| {
                    even.iter()
                        .zip(parity_configuration.iter())
                        .map(|(e, o)| e + o)
                        .collect::<ButtonPresses>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn shortest_goal_configuration(machine: &Machine) -> ButtonPresses {
    goal_configurations(machine)
        .into_iter()
        .min_by_key(|presses| presses.iter().sum::<usize>())
        .unwrap()
}

fn goal_configurations(machine: &Machine) -> Vec<ButtonPresses> {
    // optimal part 1 solution has 0 or 1 presses per button: 2 presses cancel out
    all_selections(machine.buttons.len())
        .into_iter()
        .filter(|presses| are_odd(&result_of_presses(presses, machine)) == machine.goal)
        .collect()
}

fn are_odd(state: &Vec<u32>) -> Vec<bool> {
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
fn test_shortest_goal_configuration() {
    let machine = &parse_machines(EXAMPLE)[0];
    let buttons_presses = shortest_goal_configuration(machine).iter().sum::<usize>();
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
    assert_eq!(are_odd(&state), machine.goal);
    let solution_state = result_of_presses(&vec![0, 0, 0, 0, 1, 1], machine);
    assert_eq!(are_odd(&solution_state), machine.goal);
}

#[test]
fn test_configure_joltage_single_press() {
    let machine = Machine {
        goal: vec![false],
        buttons: vec![vec![0]],
        joltage: vec![1],
    };
    let solutions = configure_joltage(machine);
    assert_eq!(solutions, vec![vec![1]]);
}

#[test]
fn test_configure_joltage_two_presses() {
    let machine = Machine {
        goal: vec![false],
        buttons: vec![vec![0]],
        joltage: vec![2],
    };
    let solutions = configure_joltage(machine);
    assert_eq!(solutions, vec![vec![2]]);
}

#[test]
fn test_configure_joltage_two_buttons_two_presses() {
    let machine = Machine {
        goal: vec![false, false],
        buttons: vec![vec![0, 1], vec![0]],
        joltage: vec![2, 1],
    };
    let solutions = configure_joltage(machine);
    assert_eq!(solutions, vec![vec![1, 1]]);
}

#[test]
fn test_configure_joltage_example_0() {
    let machines = parse_machines(EXAMPLE);
    let solutions = configure_joltage(machines[0].clone());
    let known_solution = vec![1, 3, 0, 3, 1, 2];
    assert!(solutions.contains(&known_solution));
}

#[test]
fn solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 7);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "477");
}
