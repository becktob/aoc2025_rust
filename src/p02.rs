pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_02.txt").expect("could not read file");
    if part2 {
        "wip".to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> u64 {
    parse_ranges(input)
        .into_iter()
        .map(invalid_in_range)
        .map(|v| v.iter().sum::<u64>())
        .sum()
}

fn solve_2(input: &str) -> u64 {
    parse_ranges(input)
        .into_iter()
        .map(invalid_in_range_2)
        .map(|v| v.iter().sum::<u64>())
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    first: u64,
    last: u64,
}

fn parse_ranges(line: &str) -> Vec<Range> {
    let ranges = line.trim().split(',');
    ranges
        .map(|s| {
            let ids: Vec<u64> = s.split('-').map(|s| s.parse().unwrap()).collect();
            Range {
                first: *ids.first().unwrap(),
                last: *ids.last().unwrap(),
            }
        })
        .collect()
}

fn invalid(id: u64) -> bool {
    let n_digits: u32 = id.to_string().len().try_into().unwrap(); // Speedup with log?

    let middle_digit_value = 10u64.pow(n_digits / 2);

    let first = id / middle_digit_value;
    let last = id % middle_digit_value;

    first == last
}

fn invalid_part_2(id: u64) -> bool {
    let digits = id.to_string();
    let n_digits: u32 = digits.len().try_into().unwrap(); // Speedup with log?

    for sequence_len in 1..n_digits {
        let mut chunks = digits
            .as_bytes()
            .chunks_exact(sequence_len.try_into().unwrap());
        if chunks.remainder().len() > 0 {
            // seq_len does not divide n_digits (could check before chunking)
            continue;
        }

        let first = chunks.next().unwrap();
        let all_same = chunks.into_iter().all(|c| c == first);

        if all_same {
            return true;
        }
    }
    false
}

fn invalid_in_range(range: Range) -> Vec<u64> {
    (range.first..=range.last + 1)
        .filter(|&n| invalid(n))
        .collect()
}

fn invalid_in_range_2(range: Range) -> Vec<u64> {
    (range.first..=range.last + 1)
        .filter(|&n| invalid_part_2(n))
        .collect()
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

#[test]
fn test_invalid_part_2() {
    assert_eq!(invalid_part_2(12341234), true);
    assert_eq!(invalid_part_2(12341230), false);

    assert_eq!(invalid_part_2(123123123), true);
    assert_eq!(invalid_part_2(123123120), false);

    assert_eq!(invalid_part_2(1111111), true);
    assert_eq!(invalid_part_2(1110111), false);
}

#[cfg(test)]
static EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_parse() {
    let parsed_example = parse_ranges(EXAMPLE);

    assert_eq!(
        *parsed_example.last().unwrap(),
        Range {
            first: 2121212118,
            last: 2121212124
        }
    );
}

#[test]
fn test_invalid_in_range() {
    let range = Range {
        first: 38593856,
        last: 38593862,
    };
    assert_eq!(invalid_in_range(range), vec![38593859]);
}
#[test]
fn test_invalid_in_range_end() {
    let range = Range {
        first: 998,
        last: 1010,
    };
    assert_eq!(invalid_in_range(range), vec![1010]);
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(EXAMPLE), 1227775554);
}

#[test]
fn test_solve_1() {
    let input = std::fs::read_to_string("input_02.txt").expect("could not read file");
    assert_eq!(solve_1(&input), 8576933996);
}

#[test]
fn test_solve_2_example() {
    assert_eq!(solve_2(EXAMPLE), 4174379265);
}

#[test]
fn test_solve_2() {
    let input = std::fs::read_to_string("input_02.txt").expect("could not read file");
    assert_eq!(solve_2(&input), 25663320831);
}
