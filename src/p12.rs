#[cfg(test)]
use std::iter;

pub fn solve(part2: bool) -> String {
    let _input = std::fs::read_to_string("input_10.txt").expect("could not read file");
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
        .split_once(':')
        .unwrap()
        .1
        .trim()
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

fn shape_fits(
    region_map: &mut Vec<Vec<bool>>,
    shape: &PresentShape,
    offset: (usize, usize),
    rot90: usize,
) -> bool {
    assert_eq!(rot90, 0);
    let fits = !region_map[offset.0..]
        .iter()
        .zip(shape.iter())
        .map(|(region_row, shape_row)| {
            region_row[offset.1..]
                .iter()
                .zip(shape_row.iter())
                .any(|(&r, &s)| r && s)
        })
        .any(|both| both);

    if fits {
        // insert_piece
        region_map //[offset.0..]
            .iter_mut()
            .zip(shape.iter())
            .for_each(|(region_row, shape_row)| {
                region_row[offset.1..]
                    .iter_mut()
                    .zip(shape_row.iter())
                    .for_each(|(r, &s)| *r = s)
            });
    }

    fits
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

#[test]
fn test_shape_fits_into_empty() {
    let (presents, _) = parse(EXAMPLE);
    let mut empty_region: Vec<Vec<_>> =
        iter::repeat_n(iter::repeat_n(false, 4).collect(), 4).collect();
    let fits = shape_fits(&mut empty_region, &presents[4], (0, 0), 0);
    assert!(fits);
}

#[test]
fn test_shape_fits_not_twice() {
    let (presents, _) = parse(EXAMPLE);
    let mut empty_region: Vec<Vec<_>> =
        iter::repeat_n(iter::repeat_n(false, 4).collect(), 4).collect();
    assert_eq!(empty_region[0][0], false);
    let fits = shape_fits(&mut empty_region, &presents[4], (0, 0), 0);
    assert!(fits);
    let fits_again = shape_fits(&mut empty_region, &presents[4], (0, 0), 0);
    assert_eq!(empty_region[0][0], true);
    assert!(!fits_again);
}
