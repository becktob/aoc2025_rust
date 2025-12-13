use std::io::BufRead;

pub fn solve(part2: bool) -> String {
    let input = std::fs::read_to_string("input_10.txt").expect("could not read file");
    if part2 {
        "WIP".to_string()
        //solve_2(&input).to_string()
    } else {
        "WIP".to_string()
        //crate::p10::solve_1(&input).to_string()
    }
}

type PresentShape = Vec<Vec<bool>>;
struct Region {
    width: usize,
    height: usize,
    presets_needed: Vec<usize>,
}

fn parse(input: &str) -> (Vec<PresentShape>, Vec<Region>) {
    let (presents_raw, regions_raw) = input.rsplit_once("\n\n").unwrap();

    let presents = presents_raw.split("\n\n").map(parse_shape).collect();
    let regions = regions_raw.lines().map(parse_region).collect();

    (presents, regions)
}

fn parse_shape(input: &str) -> PresentShape {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn parse_region(input: &str) -> Region {
    let (wh_raw, presents_raw) = input.split_once(':').unwrap();
    let wh = wh_raw
        .split('x')
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let (width, height) = (wh[0], wh[1]);
    let presets_needed = presents_raw
        .trim()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    Region {
        width,
        height,
        presets_needed,
    }
}

static EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[test]
fn test_parse() {
    let (presents, regions) = parse(EXAMPLE);
    assert_eq!(presents.len(), 6);
    assert_eq!(regions.len(), 3);
}
