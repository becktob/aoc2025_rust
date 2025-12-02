pub fn solve(part2: bool) -> String {
    if part2 {
        "wip".to_string()
    } else {
        "wip".to_string()
    }
}

fn invalid(id: u32) -> bool {
    let n_digits: u32 = id.to_string().len().try_into().unwrap(); // Speedup with log?

    let middle_digit_value = 10u32.pow(n_digits / 2);

    let first = id / middle_digit_value;
    let last = id % middle_digit_value;

    first == last
}

#[test]
fn test_invalid() {
    assert_eq!(invalid(1), false);

    assert_eq!(invalid(22), true);
    assert_eq!(invalid(12), false);

    assert_eq!(invalid(121), false);

    assert_eq!(invalid(1010), true);

    assert_eq!(invalid(1188511885), true);
}
