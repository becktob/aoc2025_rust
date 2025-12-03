pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_03.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
    } else {
        "WIP".to_string()
    }
}

fn solve_1(input: &str) -> u32 {
    input.lines().map(parse_bank).map(max_joltage).sum()
}

type Bank = Vec<u32>;

fn parse_bank(bank: &str) -> Bank {
    bank.chars().map(|b| b.to_digit(10).unwrap()).collect()
}

fn max_joltage(bank: Bank) -> u32 {
    let first = bank[..bank.len() - 1].iter().max().unwrap();
    let where_first = bank.iter().position(|&x| x == *first).unwrap();
    let second = bank.iter().skip(where_first + 1).max().unwrap();
    10 * first + second
}

#[test]
fn test_max_joltage() {
    let bank = parse_bank("987654321111111");
    assert_eq!(max_joltage(bank), 98);
}

#[test]
fn test_max_joltage_largest_digit_at_end() {
    let bank2 = parse_bank("811111111111119");
    assert_eq!(max_joltage(bank2), 89);
}

#[test]
fn test_max_joltage_largest_digit_repeats() {
    let bank2 = parse_bank("9988");
    assert_eq!(max_joltage(bank2), 99);
}

#[cfg(test)]
static EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 357);
}
