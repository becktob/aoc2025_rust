pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_03.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
    } else {
        "WIP".to_string()
    }
}

type Bank = Vec<u16>;

fn max_joltage(bank: Bank) -> u16 {
    let (where_first, first) = bank
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();
    let second = bank.iter().skip(where_first + 1).max().unwrap();
    10 * first + second
}

#[test]
fn test_max_joltage() {
    let bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(max_joltage(bank), 98)
}
