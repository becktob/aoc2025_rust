use crate::p10::{ButtonPresses, Machine, all_sequences, parse_machines};
use std::iter;

fn configure_machine(machine: MatrixMachine) -> usize {
    let machine_echelon = row_echelon(&machine);
    let solutions = solutions(machine_echelon);
    let minimal_solution = solutions
        .iter()
        .min_by_key(|&presses| presses.iter().sum::<usize>())
        .unwrap();
    minimal_solution.iter().sum()
}

#[derive(Debug, Clone)]
struct MatrixMachine {
    matrix_buttons: Vec<Vec<i32>>,
    vector_jolts: Vec<i32>,
}

fn convert_machine(machine: &Machine) -> MatrixMachine {
    let matrix_buttons = (0..machine.joltage.len())
        .map(|i_lamp| {
            machine
                .buttons
                .iter()
                .map(|j_button| j_button.contains(&i_lamp))
                .map(|b| b as i32)
                .collect()
        })
        .collect();

    MatrixMachine {
        matrix_buttons,
        vector_jolts: (*machine).joltage.to_owned(),
    }
}

fn row_echelon(machine: &MatrixMachine) -> MatrixMachine {
    if machine.matrix_buttons.len() == 1 {
        return machine.to_owned();
    }
    if machine
        .matrix_buttons
        .iter()
        .all(|row| row.iter().all(|el| *el == 0))
    {
        return machine.to_owned();
    }

    let matrix = machine.matrix_buttons.to_owned();
    let vector = machine.vector_jolts.to_owned();
    let h = matrix.len();
    let i_row_with_leftmost_entry = (0..h)
        .min_by_key(|i| {
            matrix[*i]
                .iter()
                .enumerate()
                .find_map(|(j, &val)| if val != 0 { Some(j) } else { None })
                .unwrap_or(usize::MAX)
        })
        .unwrap();

    // move this to top
    let top_row = matrix[i_row_with_leftmost_entry].to_owned();
    let top_vec = vector[i_row_with_leftmost_entry];

    let j_piv = top_row
        .iter()
        .enumerate()
        .find_map(|(j, &val)| if val != 0 { Some(j) } else { None })
        .unwrap();

    let mut rem_vec = vector.clone();
    rem_vec.remove(i_row_with_leftmost_entry);

    let mut rem_matrix = matrix.clone();
    rem_matrix.remove(i_row_with_leftmost_entry);

    let rowwise_factors = rem_matrix
        .iter()
        .map(|row| {
            if row[j_piv] == 0 {
                0
            } else {
                top_row[j_piv] / row[j_piv]
            }
        })
        .collect::<Vec<_>>();

    // subtract first row from remaining rows
    rowwise_factors
        .iter()
        .zip(rem_matrix.iter_mut())
        .for_each(|(f, row)| {
            row.iter_mut()
                .zip(top_row.iter())
                .for_each(|(el, top)| *el -= *top * f)
        });

    rowwise_factors
        .iter()
        .zip(rem_vec.iter_mut())
        .for_each(|(f, el)| *el -= top_vec * f);

    // first column of rem_* should be 0 now.

    let rem_machine = MatrixMachine {
        matrix_buttons: rem_matrix,
        vector_jolts: rem_vec,
    };

    let rem_echelon = row_echelon(&rem_machine);
    MatrixMachine {
        matrix_buttons: iter::once(top_row)
            .chain(rem_echelon.matrix_buttons.into_iter())
            .collect(),
        vector_jolts: iter::once(top_vec)
            .chain(rem_echelon.vector_jolts.into_iter())
            .collect(),
    }
}

fn trim_zero_rows(machine: &MatrixMachine) -> MatrixMachine {
    let matrix = machine
        .matrix_buttons
        .iter()
        .take_while(|row| row.iter().any(|el| *el != 0))
        .cloned()
        .collect::<Vec<_>>();
    let n_keep = matrix.len();

    assert!(machine.vector_jolts.iter().skip(n_keep).all(|el| *el == 0));

    let vec = machine
        .vector_jolts
        .iter()
        .take(n_keep)
        .cloned()
        .collect::<Vec<_>>();

    MatrixMachine {
        matrix_buttons: matrix,
        vector_jolts: vec,
    }
}

fn solutions(machine: MatrixMachine) -> Vec<ButtonPresses> {
    // Todo: recursion
    let max_n_presses = *machine.vector_jolts.iter().max().unwrap(); // Todo: not correct if matrix has negative coefficients, use original machine's joltages

    let i_this_row = machine.matrix_buttons.len() - 1;
    let this_row = machine.matrix_buttons[i_this_row].to_owned();
    let this_joltage = machine.vector_jolts[i_this_row];
    let nonzero_this_row = machine.matrix_buttons[i_this_row]
        .iter()
        .filter(|&el| *el != 0)
        .collect::<Vec<_>>();

    let presses_add_up_to_this_joltage = |presses: &ButtonPresses| {
        presses
            .iter()
            .zip(this_row.iter())
            .map(|(&p, &el)| p as i32 * el)
            .sum::<i32>()
            == this_joltage
    };
    let presses_this_row = (0..=max_n_presses)
        .flat_map(|n_presses| all_sequences(nonzero_this_row.len(), n_presses as usize))
        .map(|nonzero_presses| {
            // expand with zeros to row length
            let mut iter = nonzero_presses.into_iter();
            machine.matrix_buttons[i_this_row]
                .iter()
                .map(|&el| if el == 0 { 0 } else { iter.next().unwrap() })
                .collect()
        })
        .into_iter()
        .filter(presses_add_up_to_this_joltage)
        .collect::<Vec<_>>();

    if i_this_row == 0 {
        return presses_this_row;
    }

    let mut temp_matrix = machine.matrix_buttons.clone();
    temp_matrix.remove(i_this_row);
    let rem_matrix = temp_matrix
        .iter()
        .map(|row| {
            // remove columns solved by this row's buttons
            row.iter()
                .zip(this_row.iter())
                .filter_map(|(&el, &this)| if this == 0 { Some(el) } else { None })
                .collect()
        })
        .collect::<Vec<_>>();

    presses_this_row
        .iter()
        .flat_map(|presses| {
            let mut rem_jolts = machine.vector_jolts.clone();
            rem_jolts.remove(i_this_row);

            let jolts_this_press = machine.matrix_buttons.iter().map(|row| {
                row.iter()
                    .zip(presses.iter())
                    .map(|(&m, &p)| m * p as i32)
                    .sum::<i32>()
            });

            rem_jolts
                .iter_mut()
                .zip(jolts_this_press)
                .for_each(|(j, p)| *j -= p);

            let remaining_machine = MatrixMachine {
                matrix_buttons: rem_matrix.clone(),
                vector_jolts: rem_jolts,
            };
            solutions(remaining_machine)
                .into_iter()
                .map(|rem_solution| {
                    //rem_solution.iter().chain(presses.iter()).cloned().collect())
                    let mut rem_iter = rem_solution.into_iter();
                    presses
                        .iter()
                        .zip(this_row.iter())
                        .map(|(&this_press, &el)| {
                            if el != 0 {
                                this_press
                            } else {
                                rem_iter.next().unwrap()
                            }
                        })
                        .collect()
                })
        })
        .collect()
}

#[test]
fn test_convert_machine() {
    let machines = parse_machines(crate::p10::EXAMPLE);
    let matrix_0 = convert_machine(&machines[0]);
    assert_eq!(
        matrix_0.matrix_buttons,
        vec![
            vec![0, 0, 0, 0, 1, 1,],
            vec![0, 1, 0, 0, 0, 1,],
            vec![0, 0, 1, 1, 1, 0,],
            vec![1, 1, 0, 1, 0, 0,],
        ]
    )
}

#[test]
fn test_row_echelon_2by2() {
    let matrix_buttons = vec![vec![1, 2], vec![1, 1]];
    let vector_jolts = vec![2, 5];
    let machine = MatrixMachine {
        matrix_buttons,
        vector_jolts,
    };
    // need to subtract 1*row0 from row1
    let row_ech = row_echelon(&machine);
    assert_eq!(row_ech.matrix_buttons, vec![vec![1, 2,], vec![0, -1,],]);
    assert_eq!(row_ech.vector_jolts, vec![2, 3])
}

#[test]
fn test_row_echelon_example_0() {
    let machines = parse_machines(crate::p10::EXAMPLE);
    let machine = convert_machine(&machines[0]);
    let row_ech = row_echelon(&machine);
    // This one only needs to be sorted differently
    assert_eq!(
        row_ech.matrix_buttons,
        vec![
            vec![1, 1, 0, 1, 0, 0,],
            vec![0, 1, 0, 0, 0, 1,],
            vec![0, 0, 1, 1, 1, 0,],
            vec![0, 0, 0, 0, 1, 1,],
        ]
    );
    assert_eq!(row_ech.vector_jolts, vec![7, 5, 4, 3,])
}

#[test]
fn test_row_echelon_example_2() {
    let machines = parse_machines(crate::p10::EXAMPLE);
    let machine = convert_machine(&machines[2]);
    assert_eq!(
        machine.matrix_buttons,
        vec![
            vec![1, 1, 1, 0],
            vec![1, 0, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 0, 1, 0],
        ]
    );
    let row_ech = row_echelon(&machine);
    // This one only needs to be sorted differently
    assert_eq!(
        row_ech.matrix_buttons,
        vec![
            vec![1, 1, 1, 0],
            vec![0, -1, 0, 1],
            vec![0, 0, -1, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0]
        ]
    );
    assert_eq!(row_ech.vector_jolts, vec![10, 1, -5, 0, 0, 0]) // not verified, three zeros are plausible.
}

#[test]
fn test_trim_zero_rows() {
    let machines = parse_machines(crate::p10::EXAMPLE);
    let machine = convert_machine(&machines[2]);
    let row_ech = row_echelon(&machine);
    let trimmed_machine = trim_zero_rows(&row_ech);
    assert_eq!(trimmed_machine.matrix_buttons.len(), 3);
    assert_eq!(trimmed_machine.vector_jolts.len(), 3);
}

#[test]
fn test_solutions_one_button_machine() {
    let machine = MatrixMachine {
        matrix_buttons: vec![vec![1]],
        vector_jolts: vec![7],
    };
    assert_eq!(solutions(machine), vec![vec![7],]);
}

#[test]
fn test_solutions_two_button_machine() {
    let machine = MatrixMachine {
        matrix_buttons: vec![vec![1, 2]],
        vector_jolts: vec![7],
    };
    assert_eq!(
        solutions(machine),
        // implementation might change order
        vec![vec![1, 3], vec![3, 2], vec![5, 1], vec![7, 0,]]
    );
}

#[test]
fn test_solutions_three_button_machine() {
    let machine = MatrixMachine {
        matrix_buttons: vec![vec![1, 1, 0], vec![0, 1, 1]],
        vector_jolts: vec![9, 3],
    };
    assert_eq!(
        solutions(machine),
        // implementation might change order
        vec![vec![6, 3, 0,], vec![7, 2, 1], vec![8, 1, 2], vec![9, 0, 3],]
    );
}

#[test]
fn test_known_solution() {
    let machines = parse_machines(crate::p10::EXAMPLE);
    let machine = convert_machine(&machines[0]);
    let machine = row_echelon(&machine);

    let known_solution = vec![1, 3, 0, 3, 1, 2];
    let solutions = solutions(machine);
    assert!(solutions.contains(&known_solution));
}
#[test]
fn test_configure_machine() {
    let machines = parse_machines(crate::p10::EXAMPLE);
    let machine = convert_machine(&machines[0]);
    assert_eq!(10, configure_machine(machine))
}
