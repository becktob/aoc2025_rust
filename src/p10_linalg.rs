use crate::p10::{Machine, parse_machines};
use std::iter;

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

    let matrix = machine.matrix_buttons.to_owned();
    let vector = machine.vector_jolts.to_owned();
    let h = matrix.len();
    let i_row_with_leftmost_entry = (0..h)
        .min_by_key(|i| {
            matrix[*i]
                .iter()
                .enumerate()
                .find_map(|(j, &val)| if val != 0 { Some(j) } else { None })
        })
        .unwrap_or(usize::MAX);

    // move this to top
    let top_row = matrix[i_row_with_leftmost_entry].to_owned();
    let top_vec = vector[i_row_with_leftmost_entry];

    let mut rem_vec = vector
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, _)| *i != i_row_with_leftmost_entry)
        .map(|(i, val)| val)
        .collect::<Vec<_>>();

    let mut rem_matrix = matrix
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, _)| *i != i_row_with_leftmost_entry)
        .map(|(i, val)| val)
        .collect::<Vec<_>>();

    let rowwise_factors = rem_matrix
        .iter()
        .map(|row| if row[0] == 0 { 0 } else { top_row[0] / row[0] })
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
fn test_row_echelon_example() {
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
