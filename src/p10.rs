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

#[cfg(test)]
fn solve_2(input: &str) -> usize {
    let machines = parse_machines(input);

    machines
        .iter()
        .map(configure_machine_joltage_lampwise)
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
pub(crate) struct Machine {
    goal: Vec<bool>,
    pub(crate) buttons: Vec<Vec<usize>>,
    pub(crate) joltage: Vec<i32>,
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

#[cfg(test)]
fn configure_machine_joltage_lampwise(machine: &Machine) -> ButtonPresses {
    all_sequences_joltage_lampwise(&sort_machine(machine))
        .iter()
        .cloned()
        .min_by_key(|presses| presses.iter().sum::<usize>())
        .unwrap()
}

#[cfg(test)]
fn all_sequences_joltage_lampwise(machine: &Machine) -> Vec<ButtonPresses> {
    if machine.joltage.len() == 0 {
        return vec![vec![]];
    }

    if machine.joltage.iter().all(|&x| x == 0) {
        return vec![vec![]];
    }

    let buttons_first_lamp = machine
        .buttons
        .iter()
        .filter(|&buttons| buttons.contains(&0))
        .collect::<Vec<_>>();

    let joltage_first_lamp = machine.joltage[0];

    if buttons_first_lamp.is_empty() && joltage_first_lamp > 0 {
        // no way to satisfy this lamp without buttons
        return vec![];
    }

    let buttons_remaining = machine
        .buttons
        .iter()
        .filter(|buttons| !buttons.contains(&0))
        .map(|b| b.into_iter().map(|b| b - 1).collect())
        .collect::<Vec<_>>();

    if buttons_first_lamp.is_empty() && joltage_first_lamp == 0 {
        // skip this lamp, it's already satisfied
        let machine_remaining = Machine {
            goal: vec![],
            buttons: buttons_remaining.clone(),
            joltage: machine.joltage[1..].iter().cloned().collect(),
        };
        return all_sequences_joltage_lampwise(&machine_remaining);
    }

    all_sequences(buttons_first_lamp.len(), joltage_first_lamp as usize)
        .iter()
        .filter_map(|presses| {
            let mut joltage_remaining: Vec<i32> = machine.joltage[1..].iter().cloned().collect();
            buttons_first_lamp
                .iter()
                .zip(presses)
                .for_each(|(button, press)| {
                    button
                        .iter()
                        .filter(|&i| *i != 0)
                        .for_each(|&i| joltage_remaining[i - 1] -= *press as i32)
                });
            if joltage_remaining.iter().any(|&i| i < 0) {
                return None;
            }

            let machine_remaining = Machine {
                goal: vec![],
                buttons: buttons_remaining.clone(),
                joltage: joltage_remaining,
            };
            Some(
                all_sequences_joltage_lampwise(&machine_remaining)
                    .into_iter()
                    .map(move |others| {
                        presses
                            .into_iter()
                            .chain(others.iter())
                            .cloned()
                            .collect::<ButtonPresses>()
                    }),
            )
        })
        .flatten()
        .collect()
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
fn test_configure_machine_joltage_lampwise_1() {
    let machine = &parse_machines(EXAMPLE)[1];
    let button_presses = configure_machine_joltage_lampwise(machine)
        .iter()
        .sum::<usize>();
    assert_eq!(button_presses, 12);
}

#[test]
fn test_configure_machine_joltage_lampwise_1_press() {
    let machine = Machine {
        goal: vec![true, false],
        buttons: vec![vec![0, 1]],
        joltage: vec![1, 1],
    };
    let button_presses = configure_machine_joltage_lampwise(&machine)
        .iter()
        .sum::<usize>();
    assert_eq!(button_presses, 1);
}

#[test]
fn test_configure_machine_joltage_lampwise_2_presses() {
    let machine = Machine {
        goal: vec![true, false],
        buttons: vec![vec![0], vec![1]],
        joltage: vec![1, 1],
    };
    let button_presses = configure_machine_joltage_lampwise(&machine);
    assert_eq!(button_presses, vec![1, 1]);
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

#[test]
fn solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 33);
}

#[ignore]
#[test]
fn test_solve_2() {
    assert_eq!(solve(true), "42");
}

#[test]
fn test_solve_2_time() {
    let input = std::fs::read_to_string("input_10.txt").expect("could not read file");
    let machines = parse_machines(&input);
    let button_presses = configure_machine_joltage_lampwise(&machines[23])
        .iter()
        .sum::<usize>();
    assert_eq!(button_presses, 86); // assuming that is the correct answer; just adding this test for timing
}
