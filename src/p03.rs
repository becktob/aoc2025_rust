pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_03.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
    } else {
        "WIP".to_string()
    }
}

type Bank = Vec<u32>;

fn parse_bank(bank: &str) -> Bank {
    bank.chars().map(|b| b.to_digit(10).unwrap()).collect()
}

fn max_joltage(bank: Bank) -> u32 {
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
    let bank = parse_bank("987654321111111");
    assert_eq!(max_joltage(bank), 98);
}
