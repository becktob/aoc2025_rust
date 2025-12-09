pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_09.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        solve_1(&input).to_string()
    }
}

fn solve_1(input: &str) -> u64 {
    let floor = parse(input);

    floor
        .iter()
        .enumerate()
        .flat_map(|(i, tile)| floor[i + 1..].iter().map(move |other| (tile, other)))
        .map(|(a, b)| rectangle_size(a, b))
        .max()
        .unwrap()
}

type Tile = (i64, i64);
type Floor = Vec<Tile>;

fn rectangle_size(&a: &Tile, &b: &Tile) -> u64 {
    (((b.0 - a.0).abs() + 1) * ((b.1 - a.1).abs() + 1)) as u64
}

fn parse(input: &str) -> Floor {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (numbers[0], numbers[1])
        })
        .collect()
}

#[cfg(test)]
static EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[test]
fn test_parse() {
    let floor = parse(&EXAMPLE);
    assert_eq!(floor.len(), 8);
    assert_eq!(floor[7], (7, 3));
}

#[test]
fn test_rectangle_size() {
    assert_eq!(rectangle_size(&(0, 0), &(0, 0)), 1);
    assert_eq!(rectangle_size(&(2, 5), &(9, 7)), 24);
    assert_eq!(rectangle_size(&(7, 1), &(11, 7)), 35);
    assert_eq!(rectangle_size(&(2, 5), &(11, 1)), 50);
}

#[test]
fn test_solve_1_example() {
    assert_eq!(solve_1(&EXAMPLE), 50);
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(false), "4759930955")
}
