pub fn solve(part2: bool) -> String {
    if part2 {
        "wip".to_string()
    } else {
        "wip".to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    first: u32,
    last: u32,
}

fn parse_ranges(line: &str) -> Vec<Range> {
    let ranges = line.split(',');
    ranges
        .map(|s| {
            let ids: Vec<u32> = s.split('-').map(|s| s.parse().unwrap()).collect();
            Range { first: *ids.first().unwrap(), last: *ids.last().unwrap() }
        })
        .collect()
}

fn invalid(id: u32) -> bool {
    let n_digits: u32 = id.to_string().len().try_into().unwrap(); // Speedup with log?

    let middle_digit_value = 10u32.pow(n_digits / 2);

    let first = id / middle_digit_value;
    let last = id % middle_digit_value;

    first == last
}

fn invalid_in_range(range: &Range) -> Vec<u32> {
    (range.first..=range.last+1).filter(|&n| invalid(n)).collect()
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

#[cfg(test)]
static EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_parse() {
    let parsed_example = parse_ranges(EXAMPLE);

    assert_eq!(*parsed_example.first().unwrap(), Range { first: 11, last: 22 });
    assert_eq!(*parsed_example.last().unwrap(), Range { first: 2121212118, last: 2121212124 });
}

#[test]
fn test_invalid_in_range(){
    assert_eq!(invalid_in_range(&Range{ first: 998, last: 1010 }), vec![1010]);
    assert_eq!(invalid_in_range(&Range{ first: 38593856, last: 38593862 }), vec![38593859]);
}
