pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_03.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> u64 {
    input.lines().map(parse_bank).map(max_joltage).sum()
}

fn solve_2(input: &str) -> u64 {
    input
        .lines()
        .map(parse_bank)
        .map(|b| max_joltage_override(b, 12))
        .sum()
}

type Bank = Vec<u64>;

fn parse_bank(bank: &str) -> Bank {
    bank.chars()
        .map(|b| b.to_digit(10).unwrap().into())
        .collect()
}

fn max_joltage(bank: Bank) -> u64 {
    max_joltage_override(bank, 2)
}

fn max_joltage_override(bank: Bank, num_batteries: usize) -> u64 {
    let mut digits: Vec<u64> = Vec::new();
    let mut leftmost_possible_battery = 0;

    for pos in 0..num_batteries {
        let num_batteries_still_to_find = num_batteries - pos - 1;
        let largest_digit = bank[..bank.len() - num_batteries_still_to_find]
            .iter()
            .skip(leftmost_possible_battery)
            .max()
            .unwrap();
        digits.push(*largest_digit);
        let where_largest_digit_from_leftmost = bank
            .iter()
            .skip(leftmost_possible_battery)
            .position(|&x| x == *largest_digit)
            .unwrap();
        leftmost_possible_battery =
            where_largest_digit_from_leftmost + leftmost_possible_battery + 1;
    }

    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as u64 * 10u64.pow(i as u32))
        .sum()
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

#[test]
fn test_max_joltage_override() {
    let bank = parse_bank("987654321111111");
    assert_eq!(max_joltage_override(bank, 12), 987654321111);
}

#[test]
fn test_max_joltage_override_2() {
    let bank = parse_bank("234234234234278");
    assert_eq!(max_joltage_override(bank, 12), 434234234278);
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
#[test]
fn test_solve_1() {
    let input = std::fs::read_to_string("input_03.txt").expect("could not read file");
    assert_eq!(input.lines().count(), 200);
    assert_eq!(solve_1(&input), 17316);
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 3121910778619);
}

#[test]
fn test_solve_2() {
    let input = std::fs::read_to_string("input_03.txt").expect("could not read file");
    assert_eq!(solve_2(&input), 171741365473332);
}
