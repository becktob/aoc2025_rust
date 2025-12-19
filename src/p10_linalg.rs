use crate::p10::{Machine, parse_machines};

struct MatrixMachine {
    matrix_buttons: Vec<Vec<i32>>,
    vector_jolts: Vec<i32>,
}

fn convert_machine(machine: &Machine) -> MatrixMachine {
    let matrix_buttons = (0..machine.joltage.len()).map(|i_lamp| {
        machine
            .buttons
            .iter()
            .map(|j_button| j_button.contains(&i_lamp))
            .map(|b| b as i32)
            .collect()
    }).collect();

    MatrixMachine {
        matrix_buttons,
        vector_jolts: (*machine).joltage.to_owned(),
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
